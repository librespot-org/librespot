use std::{
    collections::HashMap,
    env::consts::OS,
    time::{Duration, Instant},
};

use bytes::Bytes;
use futures_util::{future::IntoStream, FutureExt};
use governor::{
    clock::MonotonicClock, middleware::NoOpMiddleware, state::InMemoryState, Quota, RateLimiter,
};
use http::{header::HeaderValue, Uri};
use http_body_util::{BodyExt, Full};
use hyper::{body::Incoming, header::USER_AGENT, HeaderMap, Request, Response, StatusCode};
use hyper_proxy2::{Intercept, Proxy, ProxyConnector};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use hyper_util::{
    client::legacy::{connect::HttpConnector, Client, ResponseFuture},
    rt::TokioExecutor,
};
use nonzero_ext::nonzero;
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use sysinfo::System;
use thiserror::Error;
use url::Url;

use crate::{
    date::Date,
    version::{spotify_version, FALLBACK_USER_AGENT, VERSION_STRING},
    Error,
};

// The 30 seconds interval is documented by Spotify, but the calls per interval
// is a guesstimate and probably subject to licensing (purchasing extra calls)
// and may change at any time.
pub const RATE_LIMIT_INTERVAL: Duration = Duration::from_secs(30);
pub const RATE_LIMIT_MAX_WAIT: Duration = Duration::from_secs(10);
pub const RATE_LIMIT_CALLS_PER_INTERVAL: u32 = 300;

#[derive(Debug, Error)]
pub enum HttpClientError {
    #[error("Response status code: {0}")]
    StatusCode(hyper::StatusCode),
}

impl From<HttpClientError> for Error {
    fn from(err: HttpClientError) -> Self {
        match err {
            HttpClientError::StatusCode(code) => {
                // not exhaustive, but what reasonably could be expected
                match code {
                    StatusCode::GATEWAY_TIMEOUT | StatusCode::REQUEST_TIMEOUT => {
                        Error::deadline_exceeded(err)
                    }
                    StatusCode::GONE
                    | StatusCode::NOT_FOUND
                    | StatusCode::MOVED_PERMANENTLY
                    | StatusCode::PERMANENT_REDIRECT
                    | StatusCode::TEMPORARY_REDIRECT => Error::not_found(err),
                    StatusCode::FORBIDDEN | StatusCode::PAYMENT_REQUIRED => {
                        Error::permission_denied(err)
                    }
                    StatusCode::NETWORK_AUTHENTICATION_REQUIRED
                    | StatusCode::PROXY_AUTHENTICATION_REQUIRED
                    | StatusCode::UNAUTHORIZED => Error::unauthenticated(err),
                    StatusCode::EXPECTATION_FAILED
                    | StatusCode::PRECONDITION_FAILED
                    | StatusCode::PRECONDITION_REQUIRED => Error::failed_precondition(err),
                    StatusCode::RANGE_NOT_SATISFIABLE => Error::out_of_range(err),
                    StatusCode::INTERNAL_SERVER_ERROR
                    | StatusCode::MISDIRECTED_REQUEST
                    | StatusCode::SERVICE_UNAVAILABLE
                    | StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS => Error::unavailable(err),
                    StatusCode::BAD_REQUEST
                    | StatusCode::HTTP_VERSION_NOT_SUPPORTED
                    | StatusCode::LENGTH_REQUIRED
                    | StatusCode::METHOD_NOT_ALLOWED
                    | StatusCode::NOT_ACCEPTABLE
                    | StatusCode::PAYLOAD_TOO_LARGE
                    | StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE
                    | StatusCode::UNSUPPORTED_MEDIA_TYPE
                    | StatusCode::URI_TOO_LONG => Error::invalid_argument(err),
                    StatusCode::TOO_MANY_REQUESTS => Error::resource_exhausted(err),
                    StatusCode::NOT_IMPLEMENTED => Error::unimplemented(err),
                    _ => Error::unknown(err),
                }
            }
        }
    }
}

type HyperClient = Client<ProxyConnector<HttpsConnector<HttpConnector>>, Full<bytes::Bytes>>;

pub struct HttpClient {
    user_agent: HeaderValue,
    proxy_url: Option<Url>,
    hyper_client: OnceCell<HyperClient>,

    // while the DashMap variant is more performant, our level of concurrency
    // is pretty low so we can save pulling in that extra dependency
    rate_limiter:
        RateLimiter<String, Mutex<HashMap<String, InMemoryState>>, MonotonicClock, NoOpMiddleware>,
}

impl HttpClient {
    pub fn new(proxy_url: Option<&Url>) -> Self {
        let zero_str = String::from("0");
        let os_version = System::os_version().unwrap_or_else(|| zero_str.clone());

        let (spotify_platform, os_version) = match OS {
            "android" => ("Android", os_version),
            "ios" => ("iOS", os_version),
            "macos" => ("OSX", zero_str),
            "windows" => ("Win32", zero_str),
            _ => ("Linux", zero_str),
        };

        let user_agent_str = &format!(
            "Spotify/{} {}/{} ({})",
            spotify_version(),
            spotify_platform,
            os_version,
            VERSION_STRING
        );

        let user_agent = HeaderValue::from_str(user_agent_str).unwrap_or_else(|err| {
            error!("Invalid user agent <{}>: {}", user_agent_str, err);
            HeaderValue::from_static(FALLBACK_USER_AGENT)
        });

        let replenish_interval_ns =
            RATE_LIMIT_INTERVAL.as_nanos() / RATE_LIMIT_CALLS_PER_INTERVAL as u128;
        let quota = Quota::with_period(Duration::from_nanos(replenish_interval_ns as u64))
            .expect("replenish interval should be valid")
            .allow_burst(nonzero![RATE_LIMIT_CALLS_PER_INTERVAL]);
        let rate_limiter = RateLimiter::keyed(quota);

        Self {
            user_agent,
            proxy_url: proxy_url.cloned(),
            hyper_client: OnceCell::new(),
            rate_limiter,
        }
    }

    fn try_create_hyper_client(proxy_url: Option<&Url>) -> Result<HyperClient, Error> {
        // configuring TLS is expensive and should be done once per process
        let https_connector = HttpsConnectorBuilder::new()
            .with_native_roots()?
            .https_or_http()
            .enable_http1()
            .enable_http2()
            .build();

        // When not using a proxy a dummy proxy is configured that will not intercept any traffic.
        // This prevents needing to carry the Client Connector generics through the whole project
        let proxy = match &proxy_url {
            Some(proxy_url) => Proxy::new(Intercept::All, proxy_url.to_string().parse()?),
            None => Proxy::new(Intercept::None, Uri::from_static("0.0.0.0")),
        };
        let proxy_connector = ProxyConnector::from_proxy(https_connector, proxy)?;

        let client = Client::builder(TokioExecutor::new())
            .http2_adaptive_window(true)
            .build(proxy_connector);
        Ok(client)
    }

    fn hyper_client(&self) -> Result<&HyperClient, Error> {
        self.hyper_client
            .get_or_try_init(|| Self::try_create_hyper_client(self.proxy_url.as_ref()))
    }

    pub async fn request(&self, req: Request<Bytes>) -> Result<Response<Incoming>, Error> {
        debug!("Requesting {}", req.uri().to_string());

        // `Request` does not implement `Clone` because its `Body` may be a single-shot stream.
        // As correct as that may be technically, we now need all this boilerplate to clone it
        // ourselves, as any `Request` is moved in the loop.
        let (parts, body_as_bytes) = req.into_parts();

        loop {
            let mut req = Request::builder()
                .method(parts.method.clone())
                .uri(parts.uri.clone())
                .version(parts.version)
                .body(body_as_bytes.clone())?;
            *req.headers_mut() = parts.headers.clone();

            let request = self.request_fut(req)?;
            let response = request.await;

            if let Ok(response) = &response {
                let code = response.status();

                if code == StatusCode::TOO_MANY_REQUESTS {
                    if let Some(duration) = Self::get_retry_after(response.headers()) {
                        warn!(
                            "Rate limited by service, retrying in {} seconds...",
                            duration.as_secs()
                        );
                        tokio::time::sleep(duration).await;
                        continue;
                    }
                }

                if code != StatusCode::OK {
                    return Err(HttpClientError::StatusCode(code).into());
                }
            }

            let response = response?;
            return Ok(response);
        }
    }

    pub async fn request_body(&self, req: Request<Bytes>) -> Result<Bytes, Error> {
        let response = self.request(req).await?;
        Ok(response.into_body().collect().await?.to_bytes())
    }

    pub fn request_stream(&self, req: Request<Bytes>) -> Result<IntoStream<ResponseFuture>, Error> {
        Ok(self.request_fut(req)?.into_stream())
    }

    pub fn request_fut(&self, mut req: Request<Bytes>) -> Result<ResponseFuture, Error> {
        let headers_mut = req.headers_mut();
        headers_mut.insert(USER_AGENT, self.user_agent.clone());

        // For rate limiting we cannot *just* depend on Spotify sending us HTTP/429
        // Retry-After headers. For example, when there is a service interruption
        // and HTTP/500 is returned, we don't want to DoS the Spotify infrastructure.
        let domain = match req.uri().host() {
            Some(host) => {
                // strip the prefix from *.domain.tld (assume rate limit is per domain, not subdomain)
                let mut parts = host
                    .split('.')
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                let n = parts.len().saturating_sub(2);
                parts.drain(n..).collect()
            }
            None => String::from(""),
        };
        self.rate_limiter.check_key(&domain).map_err(|e| {
            Error::resource_exhausted(format!(
                "rate limited for at least another {} seconds",
                e.wait_time_from(Instant::now()).as_secs()
            ))
        })?;

        Ok(self.hyper_client()?.request(req.map(Full::new)))
    }

    pub fn get_retry_after(headers: &HeaderMap<HeaderValue>) -> Option<Duration> {
        let now = Date::now_utc().as_timestamp_ms();

        let mut retry_after_ms = None;
        if let Some(header_val) = headers.get("X-RateLimit-Next") {
            // *.akamaized.net (Akamai)
            if let Ok(date_str) = header_val.to_str() {
                if let Ok(target) = Date::from_iso8601(date_str) {
                    retry_after_ms = Some(target.as_timestamp_ms().saturating_sub(now))
                }
            }
        } else if let Some(header_val) = headers.get("Fastly-RateLimit-Reset") {
            // *.scdn.co (Fastly)
            if let Ok(timestamp) = header_val.to_str() {
                if let Ok(target) = timestamp.parse::<i64>() {
                    retry_after_ms = Some(target.saturating_sub(now))
                }
            }
        } else if let Some(header_val) = headers.get("Retry-After") {
            // Generic RFC compliant (including *.spotify.com)
            if let Ok(retry_after) = header_val.to_str() {
                if let Ok(duration) = retry_after.parse::<i64>() {
                    retry_after_ms = Some(duration * 1000)
                }
            }
        }

        if let Some(retry_after) = retry_after_ms {
            let duration = Duration::from_millis(retry_after as u64);
            if duration <= RATE_LIMIT_MAX_WAIT {
                return Some(duration);
            } else {
                debug!(
                    "Waiting {} seconds would exceed {} second limit",
                    duration.as_secs(),
                    RATE_LIMIT_MAX_WAIT.as_secs()
                );
            }
        }

        None
    }
}
