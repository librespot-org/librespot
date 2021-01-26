const AP_FALLBACK: &'static str = "ap.spotify.com:443";
const APRESOLVE_ENDPOINT: &'static str = "http://apresolve.spotify.com/";

use hyper::{Body, Client, Method, Request, Uri};
use std::error::Error;
use url::Url;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct APResolveData {
    ap_list: Vec<String>,
}

async fn apresolve(proxy: &Option<Url>, ap_port: &Option<u16>) -> Result<String, Box<dyn Error>> {
    let port = ap_port.unwrap_or(443);

    let req = Request::builder()
        .method(Method::GET)
        .uri(
            APRESOLVE_ENDPOINT
                .parse::<Uri>()
                .expect("invalid AP resolve URL"),
        )
        .body(Body::empty())?;

    let client = if proxy.is_some() {
        todo!("proxies not yet supported")
        /*let proxy = {
            let proxy_url = val.as_str().parse().expect("invalid http proxy");
            let mut proxy = Proxy::new(Intercept::All, proxy_url);
            let connector = HttpConnector::new();
            let proxy_connector = ProxyConnector::from_proxy_unsecured(connector, proxy);
            proxy_connector
        };

        if let Some(headers) = proxy.http_headers(&APRESOLVE_ENDPOINT.parse().unwrap()) {
            req.headers_mut().extend(headers.clone());
        };
        Client::builder().build(proxy)*/
    } else {
        Client::new()
    };

    let response = client.request(req).await?;

    let body = hyper::body::to_bytes(response.into_body()).await?;
    let data: APResolveData = serde_json::from_slice(body.as_ref())?;

    let ap = if ap_port.is_some() || proxy.is_some() {
        data.ap_list.into_iter().find_map(|ap| {
            if ap.parse::<Uri>().ok()?.port()? == port {
                Some(ap)
            } else {
                None
            }
        })
    } else {
        data.ap_list.into_iter().next()
    }
    .ok_or("empty AP List")?;
    Ok(ap)
}

pub async fn apresolve_or_fallback(proxy: &Option<Url>, ap_port: &Option<u16>) -> String {
    apresolve(proxy, ap_port).await.unwrap_or_else(|e| {
        warn!("Failed to resolve Access Point: {}", e);
        warn!("Using fallback \"{}\"", AP_FALLBACK);
        AP_FALLBACK.into()
    })
}
