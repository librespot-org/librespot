const APRESOLVE_ENDPOINT : &'static str = "http://apresolve.spotify.com/";

use hyper;
use std::io::Read;
use rustc_serialize::json;

#[derive(RustcDecodable)]
pub struct APResolveData {
    ap_list: Vec<String>
}

pub fn apresolve() -> Result<Vec<String>, ()> {
    let client = hyper::client::Client::new();
    let mut res = String::new();
    
    client.get(APRESOLVE_ENDPOINT)
          .send().unwrap()
          .read_to_string(&mut res).unwrap();

    let data : APResolveData = json::decode(&res).unwrap();

    Ok(data.ap_list)
}
