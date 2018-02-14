extern crate rand;
extern crate vergen;

use rand::Rng;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());

    vergen::vergen(vergen::OutputFns::all()).unwrap();

    let build_id: String = rand::thread_rng().gen_ascii_chars().take(8).collect();

    let mut version_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&out.join("version.rs"))
        .unwrap();

    let build_id_fn = format!(
        "
/// Generate a random build id.
pub fn build_id() -> &'static str {{
    \"{}\"
}}
",
        build_id
    );

    if let Err(e) = version_file.write_all(build_id_fn.as_bytes()) {
        println!("{}", e);
    }
}
