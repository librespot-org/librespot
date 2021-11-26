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
    Parsing(#[from] http::uri::InvalidUri),
    #[error("could not send request: {0}")]
    Request(hyper::Error),
    #[error("could not read response: {0}")]
    Response(hyper::Error),
    #[error("could not build proxy connector: {0}")]
    ProxyBuilder(#[from] std::io::Error),
}

impl HttpClient {
    pub fn new(proxy: Option<&Url>) -> Self {
        Self {
            proxy: proxy.cloned(),
        }
    }

    pub async fn request(&self, req: Request<Body>) -> Result<Response<Body>, HttpClientError> {
        let connector = HttpsConnector::with_native_roots();
        let uri = req.uri().clone();

        let response = if let Some(url) = &self.proxy {
            let uri = url.to_string().parse()?;
            let proxy = Proxy::new(Intercept::All, uri);
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
            if response.status() != StatusCode::OK {
                debug!("{} returned status {}", uri, response.status());
            }
        }

        response
    }

    pub async fn request_body(&self, req: Request<Body>) -> Result<bytes::Bytes, HttpClientError> {
        let response = self.request(req).await?;
        hyper::body::to_bytes(response.into_body())
            .await
            .map_err(HttpClientError::Response)
    }
}
