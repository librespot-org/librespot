extern crate protobuf_build;

use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::io::{Read, Write};

fn main() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let proto = root.join("proto");

    let mut compiler = protobuf_build::Compiler::new(&proto, &out);

    let files = ["keyexchange",
                 "authentication",
                 "mercury",
                 "metadata",
                 "pubsub",
                 "spirc"];

    for file in &files {
        compiler.compile(&((*file).to_owned() + ".proto")).unwrap();

        // Hack for rust-lang/rust#18810
        // Wrap the generated rust files with "pub mod { ... }", so they
        // can be included.
        let path = out.join(&((*file).to_owned() + ".rs"));
        let contents = {
            let mut src = File::open(path).unwrap();
            let mut contents = Vec::new();
            src.read_to_end(&mut contents).unwrap();
            contents
        };

        let mut dst = File::create(out.join(&((*file).to_owned() + ".rs"))).unwrap();
        dst.write_all(format!("pub mod {} {{\n", file).as_bytes()).unwrap();
        dst.write_all(&contents).unwrap();
        dst.write_all("}".as_bytes()).unwrap();
    }
}

