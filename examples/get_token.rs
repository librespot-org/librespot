use std::env;

use librespot::core::{authentication::Credentials, config::SessionConfig, session::Session};

const SCOPES: &str =
    "streaming,user-read-playback-state,user-modify-playback-state,user-read-currently-playing";

#[tokio::main]
async fn main() {
    let mut builder = env_logger::Builder::new();
    builder.parse_filters("librespot=trace");
    builder.init();

    let mut session_config = SessionConfig::default();

    let args: Vec<_> = env::args().collect();
    if args.len() == 3 {
        // Only special client IDs have sufficient privileges e.g. Spotify's.
        session_config.client_id = args[2].clone()
    } else if args.len() != 2 {
        eprintln!("Usage: {} ACCESS_TOKEN [CLIENT_ID]", args[0]);
        return;
    }
    let access_token = &args[1];

    // Now create a new session with that token.
    let session = Session::new(session_config.clone(), None);
    let credentials = Credentials::with_access_token(access_token);
    println!("Connecting with token..");
    match session.connect(credentials, false).await {
        Ok(()) => println!("Session username: {:#?}", session.username()),
        Err(e) => {
            println!("Error connecting: {e}");
            return;
        }
    };

    let token = session.token_provider().get_token(SCOPES).await.unwrap();
    println!("Got me a token: {token:#?}");
}
