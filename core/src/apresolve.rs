const AP_FALLBACK: &'static str = "ap.spotify.com:443";
const APRESOLVE_ENDPOINT: &'static str = "http://apresolve.spotify.com/";

use hyper::client::HttpConnector;
use hyper::{self, Body, Client, Request, Uri};
use hyper_proxy::{Intercept, Proxy, ProxyConnector};
use serde_json;
use std::error;
use std::str::FromStr;
use url::Url;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct APResolveData {
    ap_list: Vec<String>,
}
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

async fn apresolve(proxy: &Option<Url>, ap_port: &Option<u16>) -> Result<String> {
    let url = Uri::from_str(APRESOLVE_ENDPOINT)?; //.expect("invalid AP resolve URL");
    let use_proxy = proxy.is_some();

    let mut req = Request::get(&url).body(Body::empty())?;
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
            client.request(req)
        }
        _ => {
            let client = Client::new();
            client.request(req)
        }
    }
    .await?;

    let body = hyper::body::to_bytes(response.into_body()).await?;
    let body = String::from_utf8(body.to_vec())?;
    let data = serde_json::from_str::<APResolveData>(&body)?;

    let ap = {
        let mut aps = data.ap_list.iter().filter(|ap| {
            if let Some(p) = ap_port {
                Uri::from_str(ap)
                    .ok()
                    .map_or(false, |uri| uri.port_u16().map_or(false, |port| &port == p))
            } else if use_proxy {
                // It is unlikely that the proxy will accept CONNECT on anything other than 443.
                Uri::from_str(ap).ok().map_or(false, |uri| {
                    uri.port_u16().map_or(false, |port| port == 443)
                })
            } else {
                true
            }
        });

        let ap = aps.next().ok_or("empty AP List")?;
        Ok(ap.clone())
    };

    ap
}

pub(crate) async fn apresolve_or_fallback<E>(
    proxy: &Option<Url>,
    ap_port: &Option<u16>,
) -> Result<String> {
    // match apresolve.await {
    //     Ok(ap)
    // }
    let ap = apresolve(proxy, ap_port).await.or_else(|e| {
        warn!("Failed to resolve Access Point: {:?}", e);
        warn!("Using fallback \"{}\"", AP_FALLBACK);
        Ok(AP_FALLBACK.into())
    });

    ap
}
