use rand::distributions::Alphanumeric;
use rand::Rng;
use vergen::{vergen, Config, ShaKind, TimestampKind};

fn main() {
    let mut config = Config::default();
    *config.git_mut().sha_kind_mut() = ShaKind::Short;
    *config.build_mut().kind_mut() = TimestampKind::DateOnly;
    vergen(config).expect("Unable to generate the cargo keys!");

    let build_id: String = rand::thread_rng()
        .sample_iter(Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();

    println!("cargo:rustc-env=LIBRESPOT_BUILD_ID={}", build_id);
}
