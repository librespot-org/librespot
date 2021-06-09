use std::error::Error;

use hyper::client::HttpConnector;
use hyper::{Body, Client, Request};
use hyper_proxy::{Intercept, Proxy, ProxyConnector};
use serde::Deserialize;
use url::Url;

const APRESOLVE_ENDPOINT: &str =
    "http://apresolve.spotify.com/?type=accesspoint&type=dealer&type=spclient";

// These addresses probably do some geo-location based traffic management or at least DNS-based
// load balancing. They are known to fail when the normal resolvers are up, so that's why they
// should only be used as fallback.
const AP_FALLBACK: &str = "ap.spotify.com";
const DEALER_FALLBACK: &str = "dealer.spotify.com";
const SPCLIENT_FALLBACK: &str = "spclient.wg.spotify.com";

const FALLBACK_PORT: u16 = 443;

pub type SocketAddress = (String, u16);

#[derive(Clone, Debug, Deserialize)]
struct ApResolveData {
    accesspoint: Vec<String>,
    dealer: Vec<String>,
    spclient: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AccessPoints {
    pub accesspoint: SocketAddress,
    pub dealer: SocketAddress,
    pub spclient: SocketAddress,
}

fn select_ap(data: Vec<String>, ap_port: Option<u16>) -> Result<SocketAddress, Box<dyn Error>> {
    let port = ap_port.unwrap_or(FALLBACK_PORT);

    let mut aps = data.into_iter().filter_map(|ap| {
        let mut split = ap.rsplitn(2, ':');
        let port = split
            .next()
            .expect("rsplitn should not return empty iterator");
        let host = split.next()?.to_owned();
        let port: u16 = port.parse().ok()?;
        Some((host, port))
    });

    let ap = if ap_port.is_some() {
        aps.find(|(_, p)| *p == port)
    } else {
        aps.next()
    }
    .ok_or("no valid AP in list")?;

    Ok(ap)
}

async fn try_apresolve(proxy: Option<&Url>) -> Result<ApResolveData, Box<dyn Error>> {
    let req = Request::builder()
        .method("GET")
        .uri(APRESOLVE_ENDPOINT)
        .body(Body::empty())
        .unwrap();

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

    Ok(data)
}

pub async fn apresolve(proxy: Option<&Url>, ap_port: Option<u16>) -> AccessPoints {
    let data = try_apresolve(proxy).await.unwrap_or_else(|e| {
        warn!("Failed to resolve access points: {}, using fallbacks.", e);
        ApResolveData {
            accesspoint: vec![],
            dealer: vec![],
            spclient: vec![],
        }
    });

    let accesspoint = select_ap(data.accesspoint, ap_port)
        .unwrap_or_else(|_| (String::from(AP_FALLBACK), FALLBACK_PORT));
    let dealer = select_ap(data.dealer, ap_port)
        .unwrap_or_else(|_| (String::from(DEALER_FALLBACK), FALLBACK_PORT));
    let spclient = select_ap(data.spclient, ap_port)
        .unwrap_or_else(|_| (String::from(SPCLIENT_FALLBACK), FALLBACK_PORT));

    AccessPoints {
        accesspoint,
        dealer,
        spclient,
    }
}

#[cfg(test)]
mod test {
    use std::net::ToSocketAddrs;

    use super::apresolve;

    #[tokio::test]
    async fn test_apresolve() {
        let aps = apresolve(None, None).await;

        // Assert that the result contains a valid host and port
        aps.accesspoint.to_socket_addrs().unwrap().next().unwrap();
        aps.dealer.to_socket_addrs().unwrap().next().unwrap();
        aps.spclient.to_socket_addrs().unwrap().next().unwrap();
    }

    #[tokio::test]
    async fn test_apresolve_port_443() {
        let aps = apresolve(None, Some(443)).await;

        let port = aps
            .accesspoint
            .to_socket_addrs()
            .unwrap()
            .next()
            .unwrap()
            .port();
        assert_eq!(port, 443);
    }
}
