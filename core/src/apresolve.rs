const AP_FALLBACK: &str = "ap.spotify.com:443";

use url::Url;

cfg_if! {
    if #[cfg(feature = "apresolve")] {
        const APRESOLVE_ENDPOINT: &str = "http://apresolve.spotify.com:80";

        use std::error::Error;

        use hyper::{Body, Client, Method, Request, Uri};
        use serde::{Serialize, Deserialize};

        use crate::proxytunnel::ProxyTunnel;

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

            let response = if let Some(url) = proxy {
                Client::builder()
                    .build(ProxyTunnel::new(&url.socket_addrs(|| None)?[..])?)
                    .request(req)
                    .await?
            } else {
                Client::new().request(req).await?
            };

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
    } else {
        pub async fn apresolve_or_fallback(_: &Option<Url>, _: &Option<u16>) -> String {
            AP_FALLBACK.to_string()
        }
    }
}
