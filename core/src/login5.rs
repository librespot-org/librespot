use std::env::consts::OS;
use std::time::{Duration, Instant};
use bytes::Bytes;
use http::{HeaderValue, Method, Request};
use http::header::ACCEPT;
use protobuf::Message;
use librespot_protocol::login5::{LoginRequest, LoginResponse};
use crate::{Error, SessionConfig};
use crate::token::Token;

component! {
    Login5Manager : Login5ManagerInner {
        auth_token: Option<Token> = None,
    }
}

impl Login5Manager {
    async fn auth_token_request<M: Message>(&self, message: &M) -> Result<Bytes, Error> {
        let client_token = self.session().spclient().client_token().await?;
        let body = message.write_to_bytes()?;

        let request = Request::builder()
            .method(&Method::POST)
            .uri("https://login5.spotify.com/v3/login")
            .header(ACCEPT, HeaderValue::from_static("application/x-protobuf"))
            .header(crate::spclient::CLIENT_TOKEN, HeaderValue::from_str(&client_token)?)
            .body(body.into())?;

        self.session().http_client().request_body(request).await
    }
    
    pub async fn auth_token(&self) -> Result<Token, Error> {
        let auth_token = self.lock(|inner| {
            if let Some(token) = &inner.auth_token {
                if token.is_expired() {
                    inner.auth_token = None;
                }
            }
            inner.auth_token.clone()
        });

        if let Some(auth_token) = auth_token {
            return Ok(auth_token);
        }

        let client_id = match OS {
            "macos" | "windows" => self.session().client_id(),
            _ => SessionConfig::default().client_id,
        };

        let mut login_request = LoginRequest::new();
        login_request.client_info.mut_or_insert_default().client_id = client_id;
        login_request.client_info.mut_or_insert_default().device_id =
            self.session().device_id().to_string();

        let stored_credential = login_request.mut_stored_credential();
        stored_credential.username = self.session().username().to_string();
        stored_credential.data = self.session().auth_data().clone();

        let mut response = self.auth_token_request(&login_request).await?;
        let mut count = 0;
        const MAX_TRIES: u8 = 3;

        let token_response = loop {
            count += 1;

            let message = LoginResponse::parse_from_bytes(&response)?;
            // TODO: Handle hash cash stuff
            if message.has_ok() {
                break message.ok().to_owned();
            }

            if count < MAX_TRIES {
                response = self.auth_token_request(&login_request).await?;
            } else {
                return Err(Error::failed_precondition(format!(
                    "Unable to solve any of {MAX_TRIES} hash cash challenges"
                )));
            }
        };

        let auth_token = Token {
            access_token: token_response.access_token.clone(),
            expires_in: Duration::from_secs(
                token_response
                    .access_token_expires_in
                    .try_into()
                    .unwrap_or(3600),
            ),
            token_type: "Bearer".to_string(),
            scopes: vec![],
            timestamp: Instant::now(),
        };
        self.lock(|inner| {
            inner.auth_token = Some(auth_token.clone());
        });

        trace!("Got auth token: {:?}", auth_token);

        Ok(auth_token)
    }
}