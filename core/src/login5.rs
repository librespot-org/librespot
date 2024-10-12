use crate::spclient::CLIENT_TOKEN;
use crate::token::Token;
use crate::{util, Error, SessionConfig};
use bytes::Bytes;
use http::header::ACCEPT;
use http::{HeaderValue, Method, Request};
use librespot_protocol::hashcash::HashcashSolution;
use librespot_protocol::login5::{
    ChallengeSolution, Challenges, LoginError, LoginRequest, LoginResponse,
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
    async fn auth_token_request(&self, message: &LoginRequest) -> Result<Bytes, Error> {
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

        let token_response = loop {
            count += 1;

            let message = LoginResponse::parse_from_bytes(&response)?;
            if message.has_ok() {
                break message.ok().to_owned();
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
                Self::handle_challenges(&mut login_request, message.challenges())?
            }

            if count < MAX_LOGIN_TRIES {
                response = self.auth_token_request(&login_request).await?;
            } else {
                return Err(Error::failed_precondition(format!(
                    "Unable to solve any of {MAX_LOGIN_TRIES} hash cash challenges"
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
}
