use std::env;

use librespot_oauth::DeviceAuthClientBuilder;

const SPOTIFY_CLIENT_ID: &str = "65b708073fc0480ea92a077233ca87bd";

fn main() {
    let mut builder = env_logger::Builder::new();
    builder.parse_filters("librespot=trace");
    builder.init();

    let args: Vec<_> = env::args().collect();
    let (client_id, scopes) = if args.len() == 3 {
        // Spotify only enables the device flow for some client IDs, and refuses
        // the rest with `unauthorized_client`.
        (args[1].as_str(), args[2].split(',').collect::<Vec<&str>>())
    } else if args.len() == 1 {
        (SPOTIFY_CLIENT_ID, vec!["streaming"])
    } else {
        eprintln!("Usage: {} [CLIENT_ID SCOPES]", args[0]);
        return;
    };

    let client = match DeviceAuthClientBuilder::new(client_id, scopes).build() {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Unable to build a device auth client: {err}");
            return;
        }
    };

    let refresh_token = match client.get_access_token() {
        Ok(token) => {
            println!("OAuth Token: {token:#?}");
            token.refresh_token
        }
        Err(err) => {
            println!("Unable to get OAuth Token: {err}");
            return;
        }
    };

    match client.refresh_token(&refresh_token) {
        Ok(token) => println!("New refreshed OAuth Token: {token:#?}"),
        Err(err) => println!("Unable to get refreshed OAuth Token: {err}"),
    }
}
