use std::path::Path;

use rand::Rng;
use rand_distr::Alphanumeric;
use vergen_gitcl::{BuildBuilder, Emitter, GitclBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gitcl = GitclBuilder::default()
        .sha(true) // outputs 'VERGEN_GIT_SHA', and sets the 'short' flag true
        .commit_date(true) // outputs 'VERGEN_GIT_COMMIT_DATE'
        .build()?;

    let build = BuildBuilder::default()
        .build_date(true) // outputs 'VERGEN_BUILD_DATE'
        .build()?;

    let mut emitter = Emitter::default();
    emitter.add_instructions(&build)?;

    // Only describe librespot's own repository, not one it is unpacked inside.
    if Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../.git")
        .exists()
    {
        emitter.add_instructions(&gitcl)?;
    } else {
        println!("cargo:rustc-env=VERGEN_GIT_SHA=unknown");
        println!("cargo:rustc-env=VERGEN_GIT_COMMIT_DATE=unknown");
    }

    emitter.emit().expect("Unable to generate the cargo keys!");
    let build_id = match std::env::var("SOURCE_DATE_EPOCH") {
        Ok(val) => val,
        Err(_) => rand::rng()
            .sample_iter(Alphanumeric)
            .take(8)
            .map(char::from)
            .collect(),
    };

    println!("cargo:rustc-env=LIBRESPOT_BUILD_ID={build_id}");
    Ok(())
}
