#![crate_name = "librespot"]

#![feature(plugin,core)]

#![plugin(mod_path)]
#![plugin(protobuf_macros)]
#[macro_use] extern crate lazy_static;

extern crate byteorder;
extern crate crypto;
extern crate gmp;
extern crate num;
extern crate protobuf;
extern crate rand;

mod connection;
mod cryptoutil;
mod protocol;
mod session;
mod util;

use session::Session;

fn main() {
    let mut s = Session::new();
    s.login();
}

