use std::env::consts::OS;

use bytes::Bytes;
use futures_util::{future::IntoStream, FutureExt};
use http::{header::HeaderValue, Uri};
use hyper::{
    client::{HttpConnector, ResponseFuture},
    header::USER_AGENT,
    Body, Client, Request, Response, StatusCode,
};
use hyper_proxy::{Intercept, Proxy, ProxyConnector};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use once_cell::sync::OnceCell;
use thiserror::Error;
use url::Url;

use crate::{
    version::{spotify_version, FALLBACK_USER_AGENT, VERSION_STRING},
    Error,
};

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

#[derive(Clone)]
pub struct HttpClient {
    user_agent: HeaderValue,
    proxy_url: Option<Url>,
    hyper_client: OnceCell<HyperClient>,
}

impl HttpClient {
    pub fn new(proxy_url: Option<&Url>) -> Self {
        let spotify_platform = match OS {
            "android" => "Android/31",
            "ios" => "iOS/15.2.1",
            "macos" => "OSX/0",
            "windows" => "Win32/0",
            _ => "Linux/0",
        };

        let user_agent_str = &format!(
            "Spotify/{} {} ({})",
            spotify_version(),
            spotify_platform,
            VERSION_STRING
        );

        let user_agent = HeaderValue::from_str(user_agent_str).unwrap_or_else(|err| {
            error!("Invalid user agent <{}>: {}", user_agent_str, err);
            HeaderValue::from_static(FALLBACK_USER_AGENT)
        });

        Self {
            user_agent,
            proxy_url: proxy_url.cloned(),
            hyper_client: OnceCell::new(),
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

        let request = self.request_fut(req)?;
        let response = request.await;

        if let Ok(response) = &response {
            let code = response.status();
            if code != StatusCode::OK {
                return Err(HttpClientError::StatusCode(code).into());
            }
        }

        Ok(response?)
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
