use librespot_oauth::get_access_token;

// You can use any client ID here but it must be configured to allow redirect URI http://127.0.0.1
const SPOTIFY_CLIENT_ID: &str = "65b708073fc0480ea92a077233ca87bd";

fn main() {
    let mut builder = env_logger::Builder::new();
    builder.parse_filters("librespot=trace");
    builder.init();

    match get_access_token(SPOTIFY_CLIENT_ID, vec!["streaming"], Some(1337)) {
        Ok(token) => println!("Success: {token:#?}"),
        Err(e) => println!("Failed: {e}"),
    };
}
