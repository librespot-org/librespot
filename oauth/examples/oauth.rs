use std::env;

use librespot_oauth::get_access_token;

const SPOTIFY_CLIENT_ID: &str = "65b708073fc0480ea92a077233ca87bd";
const SPOTIFY_REDIRECT_URI: &str = "http://127.0.0.1:8898/login";

fn main() {
    let mut builder = env_logger::Builder::new();
    builder.parse_filters("librespot=trace");
    builder.init();

    let args: Vec<_> = env::args().collect();
    let (client_id, redirect_uri, scopes) = if args.len() == 4 {
        // You can use your own client ID, along with it's associated redirect URI.
        (
            args[1].as_str(),
            args[2].as_str(),
            args[3].split(',').collect::<Vec<&str>>(),
        )
    } else if args.len() == 1 {
        (SPOTIFY_CLIENT_ID, SPOTIFY_REDIRECT_URI, vec!["streaming"])
    } else {
        eprintln!("Usage: {} [CLIENT_ID REDIRECT_URI SCOPES]", args[0]);
        return;
    };

    match get_access_token(client_id, redirect_uri, scopes) {
        Ok(token) => println!("Success: {token:#?}"),
        Err(e) => println!("Failed: {e}"),
    };
}
