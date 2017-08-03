const AP_FALLBACK : &'static str = "ap.spotify.com:80";
const APRESOLVE_ENDPOINT : &'static str = "http://apresolve.spotify.com/";

use std::str::FromStr;
use futures::{Future, Stream};
use hyper::{self, Uri, Client};
use serde_json;
use tokio_core::reactor::Handle;

error_chain! { }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct APResolveData {
    ap_list: Vec<String>
}

pub fn apresolve(handle: &Handle) -> Box<Future<Item=String, Error=Error>> {
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
    let body = body.and_then(|body| {
        String::from_utf8(body).chain_err(|| "invalid UTF8 in response")
    });

    let data = body.and_then(|body| {
        serde_json::from_str::<APResolveData>(&body)
            .chain_err(|| "invalid JSON")
    });

    let ap = data.and_then(|data| {
        let ap = data.ap_list.first().ok_or("empty AP List")?;
        Ok(ap.clone())
    });

    Box::new(ap)
}

pub fn apresolve_or_fallback<E>(handle: &Handle)
    -> Box<Future<Item=String, Error=E>>
    where E: 'static
{
    let ap = apresolve(handle).or_else(|e| {
        warn!("Failed to resolve Access Point: {}", e.description());
        warn!("Using fallback \"{}\"", AP_FALLBACK);
        Ok(AP_FALLBACK.into())
    });

    Box::new(ap)
}
