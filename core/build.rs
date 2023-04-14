use rand::{distributions::Alphanumeric, Rng};
use vergen::EmitBuilder;

fn main() {
    EmitBuilder::builder()
        .build_date() // outputs 'VERGEN_BUILD_DATE'
        .git_sha(true) // outputs 'VERGEN_GIT_SHA', and sets the 'short' flag true
        .git_commit_date() // outputs 'VERGEN_GIT_COMMIT_DATE'
        .emit()
        .expect("Unable to generate the cargo keys!");
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
