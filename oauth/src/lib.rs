#![warn(missing_docs)]
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
use oauth2::basic::BasicTokenType;
use oauth2::reqwest::{async_http_client, http_client};
use oauth2::{
    basic::BasicClient, AuthUrl, AuthorizationCode, ClientId, CsrfToken, PkceCodeChallenge,
    RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use oauth2::{EmptyExtraTokenFields, PkceCodeVerifier, RefreshToken, StandardTokenResponse};
use std::io;
use std::sync::mpsc;
use std::time::{Duration, Instant};
use std::{
    io::{BufRead, BufReader, Write},
    net::{SocketAddr, TcpListener},
};
use thiserror::Error;
use url::Url;

/// Possible errors encountered during the OAuth authentication flow.
#[derive(Debug, Error)]
pub enum OAuthError {
    /// The redirect URI cannot be parsed as a valid URL.
    #[error("Unable to parse redirect URI {uri} ({e})")]
    AuthCodeBadUri {
        /// Auth URI.
        uri: String,
        /// Inner error code.
        e: url::ParseError,
    },

    /// The authorization code parameter is missing in the redirect URI.
    #[error("Auth code param not found in URI {uri}")]
    AuthCodeNotFound {
        /// Auth URI.
        uri: String,
    },

    /// Failed to read input from standard input when manually collecting auth code.
    #[error("Failed to read redirect URI from stdin")]
    AuthCodeStdinRead,

    /// Could not bind TCP listener to the specified socket address for OAuth callback.
    #[error("Failed to bind server to {addr} ({e})")]
    AuthCodeListenerBind {
        /// Callback address.
        addr: SocketAddr,
        /// Inner error code.
        e: io::Error,
    },

    /// Listener terminated before receiving an OAuth callback connection.
    #[error("Listener terminated without accepting a connection")]
    AuthCodeListenerTerminated,

    /// Failed to read incoming HTTP request containing OAuth callback.
    #[error("Failed to read redirect URI from HTTP request")]
    AuthCodeListenerRead,

    /// Received malformed HTTP request for OAuth callback.
    #[error("Failed to parse redirect URI from HTTP request")]
    AuthCodeListenerParse,

    /// Could not send HTTP response after handling OAuth callback.
    #[error("Failed to write HTTP response")]
    AuthCodeListenerWrite,

    /// Invalid Spotify authorization endpoint URL.
    #[error("Invalid Spotify OAuth URI")]
    InvalidSpotifyUri,

    /// Redirect URI failed validation.
    #[error("Invalid Redirect URI {uri} ({e})")]
    InvalidRedirectUri {
        /// Auth URI.
        uri: String,
        /// Inner error code
        e: url::ParseError,
    },

    /// Channel communication failure.
    #[error("Failed to receive code")]
    Recv,

    /// Token exchange failure with Spotify's authorization server.
    #[error("Failed to exchange code for access token ({e})")]
    ExchangeCode {
        /// Inner error description
        e: String,
    },
}

/// Represents an OAuth token used for accessing Spotify's Web API and sessions.
#[derive(Debug, Clone)]
pub struct OAuthToken {
    /// Bearer token used for authenticated Spotify API requests
    pub access_token: String,
    /// Long-lived token used to obtain new access tokens
    pub refresh_token: String,
    /// Instant when the access token becomes invalid
    pub expires_at: Instant,
    /// Type of token
    pub token_type: String,
    /// Permission scopes granted by this token
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

/// Spawn HTTP server at provided socket address to accept OAuth callback and return auth code.
fn get_authcode_listener(
    socket_address: SocketAddr,
    message: String,
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
    let code = get_code(&("http://localhost".to_string() + redirect_url));

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

// If the specified `redirect_uri` is HTTP and contains a port,
// then the corresponding socket address is returned.
fn get_socket_address(redirect_uri: &str) -> Option<SocketAddr> {
    let url = match Url::parse(redirect_uri) {
        Ok(u) if u.scheme() == "http" && u.port().is_some() => u,
        _ => return None,
    };
    match url.socket_addrs(|| None) {
        Ok(mut addrs) => addrs.pop(),
        _ => None,
    }
}

/// Struct that handle obtaining and refreshing access tokens.
pub struct OAuthClient {
    scopes: Vec<String>,
    redirect_uri: String,
    should_open_url: bool,
    message: String,
    client: BasicClient,
}

impl OAuthClient {
    /// Generates and opens/shows the authorization URL to obtain an access token.
    ///
    /// Returns a verifier that must be included in the final request for validation.
    fn set_auth_url(&self) -> PkceCodeVerifier {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
        // Generate the full authorization URL.
        // Some of these scopes are unavailable for custom client IDs. Which?
        let request_scopes: Vec<oauth2::Scope> =
            self.scopes.iter().map(|s| Scope::new(s.into())).collect();
        let (auth_url, _) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .add_scopes(request_scopes)
            .set_pkce_challenge(pkce_challenge)
            .url();

        if self.should_open_url {
            open::that_in_background(auth_url.as_str());
        }
        println!("Browse to: {auth_url}");

        pkce_verifier
    }

    fn build_token(
        &self,
        resp: StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    ) -> Result<OAuthToken, OAuthError> {
        trace!("Obtained new access token: {resp:?}");

        let token_scopes: Vec<String> = match resp.scopes() {
            Some(s) => s.iter().map(|s| s.to_string()).collect(),
            _ => self.scopes.clone(),
        };
        let refresh_token = match resp.refresh_token() {
            Some(t) => t.secret().to_string(),
            _ => "".to_string(), // Spotify always provides a refresh token.
        };
        Ok(OAuthToken {
            access_token: resp.access_token().secret().to_string(),
            refresh_token,
            expires_at: Instant::now()
                + resp
                    .expires_in()
                    .unwrap_or_else(|| Duration::from_secs(3600)),
            token_type: format!("{:?}", resp.token_type()),
            scopes: token_scopes,
        })
    }

    /// Syncronously obtain a Spotify access token using the authorization code with PKCE OAuth flow.
    pub fn get_access_token(&self) -> Result<OAuthToken, OAuthError> {
        let pkce_verifier = self.set_auth_url();

        let code = match get_socket_address(&self.redirect_uri) {
            Some(addr) => get_authcode_listener(addr, self.message.clone()),
            _ => get_authcode_stdin(),
        }?;
        trace!("Exchange {code:?} for access token");

        let (tx, rx) = mpsc::channel();
        let client = self.client.clone();
        std::thread::spawn(move || {
            let resp = client
                .exchange_code(code)
                .set_pkce_verifier(pkce_verifier)
                .request(http_client);
            if let Err(e) = tx.send(resp) {
                error!("OAuth channel send error: {e}");
            }
        });
        let channel_response = rx.recv().map_err(|_| OAuthError::Recv)?;
        let token_response =
            channel_response.map_err(|e| OAuthError::ExchangeCode { e: e.to_string() })?;

        self.build_token(token_response)
    }

    /// Synchronously obtain a new valid OAuth token from `refresh_token`
    pub fn refresh_token(&self, refresh_token: &str) -> Result<OAuthToken, OAuthError> {
        let refresh_token = RefreshToken::new(refresh_token.to_string());
        let resp = self
            .client
            .exchange_refresh_token(&refresh_token)
            .request(http_client);

        let resp = resp.map_err(|e| OAuthError::ExchangeCode { e: e.to_string() })?;
        self.build_token(resp)
    }

    /// Asyncronously obtain a Spotify access token using the authorization code with PKCE OAuth flow.
    pub async fn get_access_token_async(&self) -> Result<OAuthToken, OAuthError> {
        let pkce_verifier = self.set_auth_url();

        let code = match get_socket_address(&self.redirect_uri) {
            Some(addr) => get_authcode_listener(addr, self.message.clone()),
            _ => get_authcode_stdin(),
        }?;
        trace!("Exchange {code:?} for access token");

        let resp = self
            .client
            .exchange_code(code)
            .set_pkce_verifier(pkce_verifier)
            .request_async(async_http_client)
            .await;

        let resp = resp.map_err(|e| OAuthError::ExchangeCode { e: e.to_string() })?;
        self.build_token(resp)
    }

    /// Asynchronously obtain a new valid OAuth token from `refresh_token`
    pub async fn refresh_token_async(&self, refresh_token: &str) -> Result<OAuthToken, OAuthError> {
        let refresh_token = RefreshToken::new(refresh_token.to_string());
        let resp = self
            .client
            .exchange_refresh_token(&refresh_token)
            .request_async(async_http_client)
            .await;

        let resp = resp.map_err(|e| OAuthError::ExchangeCode { e: e.to_string() })?;
        self.build_token(resp)
    }
}

/// Builder struct through which structures of type OAuthClient are instantiated.
pub struct OAuthClientBuilder {
    client_id: String,
    redirect_uri: String,
    scopes: Vec<String>,
    should_open_url: bool,
    message: String,
}

impl OAuthClientBuilder {
    /// Create a new OAuthClientBuilder with provided params and default config.
    ///
    /// `redirect_uri` must match to the registered Uris of `client_id`
    pub fn new(client_id: &str, redirect_uri: &str, scopes: Vec<&str>) -> Self {
        Self {
            client_id: client_id.to_string(),
            redirect_uri: redirect_uri.to_string(),
            scopes: scopes.into_iter().map(Into::into).collect(),
            should_open_url: false,
            message: String::from("Go back to your terminal :)"),
        }
    }

    /// When this function is added to the building process pipeline, the auth url will be
    /// opened with the default web browser. Otherwise, it will be printed to standard output.
    pub fn open_in_browser(mut self) -> Self {
        self.should_open_url = true;
        self
    }

    /// When this function is added to the building process pipeline, the body of the response to
    /// the callback request will be `message`. This is useful to load custom HTMLs to that &str.
    pub fn with_custom_message(mut self, message: &str) -> Self {
        self.message = message.to_string();
        self
    }

    /// End of the building process pipeline. If Ok, a OAuthClient instance will be returned.
    pub fn build(self) -> Result<OAuthClient, OAuthError> {
        let auth_url = AuthUrl::new("https://accounts.spotify.com/authorize".to_string())
            .map_err(|_| OAuthError::InvalidSpotifyUri)?;
        let token_url = TokenUrl::new("https://accounts.spotify.com/api/token".to_string())
            .map_err(|_| OAuthError::InvalidSpotifyUri)?;
        let redirect_url = RedirectUrl::new(self.redirect_uri.clone()).map_err(|e| {
            OAuthError::InvalidRedirectUri {
                uri: self.redirect_uri.clone(),
                e,
            }
        })?;

        let client = BasicClient::new(
            ClientId::new(self.client_id.to_string()),
            None,
            auth_url,
            Some(token_url),
        )
        .set_redirect_uri(redirect_url);

        Ok(OAuthClient {
            scopes: self.scopes,
            should_open_url: self.should_open_url,
            message: self.message,
            redirect_uri: self.redirect_uri,
            client,
        })
    }
}

/// Obtain a Spotify access token using the authorization code with PKCE OAuth flow.
/// The `redirect_uri` must match what is registered to the client ID.
#[deprecated(
    since = "0.7.0",
    note = "please use builder pattern with `OAuthClientBuilder` instead"
)]
/// Obtain a Spotify access token using the authorization code with PKCE OAuth flow.
/// The redirect_uri must match what is registered to the client ID.
pub fn get_access_token(
    client_id: &str,
    redirect_uri: &str,
    scopes: Vec<&str>,
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
    let (auth_url, _) = client
        .authorize_url(CsrfToken::new_random)
        .add_scopes(request_scopes)
        .set_pkce_challenge(pkce_challenge)
        .url();

    println!("Browse to: {auth_url}");

    let code = match get_socket_address(redirect_uri) {
        Some(addr) => get_authcode_listener(addr, String::from("Go back to your terminal :)")),
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
        _ => scopes.into_iter().map(|s| s.to_string()).collect(),
    };
    let refresh_token = match token.refresh_token() {
        Some(t) => t.secret().to_string(),
        _ => "".to_string(), // Spotify always provides a refresh token.
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
        // Not http
        assert_eq!(get_socket_address("https://127.0.0.1/foo"), None);
    }

    #[test]
    fn get_socket_address_some() {
        let localhost_v4 = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 1234);
        let localhost_v6 = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), 8888);
        let addr_v4 = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)), 1234);
        let addr_v6 = SocketAddr::new(
            IpAddr::V6(Ipv6Addr::new(0x2001, 0x4860, 0x4860, 0, 0, 0, 0, 0x8888)),
            8888,
        );

        // Loopback addresses
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

        // Non-loopback addresses
        assert_eq!(get_socket_address("http://8.8.8.8:1234/foo"), Some(addr_v4));
        assert_eq!(
            get_socket_address("http://[2001:4860:4860::8888]:8888/foo"),
            Some(addr_v6)
        );
    }
}
