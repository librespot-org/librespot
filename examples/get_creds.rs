use librespot::core::{
    Error, authentication::Credentials, cache::Cache, config::SessionConfig, session::Session,
};
use std::{env, process::exit};

use log::LevelFilter;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::builder()
        .filter_module("librespot", LevelFilter::Info)
        .init();

    let mut creds_path = ".cache";
    let args: Vec<_> = env::args().collect();
    if args.len() == 2 {
        creds_path = &args[1];
    }

    let session_config = SessionConfig::default();

    let cache = Cache::new(Some(creds_path), None, None, None)?;
    let credentials = librespot_oauth::OAuthClientBuilder::new(
        &session_config.client_id,
        "http://127.0.0.1:8898/login",
        vec!["streaming"],
    )
    .open_in_browser()
    .build()?
    .get_access_token()
    .map(|t| Credentials::with_access_token(t.access_token))
    .unwrap_or_else(|e| {
        eprintln!("Error performing OAuth: {e}");
        exit(1);
    });

    let session = Session::new(session_config, Some(cache));
    if let Err(e) = session.connect(credentials, true).await {
        eprintln!("Error connecting: {e}");
        exit(1);
    }

    println!("Credentials file stored in {creds_path}");

    Ok(())
}
