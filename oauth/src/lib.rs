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

use log::{error, info, trace};
use oauth2::reqwest::http_client;
use oauth2::{
    basic::BasicClient, AuthUrl, AuthorizationCode, ClientId, CsrfToken, PkceCodeChallenge,
    RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use std::io;
use std::time::{Duration, Instant};
use std::{
    io::{BufRead, BufReader, Write},
    net::{SocketAddr, TcpListener},
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

    #[error("CSRF token param not found in URI {uri}")]
    CsrfTokenNotFound { uri: String },

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

    #[error("Failed to receive code")]
    Recv,

    #[error("Failed to exchange code for access token ({e})")]
    ExchangeCode { e: String },

    #[error("Spotify did not provide a refresh token")]
    NoRefreshToken,

    #[error("Spotify did not return the token scopes")]
    NoTokenScopes,
}

#[derive(Debug)]
pub struct OAuthToken {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: Instant,
    pub token_type: String,
    pub scopes: Vec<String>,
}

/// Return URL from the redirect URI &str.
fn get_url(redirect_url: &str) -> Result<Url, OAuthError> {
    let url = Url::parse(redirect_url).map_err(|e| OAuthError::AuthCodeBadUri {
        uri: redirect_url.to_string(),
        e,
    })?;
    Ok(url)
}

/// Return a query-string parameter from the redirect URI.
fn get_query_string_parameter(url: &Url, query_string_parameter_key: &str) -> Option<String> {
    url.query_pairs()
        .find(|(key, _)| key == query_string_parameter_key)
        .map(|(_, query_string_parameter)| query_string_parameter.into_owned())
}

/// Return state query-string parameter from the redirect URI (CSRF token).
fn get_state(url: &Url) -> Result<String, OAuthError> {
    let state = get_query_string_parameter(url, "state").ok_or(OAuthError::CsrfTokenNotFound {
        uri: url.to_string(),
    })?;

    Ok(state)
}

/// Return code query-string parameter from the redirect URI.
fn get_code(url: &Url) -> Result<AuthorizationCode, OAuthError> {
    let code = get_query_string_parameter(url, "code")
        .map(AuthorizationCode::new)
        .ok_or(OAuthError::AuthCodeNotFound {
            uri: url.to_string(),
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

    let url = get_url(buffer.trim())?;
    get_code(&url)
}

/// Spawn HTTP server at provided socket address to accept OAuth callback and return auth code.
fn get_authcode_listener(
    socket_address: SocketAddr,
    csrf_token: CsrfToken,
    success_message: Option<String>,
) -> Result<AuthorizationCode, OAuthError> {
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

    let url = get_url(&("http://localhost".to_string() + redirect_url))?;

    let token = get_state(&url)?;
    if !token.eq(csrf_token.secret()) {
        return Err(OAuthError::CsrfTokenNotFound {
            uri: redirect_url.to_string(),
        });
    }
    let code = get_code(&url)?;

    let message = "Go back to your terminal :)";
    let response = format!(
        "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
        message.len(),
        success_message.unwrap_or(message.to_owned())
    );
    stream
        .write_all(response.as_bytes())
        .map_err(|_| OAuthError::AuthCodeListenerWrite)?;

    Ok(code)
}

// If the specified `redirect_uri` is HTTP, loopback, and contains a port,
// then the corresponding socket address is returned.
fn get_socket_address(redirect_uri: &str) -> Option<SocketAddr> {
    let url = match Url::parse(redirect_uri) {
        Ok(u) if u.scheme() == "http" && u.port().is_some() => u,
        _ => return None,
    };
    let socket_addr = match url.socket_addrs(|| None) {
        Ok(mut addrs) => addrs.pop(),
        _ => None,
    };
    if let Some(s) = socket_addr {
        if s.ip().is_loopback() {
            return socket_addr;
        }
    }
    None
}

/// Obtain a Spotify access token using the authorization code with PKCE OAuth flow.
/// The redirect_uri must match what is registered to the client ID.
pub fn get_access_token(
    client_id: &str,
    redirect_uri: &str,
    scopes: Vec<&str>,
    success_message: Option<String>,
) -> Result<OAuthToken, OAuthError> {
    let auth_url = AuthUrl::new("https://accounts.spotify.com/authorize".to_string())
        .map_err(|_| OAuthError::InvalidSpotifyUri)?;
    let token_url = TokenUrl::new("https://accounts.spotify.com/api/token".to_string())
        .map_err(|_| OAuthError::InvalidSpotifyUri)?;
    let redirect_url =
        RedirectUrl::new(redirect_uri.to_string()).map_err(|e| OAuthError::InvalidRedirectUri {
            uri: redirect_uri.to_string(),
            e,
        })?;
    let client = BasicClient::new(
        ClientId::new(client_id.to_string()),
        None,
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(redirect_url);

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the full authorization URL.
    // Some of these scopes are unavailable for custom client IDs. Which?
    let request_scopes: Vec<oauth2::Scope> = scopes
        .clone()
        .into_iter()
        .map(|s| Scope::new(s.into()))
        .collect();
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scopes(request_scopes)
        .set_pkce_challenge(pkce_challenge)
        .url();

    println!("Browse to: {}", auth_url);
    if let Err(err) = open::that(auth_url.to_string()) {
        eprintln!("An error occurred when opening '{}': {}", auth_url, err)
    }

    let code = match get_socket_address(redirect_uri) {
        Some(addr) => get_authcode_listener(addr, csrf_token, success_message),
        _ => get_authcode_stdin(),
    }?;
    trace!("Exchange {code:?} for access token");

    // Do this sync in another thread because I am too stupid to make the async version work.
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let resp = client
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
        None => {
            error!("Spotify did not return the token scopes.");
            return Err(OAuthError::NoTokenScopes);
        }
    };
    let refresh_token = match token.refresh_token() {
        Some(t) => t.secret().to_string(),
        None => {
            error!("Spotify did not provide a refresh token.");
            return Err(OAuthError::NoRefreshToken);
        }
    };
    Ok(OAuthToken {
        access_token: token.access_token().secret().to_string(),
        refresh_token,
        expires_at: Instant::now()
            + token
                .expires_in()
                .unwrap_or_else(|| Duration::from_secs(3600)),
        token_type: format!("{:?}", token.token_type()).to_string(), // Urgh!?
        scopes: token_scopes,
    })
}

#[cfg(test)]
mod test {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

    use super::*;

    #[test]
    fn get_socket_address_none() {
        // No port
        assert_eq!(get_socket_address("http://127.0.0.1/foo"), None);
        assert_eq!(get_socket_address("http://127.0.0.1:/foo"), None);
        assert_eq!(get_socket_address("http://[::1]/foo"), None);
        // Not localhost
        assert_eq!(get_socket_address("http://56.0.0.1:1234/foo"), None);
        assert_eq!(
            get_socket_address("http://[3ffe:2a00:100:7031::1]:1234/foo"),
            None
        );
        // Not http
        assert_eq!(get_socket_address("https://127.0.0.1/foo"), None);
    }

    #[test]
    fn get_socket_address_localhost() {
        let localhost_v4 = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 1234);
        let localhost_v6 = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), 8888);

        assert_eq!(
            get_socket_address("http://127.0.0.1:1234/foo"),
            Some(localhost_v4)
        );
        assert_eq!(
            get_socket_address("http://[0:0:0:0:0:0:0:1]:8888/foo"),
            Some(localhost_v6)
        );
        assert_eq!(
            get_socket_address("http://[::1]:8888/foo"),
            Some(localhost_v6)
        );
    }
    #[test]
    fn test_get_url_valid() {
        let redirect_url = "https://example.com/callback?code=1234&state=abcd";
        let result = get_url(redirect_url);
        assert!(result.is_ok());
        let url = result.unwrap();
        assert_eq!(url.as_str(), redirect_url);
    }

    #[test]
    fn test_get_url_invalid() {
        let redirect_url = "invalid_url";
        let result = get_url(redirect_url);
        assert!(result.is_err());
        if let Err(OAuthError::AuthCodeBadUri { uri, .. }) = result {
            assert_eq!(uri, redirect_url.to_string());
        } else {
            panic!("Expected OAuthError::AuthCodeBadUri");
        }
    }

    #[test]
    fn test_get_query_string_parameter_found() {
        let url = Url::parse("https://example.com/callback?code=1234&state=abcd").unwrap();
        let key = "code";
        let result = get_query_string_parameter(&url, key);
        assert_eq!(result, Some("1234".to_string()));
    }

    #[test]
    fn test_get_query_string_parameter_not_found() {
        let url = Url::parse("https://example.com/callback?code=1234&state=abcd").unwrap();
        let key = "missing_key";
        let result = get_query_string_parameter(&url, key);
        assert!(result.is_none());
    }

    #[test]
    fn test_get_state_valid() {
        let url = Url::parse("https://example.com/callback?state=abcd").unwrap();
        let result = get_state(&url);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "abcd");
    }

    #[test]
    fn test_get_state_missing() {
        let url = Url::parse("https://example.com/callback").unwrap();
        let result = get_state(&url);
        assert!(result.is_err());
        if let Err(OAuthError::CsrfTokenNotFound { uri }) = result {
            assert_eq!(uri, url.to_string());
        } else {
            panic!("Expected OAuthError::CsrfTokenNotFound");
        }
    }

    #[test]
    fn test_get_code_valid() {
        let url = Url::parse("https://example.com/callback?code=1234").unwrap();
        let result = get_code(&url);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().secret(), "1234");
    }

    #[test]
    fn test_get_code_missing() {
        let url = Url::parse("https://example.com/callback").unwrap();
        let result = get_code(&url);
        assert!(result.is_err());
        if let Err(OAuthError::AuthCodeNotFound { uri }) = result {
            assert_eq!(uri, url.to_string());
        } else {
            panic!("Expected OAuthError::AuthCodeNotFound");
        }
    }
}
