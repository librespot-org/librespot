use hyper::{Body, Request};
use serde::Deserialize;
use std::error::Error;

pub type SocketAddress = (String, u16);

#[derive(Default)]
struct AccessPoints {
    accesspoint: Vec<SocketAddress>,
    dealer: Vec<SocketAddress>,
    spclient: Vec<SocketAddress>,
}

#[derive(Deserialize)]
struct ApResolveData {
    accesspoint: Vec<String>,
    dealer: Vec<String>,
    spclient: Vec<String>,
}

// These addresses probably do some geo-location based traffic management or at least DNS-based
// load balancing. They are known to fail when the normal resolvers are up, so that's why they
// should only be used as fallback.
impl Default for ApResolveData {
    fn default() -> Self {
        Self {
            accesspoint: vec![String::from("ap.spotify.com:443")],
            dealer: vec![String::from("dealer.spotify.com:443")],
            spclient: vec![String::from("spclient.wg.spotify.com:443")],
        }
    }
}

component! {
    ApResolver : ApResolverInner {
        data: AccessPoints = AccessPoints::default(),
    }
}

impl ApResolver {
    fn split_aps(data: Vec<String>) -> Vec<SocketAddress> {
        data.into_iter()
            .filter_map(|ap| {
                let mut split = ap.rsplitn(2, ':');
                let port = split
                    .next()
                    .expect("rsplitn should not return empty iterator");
                let host = split.next()?.to_owned();
                let port: u16 = port.parse().ok()?;
                Some((host, port))
            })
            .collect()
    }

    fn find_ap(&self, data: &[SocketAddress]) -> usize {
        match self.session().config().proxy {
            Some(_) => data
                .iter()
                .position(|(_, port)| *port == self.session().config().ap_port.unwrap_or(443))
                .expect("No access points available with that proxy port."),
            None => 0, // just pick the first one
        }
    }

    async fn try_apresolve(&self) -> Result<ApResolveData, Box<dyn Error>> {
        let req = Request::builder()
            .method("GET")
            .uri("http://apresolve.spotify.com/?type=accesspoint&type=dealer&type=spclient")
            .body(Body::empty())
            .unwrap();

        let body = self.session().http_client().request_body(req).await?;
        let data: ApResolveData = serde_json::from_slice(body.as_ref())?;

        Ok(data)
    }

    async fn apresolve(&self) {
        let result = self.try_apresolve().await;
        self.lock(|inner| {
            let data = match result {
                Ok(data) => data,
                Err(e) => {
                    warn!("Failed to resolve access points, using fallbacks: {}", e);
                    ApResolveData::default()
                }
            };

            inner.data.accesspoint = Self::split_aps(data.accesspoint);
            inner.data.dealer = Self::split_aps(data.dealer);
            inner.data.spclient = Self::split_aps(data.spclient);
        })
    }

    fn is_empty(&self) -> bool {
        self.lock(|inner| {
            inner.data.accesspoint.is_empty()
                || inner.data.dealer.is_empty()
                || inner.data.spclient.is_empty()
        })
    }

    pub async fn resolve(&self, endpoint: &str) -> SocketAddress {
        if self.is_empty() {
            self.apresolve().await;
        }

        self.lock(|inner| match endpoint {
            "accesspoint" => {
                let pos = self.find_ap(&inner.data.accesspoint);
                inner.data.accesspoint.remove(pos)
            }
            "dealer" => {
                let pos = self.find_ap(&inner.data.dealer);
                inner.data.dealer.remove(pos)
            }
            "spclient" => {
                let pos = self.find_ap(&inner.data.spclient);
                inner.data.spclient.remove(pos)
            }
            _ => unimplemented!(),
        })
    }
}
