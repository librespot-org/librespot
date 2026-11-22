use rand::Rng;
use rand_distr::Alphanumeric;
use vergen_gitcl::{BuildBuilder, Emitter, GitclBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gitcl = GitclBuilder::default()
        .sha(true) // outputs 'VERGEN_GIT_SHA', and sets the 'short' flag true
        .commit_date(true) // outputs 'VERGEN_GIT_COMMIT_DATE'
        .build()?;

    if let Err(why) = Emitter::default()
        .fail_on_error()
        .add_instructions(&gitcl)
        .and_then(|emitter| emitter.emit())
    {
        // if we are running into an error here, the git repo is most likely missing
        // in that case we are probably a dependency and want to emit default values
        println!("cargo:warning=emitting vergen-gitcl default values: {why}");
        println!("cargo:rustc-env=VERGEN_GIT_SHA=stable");
        println!("cargo:rustc-env=VERGEN_GIT_COMMIT_DATE=unknown");
    }

    let build = BuildBuilder::default()
        .build_date(true) // outputs 'VERGEN_BUILD_DATE'
        .build()?;

    Emitter::default()
        .add_instructions(&build)?
        .emit()
        .expect("Unable to generate the cargo keys!");

    let build_id = std::env::var("SOURCE_DATE_EPOCH").unwrap_or_else(|_| {
        rand::rng()
            .sample_iter(Alphanumeric)
            .take(8)
            .map(char::from)
            .collect()
    });

    println!("cargo:rustc-env=LIBRESPOT_BUILD_ID={build_id}");
    Ok(())
}
