use rand::distributions::Alphanumeric;
use rand::Rng;
use std::env;
use vergen::{generate_cargo_keys, ConstantsFlags};

fn main() {
    let mut flags = ConstantsFlags::all();
    flags.toggle(ConstantsFlags::REBUILD_ON_HEAD_CHANGE);
    generate_cargo_keys(ConstantsFlags::all()).expect("Unable to generate the cargo keys!");

    let build_id = match env::var("SOURCE_DATE_EPOCH") {
        Ok(val) => val,
        Err(_) => rand::thread_rng()
            .sample_iter(Alphanumeric)
            .take(8)
            .map(char::from)
            .collect(),
    };

    println!("cargo:rustc-env=LIBRESPOT_BUILD_ID={}", build_id);
}
