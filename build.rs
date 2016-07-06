extern crate vergen;

#[cfg(feature = "with-syntex")]
fn codegen() {
    extern crate json_macros;
    extern crate protobuf_macros;
    extern crate serde_codegen;

    use std::env;
    use std::path::PathBuf;

    let out = PathBuf::from(env::var("OUT_DIR").unwrap());

    json_macros::expand("src/lib.in.rs", &out.join("lib.tmp0.rs")).unwrap();
    serde_codegen::expand(&out.join("lib.tmp0.rs"), &out.join("lib.tmp1.rs")).unwrap();
    protobuf_macros::expand(&out.join("lib.tmp1.rs"), &out.join("lib.rs")).unwrap();
}

#[cfg(not(feature = "with-syntex"))]
fn codegen() { }

fn main() {
    vergen::vergen(vergen::OutputFns::all()).unwrap();
    codegen();
}

