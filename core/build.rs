extern crate rand;
extern crate vergen;

use rand::distributions::Alphanumeric;
use rand::Rng;
use vergen::{generate_cargo_keys, ConstantsFlags};

fn main() {
    let mut flags = ConstantsFlags::all();
    flags.toggle(ConstantsFlags::REBUILD_ON_HEAD_CHANGE);
    generate_cargo_keys(ConstantsFlags::all()).expect("Unable to generate the cargo keys!");

    let mut rng = rand::thread_rng();
    let build_id: String = ::std::iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(8)
        .collect();

    println!("cargo:rustc-env=LIBRESPOT_BUILD_ID={}", build_id);
}
