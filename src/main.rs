#![crate_name = "librespot"]

#![feature(plugin)]

#![plugin(protobuf_macros)]
#[macro_use] extern crate lazy_static;

extern crate byteorder;
extern crate crypto;
extern crate gmp;
extern crate num;
extern crate protobuf;
extern crate shannon;
extern crate rand;
extern crate readall;

extern crate librespot_protocol;

mod connection;
mod keys;
mod session;
mod util;

use std::fs::File;
use std::io::Read;
use std::path::Path;

use session::{Session,Config};

fn main() {
    let mut args = std::env::args().skip(1);
    let mut appkey_file = File::open(Path::new(&args.next().unwrap())).unwrap();
    let username = args.next().unwrap();
    let password = args.next().unwrap();

    let mut appkey = Vec::new();
    appkey_file.read_to_end(&mut appkey).unwrap();

    let config = Config {
        application_key: appkey,
        user_agent: "ABC".to_string(),
        device_id: "ABC".to_string()
    };
    let mut s = Session::new(config);

    s.login(username, password);
}

