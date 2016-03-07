extern crate vergen;
#[cfg(feature = "with-syntex")] extern crate syntex;
#[cfg(feature = "with-syntex")] extern crate json_macros;
#[cfg(feature = "with-syntex")] extern crate protobuf_macros;

#[cfg(feature = "with-syntex")]
fn codegen() {
    use std::env;
    use std::path::PathBuf;
    use std::path::Path;

    let mut registry = syntex::Registry::new();
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());

    json_macros::plugin_registrar(&mut registry);
    protobuf_macros::plugin_registrar(&mut registry);
    registry.expand("librespot", Path::new("src/lib.in.rs"), &out.join("lib.rs")).unwrap();
}

#[cfg(not(feature = "with-syntex"))]
fn codegen() { }

fn main() {
    vergen::vergen(vergen::SHORT_SHA).unwrap();
    codegen();
}

