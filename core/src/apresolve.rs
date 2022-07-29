use std::error::Error;

use hyper::client::HttpConnector;
use hyper::{Body, Client, Method, Request, Uri};
use hyper_proxy::{Intercept, Proxy, ProxyConnector};
use serde::Deserialize;
use url::Url;

const APRESOLVE_ENDPOINT: &str = "http://apresolve.spotify.com:80";
const AP_FALLBACK: &str = "ap.spotify.com:443";
const AP_BLACKLIST: [&str; 2] = ["ap-gew4.spotify.com", "ap-gue1.spotify.com"];

#[derive(Clone, Debug, Deserialize)]
struct ApResolveData {
    ap_list: Vec<String>,
}

async fn try_apresolve(
    proxy: Option<&Url>,
    ap_port: Option<u16>,
) -> Result<String, Box<dyn Error>> {
    let port = ap_port.unwrap_or(443);

    let mut req = Request::new(Body::empty());
    *req.method_mut() = Method::GET;
    // panic safety: APRESOLVE_ENDPOINT above is valid url.
    *req.uri_mut() = APRESOLVE_ENDPOINT.parse().expect("invalid AP resolve URL");

    let response = if let Some(url) = proxy {
        // Panic safety: all URLs are valid URIs
        let uri = url.to_string().parse().unwrap();
        let proxy = Proxy::new(Intercept::All, uri);
        let connector = HttpConnector::new();
        let proxy_connector = ProxyConnector::from_proxy_unsecured(connector, proxy);
        Client::builder()
            .build(proxy_connector)
            .request(req)
            .await?
    } else {
        Client::new().request(req).await?
    };

    let body = hyper::body::to_bytes(response.into_body()).await?;
    let data: ApResolveData = serde_json::from_slice(body.as_ref())?;

    // filter APs that are known to cause channel errors
    let aps: Vec<String> = data
        .ap_list
        .into_iter()
        .filter_map(|ap| {
            let host = ap.parse::<Uri>().ok()?.host()?.to_owned();
            if !AP_BLACKLIST.iter().any(|&blacklisted| host == blacklisted) {
                Some(ap)
            } else {
                warn!("Ignoring blacklisted access point {}", ap);
                None
            }
        })
        .collect();

    let ap = if ap_port.is_some() || proxy.is_some() {
        // filter on ports if specified on the command line...
        aps.into_iter().find_map(|ap| {
            if ap.parse::<Uri>().ok()?.port()? == port {
                Some(ap)
            } else {
                None
            }
        })
    } else {
        // ...or pick the first on the list
        aps.into_iter().next()
    }
    .ok_or("Unable to resolve any viable access points.")?;

    Ok(ap)
}

pub async fn apresolve(proxy: Option<&Url>, ap_port: Option<u16>) -> String {
    try_apresolve(proxy, ap_port).await.unwrap_or_else(|e| {
        warn!("Failed to resolve Access Point: {}", e);
        warn!("Using fallback \"{}\"", AP_FALLBACK);
        AP_FALLBACK.into()
    })
}

#[cfg(test)]
mod test {
    use std::net::ToSocketAddrs;

    use super::try_apresolve;

    #[tokio::test]
    async fn test_apresolve() {
        let ap = try_apresolve(None, None).await.unwrap();

        // Assert that the result contains a valid host and port
        ap.to_socket_addrs().unwrap().next().unwrap();
    }

    #[tokio::test]
    async fn test_apresolve_port_443() {
        let ap = try_apresolve(None, Some(443)).await.unwrap();

        let port = ap.to_socket_addrs().unwrap().next().unwrap().port();
        assert_eq!(port, 443);
    }
}
