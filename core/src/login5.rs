use crate::spclient::CLIENT_TOKEN;
use crate::token::Token;
use crate::{util, Error, SessionConfig};
use bytes::Bytes;
use http::{header::ACCEPT, HeaderValue, Method, Request};
use librespot_protocol::{
    client_info::ClientInfo,
    credentials::{Password, StoredCredential},
    hashcash::HashcashSolution,
    login5::{
        login_request::Login_method, ChallengeSolution, Challenges, LoginError, LoginOk,
        LoginRequest, LoginResponse,
    },
};
use protobuf::well_known_types::duration::Duration as ProtoDuration;
use protobuf::{Message, MessageField};
use std::env::consts::OS;
use std::time::{Duration, Instant};
use thiserror::Error;
use tokio::time::sleep;

const MAX_LOGIN_TRIES: u8 = 3;
const LOGIN_TIMEOUT: Duration = Duration::from_secs(3);

component! {
    Login5Manager : Login5ManagerInner {
        auth_token: Option<Token> = None,
    }
}

#[derive(Debug, Error)]
enum Login5Error {
    #[error("Requesting login failed: {0:?}")]
    FaultyRequest(LoginError),
    #[error("doesn't support code challenge")]
    CodeChallenge,
}

impl From<Login5Error> for Error {
    fn from(err: Login5Error) -> Self {
        Error::failed_precondition(err)
    }
}

impl Login5Manager {
    async fn request(&self, message: &LoginRequest) -> Result<Bytes, Error> {
        let client_token = self.session().spclient().client_token().await?;
        let body = message.write_to_bytes()?;

        let request = Request::builder()
            .method(&Method::POST)
            .uri("https://login5.spotify.com/v3/login")
            .header(ACCEPT, HeaderValue::from_static("application/x-protobuf"))
            .header(CLIENT_TOKEN, HeaderValue::from_str(&client_token)?)
            .body(body.into())?;

        self.session().http_client().request_body(request).await
    }

    fn new_login_request(&self, login: Login_method) -> LoginRequest {
        let client_id = match OS {
            "macos" | "windows" => self.session().client_id(),
            _ => SessionConfig::default().client_id,
        };

        LoginRequest {
            client_info: MessageField::some(ClientInfo {
                client_id,
                device_id: self.session().device_id().to_string(),
                special_fields: Default::default(),
            }),
            login_method: Some(login),
            ..Default::default()
        }
    }

    async fn login5_request(&self, login: Login_method) -> Result<LoginOk, Error> {
        let mut login_request = self.new_login_request(login.clone());

        let mut response = self.request(&login_request).await?;
        let mut count = 0;

        loop {
            count += 1;

            let message = LoginResponse::parse_from_bytes(&response)?;
            if message.has_ok() {
                break Ok(message.ok().to_owned());
            }

            if message.has_error() {
                match message.error() {
                    LoginError::TIMEOUT | LoginError::TOO_MANY_ATTEMPTS => {
                        sleep(LOGIN_TIMEOUT).await
                    }
                    others => return Err(Login5Error::FaultyRequest(others).into()),
                }
            }

            if message.has_challenges() {
                login_request.login_context = message.login_context.clone();
                Self::handle_challenges(&mut login_request, message.challenges())?;
            }

            if count < MAX_LOGIN_TRIES {
                response = self.request(&login_request).await?;
            } else {
                return Err(Error::failed_precondition(format!(
                    "Unable to solve any of {MAX_LOGIN_TRIES} hash cash challenges"
                )));
            }
        }
    }

    /// Login for android and ios
    ///
    /// This request doesn't require a connected session as it is the entrypoint for android or ios
    ///
    /// This request will only work when:
    /// - client_id => android or ios | can be easily adjusted in [SessionConfig::default_for_os]
    /// - user-agent => android or ios | has to be adjusted in [HttpClient::new](crate::http_client::HttpClient::new)
    pub async fn login(
        &self,
        id: impl Into<String>,
        password: impl Into<String>,
    ) -> Result<(Token, Vec<u8>), Error> {
        if !matches!(OS, "android" | "ios") {
            // by manipulating the user-agent and client-id it can be also used/tested on desktop
            return Err(Error::unavailable(
                "login5 login only works for android and ios",
            ));
        }

        let method = Login_method::Password(Password {
            id: id.into(),
            password: password.into(),
            ..Default::default()
        });

        let token_response = self.login5_request(method).await?;
        let auth_token = Self::token_from_login(&token_response);

        Ok((auth_token, token_response.stored_credential))
    }

    /// Retrieve the access_token via login5
    ///
    /// This request will only work when the store credentials match the client-id. Meaning that
    /// stored credentials generated with the keymaster client-id will not work, for example, with
    /// the android client-id.
    pub async fn auth_token(&self) -> Result<Token, Error> {
        let _lock = self.unique_lock().await?;

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

        let method = Login_method::StoredCredential(StoredCredential {
            username: self.session().username().to_string(),
            data: self.session().auth_data().clone(),
            ..Default::default()
        });

        let token_response = self.login5_request(method).await?;
        let auth_token = Self::token_from_login(&token_response);

        self.lock(|inner| {
            inner.auth_token = Some(auth_token.clone());
        });

        trace!("Got auth token: {:?}", auth_token);

        Ok(auth_token)
    }

    fn handle_challenges(
        login_request: &mut LoginRequest,
        challenges: &Challenges,
    ) -> Result<(), Error> {
        info!(
            "login5 response has {} challenges...",
            challenges.challenges.len()
        );

        for challenge in &challenges.challenges {
            if challenge.has_code() {
                debug!("empty challenge, skipping");
                return Err(Login5Error::CodeChallenge.into());
            } else if !challenge.has_hashcash() {
                debug!("empty challenge, skipping");
                continue;
            }

            let hash_cash_challenge = challenge.hashcash();

            let mut suffix = [0u8; 0x10];
            let duration = util::solve_hash_cash(
                &login_request.login_context,
                &hash_cash_challenge.prefix,
                hash_cash_challenge.length,
                &mut suffix,
            )?;

            let (seconds, nanos) = (duration.as_secs() as i64, duration.subsec_nanos() as i32);
            info!("solving login5 hashcash took {seconds}.{nanos}s");

            let mut solution = ChallengeSolution::new();
            solution.set_hashcash(HashcashSolution {
                suffix: Vec::from(suffix),
                duration: MessageField::some(ProtoDuration {
                    seconds,
                    nanos,
                    ..Default::default()
                }),
                ..Default::default()
            });

            login_request
                .challenge_solutions
                .mut_or_insert_default()
                .solutions
                .push(solution);
        }

        Ok(())
    }

    fn token_from_login(login: &LoginOk) -> Token {
        Token {
            access_token: login.access_token.clone(),
            expires_in: Duration::from_secs(
                login.access_token_expires_in.try_into().unwrap_or(3600),
            ),
            token_type: "Bearer".to_string(),
            scopes: vec![],
            timestamp: Instant::now(),
        }
    }
}
