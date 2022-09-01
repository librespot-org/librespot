use std::{env::consts::OS, time::Duration};

use bytes::Bytes;
use futures_util::{future::IntoStream, FutureExt};
use governor::{
    clock::MonotonicClock,
    middleware::NoOpMiddleware,
    state::{InMemoryState, NotKeyed},
    Jitter, Quota, RateLimiter,
};
use http::{header::HeaderValue, Uri};
use hyper::{
    client::{HttpConnector, ResponseFuture},
    header::USER_AGENT,
    Body, Client, Request, Response, StatusCode,
};
use hyper_proxy::{Intercept, Proxy, ProxyConnector};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use nonzero_ext::nonzero;
use once_cell::sync::OnceCell;
use sysinfo::{System, SystemExt};
use thiserror::Error;
use url::Url;

use crate::{
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

type HyperClient = Client<ProxyConnector<HttpsConnector<HttpConnector>>, Body>;

pub struct HttpClient {
    user_agent: HeaderValue,
    proxy_url: Option<Url>,
    hyper_client: OnceCell<HyperClient>,
    rate_limiter: RateLimiter<NotKeyed, InMemoryState, MonotonicClock, NoOpMiddleware>,
}

impl HttpClient {
    pub fn new(proxy_url: Option<&Url>) -> Self {
        let zero_str = String::from("0");
        let os_version = System::new()
            .os_version()
            .unwrap_or_else(|| zero_str.clone());

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
        let rate_limiter = RateLimiter::direct(quota);

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
            .with_native_roots()
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

        let client = Client::builder()
            .http2_adaptive_window(true)
            .build(proxy_connector);
        Ok(client)
    }

    fn hyper_client(&self) -> Result<&HyperClient, Error> {
        self.hyper_client
            .get_or_try_init(|| Self::try_create_hyper_client(self.proxy_url.as_ref()))
    }

    pub async fn request(&self, req: Request<Body>) -> Result<Response<Body>, Error> {
        debug!("Requesting {}", req.uri().to_string());

        // `Request` does not implement `Clone` because its `Body` may be a single-shot stream.
        // As correct as that may be technically, we now need all this boilerplate to clone it
        // ourselves, as any `Request` is moved in the loop.
        let (parts, body) = req.into_parts();
        let body_as_bytes = hyper::body::to_bytes(body)
            .await
            .unwrap_or_else(|_| Bytes::new());

        loop {
            let mut req = Request::new(Body::from(body_as_bytes.clone()));
            *req.method_mut() = parts.method.clone();
            *req.uri_mut() = parts.uri.clone();
            *req.version_mut() = parts.version;
            *req.headers_mut() = parts.headers.clone();

            // For rate limiting we cannot *just* depend on Spotify sending us HTTP/429
            // Retry-After headers. For example, when there is a service interruption
            // and HTTP/500 is returned, we don't want to DoS the Spotify infrastructure.
            self.rate_limiter
                .until_ready_with_jitter(Jitter::up_to(Duration::from_secs(5)))
                .await;

            let request = self.request_fut(req)?;
            let response = request.await;

            if let Ok(response) = &response {
                let code = response.status();

                if code == StatusCode::TOO_MANY_REQUESTS {
                    if let Some(retry_after) = response.headers().get("Retry-After") {
                        if let Ok(retry_after_str) = retry_after.to_str() {
                            if let Ok(retry_after_secs) = retry_after_str.parse::<u64>() {
                                let duration = Duration::from_secs(retry_after_secs);
                                if duration <= RATE_LIMIT_MAX_WAIT {
                                    warn!(
                                        "Rate limiting, retrying in {} seconds...",
                                        retry_after_secs
                                    );
                                    tokio::time::sleep(duration).await;
                                    continue;
                                } else {
                                    debug!("Not going to wait {} seconds", retry_after_secs);
                                }
                            }
                        }
                    }
                }

                if code != StatusCode::OK {
                    return Err(HttpClientError::StatusCode(code).into());
                }
            }

            return Ok(response?);
        }
    }

    pub async fn request_body(&self, req: Request<Body>) -> Result<Bytes, Error> {
        let response = self.request(req).await?;
        Ok(hyper::body::to_bytes(response.into_body()).await?)
    }

    pub fn request_stream(&self, req: Request<Body>) -> Result<IntoStream<ResponseFuture>, Error> {
        Ok(self.request_fut(req)?.into_stream())
    }

    pub fn request_fut(&self, mut req: Request<Body>) -> Result<ResponseFuture, Error> {
        let headers_mut = req.headers_mut();
        headers_mut.insert(USER_AGENT, self.user_agent.clone());

        let request = self.hyper_client()?.request(req);
        Ok(request)
    }
}
