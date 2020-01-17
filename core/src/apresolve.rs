const AP_FALLBACK: &'static str = "ap.spotify.com:443";
const APRESOLVE_ENDPOINT: &'static str = "http://apresolve.spotify.com/";

use futures::TryStreamExt;
use futures::{Future, Stream};
use http::Uri;
use hyper::client::HttpConnector;
use hyper::{self, Client, Method, Request, Response};
use hyper_proxy::{Intercept, Proxy, ProxyConnector};
use serde_json;
use std::str::FromStr;
use url::Url;

error_chain! {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct APResolveData {
    ap_list: Vec<String>,
}

async fn apresolve(
    proxy: &Option<Url>,
    ap_port: &Option<u16>,
) -> std::result::Result<String, Error> {
    let url = Uri::from_str(APRESOLVE_ENDPOINT).expect("invalid AP resolve URL");
    let use_proxy = proxy.is_some();

    let mut req = Request::get(url.clone())
        .body(hyper::Body::empty())
        .chain_err(|| "invalid http request arguments")?;
    let response = match *proxy {
        Some(ref val) => {
            let proxy_url = Uri::from_str(val.as_str()).expect("invalid http proxy");
            let proxy = Proxy::new(Intercept::All, proxy_url);
            let connector = HttpConnector::new();
            let proxy_connector = ProxyConnector::from_proxy_unsecured(connector, proxy);
            if let Some(headers) = proxy_connector.http_headers(&url) {
                req.headers_mut().extend(headers.clone().into_iter());
            }
            let client = Client::builder().build(proxy_connector);
            client
                .request(req)
                .await
                .chain_err(|| "HTTP request error")?
        }
        _ => {
            let client = Client::new();
            client
                .request(req)
                .await
                .chain_err(|| "HTTP request error")?
        }
    };

    let body = hyper::body::to_bytes(response.into_body())
        .await
        .chain_err(|| "HTTP body error")?
        .to_vec();
    let body = String::from_utf8(body).chain_err(|| "invalid UTF8 in response")?;
    let data = serde_json::from_str::<APResolveData>(&body).chain_err(|| "invalid JSON")?;

    let p = ap_port.clone();

    let mut aps = data.ap_list.iter().filter(|ap| {
        if p.is_some() {
            Uri::from_str(ap).ok().map_or(false, |uri| {
                uri.port().map_or(false, |port| port == p.unwrap())
            })
        } else if use_proxy {
            // It is unlikely that the proxy will accept CONNECT on anything other than 443.
            Uri::from_str(ap)
                .ok()
                .map_or(false, |uri| uri.port().map_or(false, |port| port == 443))
        } else {
            true
        }
    });

    let ap = aps.next().ok_or("empty AP List")?;

    Ok(*ap)
}

pub(crate) async fn apresolve_or_fallback<E>(
    proxy: &Option<Url>,
    ap_port: &Option<u16>,
) -> std::result::Result<String, E>
where
    E: 'static,
{
    let ap = apresolve(proxy, ap_port).await.or_else(|e| {
        warn!("Failed to resolve Access Point: {}", e.description());
        warn!("Using fallback \"{}\"", AP_FALLBACK);
        Ok(AP_FALLBACK.into())
    })?;

    Ok(ap)
}
