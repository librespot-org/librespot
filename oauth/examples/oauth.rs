use std::env;

use librespot_oauth::OAuthClientBuilder;

const SPOTIFY_CLIENT_ID: &str = "65b708073fc0480ea92a077233ca87bd";
const SPOTIFY_REDIRECT_URI: &str = "http://127.0.0.1:8898/login";

const RESPONSE: &str = r#"
<!doctype html>
<html>
    <body>
        <h1>Return to your app!</h1>
    </body>
</html>
"#;

#[tokio::main]
async fn main() {
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

    let client = match OAuthClientBuilder::new(client_id, redirect_uri, scopes)
        .open_in_browser()
        .with_custom_message(RESPONSE)
        .build()
    {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Unable to build an OAuth client: {}", err);
            return;
        }
    };

    let refresh_token = match client.get_access_token().await {
        Ok(token) => {
            println!("OAuth Token: {token:#?}");
            token.refresh_token
        }
        Err(err) => {
            println!("Unable to get OAuth Token: {err}");
            return;
        }
    };

    match client.refresh_token(&refresh_token).await {
        Ok(token) => println!("New refreshed OAuth Token: {token:#?}"),
        Err(err) => println!("Unable to get refreshed OAuth Token: {err}"),
    }
}
