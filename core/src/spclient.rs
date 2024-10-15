use std::{
    env::consts::OS,
    fmt::Write,
    time::{Duration, Instant},
};

use bytes::Bytes;
use data_encoding::HEXUPPER_PERMISSIVE;
use futures_util::future::IntoStream;
use http::header::HeaderValue;
use hyper::{
    header::{HeaderName, ACCEPT, AUTHORIZATION, CONTENT_TYPE, RANGE},
    HeaderMap, Method, Request,
};
use hyper_util::client::legacy::ResponseFuture;
use protobuf::{Enum, Message, MessageFull};
use rand::RngCore;
use sysinfo::System;
use thiserror::Error;

use crate::{
    apresolve::SocketAddress,
    cdn_url::CdnUrl,
    config::SessionConfig,
    error::ErrorKind,
    protocol::{
        canvaz::EntityCanvazRequest,
        clienttoken_http::{
            ChallengeAnswer, ChallengeType, ClientTokenRequest, ClientTokenRequestType,
            ClientTokenResponse, ClientTokenResponseType,
        },
        connect::PutStateRequest,
        extended_metadata::BatchedEntityRequest,
    },
    token::Token,
    util,
    version::spotify_semantic_version,
    Error, FileId, SpotifyId,
};

component! {
    SpClient : SpClientInner {
        accesspoint: Option<SocketAddress> = None,
        strategy: RequestStrategy = RequestStrategy::default(),
        client_token: Option<Token> = None,
    }
}

pub type SpClientResult = Result<Bytes, Error>;

#[allow(clippy::declare_interior_mutable_const)]
pub const CLIENT_TOKEN: HeaderName = HeaderName::from_static("client-token");

#[derive(Debug, Error)]
pub enum SpClientError {
    #[error("missing attribute {0}")]
    Attribute(String),
}

impl From<SpClientError> for Error {
    fn from(err: SpClientError) -> Self {
        Self::failed_precondition(err)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum RequestStrategy {
    TryTimes(usize),
    Infinitely,
}

impl Default for RequestStrategy {
    fn default() -> Self {
        RequestStrategy::TryTimes(10)
    }
}

impl SpClient {
    pub fn set_strategy(&self, strategy: RequestStrategy) {
        self.lock(|inner| inner.strategy = strategy)
    }

    pub async fn flush_accesspoint(&self) {
        self.lock(|inner| inner.accesspoint = None)
    }

    pub async fn get_accesspoint(&self) -> Result<SocketAddress, Error> {
        // Memoize the current access point.
        let ap = self.lock(|inner| inner.accesspoint.clone());
        let tuple = match ap {
            Some(tuple) => tuple,
            None => {
                let tuple = self.session().apresolver().resolve("spclient").await?;
                self.lock(|inner| inner.accesspoint = Some(tuple.clone()));
                info!(
                    "Resolved \"{}:{}\" as spclient access point",
                    tuple.0, tuple.1
                );
                tuple
            }
        };
        Ok(tuple)
    }

    pub async fn base_url(&self) -> Result<String, Error> {
        let ap = self.get_accesspoint().await?;
        Ok(format!("https://{}:{}", ap.0, ap.1))
    }

    async fn client_token_request<M: Message>(&self, message: &M) -> Result<Bytes, Error> {
        let body = message.write_to_bytes()?;

        let request = Request::builder()
            .method(&Method::POST)
            .uri("https://clienttoken.spotify.com/v1/clienttoken")
            .header(ACCEPT, HeaderValue::from_static("application/x-protobuf"))
            .body(body.into())?;

        self.session().http_client().request_body(request).await
    }

    pub async fn client_token(&self) -> Result<String, Error> {
        let client_token = self.lock(|inner| {
            if let Some(token) = &inner.client_token {
                if token.is_expired() {
                    inner.client_token = None;
                }
            }
            inner.client_token.clone()
        });

        if let Some(client_token) = client_token {
            return Ok(client_token.access_token);
        }

        debug!("Client token unavailable or expired, requesting new token.");

        let mut request = ClientTokenRequest::new();
        request.request_type = ClientTokenRequestType::REQUEST_CLIENT_DATA_REQUEST.into();

        let client_data = request.mut_client_data();

        client_data.client_version = spotify_semantic_version();

        // Current state of affairs: keymaster ID works on all tested platforms, but may be phased out,
        // so it seems a good idea to mimick the real clients. `self.session().client_id()` returns the
        // ID of the client that last connected, but requesting a client token with this ID only works
        // on macOS and Windows. On Android and iOS we can send a platform-specific client ID and are
        // then presented with a hash cash challenge. On Linux, we have to pass the old keymaster ID.
        // We delegate most of this logic to `SessionConfig`.
        let os = OS;
        let client_id = match os {
            "macos" | "windows" => self.session().client_id(),
            os => SessionConfig::default_for_os(os).client_id,
        };
        client_data.client_id = client_id;

        let connectivity_data = client_data.mut_connectivity_sdk_data();
        connectivity_data.device_id = self.session().device_id().to_string();

        let platform_data = connectivity_data
            .platform_specific_data
            .mut_or_insert_default();

        let os_version = System::os_version().unwrap_or_else(|| String::from("0"));
        let kernel_version = System::kernel_version().unwrap_or_else(|| String::from("0"));

        match os {
            "windows" => {
                let os_version = os_version.parse::<f32>().unwrap_or(10.) as i32;
                let kernel_version = kernel_version.parse::<i32>().unwrap_or(21370);

                let (pe, image_file) = match std::env::consts::ARCH {
                    "arm" => (448, 452),
                    "aarch64" => (43620, 452),
                    "x86_64" => (34404, 34404),
                    _ => (332, 332), // x86
                };

                let windows_data = platform_data.mut_desktop_windows();
                windows_data.os_version = os_version;
                windows_data.os_build = kernel_version;
                windows_data.platform_id = 2;
                windows_data.unknown_value_6 = 9;
                windows_data.image_file_machine = image_file;
                windows_data.pe_machine = pe;
                windows_data.unknown_value_10 = true;
            }
            "ios" => {
                let ios_data = platform_data.mut_ios();
                ios_data.user_interface_idiom = 0;
                ios_data.target_iphone_simulator = false;
                ios_data.hw_machine = "iPhone14,5".to_string();
                // example system_version: 17
                ios_data.system_version = os_version;
            }
            "android" => {
                let android_data = platform_data.mut_android();
                // example android_version: 30
                android_data.android_version = os_version;
                android_data.api_version = 31;
                "Pixel".clone_into(&mut android_data.device_name);
                "GF5KQ".clone_into(&mut android_data.model_str);
                "Google".clone_into(&mut android_data.vendor);
            }
            "macos" => {
                let macos_data = platform_data.mut_desktop_macos();
                macos_data.system_version = os_version;
                macos_data.hw_model = "iMac21,1".to_string();
                macos_data.compiled_cpu_type = std::env::consts::ARCH.to_string();
            }
            _ => {
                let linux_data = platform_data.mut_desktop_linux();
                linux_data.system_name = "Linux".to_string();
                linux_data.system_release = kernel_version;
                linux_data.system_version = os_version;
                linux_data.hardware = std::env::consts::ARCH.to_string();
            }
        }

        let mut response = self.client_token_request(&request).await?;
        let mut count = 0;
        const MAX_TRIES: u8 = 3;

        let token_response = loop {
            count += 1;

            let message = ClientTokenResponse::parse_from_bytes(&response)?;

            match ClientTokenResponseType::from_i32(message.response_type.value()) {
                // depending on the platform, you're either given a token immediately
                // or are presented a hash cash challenge to solve first
                Some(ClientTokenResponseType::RESPONSE_GRANTED_TOKEN_RESPONSE) => {
                    debug!("Received a granted token");
                    break message;
                }
                Some(ClientTokenResponseType::RESPONSE_CHALLENGES_RESPONSE) => {
                    debug!("Received a hash cash challenge, solving...");

                    let challenges = message.challenges().clone();
                    let state = challenges.state;
                    if let Some(challenge) = challenges.challenges.first() {
                        let hash_cash_challenge = challenge.evaluate_hashcash_parameters();

                        let ctx = vec![];
                        let prefix = HEXUPPER_PERMISSIVE
                            .decode(hash_cash_challenge.prefix.as_bytes())
                            .map_err(|e| {
                                Error::failed_precondition(format!(
                                    "Unable to decode hash cash challenge: {e}"
                                ))
                            })?;
                        let length = hash_cash_challenge.length;

                        let mut suffix = [0u8; 0x10];
                        let answer = util::solve_hash_cash(&ctx, &prefix, length, &mut suffix);

                        match answer {
                            Ok(_) => {
                                // the suffix must be in uppercase
                                let suffix = HEXUPPER_PERMISSIVE.encode(&suffix);

                                let mut answer_message = ClientTokenRequest::new();
                                answer_message.request_type =
                                    ClientTokenRequestType::REQUEST_CHALLENGE_ANSWERS_REQUEST
                                        .into();

                                let challenge_answers = answer_message.mut_challenge_answers();

                                let mut challenge_answer = ChallengeAnswer::new();
                                challenge_answer.mut_hash_cash().suffix = suffix;
                                challenge_answer.ChallengeType =
                                    ChallengeType::CHALLENGE_HASH_CASH.into();

                                challenge_answers.state = state.to_string();
                                challenge_answers.answers.push(challenge_answer);

                                trace!("Answering hash cash challenge");
                                match self.client_token_request(&answer_message).await {
                                    Ok(token) => {
                                        response = token;
                                        continue;
                                    }
                                    Err(e) => {
                                        trace!(
                                            "Answer not accepted {}/{}: {}",
                                            count,
                                            MAX_TRIES,
                                            e
                                        );
                                    }
                                }
                            }
                            Err(e) => trace!(
                                "Unable to solve hash cash challenge {}/{}: {}",
                                count,
                                MAX_TRIES,
                                e
                            ),
                        }

                        if count < MAX_TRIES {
                            response = self.client_token_request(&request).await?;
                        } else {
                            return Err(Error::failed_precondition(format!(
                                "Unable to solve any of {MAX_TRIES} hash cash challenges"
                            )));
                        }
                    } else {
                        return Err(Error::failed_precondition("No challenges found"));
                    }
                }

                Some(unknown) => {
                    return Err(Error::unimplemented(format!(
                        "Unknown client token response type: {unknown:?}"
                    )))
                }
                None => return Err(Error::failed_precondition("No client token response type")),
            }
        };

        let granted_token = token_response.granted_token();
        let access_token = granted_token.token.to_owned();

        self.lock(|inner| {
            let client_token = Token {
                access_token: access_token.clone(),
                expires_in: Duration::from_secs(
                    granted_token
                        .refresh_after_seconds
                        .try_into()
                        .unwrap_or(7200),
                ),
                token_type: "client-token".to_string(),
                scopes: granted_token
                    .domains
                    .iter()
                    .map(|d| d.domain.clone())
                    .collect(),
                timestamp: Instant::now(),
            };

            inner.client_token = Some(client_token);
        });

        trace!("Got client token: {:?}", granted_token);

        Ok(access_token)
    }

    pub async fn request_with_protobuf<M: Message + MessageFull>(
        &self,
        method: &Method,
        endpoint: &str,
        headers: Option<HeaderMap>,
        message: &M,
    ) -> SpClientResult {
        let body = protobuf::text_format::print_to_string(message);

        let mut headers = headers.unwrap_or_default();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-protobuf"),
        );

        self.request(method, endpoint, Some(headers), Some(&body))
            .await
    }

    pub async fn request_as_json(
        &self,
        method: &Method,
        endpoint: &str,
        headers: Option<HeaderMap>,
        body: Option<&str>,
    ) -> SpClientResult {
        let mut headers = headers.unwrap_or_default();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

        self.request(method, endpoint, Some(headers), body).await
    }

    pub async fn request(
        &self,
        method: &Method,
        endpoint: &str,
        headers: Option<HeaderMap>,
        body: Option<&str>,
    ) -> SpClientResult {
        let mut tries: usize = 0;
        let mut last_response;

        let body = body.unwrap_or_default();

        loop {
            tries += 1;

            // Reconnection logic: retrieve the endpoint every iteration, so we can try
            // another access point when we are experiencing network issues (see below).
            let mut url = self.base_url().await?;
            url.push_str(endpoint);

            let separator = match url.find('?') {
                Some(_) => "&",
                None => "?",
            };

            // Add metrics. There is also an optional `partner` key with a value like
            // `vodafone-uk` but we've yet to discover how we can find that value.
            // For the sake of documentation you could also do "product=free" but
            // we only support premium anyway.
            let _ = write!(
                url,
                "{}product=0&country={}",
                separator,
                self.session().country()
            );

            // Defeat caches. Spotify-generated URLs already contain this.
            if !url.contains("salt=") {
                let _ = write!(url, "&salt={}", rand::thread_rng().next_u32());
            }

            let mut request = Request::builder()
                .method(method)
                .uri(url)
                .body(body.to_owned().into())?;

            // Reconnection logic: keep getting (cached) tokens because they might have expired.
            let token = self.session().login5().auth_token().await?;

            let headers_mut = request.headers_mut();
            if let Some(ref hdrs) = headers {
                *headers_mut = hdrs.clone();
            }
            headers_mut.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("{} {}", token.token_type, token.access_token,))?,
            );

            match self.client_token().await {
                Ok(client_token) => {
                    let _ = headers_mut.insert(CLIENT_TOKEN, HeaderValue::from_str(&client_token)?);
                }
                Err(e) => {
                    // currently these endpoints seem to work fine without it
                    warn!("Unable to get client token: {e} Trying to continue without...")
                }
            }

            last_response = self.session().http_client().request_body(request).await;

            if last_response.is_ok() {
                return last_response;
            }

            // Break before the reconnection logic below, so that the current access point
            // is retained when max_tries == 1. Leave it up to the caller when to flush.
            if let RequestStrategy::TryTimes(max_tries) = self.lock(|inner| inner.strategy) {
                if tries >= max_tries {
                    break;
                }
            }

            // Reconnection logic: drop the current access point if we are experiencing issues.
            // This will cause the next call to base_url() to resolve a new one.
            if let Err(ref network_error) = last_response {
                match network_error.kind {
                    ErrorKind::Unavailable | ErrorKind::DeadlineExceeded => {
                        // Keep trying the current access point three times before dropping it.
                        if tries % 3 == 0 {
                            self.flush_accesspoint().await
                        }
                    }
                    _ => break, // if we can't build the request now, then we won't ever
                }
            }

            debug!("Error was: {:?}", last_response);
        }

        last_response
    }

    pub async fn put_connect_state(
        &self,
        connection_id: &str,
        state: &PutStateRequest,
    ) -> SpClientResult {
        let endpoint = format!("/connect-state/v1/devices/{}", self.session().device_id());

        let mut headers = HeaderMap::new();
        headers.insert("X-Spotify-Connection-Id", connection_id.parse()?);

        self.request_with_protobuf(&Method::PUT, &endpoint, Some(headers), state)
            .await
    }

    pub async fn get_metadata(&self, scope: &str, id: &SpotifyId) -> SpClientResult {
        let endpoint = format!("/metadata/4/{}/{}", scope, id.to_base16()?);
        self.request(&Method::GET, &endpoint, None, None).await
    }

    pub async fn get_track_metadata(&self, track_id: &SpotifyId) -> SpClientResult {
        self.get_metadata("track", track_id).await
    }

    pub async fn get_episode_metadata(&self, episode_id: &SpotifyId) -> SpClientResult {
        self.get_metadata("episode", episode_id).await
    }

    pub async fn get_album_metadata(&self, album_id: &SpotifyId) -> SpClientResult {
        self.get_metadata("album", album_id).await
    }

    pub async fn get_artist_metadata(&self, artist_id: &SpotifyId) -> SpClientResult {
        self.get_metadata("artist", artist_id).await
    }

    pub async fn get_show_metadata(&self, show_id: &SpotifyId) -> SpClientResult {
        self.get_metadata("show", show_id).await
    }

    pub async fn get_lyrics(&self, track_id: &SpotifyId) -> SpClientResult {
        let endpoint = format!("/color-lyrics/v2/track/{}", track_id.to_base62()?);

        self.request_as_json(&Method::GET, &endpoint, None, None)
            .await
    }

    pub async fn get_lyrics_for_image(
        &self,
        track_id: &SpotifyId,
        image_id: &FileId,
    ) -> SpClientResult {
        let endpoint = format!(
            "/color-lyrics/v2/track/{}/image/spotify:image:{}",
            track_id.to_base62()?,
            image_id
        );

        self.request_as_json(&Method::GET, &endpoint, None, None)
            .await
    }

    pub async fn get_playlist(&self, playlist_id: &SpotifyId) -> SpClientResult {
        let endpoint = format!("/playlist/v2/playlist/{}", playlist_id.to_base62()?);

        self.request(&Method::GET, &endpoint, None, None).await
    }

    pub async fn get_user_profile(
        &self,
        username: &str,
        playlist_limit: Option<u32>,
        artist_limit: Option<u32>,
    ) -> SpClientResult {
        let mut endpoint = format!("/user-profile-view/v3/profile/{username}");

        if playlist_limit.is_some() || artist_limit.is_some() {
            let _ = write!(endpoint, "?");

            if let Some(limit) = playlist_limit {
                let _ = write!(endpoint, "playlist_limit={limit}");
                if artist_limit.is_some() {
                    let _ = write!(endpoint, "&");
                }
            }

            if let Some(limit) = artist_limit {
                let _ = write!(endpoint, "artist_limit={limit}");
            }
        }

        self.request_as_json(&Method::GET, &endpoint, None, None)
            .await
    }

    pub async fn get_user_followers(&self, username: &str) -> SpClientResult {
        let endpoint = format!("/user-profile-view/v3/profile/{username}/followers");

        self.request_as_json(&Method::GET, &endpoint, None, None)
            .await
    }

    pub async fn get_user_following(&self, username: &str) -> SpClientResult {
        let endpoint = format!("/user-profile-view/v3/profile/{username}/following");

        self.request_as_json(&Method::GET, &endpoint, None, None)
            .await
    }

    pub async fn get_radio_for_track(&self, track_id: &SpotifyId) -> SpClientResult {
        let endpoint = format!(
            "/inspiredby-mix/v2/seed_to_playlist/{}?response-format=json",
            track_id.to_uri()?
        );

        self.request_as_json(&Method::GET, &endpoint, None, None)
            .await
    }

    // Known working scopes: stations, tracks
    // For others see: https://gist.github.com/roderickvd/62df5b74d2179a12de6817a37bb474f9
    //
    // Seen-in-the-wild but unimplemented query parameters:
    // - image_style=gradient_overlay
    // - excludeClusters=true
    // - language=en
    // - count_tracks=0
    // - market=from_token
    pub async fn get_apollo_station(
        &self,
        scope: &str,
        context_uri: &str,
        count: Option<usize>,
        previous_tracks: Vec<SpotifyId>,
        autoplay: bool,
    ) -> SpClientResult {
        let mut endpoint = format!("/radio-apollo/v3/{scope}/{context_uri}?autoplay={autoplay}");

        // Spotify has a default of 50
        if let Some(count) = count {
            let _ = write!(endpoint, "&count={count}");
        }

        let previous_track_str = previous_tracks
            .iter()
            .map(|track| track.to_base62())
            .collect::<Result<Vec<_>, _>>()?
            .join(",");
        // better than checking `previous_tracks.len() > 0` because the `filter_map` could still return 0 items
        if !previous_track_str.is_empty() {
            let _ = write!(endpoint, "&prev_tracks={previous_track_str}");
        }

        self.request_as_json(&Method::GET, &endpoint, None, None)
            .await
    }

    pub async fn get_next_page(&self, next_page_uri: &str) -> SpClientResult {
        let endpoint = next_page_uri.trim_start_matches("hm:/");
        self.request_as_json(&Method::GET, endpoint, None, None)
            .await
    }

    // TODO: Seen-in-the-wild but unimplemented endpoints
    // - /presence-view/v1/buddylist

    // TODO: Find endpoint for newer canvas.proto and upgrade to that.
    pub async fn get_canvases(&self, request: EntityCanvazRequest) -> SpClientResult {
        let endpoint = "/canvaz-cache/v0/canvases";
        self.request_with_protobuf(&Method::POST, endpoint, None, &request)
            .await
    }

    pub async fn get_extended_metadata(&self, request: BatchedEntityRequest) -> SpClientResult {
        let endpoint = "/extended-metadata/v0/extended-metadata";
        self.request_with_protobuf(&Method::POST, endpoint, None, &request)
            .await
    }

    pub async fn get_audio_storage(&self, file_id: &FileId) -> SpClientResult {
        let endpoint = format!(
            "/storage-resolve/files/audio/interactive/{}",
            file_id.to_base16()?
        );
        self.request(&Method::GET, &endpoint, None, None).await
    }

    pub fn stream_from_cdn(
        &self,
        cdn_url: &CdnUrl,
        offset: usize,
        length: usize,
    ) -> Result<IntoStream<ResponseFuture>, Error> {
        let url = cdn_url.try_get_url()?;
        let req = Request::builder()
            .method(&Method::GET)
            .uri(url)
            .header(
                RANGE,
                HeaderValue::from_str(&format!("bytes={}-{}", offset, offset + length - 1))?,
            )
            .body(Bytes::new())?;

        let stream = self.session().http_client().request_stream(req)?;

        Ok(stream)
    }

    pub async fn request_url(&self, url: &str) -> SpClientResult {
        let request = Request::builder()
            .method(&Method::GET)
            .uri(url)
            .body(Bytes::new())?;

        self.session().http_client().request_body(request).await
    }

    // Audio preview in 96 kbps MP3, unencrypted
    pub async fn get_audio_preview(&self, preview_id: &FileId) -> SpClientResult {
        let attribute = "audio-preview-url-template";
        let template = self
            .session()
            .get_user_attribute(attribute)
            .ok_or_else(|| SpClientError::Attribute(attribute.to_string()))?;

        let mut url = template.replace("{id}", &preview_id.to_base16()?);
        let separator = match url.find('?') {
            Some(_) => "&",
            None => "?",
        };
        let _ = write!(url, "{}cid={}", separator, self.session().client_id());

        self.request_url(&url).await
    }

    // The first 128 kB of a track, unencrypted
    pub async fn get_head_file(&self, file_id: &FileId) -> SpClientResult {
        let attribute = "head-files-url";
        let template = self
            .session()
            .get_user_attribute(attribute)
            .ok_or_else(|| SpClientError::Attribute(attribute.to_string()))?;

        let url = template.replace("{file_id}", &file_id.to_base16()?);

        self.request_url(&url).await
    }

    pub async fn get_image(&self, image_id: &FileId) -> SpClientResult {
        let attribute = "image-url";
        let template = self
            .session()
            .get_user_attribute(attribute)
            .ok_or_else(|| SpClientError::Attribute(attribute.to_string()))?;
        let url = template.replace("{file_id}", &image_id.to_base16()?);

        self.request_url(&url).await
    }
}
