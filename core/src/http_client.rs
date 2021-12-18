use bytes::Bytes;
use futures_util::future::IntoStream;
use futures_util::FutureExt;
use http::header::HeaderValue;
use http::uri::InvalidUri;
use hyper::client::{HttpConnector, ResponseFuture};
use hyper::header::USER_AGENT;
use hyper::{Body, Client, Request, Response, StatusCode};
use hyper_proxy::{Intercept, Proxy, ProxyConnector};
use hyper_rustls::HttpsConnector;
use rustls::{ClientConfig, RootCertStore};
use thiserror::Error;
use url::Url;

use std::env::consts::OS;

use crate::version::{
    FALLBACK_USER_AGENT, SPOTIFY_MOBILE_VERSION, SPOTIFY_VERSION, VERSION_STRING,
};

pub struct HttpClient {
    user_agent: HeaderValue,
    proxy: Option<Url>,
    tls_config: ClientConfig,
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

impl From<InvalidUri> for HttpClientError {
    fn from(err: InvalidUri) -> Self {
        Self::Parsing(err.into())
    }
}

impl HttpClient {
    pub fn new(proxy: Option<&Url>) -> Self {
        let spotify_version = match OS {
            "android" | "ios" => SPOTIFY_MOBILE_VERSION.to_owned(),
            _ => SPOTIFY_VERSION.to_string(),
        };

        let spotify_platform = match OS {
            "android" => "Android/31",
            "ios" => "iOS/15.1.1",
            "macos" => "OSX/0",
            "windows" => "Win32/0",
            _ => "Linux/0",
        };

        let user_agent_str = &format!(
            "Spotify/{} {} ({})",
            spotify_version, spotify_platform, VERSION_STRING
        );

        let user_agent = HeaderValue::from_str(user_agent_str).unwrap_or_else(|err| {
            error!("Invalid user agent <{}>: {}", user_agent_str, err);
            error!("Please report this as a bug.");
            HeaderValue::from_static(FALLBACK_USER_AGENT)
        });

        // configuring TLS is expensive and should be done once per process
        let root_store = match rustls_native_certs::load_native_certs() {
            Ok(store) => store,
            Err((Some(store), err)) => {
                warn!("Could not load all certificates: {:?}", err);
                store
            }
            Err((None, err)) => {
                error!("Cannot access native certificate store: {}", err);
                error!("Continuing, but most requests will probably fail until you fix your system certificate store.");
                RootCertStore::empty()
            }
        };

        let mut tls_config = ClientConfig::new();
        tls_config.root_store = root_store;
        tls_config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];

        Self {
            user_agent,
            proxy: proxy.cloned(),
            tls_config,
        }
    }

    pub async fn request(&self, req: Request<Body>) -> Result<Response<Body>, HttpClientError> {
        debug!("Requesting {:?}", req.uri().to_string());

        let request = self.request_fut(req)?;
        {
            let response = request.await;
            if let Ok(response) = &response {
                let status = response.status();
                if status != StatusCode::OK {
                    return Err(HttpClientError::NotOK(status.into()));
                }
            }
            response.map_err(HttpClientError::Response)
        }
    }

    pub async fn request_body(&self, req: Request<Body>) -> Result<Bytes, HttpClientError> {
        let response = self.request(req).await?;
        hyper::body::to_bytes(response.into_body())
            .await
            .map_err(HttpClientError::Response)
    }

    pub fn request_stream(
        &self,
        req: Request<Body>,
    ) -> Result<IntoStream<ResponseFuture>, HttpClientError> {
        Ok(self.request_fut(req)?.into_stream())
    }

    pub fn request_fut(&self, mut req: Request<Body>) -> Result<ResponseFuture, HttpClientError> {
        let mut http = HttpConnector::new();
        http.enforce_http(false);
        let connector = HttpsConnector::from((http, self.tls_config.clone()));

        let headers_mut = req.headers_mut();
        headers_mut.insert(USER_AGENT, self.user_agent.clone());

        let request = if let Some(url) = &self.proxy {
            let proxy_uri = url.to_string().parse()?;
            let proxy = Proxy::new(Intercept::All, proxy_uri);
            let proxy_connector = ProxyConnector::from_proxy(connector, proxy)?;

            Client::builder().build(proxy_connector).request(req)
        } else {
            Client::builder().build(connector).request(req)
        };

        Ok(request)
    }
}
