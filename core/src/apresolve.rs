use std::sync::atomic::{AtomicUsize, Ordering};

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

#[derive(Deserialize)]
pub struct ApResolveData {
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
        spinlock: AtomicUsize = AtomicUsize::new(0),
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

    fn process_data(&self, data: Vec<String>) -> Vec<SocketAddress> {
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
            .uri("http://apresolve.spotify.com/?type=accesspoint&type=dealer&type=spclient")
            .body(Body::empty())?;

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

            inner.data.accesspoint = self.process_data(data.accesspoint);
            inner.data.dealer = self.process_data(data.dealer);
            inner.data.spclient = self.process_data(data.spclient);
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
        // Use a spinlock to make this function atomic. Otherwise, various race conditions may
        // occur, e.g. when the session is created, multiple components are launched almost in
        // parallel and they will all call this function, while resolving is still in progress.
        self.lock(|inner| {
            while inner.spinlock.load(Ordering::SeqCst) != 0 {
                #[allow(deprecated)]
                std::sync::atomic::spin_loop_hint()
            }
            inner.spinlock.store(1, Ordering::SeqCst);
        });

        if self.is_empty() {
            self.apresolve().await;
        }

        self.lock(|inner| {
            let access_point = match endpoint {
                // take the first position instead of the last with `pop`, because Spotify returns
                // access points with ports 4070, 443 and 80 in order of preference from highest
                // to lowest.
                "accesspoint" => inner.data.accesspoint.remove(0),
                "dealer" => inner.data.dealer.remove(0),
                "spclient" => inner.data.spclient.remove(0),
                _ => unimplemented!(),
            };
            inner.spinlock.store(0, Ordering::SeqCst);
            access_point
        })
    }
}
