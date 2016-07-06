const APRESOLVE_ENDPOINT : &'static str = "http://apresolve.spotify.com/";

use hyper;
use std::io::Read;
use serde_json;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct APResolveData {
    ap_list: Vec<String>
}

pub fn apresolve() -> Result<Vec<String>, ()> {
    let client = hyper::client::Client::new();
    
    let mut response = client.get(APRESOLVE_ENDPOINT).send().unwrap();
    let mut data = String::new();
    response.read_to_string(&mut data).unwrap();

    let data : APResolveData = serde_json::from_str(&data).unwrap();

    Ok(data.ap_list)
}
