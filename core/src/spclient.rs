use std::{
    convert::TryInto,
    fmt::Write,
    time::{Duration, Instant},
};

use bytes::Bytes;
use futures_util::future::IntoStream;
use http::header::HeaderValue;
use hyper::{
    client::ResponseFuture,
    header::{ACCEPT, AUTHORIZATION, CONTENT_ENCODING, CONTENT_TYPE, RANGE},
    Body, HeaderMap, Method, Request,
};
use protobuf::Message;
use rand::Rng;
use thiserror::Error;

use crate::{
    apresolve::SocketAddress,
    cdn_url::CdnUrl,
    error::ErrorKind,
    protocol::{
        canvaz::EntityCanvazRequest,
        clienttoken_http::{ClientTokenRequest, ClientTokenRequestType, ClientTokenResponse},
        connect::PutStateRequest,
        extended_metadata::BatchedEntityRequest,
    },
    token::Token,
    version, Error, FileId, SpotifyId,
};

component! {
    SpClient : SpClientInner {
        accesspoint: Option<SocketAddress> = None,
        strategy: RequestStrategy = RequestStrategy::default(),
        client_token: Option<Token> = None,
    }
}

pub type SpClientResult = Result<Bytes, Error>;

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

        trace!("Client token unavailable or expired, requesting new token.");

        let mut message = ClientTokenRequest::new();
        message.set_request_type(ClientTokenRequestType::REQUEST_CLIENT_DATA_REQUEST);

        let client_data = message.mut_client_data();
        client_data.set_client_id(self.session().client_id());
        client_data.set_client_version(version::SEMVER.to_string());

        let connectivity_data = client_data.mut_connectivity_sdk_data();
        connectivity_data.set_device_id(self.session().device_id().to_string());

        let platform_data = connectivity_data.mut_platform_specific_data();

        match std::env::consts::OS {
            "windows" => {
                let (pe, image_file) = match std::env::consts::ARCH {
                    "arm" => (448, 452),
                    "aarch64" => (43620, 452),
                    "x86_64" => (34404, 34404),
                    _ => (332, 332), // x86
                };

                let windows_data = platform_data.mut_desktop_windows();
                windows_data.set_os_version(10);
                windows_data.set_os_build(21370);
                windows_data.set_platform_id(2);
                windows_data.set_unknown_value_6(9);
                windows_data.set_image_file_machine(image_file);
                windows_data.set_pe_machine(pe);
                windows_data.set_unknown_value_10(true);
            }
            "ios" => {
                let ios_data = platform_data.mut_ios();
                ios_data.set_user_interface_idiom(0);
                ios_data.set_target_iphone_simulator(false);
                ios_data.set_hw_machine("iPhone14,5".to_string());
                ios_data.set_system_version("15.2.1".to_string());
            }
            "android" => {
                let android_data = platform_data.mut_android();
                android_data.set_android_version("12.0.0_r26".to_string());
                android_data.set_api_version(31);
                android_data.set_device_name("Pixel".to_owned());
                android_data.set_model_str("GF5KQ".to_owned());
                android_data.set_vendor("Google".to_owned());
            }
            "macos" => {
                let macos_data = platform_data.mut_desktop_macos();
                macos_data.set_system_version("Darwin Kernel Version 17.7.0: Fri Oct 30 13:34:27 PDT 2020; root:xnu-4570.71.82.8~1/RELEASE_X86_64".to_string());
                macos_data.set_hw_model("iMac21,1".to_string());
                macos_data.set_compiled_cpu_type(std::env::consts::ARCH.to_string());
            }
            _ => {
                let linux_data = platform_data.mut_desktop_linux();
                linux_data.set_system_name("Linux".to_string());
                linux_data.set_system_release("5.4.0-56-generic".to_string());
                linux_data
                    .set_system_version("#62-Ubuntu SMP Mon Nov 23 19:20:19 UTC 2020".to_string());
                linux_data.set_hardware(std::env::consts::ARCH.to_string());
            }
        }

        let body = message.write_to_bytes()?;

        let request = Request::builder()
            .method(&Method::POST)
            .uri("https://clienttoken.spotify.com/v1/clienttoken")
            .header(ACCEPT, HeaderValue::from_static("application/x-protobuf"))
            .header(CONTENT_ENCODING, HeaderValue::from_static(""))
            .body(Body::from(body))?;

        let response = self.session().http_client().request_body(request).await?;
        let message = ClientTokenResponse::parse_from_bytes(&response)?;

        let client_token = self.lock(|inner| {
            let access_token = message.get_granted_token().get_token().to_owned();

            let client_token = Token {
                access_token: access_token.clone(),
                expires_in: Duration::from_secs(
                    message
                        .get_granted_token()
                        .get_refresh_after_seconds()
                        .try_into()
                        .unwrap_or(7200),
                ),
                token_type: "client-token".to_string(),
                scopes: message
                    .get_granted_token()
                    .get_domains()
                    .iter()
                    .map(|d| d.domain.clone())
                    .collect(),
                timestamp: Instant::now(),
            };

            trace!("Got client token: {:?}", client_token);

            inner.client_token = Some(client_token);
            access_token
        });

        Ok(client_token)
    }

    pub async fn request_with_protobuf(
        &self,
        method: &Method,
        endpoint: &str,
        headers: Option<HeaderMap>,
        message: &dyn Message,
    ) -> SpClientResult {
        let body = protobuf::text_format::print_to_string(message);

        let mut headers = headers.unwrap_or_else(HeaderMap::new);
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-protobuf"),
        );

        self.request(method, endpoint, Some(headers), Some(body))
            .await
    }

    pub async fn request_as_json(
        &self,
        method: &Method,
        endpoint: &str,
        headers: Option<HeaderMap>,
        body: Option<String>,
    ) -> SpClientResult {
        let mut headers = headers.unwrap_or_else(HeaderMap::new);
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

        self.request(method, endpoint, Some(headers), body).await
    }

    pub async fn request(
        &self,
        method: &Method,
        endpoint: &str,
        headers: Option<HeaderMap>,
        body: Option<String>,
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

            // Add metrics. There is also an optional `partner` key with a value like
            // `vodafone-uk` but we've yet to discover how we can find that value.
            let separator = match url.find('?') {
                Some(_) => "&",
                None => "?",
            };
            let _ = write!(url, "{}product=0", separator);

            let mut request = Request::builder()
                .method(method)
                .uri(url)
                .body(Body::from(body.clone()))?;

            // Reconnection logic: keep getting (cached) tokens because they might have expired.
            let token = self
                .session()
                .token_provider()
                .get_token("playlist-read")
                .await?;

            let headers_mut = request.headers_mut();
            if let Some(ref hdrs) = headers {
                *headers_mut = hdrs.clone();
            }
            headers_mut.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("{} {}", token.token_type, token.access_token,))?,
            );

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

            // When retrying, avoid hammering the Spotify infrastructure by sleeping a while.
            // The backoff time is chosen randomly from an ever-increasing range.
            let max_seconds = u64::pow(tries as u64, 2) * 3;
            let backoff = Duration::from_secs(rand::thread_rng().gen_range(1..=max_seconds));
            warn!(
                "Unable to complete API request, waiting {} seconds before retrying...",
                backoff.as_secs(),
            );
            debug!("Error was: {:?}", last_response);
            tokio::time::sleep(backoff).await;
        }

        last_response
    }

    pub async fn put_connect_state(
        &self,
        connection_id: String,
        state: PutStateRequest,
    ) -> SpClientResult {
        let endpoint = format!("/connect-state/v1/devices/{}", self.session().device_id());

        let mut headers = HeaderMap::new();
        headers.insert("X-Spotify-Connection-Id", connection_id.parse()?);

        self.request_with_protobuf(&Method::PUT, &endpoint, Some(headers), &state)
            .await
    }

    pub async fn get_metadata(&self, scope: &str, id: SpotifyId) -> SpClientResult {
        let endpoint = format!("/metadata/4/{}/{}", scope, id.to_base16()?);
        self.request(&Method::GET, &endpoint, None, None).await
    }

    pub async fn get_track_metadata(&self, track_id: SpotifyId) -> SpClientResult {
        self.get_metadata("track", track_id).await
    }

    pub async fn get_episode_metadata(&self, episode_id: SpotifyId) -> SpClientResult {
        self.get_metadata("episode", episode_id).await
    }

    pub async fn get_album_metadata(&self, album_id: SpotifyId) -> SpClientResult {
        self.get_metadata("album", album_id).await
    }

    pub async fn get_artist_metadata(&self, artist_id: SpotifyId) -> SpClientResult {
        self.get_metadata("artist", artist_id).await
    }

    pub async fn get_show_metadata(&self, show_id: SpotifyId) -> SpClientResult {
        self.get_metadata("show", show_id).await
    }

    pub async fn get_lyrics(&self, track_id: SpotifyId) -> SpClientResult {
        let endpoint = format!("/color-lyrics/v1/track/{}", track_id.to_base62()?);

        self.request_as_json(&Method::GET, &endpoint, None, None)
            .await
    }

    pub async fn get_lyrics_for_image(
        &self,
        track_id: SpotifyId,
        image_id: FileId,
    ) -> SpClientResult {
        let endpoint = format!(
            "/color-lyrics/v2/track/{}/image/spotify:image:{}",
            track_id.to_base62()?,
            image_id
        );

        self.request_as_json(&Method::GET, &endpoint, None, None)
            .await
    }

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

    pub async fn get_audio_storage(&self, file_id: FileId) -> SpClientResult {
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
            .body(Body::empty())?;

        let stream = self.session().http_client().request_stream(req)?;

        Ok(stream)
    }

    pub async fn request_url(&self, url: String) -> SpClientResult {
        let request = Request::builder()
            .method(&Method::GET)
            .uri(url)
            .body(Body::empty())?;

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

        self.request_url(url).await
    }

    // The first 128 kB of a track, unencrypted
    pub async fn get_head_file(&self, file_id: FileId) -> SpClientResult {
        let attribute = "head-files-url";
        let template = self
            .session()
            .get_user_attribute(attribute)
            .ok_or_else(|| SpClientError::Attribute(attribute.to_string()))?;

        let url = template.replace("{file_id}", &file_id.to_base16()?);

        self.request_url(url).await
    }

    pub async fn get_image(&self, image_id: FileId) -> SpClientResult {
        let attribute = "image-url";
        let template = self
            .session()
            .get_user_attribute(attribute)
            .ok_or_else(|| SpClientError::Attribute(attribute.to_string()))?;
        let url = template.replace("{file_id}", &image_id.to_base16()?);

        self.request_url(url).await
    }
}
