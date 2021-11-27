use crate::apresolve::SocketAddress;
use crate::http_client::HttpClientError;
use crate::mercury::MercuryError;
use crate::protocol::canvaz::EntityCanvazRequest;
use crate::protocol::connect::PutStateRequest;
use crate::protocol::extended_metadata::BatchedEntityRequest;
use crate::spotify_id::{FileId, SpotifyId};

use bytes::Bytes;
use http::header::HeaderValue;
use hyper::header::InvalidHeaderValue;
use hyper::{Body, HeaderMap, Request};
use protobuf::Message;
use rand::Rng;
use std::time::Duration;
use thiserror::Error;

component! {
    SpClient : SpClientInner {
        accesspoint: Option<SocketAddress> = None,
        strategy: RequestStrategy = RequestStrategy::default(),
    }
}

pub type SpClientResult = Result<Bytes, SpClientError>;

#[derive(Error, Debug)]
pub enum SpClientError {
    #[error("could not get authorization token")]
    Token(#[from] MercuryError),
    #[error("could not parse request: {0}")]
    Parsing(#[from] http::Error),
    #[error("could not complete request: {0}")]
    Network(#[from] HttpClientError),
}

impl From<InvalidHeaderValue> for SpClientError {
    fn from(err: InvalidHeaderValue) -> Self {
        Self::Parsing(err.into())
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

    pub async fn get_accesspoint(&self) -> SocketAddress {
        // Memoize the current access point.
        let ap = self.lock(|inner| inner.accesspoint.clone());
        match ap {
            Some(tuple) => tuple,
            None => {
                let tuple = self.session().apresolver().resolve("spclient").await;
                self.lock(|inner| inner.accesspoint = Some(tuple.clone()));
                info!(
                    "Resolved \"{}:{}\" as spclient access point",
                    tuple.0, tuple.1
                );
                tuple
            }
        }
    }

    pub async fn base_url(&self) -> String {
        let ap = self.get_accesspoint().await;
        format!("https://{}:{}", ap.0, ap.1)
    }

    pub async fn protobuf_request(
        &self,
        method: &str,
        endpoint: &str,
        headers: Option<HeaderMap>,
        message: &dyn Message,
    ) -> SpClientResult {
        let body = protobuf::text_format::print_to_string(message);

        let mut headers = headers.unwrap_or_else(HeaderMap::new);
        headers.insert("Content-Type", "application/protobuf".parse()?);

        self.request(method, endpoint, Some(headers), Some(body))
            .await
    }

    pub async fn request(
        &self,
        method: &str,
        endpoint: &str,
        headers: Option<HeaderMap>,
        body: Option<String>,
    ) -> SpClientResult {
        let mut tries: usize = 0;
        let mut last_response;

        let body = body.unwrap_or_else(String::new);

        loop {
            tries += 1;

            // Reconnection logic: retrieve the endpoint every iteration, so we can try
            // another access point when we are experiencing network issues (see below).
            let mut uri = self.base_url().await;
            uri.push_str(endpoint);

            let mut request = Request::builder()
                .method(method)
                .uri(uri)
                .body(Body::from(body.clone()))?;

            // Reconnection logic: keep getting (cached) tokens because they might have expired.
            let headers_mut = request.headers_mut();
            if let Some(ref hdrs) = headers {
                *headers_mut = hdrs.clone();
            }
            headers_mut.insert(
                "Authorization",
                HeaderValue::from_str(&format!(
                    "Bearer {}",
                    self.session()
                        .token_provider()
                        .get_token("playlist-read")
                        .await?
                        .access_token
                ))?,
            );

            last_response = self
                .session()
                .http_client()
                .request_body(request)
                .await
                .map_err(SpClientError::Network);
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
            if let Err(SpClientError::Network(ref network_error)) = last_response {
                match network_error {
                    HttpClientError::Response(_) | HttpClientError::Request(_) => {
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

        self.protobuf_request("PUT", &endpoint, Some(headers), &state)
            .await
    }

    pub async fn get_metadata(&self, scope: &str, id: SpotifyId) -> SpClientResult {
        let endpoint = format!("/metadata/4/{}/{}", scope, id.to_base16());
        self.request("GET", &endpoint, None, None).await
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

    // TODO: Not working at the moment, always returns 400.
    pub async fn get_lyrics(&self, track_id: SpotifyId, image_id: FileId) -> SpClientResult {
        let endpoint = format!(
            "/color-lyrics/v2/track/{}/image/spotify:image:{}",
            track_id.to_base16(),
            image_id
        );

        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse()?);

        self.request("GET", &endpoint, Some(headers), None).await
    }

    // TODO: Find endpoint for newer canvas.proto and upgrade to that.
    pub async fn get_canvases(&self, request: EntityCanvazRequest) -> SpClientResult {
        let endpoint = "/canvaz-cache/v0/canvases";
        self.protobuf_request("POST", endpoint, None, &request)
            .await
    }

    pub async fn get_extended_metadata(&self, request: BatchedEntityRequest) -> SpClientResult {
        let endpoint = "/extended-metadata/v0/extended-metadata";
        self.protobuf_request("POST", endpoint, None, &request)
            .await
    }
}
