use bytes::Bytes;
use http::header::HeaderValue;
use http::uri::InvalidUri;
use hyper::header::InvalidHeaderValue;
use hyper::{Body, Client, Request, Response, StatusCode};
use hyper_proxy::{Intercept, Proxy, ProxyConnector};
use hyper_rustls::HttpsConnector;
use thiserror::Error;
use url::Url;

pub struct HttpClient {
    proxy: Option<Url>,
}

#[derive(Error, Debug)]
pub enum HttpClientError {
    #[error("could not parse request: {0}")]
    Parsing(#[from] http::Error),
    #[error("could not send request: {0}")]
    Request(hyper::Error),
    #[error("could not read response: {0}")]
    Response(hyper::Error),
    #[error("status code: {0}")]
    NotOK(u16),
    #[error("could not build proxy connector: {0}")]
    ProxyBuilder(#[from] std::io::Error),
}

impl From<InvalidHeaderValue> for HttpClientError {
    fn from(err: InvalidHeaderValue) -> Self {
        Self::Parsing(err.into())
    }
}

impl From<InvalidUri> for HttpClientError {
    fn from(err: InvalidUri) -> Self {
        Self::Parsing(err.into())
    }
}

impl HttpClient {
    pub fn new(proxy: Option<&Url>) -> Self {
        Self {
            proxy: proxy.cloned(),
        }
    }

    pub async fn request(&self, mut req: Request<Body>) -> Result<Response<Body>, HttpClientError> {
        trace!("Requesting {:?}", req.uri().to_string());

        let connector = HttpsConnector::with_native_roots();

        let headers_mut = req.headers_mut();
        headers_mut.insert(
            "User-Agent",
            // Some features like lyrics are version-gated and require an official version string.
            HeaderValue::from_str("Spotify/8.6.80 iOS/13.5 (iPhone11,2)")?,
        );

        let response = if let Some(url) = &self.proxy {
            let proxy_uri = url.to_string().parse()?;
            let proxy = Proxy::new(Intercept::All, proxy_uri);
            let proxy_connector = ProxyConnector::from_proxy(connector, proxy)?;

            Client::builder()
                .build(proxy_connector)
                .request(req)
                .await
                .map_err(HttpClientError::Request)
        } else {
            Client::builder()
                .build(connector)
                .request(req)
                .await
                .map_err(HttpClientError::Request)
        };

        if let Ok(response) = &response {
            let status = response.status();
            if status != StatusCode::OK {
                return Err(HttpClientError::NotOK(status.into()));
            }
        }

        response
    }

    pub async fn request_body(&self, req: Request<Body>) -> Result<Bytes, HttpClientError> {
        let response = self.request(req).await?;
        hyper::body::to_bytes(response.into_body())
            .await
            .map_err(HttpClientError::Response)
    }
}
