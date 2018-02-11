extern crate protobuf_macros;

use std::env;
use std::path::PathBuf;

fn main() {
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());

    protobuf_macros::expand("src/lib.in.rs", &out.join("lib.rs")).unwrap();

    println!("cargo:rerun-if-changed=src/lib.in.rs");
    println!("cargo:rerun-if-changed=src/spirc.rs");

}
