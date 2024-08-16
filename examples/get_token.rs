use std::env;

use librespot::core::{authentication::Credentials, config::SessionConfig, session::Session};
use librespot::protocol::authentication::AuthenticationType;

const SCOPES: &str =
    "streaming,user-read-playback-state,user-modify-playback-state,user-read-currently-playing";

#[tokio::main]
async fn main() {
    let mut session_config = SessionConfig::default();

    let args: Vec<_> = env::args().collect();
    if args.len() == 3 {
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

    // This will fail. You can't use keymaster from an access token authed session.
    // let token = session.token_provider().get_token(SCOPES).await.unwrap();

    // Instead, derive stored credentials and auth a new session using those.
    let stored_credentials = Credentials {
        username: Some(session.username()),
        auth_type: AuthenticationType::AUTHENTICATION_STORED_SPOTIFY_CREDENTIALS,
        auth_data: session.auth_data(),
    };
    let session2 = Session::new(session_config, None);
    match session2.connect(stored_credentials, false).await {
        Ok(()) => println!("Session username: {:#?}", session2.username()),
        Err(e) => {
            println!("Error connecting: {}", e);
            return;
        }
    };
    let token = session2.token_provider().get_token(SCOPES).await.unwrap();
    println!("Got me a token: {token:#?}");
}
