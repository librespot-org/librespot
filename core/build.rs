use rand::distributions::Alphanumeric;
use rand::Rng;
use std::env;
use vergen::{vergen, Config, ShaKind, TimestampKind};

fn main() {
    let mut config = Config::default();
    let gitcfg = config.git_mut();
    *gitcfg.sha_kind_mut() = ShaKind::Short;
    *gitcfg.commit_timestamp_mut() = true;
    *gitcfg.commit_timestamp_kind_mut() = TimestampKind::DateOnly;
    *gitcfg.rerun_on_head_change_mut() = true;
    *config.build_mut().kind_mut() = TimestampKind::DateOnly;

    vergen(config).expect("Unable to generate the cargo keys!");

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
