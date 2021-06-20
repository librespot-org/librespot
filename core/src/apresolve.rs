use crate::http_client::HttpClient;
use hyper::{Body, Request};
use serde::Deserialize;
use std::error::Error;

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

#[derive(Clone, Debug, Default, Deserialize)]
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

fn select_ap(data: Vec<String>, fallback: &str, ap_port: Option<u16>) -> SocketAddress {
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
    };

    ap.unwrap_or_else(|| (String::from(fallback), port))
}

async fn try_apresolve(http_client: &HttpClient) -> Result<ApResolveData, Box<dyn Error>> {
    let req = Request::builder()
        .method("GET")
        .uri(APRESOLVE_ENDPOINT)
        .body(Body::empty())
        .unwrap();

    let body = http_client.request_body(req).await?;
    let data: ApResolveData = serde_json::from_slice(body.as_ref())?;

    Ok(data)
}

pub async fn apresolve(http_client: &HttpClient, ap_port: Option<u16>) -> AccessPoints {
    let data = try_apresolve(http_client).await.unwrap_or_else(|e| {
        warn!("Failed to resolve access points: {}, using fallbacks.", e);
        ApResolveData::default()
    });

    let accesspoint = select_ap(data.accesspoint, AP_FALLBACK, ap_port);
    let dealer = select_ap(data.dealer, DEALER_FALLBACK, ap_port);
    let spclient = select_ap(data.spclient, SPCLIENT_FALLBACK, ap_port);

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
    use crate::http_client::HttpClient;

    #[tokio::test]
    async fn test_apresolve() {
        let http_client = HttpClient::new(None);
        let aps = apresolve(&http_client, None).await;

        // Assert that the result contains a valid host and port
        aps.accesspoint.to_socket_addrs().unwrap().next().unwrap();
        aps.dealer.to_socket_addrs().unwrap().next().unwrap();
        aps.spclient.to_socket_addrs().unwrap().next().unwrap();
    }

    #[tokio::test]
    async fn test_apresolve_port_443() {
        let http_client = HttpClient::new(None);
        let aps = apresolve(&http_client, Some(443)).await;

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
