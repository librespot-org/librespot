const AP_FALLBACK: &'static str = "ap.spotify.com:443";
const APRESOLVE_ENDPOINT: &'static str = "http://apresolve.spotify.com/";

use futures::{future, Future, Stream};
use hyper::{self, Client, Uri};
use serde_json;
use std::str::FromStr;
use tokio_core::reactor::Handle;

error_chain!{}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct APResolveData {
    ap_list: Vec<String>,
}

fn apresolve(handle: &Handle) -> Box<Future<Item = String, Error = Error>> {
    let url = Uri::from_str(APRESOLVE_ENDPOINT).expect("invalid AP resolve URL");

    let client = Client::new(handle);
    let response = client.get(url);

    let body = response.and_then(|response| {
        response.body().fold(Vec::new(), |mut acc, chunk| {
            acc.extend_from_slice(chunk.as_ref());
            Ok::<_, hyper::Error>(acc)
        })
    });
    let body = body.then(|result| result.chain_err(|| "HTTP error"));
    let body = body.and_then(|body| String::from_utf8(body).chain_err(|| "invalid UTF8 in response"));

    let data =
        body.and_then(|body| serde_json::from_str::<APResolveData>(&body).chain_err(|| "invalid JSON"));

    let ap = data.and_then(|data| {
        let ap = data.ap_list.first().ok_or("empty AP List")?;
        Ok(ap.clone())
    });

    Box::new(ap)
}

pub(crate) fn apresolve_or_fallback<E>(
    handle: &Handle,
    proxy: &Option<String>,
) -> Box<Future<Item = String, Error = E>>
where
    E: 'static,
{
    if proxy.is_some() {
        // TODO: Use a proper proxy library and filter out a 443 proxy instead of relying on fallback.
        //       The problem with current libraries (hyper-proxy, reqwest) is that they depend on TLS
        //       and this is a dependency we might not want.
        Box::new(future::result(Ok(AP_FALLBACK.into())))
    } else {
        let ap = apresolve(handle).or_else(|e| {
            warn!("Failed to resolve Access Point: {}", e.description());
            warn!("Using fallback \"{}\"", AP_FALLBACK);
            Ok(AP_FALLBACK.into())
        });
        Box::new(ap)
    }
}
