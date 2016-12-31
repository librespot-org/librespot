const APRESOLVE_ENDPOINT : &'static str = "https://apresolve.spotify.com/";
const AP_FALLBACK : &'static str = "ap.spotify.com:80";

use hyper;
use hyper_rustls;
use std::io::Read;
use serde_json;
use hyper::net::HttpsConnector;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct APResolveData {
    ap_list: Vec<String>
}

pub fn apresolve() -> String {
    let connector = HttpsConnector::new(hyper_rustls::TlsClient::new());
    let client = hyper::Client::with_connector(connector);

    (|| {
        let mut response = client.get(APRESOLVE_ENDPOINT).send().map_err(|_| ())?;
        let mut data = String::new();
        response.read_to_string(&mut data).map_err(|_| ())?;

        let data : APResolveData = serde_json::from_str(&data).map_err(|_| ())?;
        data.ap_list.first().map(Clone::clone).ok_or(())
    })().unwrap_or_else(|_| {
        warn!("failed to resolve AP, using fallback");
        AP_FALLBACK.into()
    })
}
