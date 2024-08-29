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
use std::time::{Duration, Instant};
use std::{
    io::{BufRead, BufReader, Write},
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener},
    sync::mpsc,
};
use thiserror::Error;
use url::Url;

#[derive(Debug, Error)]
pub enum OAuthError {
    #[error("Unable to parse redirect URI {uri} ({e})")]
    AuthCodeBadUri { uri: String, e: url::ParseError },

    #[error("Auth code param not found in URI {uri}")]
    AuthCodeNotFound { uri: String },

    #[error("Failed to read redirect URI from stdin")]
    AuthCodeStdinRead,

    #[error("Failed to bind server to {addr} ({e})")]
    AuthCodeListenerBind { addr: SocketAddr, e: io::Error },

    #[error("Listener terminated without accepting a connection")]
    AuthCodeListenerTerminated,

    #[error("Failed to read redirect URI from HTTP request")]
    AuthCodeListenerRead,

    #[error("Failed to parse redirect URI from HTTP request")]
    AuthCodeListenerParse,

    #[error("Failed to write HTTP response")]
    AuthCodeListenerWrite,

    #[error("Invalid Spotify OAuth URI")]
    InvalidSpotifyUri,

    #[error("Invalid Redirect URI {uri} ({e})")]
    InvalidRedirectUri { uri: String, e: url::ParseError },

    #[error("Failed to recieve code")]
    Recv,

    #[error("Failed to exchange code for access token ({e})")]
    ExchangeCode { e: String },
}

#[derive(Debug)]
pub struct AccessToken {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: Instant,
    pub token_type: String,
    pub scopes: Vec<String>,
}

/// Return code query-string parameter from the redirect URI.
fn get_code(redirect_url: &str) -> Result<AuthorizationCode, OAuthError> {
    let url = Url::parse(redirect_url).map_err(|e| OAuthError::AuthCodeBadUri {
        uri: redirect_url.to_string(),
        e,
    })?;
    let code = url
        .query_pairs()
        .find(|(key, _)| key == "code")
        .map(|(_, code)| AuthorizationCode::new(code.into_owned()))
        .ok_or(OAuthError::AuthCodeNotFound {
            uri: redirect_url.to_string(),
        })?;

    Ok(code)
}

/// Prompt for redirect URI on stdin and return auth code.
fn get_authcode_stdin() -> Result<AuthorizationCode, OAuthError> {
    println!("Provide redirect URL");
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin
        .read_line(&mut buffer)
        .map_err(|_| OAuthError::AuthCodeStdinRead)?;

    get_code(buffer.trim())
}

/// Spawn HTTP server on provided socket to accept OAuth callback and return auth code.
fn get_authcode_listener(socket_address: SocketAddr) -> Result<AuthorizationCode, OAuthError> {
    let listener =
        TcpListener::bind(socket_address).map_err(|e| OAuthError::AuthCodeListenerBind {
            addr: socket_address,
            e,
        })?;
    info!("OAuth server listening on {:?}", socket_address);

    // The server will terminate itself after collecting the first code.
    let mut stream = listener
        .incoming()
        .flatten()
        .next()
        .ok_or(OAuthError::AuthCodeListenerTerminated)?;
    let mut reader = BufReader::new(&stream);
    let mut request_line = String::new();
    reader
        .read_line(&mut request_line)
        .map_err(|_| OAuthError::AuthCodeListenerRead)?;

    let redirect_url = request_line
        .split_whitespace()
        .nth(1)
        .ok_or(OAuthError::AuthCodeListenerParse)?;
    let code = get_code(&("http://localhost".to_string() + redirect_url));

    let message = "Go back to your terminal :)";
    let response = format!(
        "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
        message.len(),
        message
    );
    stream
        .write_all(response.as_bytes())
        .map_err(|_| OAuthError::AuthCodeListenerWrite)?;

    code
}

// TODO: Pass in redirect_address instead since the redirect host depends on client ID?
/// Obtain a Spotify access token using the authorization code with PKCE OAuth flow.
pub fn get_access_token(
    client_id: &str,
    scopes: Vec<&str>,
    redirect_port: Option<u16>,
) -> Result<AccessToken, OAuthError> {
    // Must use host 127.0.0.1 with Spotify Desktop client ID.
    let redirect_address = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        redirect_port.unwrap_or_default(),
    );
    let redirect_uri = format!("http://{redirect_address}/login");

    let auth_url = AuthUrl::new("https://accounts.spotify.com/authorize".to_string())
        .map_err(|_| OAuthError::InvalidSpotifyUri)?;
    let token_url = TokenUrl::new("https://accounts.spotify.com/api/token".to_string())
        .map_err(|_| OAuthError::InvalidSpotifyUri)?;
    let client = BasicClient::new(
        ClientId::new(client_id.to_string()),
        None,
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_uri.clone()).map_err(|e| {
        OAuthError::InvalidRedirectUri {
            uri: redirect_uri,
            e,
        }
    })?);

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the full authorization URL.
    // Some of these scopes are unavailable for custom client IDs. Which?
    let request_scopes: Vec<oauth2::Scope> = scopes
        .clone()
        .into_iter()
        .map(|s| Scope::new(s.into()))
        .collect();
    let (auth_url, _) = client
        .authorize_url(CsrfToken::new_random)
        .add_scopes(request_scopes)
        .set_pkce_challenge(pkce_challenge)
        .url();

    println!("Browse to: {}", auth_url);

    let code = if redirect_port.is_some() {
        get_authcode_listener(redirect_address)
    } else {
        get_authcode_stdin()
    }?;
    debug!("Exchange {code:?} for access token");

    // Do this sync in another thread because I am too stupid to make the async version work.
    let (tx, rx) = mpsc::channel();
    let client_clone = client.clone();
    std::thread::spawn(move || {
        let resp = client_clone
            .exchange_code(code)
            .set_pkce_verifier(pkce_verifier)
            .request(http_client);
        if let Err(e) = tx.send(resp) {
            error!("OAuth channel send error: {e}");
        }
    });
    let token_response = rx.recv().map_err(|_| OAuthError::Recv)?;
    let token = token_response.map_err(|e| OAuthError::ExchangeCode { e: e.to_string() })?;
    trace!("Obtained new access token: {token:?}");

    let token_scopes: Vec<String> = match token.scopes() {
        Some(s) => s.iter().map(|s| s.to_string()).collect(),
        _ => scopes.into_iter().map(|s| s.to_string()).collect(),
    };
    Ok(AccessToken {
        access_token: token.access_token().secret().to_string(),
        refresh_token: token.refresh_token().unwrap().secret().to_string(),
        expires_at: Instant::now() + token.expires_in().unwrap_or(Duration::from_secs(3600)),
        token_type: format!("{:?}", token.token_type()).to_string(), // Urgh!?
        scopes: token_scopes,
    })
}
