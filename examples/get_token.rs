use std::env;

use librespot::core::{authentication::Credentials, config::SessionConfig, session::Session};

const SCOPES: &str =
    "streaming,user-read-playback-state,user-modify-playback-state,user-read-currently-playing";

#[tokio::main]
async fn main() {
    let mut session_config = SessionConfig::default();

    let args: Vec<_> = env::args().collect();
    if args.len() == 4 {
        session_config.client_id = args[3].clone()
    } else if args.len() != 3 {
        eprintln!("Usage: {} USERNAME PASSWORD [CLIENT_ID]", args[0]);
        return;
    }
    let username = &args[1];
    let password = &args[2];

    let session = Session::new(session_config.clone(), None);
    let credentials = Credentials::with_password(username, password);
    println!("Connecting with password..");
    let token = match session.connect(credentials, false).await {
        Ok(()) => {
            println!("Session username: {:#?}", session.username());
            session.token_provider().get_token(SCOPES).await.unwrap()
        }
        Err(e) => {
            println!("Error connecting: {}", e);
            return;
        }
    };
    println!("Token: {:#?}", token);

    // Now create a new session with that token.
    let session = Session::new(session_config, None);
    let credentials = Credentials::with_access_token(username, token.access_token);
    println!("Connecting with token..");
    match session.connect(credentials, false).await {
        Ok(()) => println!("Session username: {:#?}", session.username()),
        Err(e) => println!("Error connecting: {}", e),
    }
}
