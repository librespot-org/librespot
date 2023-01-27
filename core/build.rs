use rand::{distributions::Alphanumeric, Rng};
use vergen::{vergen, Config, ShaKind, TimestampKind};

fn main() {
    let mut config = Config::default();
    *config.build_mut().kind_mut() = TimestampKind::DateOnly;
    *config.git_mut().enabled_mut() = true;
    *config.git_mut().commit_timestamp_mut() = true;
    *config.git_mut().commit_timestamp_kind_mut() = TimestampKind::DateOnly;
    *config.git_mut().sha_mut() = true;
    *config.git_mut().sha_kind_mut() = ShaKind::Short;
    *config.git_mut().rerun_on_head_change_mut() = true;

    vergen(config).expect("Unable to generate the cargo keys!");

    let build_id = match std::env::var("SOURCE_DATE_EPOCH") {
        Ok(val) => val,
        Err(_) => rand::thread_rng()
            .sample_iter(Alphanumeric)
            .take(8)
            .map(char::from)
            .collect(),
    };

    println!("cargo:rustc-env=LIBRESPOT_BUILD_ID={build_id}");
}
