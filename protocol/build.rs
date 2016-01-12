extern crate protobuf_build;

use std::env;
use std::path::PathBuf;

fn main() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let proto = root.join("proto");

    let mut compiler = protobuf_build::Compiler::new(&proto, &out);

    for file in &["keyexchange.proto",
                  "authentication.proto",
                  "mercury.proto",
                  "metadata.proto",
                  "pubsub.proto",
                  "spirc.proto"] {
        compiler.compile(file).unwrap();
    }
}

