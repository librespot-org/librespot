const AP_FALLBACK: &'static str = "ap.spotify.com:443";
const APRESOLVE_ENDPOINT: &'static str = "http://apresolve.spotify.com/";

use futures::{Future, Stream};
use hyper::client::HttpConnector;
use hyper::{self, Client, Request, Uri};
use hyper_proxy::{Intercept, Proxy, ProxyConnector};
use serde_json;
use std::str::FromStr;
use url::Url;

error_chain! {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct APResolveData {
    ap_list: Vec<String>,
}

fn apresolve(
    proxy: &Option<Url>,
    ap_port: &Option<u16>,
) -> Box<dyn Future<Item = String, Error = Error>> {
    let url = Uri::from_str(APRESOLVE_ENDPOINT).expect("invalid AP resolve URL");
    let use_proxy = proxy.is_some();

    // let mut req = Request::new(url.clone());
    let mut req = Request::get(url.clone())
        .body(hyper::Body::from(vec![]))
        .unwrap();
    let response = match *proxy {
        Some(ref val) => {
            let proxy_url = Uri::from_str(val.as_str()).expect("invalid http proxy");
            let proxy = Proxy::new(Intercept::All, proxy_url);
            let connector = HttpConnector::new(4);
            let proxy_connector = ProxyConnector::from_proxy_unsecured(connector, proxy);
            if let Some(headers) = proxy_connector.http_headers(&url) {
                req.headers_mut().extend(headers.clone().into_iter());
                // req.set_proxy(true);
            }
            let client = Client::builder().build(proxy_connector);
            client.request(req)
        }
        _ => {
            let client = Client::new();
            client.request(req)
        }
    };

    let body = response.and_then(|response| {
        response.into_body().fold(Vec::new(), |mut acc, chunk| {
            acc.extend_from_slice(chunk.as_ref());
            Ok::<_, hyper::Error>(acc)
        })
    });
    let body = body.then(|result| result.chain_err(|| "HTTP error"));
    let body =
        body.and_then(|body| String::from_utf8(body).chain_err(|| "invalid UTF8 in response"));

    let data = body
        .and_then(|body| serde_json::from_str::<APResolveData>(&body).chain_err(|| "invalid JSON"));

    let p = ap_port.clone();

    let ap = data.and_then(move |data| {
        let mut aps = data.ap_list.iter().filter(|ap| {
            if p.is_some() {
                Uri::from_str(ap).ok().map_or(false, |uri| {
                    uri.port_u16().map_or(false, |port| port == p.unwrap())
                })
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
    });

    Box::new(ap)
}

pub(crate) fn apresolve_or_fallback<E>(
    proxy: &Option<Url>,
    ap_port: &Option<u16>,
) -> Box<dyn Future<Item = String, Error = E>>
where
    E: 'static,
{
    let ap = apresolve(proxy, ap_port).or_else(|e| {
        warn!("Failed to resolve Access Point: {}", e.description());
        warn!("Using fallback \"{}\"", AP_FALLBACK);
        Ok(AP_FALLBACK.into())
    });

    Box::new(ap)
}
