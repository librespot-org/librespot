use hyper::client::HttpConnector;
use hyper::{Body, Client, Request, Response};
use hyper_proxy::{Intercept, Proxy, ProxyConnector};
use url::Url;

pub struct HttpClient {
    proxy: Option<Url>,
}

impl HttpClient {
    pub fn new(proxy: Option<&Url>) -> Self {
        Self {
            proxy: proxy.cloned(),
        }
    }

    pub async fn request(&self, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        if let Some(url) = &self.proxy {
            // Panic safety: all URLs are valid URIs
            let uri = url.to_string().parse().unwrap();
            let proxy = Proxy::new(Intercept::All, uri);
            let connector = HttpConnector::new();
            let proxy_connector = ProxyConnector::from_proxy_unsecured(connector, proxy);
            Client::builder().build(proxy_connector).request(req).await
        } else {
            Client::new().request(req).await
        }
    }

    pub async fn request_body(&self, req: Request<Body>) -> Result<bytes::Bytes, hyper::Error> {
        let response = self.request(req).await?;
        hyper::body::to_bytes(response.into_body()).await
    }
}
