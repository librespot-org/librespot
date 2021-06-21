// Ported from librespot-java. Relicensed under MIT with permission.

// Known tokens:
//   ugc-image-upload, playlist-read-collaborative, playlist-modify-private,
//   playlist-modify-public, playlist-read-private, user-read-playback-position,
//   user-read-recently-played, user-top-read, user-modify-playback-state,
//   user-read-currently-playing, user-read-playback-state, user-read-private, user-read-email,
//   user-library-modify, user-library-read, user-follow-modify, user-follow-read, streaming,
//   app-remote-control

use crate::mercury::MercuryError;

use serde::Deserialize;

use std::error::Error;
use std::time::{Duration, Instant};

component! {
    TokenProvider : TokenProviderInner {
        tokens: Vec<Token> = vec![],
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    access_token: String,
    expires_in: Duration,
    token_type: String,
    scopes: Vec<String>,
    timestamp: Instant,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TokenData {
    access_token: String,
    expires_in: u64,
    token_type: String,
    scope: Vec<String>,
}

impl TokenProvider {
    const KEYMASTER_CLIENT_ID: &'static str = "65b708073fc0480ea92a077233ca87bd";

    fn find_token(&self, scopes: Vec<&str>) -> Option<usize> {
        self.lock(|inner| {
            for i in 0..inner.tokens.len() {
                if inner.tokens[i].in_scopes(scopes.clone()) {
                    return Some(i);
                }
            }
            None
        })
    }

    // scopes must be comma-separated
    pub async fn get_token(&self, scopes: &str) -> Result<Token, MercuryError> {
        if scopes.is_empty() {
            return Err(MercuryError);
        }

        if let Some(index) = self.find_token(scopes.split(',').collect()) {
            let cached_token = self.lock(|inner| inner.tokens[index].clone());
            if cached_token.is_expired() {
                self.lock(|inner| inner.tokens.remove(index));
            } else {
                return Ok(cached_token);
            }
        }

        trace!(
            "Requested token in scopes {:?} unavailable or expired, requesting new token.",
            scopes
        );

        let query_uri = format!(
            "hm://keymaster/token/authenticated?scope={}&client_id={}&device_id={}",
            scopes,
            Self::KEYMASTER_CLIENT_ID,
            self.session().device_id()
        );
        let request = self.session().mercury().get(query_uri);
        let response = request.await?;
        let data = response
            .payload
            .first()
            .expect("No tokens received")
            .to_vec();
        let token = Token::new(String::from_utf8(data).unwrap()).map_err(|_| MercuryError)?;
        trace!("Got token: {:?}", token);
        self.lock(|inner| inner.tokens.push(token.clone()));
        Ok(token)
    }
}

impl Token {
    const EXPIRY_THRESHOLD: Duration = Duration::from_secs(10);

    pub fn new(body: String) -> Result<Self, Box<dyn Error>> {
        let data: TokenData = serde_json::from_slice(body.as_ref())?;
        Ok(Self {
            access_token: data.access_token,
            expires_in: Duration::from_secs(data.expires_in),
            token_type: data.token_type,
            scopes: data.scope,
            timestamp: Instant::now(),
        })
    }

    pub fn is_expired(&self) -> bool {
        self.timestamp + (self.expires_in - Self::EXPIRY_THRESHOLD) < Instant::now()
    }

    pub fn in_scope(&self, scope: &str) -> bool {
        for s in &self.scopes {
            if *s == scope {
                return true;
            }
        }
        false
    }

    pub fn in_scopes(&self, scopes: Vec<&str>) -> bool {
        for s in scopes {
            if !self.in_scope(s) {
                return false;
            }
        }
        true
    }
}
