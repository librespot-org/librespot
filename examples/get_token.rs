use std::env;

use librespot::core::{authentication::Credentials, config::SessionConfig, session::Session};

const SCOPES: &str =
    "streaming,user-read-playback-state,user-modify-playback-state,user-read-currently-playing";

#[tokio::main]
async fn main() {
    let session_config = SessionConfig::default();

    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} USERNAME PASSWORD", args[0]);
        return;
    }

    println!("Connecting...");
    let credentials = Credentials::with_password(&args[1], &args[2]);
    let session = Session::new(session_config, None);

    match session.connect(credentials).await {
        Ok(()) => println!(
            "Token: {:#?}",
            session.token_provider().get_token(SCOPES).await.unwrap()
        ),
        Err(e) => println!("Error connecting: {}", e),
    }
}
