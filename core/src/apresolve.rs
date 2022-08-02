use hyper::{Body, Method, Request};
use serde::Deserialize;

use crate::Error;

pub type SocketAddress = (String, u16);

#[derive(Default)]
pub struct AccessPoints {
    accesspoint: Vec<SocketAddress>,
    dealer: Vec<SocketAddress>,
    spclient: Vec<SocketAddress>,
}

#[derive(Deserialize, Default)]
pub struct ApResolveData {
    accesspoint: Vec<String>,
    dealer: Vec<String>,
    spclient: Vec<String>,
}

impl AccessPoints {
    // These addresses probably do some geo-location based traffic management or at least DNS-based
    // load balancing. They are known to fail when the normal resolvers are up, so that's why they
    // should only be used as fallback.
    fn fallback() -> Self {
        Self {
            accesspoint: vec![("ap.spotify.com".to_string(), 443)],
            dealer: vec![("dealer.spotify.com".to_string(), 443)],
            spclient: vec![("spclient.wg.spotify.com".to_string(), 443)],
        }
    }

    fn extend_filter_port(&mut self, other: Self, port: Option<u16>) {
        if let Some(port) = port {
            self.accesspoint
                .extend(other.accesspoint.into_iter().filter(|(_, p)| *p == port));
            self.dealer
                .extend(other.dealer.into_iter().filter(|(_, p)| *p == port));
            self.spclient
                .extend(other.spclient.into_iter().filter(|(_, p)| *p == port));
        } else {
            self.accesspoint.extend(other.accesspoint);
            self.dealer.extend(other.dealer);
            self.spclient.extend(other.spclient);
        }
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

    fn process_ap_strings(&self, data: Vec<String>) -> Vec<SocketAddress> {
        data.into_iter()
            .filter_map(|ap| {
                let mut split = ap.rsplitn(2, ':');
                let port = split.next()?;
                let host = split.next()?.to_owned();
                let port: u16 = port.parse().ok()?;
                if let Some(p) = self.port_config() {
                    if p != port {
                        return None;
                    }
                }
                Some((host, port))
            })
            .collect()
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

            inner.data.accesspoint = self.process_ap_strings(data.accesspoint);
            inner.data.dealer = self.process_ap_strings(data.dealer);
            inner.data.spclient = self.process_ap_strings(data.spclient);

            if self.is_any_empty() {
                if let Some(error) = error {
                    warn!(
                        "Failed to resolve all access points, using fallbacks: {}",
                        error
                    );
                } else {
                    warn!("Failed to resolve all access points, using fallbacks");
                }
                inner
                    .data
                    .extend_filter_port(AccessPoints::fallback(), self.port_config());
            }
        })
    }

    fn is_any_empty(&self) -> bool {
        self.lock(|inner| {
            inner.data.accesspoint.is_empty()
                || inner.data.dealer.is_empty()
                || inner.data.spclient.is_empty()
        })
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
                "accesspoint" => inner.data.accesspoint.first(),
                "dealer" => inner.data.dealer.first(),
                "spclient" => inner.data.spclient.first(),
                _ => {
                    return Err(Error::unimplemented(format!(
                        "No implementation to resolve access point {}",
                        endpoint
                    )))
                }
            };

            let access_point = access_point.cloned().ok_or(Error::unavailable(format!(
                "No access point available for endpoint {}",
                endpoint
            )))?;

            Ok(access_point)
        })
    }
}
