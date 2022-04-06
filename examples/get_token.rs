use std::env;

use librespot::core::authentication::Credentials;
use librespot::core::config::SessionConfig;
use librespot::core::keymaster;
use librespot::core::session::Session;

const SCOPES: &str =
    "streaming,user-read-playback-state,user-modify-playback-state,user-read-currently-playing";

#[tokio::main]
async fn main() {
    let session_config = SessionConfig::default();

    let args: Vec<_> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} USERNAME PASSWORD CLIENT_ID", args[0]);
        return;
    }

    println!("Connecting..");
    let credentials = Credentials::with_password(&args[1], &args[2]);
    let (session, _) = Session::connect(session_config, credentials, None, false)
        .await
        .unwrap();

    println!(
        "Token: {:#?}",
        keymaster::get_token(&session, &args[3], SCOPES)
            .await
            .unwrap()
    );
}
