use std::collections::VecDeque;

use hyper::{Body, Method, Request};
use serde::Deserialize;

use crate::Error;

pub type SocketAddress = (String, u16);

#[derive(Default)]
pub struct AccessPoints {
    accesspoint: VecDeque<SocketAddress>,
    dealer: VecDeque<SocketAddress>,
    spclient: VecDeque<SocketAddress>,
}

#[derive(Deserialize, Default)]
pub struct ApResolveData {
    accesspoint: Vec<String>,
    dealer: Vec<String>,
    spclient: Vec<String>,
}

impl ApResolveData {
    // These addresses probably do some geo-location based traffic management or at least DNS-based
    // load balancing. They are known to fail when the normal resolvers are up, so that's why they
    // should only be used as fallback.
    fn fallback() -> Self {
        Self {
            accesspoint: vec![String::from("ap.spotify.com:443")],
            dealer: vec![String::from("dealer.spotify.com:443")],
            spclient: vec![String::from("spclient.wg.spotify.com:443")],
        }
    }
}

impl AccessPoints {
    fn is_any_empty(&self) -> bool {
        self.accesspoint.is_empty() || self.dealer.is_empty() || self.spclient.is_empty()
    }
}

component! {
    ApResolver : ApResolverInner {
        data: AccessPoints = AccessPoints::default(),
    }
}

impl ApResolver {
    // return a port if a proxy URL and/or a proxy port was specified. This is useful even when
    // there is no proxy, but firewalls only allow certain ports (e.g. 443 and not 4070).
    pub fn port_config(&self) -> Option<u16> {
        if self.session().config().proxy.is_some() || self.session().config().ap_port.is_some() {
            Some(self.session().config().ap_port.unwrap_or(443))
        } else {
            None
        }
    }

    fn process_ap_strings(&self, data: Vec<String>) -> VecDeque<SocketAddress> {
        let filter_port = self.port_config();
        data.into_iter()
            .filter_map(|ap| {
                let mut split = ap.rsplitn(2, ':');
                let port = split.next()?;
                let port: u16 = port.parse().ok()?;
                let host = split.next()?.to_owned();
                match filter_port {
                    Some(filter_port) if filter_port != port => None,
                    _ => Some((host, port)),
                }
            })
            .collect()
    }

    fn parse_resolve_to_access_points(&self, resolve: ApResolveData) -> AccessPoints {
        AccessPoints {
            accesspoint: self.process_ap_strings(resolve.accesspoint),
            dealer: self.process_ap_strings(resolve.dealer),
            spclient: self.process_ap_strings(resolve.spclient),
        }
    }

    pub async fn try_apresolve(&self) -> Result<ApResolveData, Error> {
        let req = Request::builder()
            .method(Method::GET)
            .uri("https://apresolve.spotify.com/?type=accesspoint&type=dealer&type=spclient")
            .body(Body::empty())?;

        let body = self.session().http_client().request_body(req).await?;
        let data: ApResolveData = serde_json::from_slice(body.as_ref())?;

        Ok(data)
    }

    async fn apresolve(&self) {
        let result = self.try_apresolve().await;

        self.lock(|inner| {
            let (data, error) = match result {
                Ok(data) => (data, None),
                Err(e) => (ApResolveData::default(), Some(e)),
            };

            inner.data = self.parse_resolve_to_access_points(data);

            if inner.data.is_any_empty() {
                warn!("Failed to resolve all access points, using fallbacks");
                if let Some(error) = error {
                    warn!("Resolve access points error: {}", error);
                }

                let fallback = self.parse_resolve_to_access_points(ApResolveData::fallback());
                inner.data.accesspoint.extend(fallback.accesspoint);
                inner.data.dealer.extend(fallback.dealer);
                inner.data.spclient.extend(fallback.spclient);
            }
        })
    }

    fn is_any_empty(&self) -> bool {
        self.lock(|inner| inner.data.is_any_empty())
    }

    pub async fn resolve(&self, endpoint: &str) -> Result<SocketAddress, Error> {
        if self.is_any_empty() {
            self.apresolve().await;
        }

        self.lock(|inner| {
            let access_point = match endpoint {
                // take the first position instead of the last with `pop`, because Spotify returns
                // access points with ports 4070, 443 and 80 in order of preference from highest
                // to lowest.
                "accesspoint" => inner.data.accesspoint.pop_front(),
                "dealer" => inner.data.dealer.pop_front(),
                "spclient" => inner.data.spclient.pop_front(),
                _ => {
                    return Err(Error::unimplemented(format!(
                        "No implementation to resolve access point {endpoint}"
                    )))
                }
            };

            let access_point = access_point.ok_or_else(|| {
                Error::unavailable(format!("No access point available for endpoint {endpoint}"))
            })?;

            Ok(access_point)
        })
    }
}
