//! Provides a Spotify access token using the OAuth authorization code flow
//! with PKCE.
//!
//! Assuming sufficient scopes, the returned access token may be used with Spotify's
//! Web API, and/or to establish a new Session with [`librespot_core`].
//!
//! The authorization code flow is an interactive process which requires a web browser
//! to complete. The resulting code must then be provided back from the browser to this
//! library for exchange into an access token. Providing the code can be automatic via
//! a spawned http server (mimicking Spotify's client), or manually via stdin. The latter
//! is appropriate for headless systems.

use log::{debug, error, info, trace};
use oauth2::reqwest::http_client;
use oauth2::{
    basic::BasicClient, AuthUrl, AuthorizationCode, ClientId, CsrfToken, PkceCodeChallenge,
    RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use std::io;
use std::{
    io::{BufRead, BufReader, Write},
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener},
    process::exit,
    sync::mpsc,
};
use url::Url;

fn get_code(redirect_url: &str) -> AuthorizationCode {
    let url = Url::parse(redirect_url).unwrap();
    let code = url
        .query_pairs()
        .find(|(key, _)| key == "code")
        .map(|(_, code)| AuthorizationCode::new(code.into_owned()))
        .unwrap();
    code
}

fn get_authcode_stdin() -> AuthorizationCode {
    println!("Provide redirect URL");
    let mut buffer = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut buffer).unwrap();

    get_code(buffer.trim())
}

fn get_authcode_listener(socket_address: SocketAddr) -> AuthorizationCode {
    // A very naive implementation of the redirect server.
    let listener = TcpListener::bind(socket_address).unwrap();

    info!("OAuth server listening on {:?}", socket_address);

    // The server will terminate itself after collecting the first code.
    let Some(mut stream) = listener.incoming().flatten().next() else {
        panic!("listener terminated without accepting a connection");
    };

    let mut reader = BufReader::new(&stream);

    let mut request_line = String::new();
    reader.read_line(&mut request_line).unwrap();

    let redirect_url = request_line.split_whitespace().nth(1).unwrap();
    let code = get_code(&("http://localhost".to_string() + redirect_url));

    let message = "Go back to your terminal :)";
    let response = format!(
        "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
        message.len(),
        message
    );
    stream.write_all(response.as_bytes()).unwrap();

    code
}

// TODO: Return a Result?
// TODO: Pass in redirect_address instead since the redirect host depends on client ID?
// TODO: Pass in scopes.
pub fn get_access_token(client_id: &str, redirect_port: u16) -> String {
    // Must use host 127.0.0.1 with Spotify Desktop client ID.
    let redirect_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), redirect_port);
    let redirect_uri = format!("http://{redirect_address}/login");

    let client = BasicClient::new(
        ClientId::new(client_id.to_string()),
        None,
        AuthUrl::new("https://accounts.spotify.com/authorize".to_string()).unwrap(),
        Some(TokenUrl::new("https://accounts.spotify.com/api/token".to_string()).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_uri).expect("Invalid redirect URL"));

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the full authorization URL.
    // Some of these scopes are unavailable for custom client IDs. Which?
    let scopes = vec![
        "app-remote-control",
        "playlist-modify",
        "playlist-modify-private",
        "playlist-modify-public",
        "playlist-read",
        "playlist-read-collaborative",
        "playlist-read-private",
        "streaming",
        "ugc-image-upload",
        "user-follow-modify",
        "user-follow-read",
        "user-library-modify",
        "user-library-read",
        "user-modify",
        "user-modify-playback-state",
        "user-modify-private",
        "user-personalized",
        "user-read-birthdate",
        "user-read-currently-playing",
        "user-read-email",
        "user-read-play-history",
        "user-read-playback-position",
        "user-read-playback-state",
        "user-read-private",
        "user-read-recently-played",
        "user-top-read",
    ];
    let scopes: Vec<oauth2::Scope> = scopes.into_iter().map(|s| Scope::new(s.into())).collect();
    let (auth_url, _) = client
        .authorize_url(CsrfToken::new_random)
        .add_scopes(scopes)
        .set_pkce_challenge(pkce_challenge)
        .url();

    println!("Browse to: {}", auth_url);

    let code = if redirect_port > 0 {
        get_authcode_listener(redirect_address)
    } else {
        get_authcode_stdin()
    };
    debug!("Exchange {code:?} for access token");

    // Do this sync in another thread because I am too stupid to make the async version work.
    let (tx, rx) = mpsc::channel();
    let client_clone = client.clone();
    std::thread::spawn(move || {
        let resp = client_clone
            .exchange_code(code)
            .set_pkce_verifier(pkce_verifier)
            .request(http_client);
        tx.send(resp).unwrap();
    });
    let token_response = rx.recv().unwrap();
    let token = match token_response {
        Ok(tok) => {
            trace!("Obtained new access token: {tok:?}");
            tok
        }
        Err(e) => {
            error!("Failed to exchange code for access token: {e:?}");
            exit(1);
        }
    };

    token.access_token().secret().to_string()
}
