use std::env;
use tokio_core::reactor::Core;

use librespot::core::authentication::Credentials;
use librespot::core::config::SessionConfig;
use librespot::core::keymaster;
use librespot::core::session::Session;

const SCOPES: &str =
    "streaming,user-read-playback-state,user-modify-playback-state,user-read-currently-playing";

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let session_config = SessionConfig::default();

    let args: Vec<_> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: {} USERNAME PASSWORD CLIENT_ID", args[0]);
    }
    let username = args[1].to_owned();
    let password = args[2].to_owned();
    let client_id = &args[3];

    println!("Connecting..");
    let credentials = Credentials::with_password(username, password);
    let session = core
        .run(Session::connect(session_config, credentials, None, handle))
        .unwrap();

    println!(
        "Token: {:#?}",
        core.run(keymaster::get_token(&session, &client_id, SCOPES))
            .unwrap()
    );
}
