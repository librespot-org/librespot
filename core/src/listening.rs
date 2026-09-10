//! Reports audio that a player actually delivered to its output.
use std::{
    io::Write,
    time::{Duration, Instant, SystemTime},
};

use crate::{Error, FileId, Session, SpotifyUri, config, protocol};
use data_encoding::HEXLOWER;
use flate2::{Compression, write::GzEncoder};
use protobuf::Message;
use protocol::{
    RawCoreStream::RawCoreStream,
    gabito::{
        EventEnvelope, PublishEventsRequest, PublishEventsResponse, event_envelope::EventFragment,
    },
};
use rand::RngCore;
use sha1::{Digest, Sha1};
use uuid::Uuid;

// Event schema compatibility identifiers observed in desktop 1.2.96.518.
// The receiver accepts other values without error, but history ingestion may
// discard the event later. Keep these separate from librespot's own version.
const REPORTING_VERSION: &str = "1.2.96.518";
const REPORTING_VERSION_CODE: i64 = 129600518;
const REPORTING_CORE_VERSION: i64 = 6005400000002005;
const REPORTING_SDK: &str = "0.9.4-rl-essopt-loginsend-onlinesend-bcdsend-heartbeat300.0s/30.0s-modern-payload125kB-batch100";

#[derive(Clone, Copy, Debug)]
pub enum EndReason {
    TrackDone,
    EndPlay,
}

/// A completed playback, measured from rendered audio rather than elapsed time
/// or the position of the seek cursor. Local files must not be reported.
#[derive(Debug)]
pub struct PlaybackReport {
    pub uri: SpotifyUri,
    pub file_id: FileId,
    pub audio_format: String,
    pub context_uri: Option<String>,
    pub started_at: SystemTime,
    pub ended_at: SystemTime,
    pub played: Duration,
    pub reason: EndReason,
}

fn timestamp(time: SystemTime) -> i64 {
    time.duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .min(i64::MAX as u128) as i64
}

fn stream_message(report: &PlaybackReport, audio_id: &[u8]) -> RawCoreStream {
    let played = report.played.as_millis().min(i32::MAX as u128) as i32;
    RawCoreStream {
        playback_id: Some(Uuid::new_v4().as_bytes().to_vec()),
        parent_playback_id: Some(vec![0; 16]),
        stream_id: Some(Uuid::new_v4().as_bytes().to_vec()),
        media_id: Some(report.file_id.0.to_vec()),
        audio_id: Some(HEXLOWER.encode(audio_id).into_bytes()),
        media_type: Some("audio".into()),
        video_session_id: Some(String::new()),
        content_uri: Some(report.uri.to_string()),
        play_context: Some(
            report
                .context_uri
                .clone()
                .unwrap_or_else(|| report.uri.to_string()),
        ),
        displayed_content_uri: Some(String::new()),
        audio_format: Some(report.audio_format.clone()),
        playback_start_time: Some(timestamp(report.started_at)),
        end_timestamp: Some(timestamp(report.ended_at)),
        ms_played: Some(played),
        ms_played_nominal: Some(played),
        ms_played_overlapping: Some(0),
        ms_played_video: Some(0),
        ms_played_fullscreen: Some(0),
        ms_played_external: Some(0),
        ms_played_nominal_overlapping: Some(0),
        ms_narration_overlapping: Some(0),
        ms_trimmed: Some(0),
        ms_nominal_trimmed: Some(0),
        reason_start: Some("playbtn".into()),
        reason_end: Some(
            match report.reason {
                EndReason::TrackDone => "trackdone",
                EndReason::EndPlay => "endplay",
            }
            .into(),
        ),
        source_start: Some("librespot".into()),
        source_end: Some("librespot".into()),
        referrer: Some("unknown".into()),
        provider: Some("context".into()),
        streaming_rule: Some("none".into()),
        live: Some(false),
        content_is_downloaded: Some(false),
        incognito_mode: Some(false),
        client_offline_at_stream_start: Some(false),
        social_listening: Some(false),
        is_assumed_premium: Some(true),
        connect_controller_device_id_v2: Some("local".into()),
        core_version: Some(REPORTING_CORE_VERSION),
        core_bundle: Some("full".into()),
        playback_stack: Some("boombox".into()),
        playback_stacks: vec!["boombox".into()],
        orchestration_stack: Some("context-player".into()),
        controlling_device_brand: Some("librespot".into()),
        controlling_device_model: Some("librespot".into()),
        controlling_device_type: Some("computer".into()),
        ..Default::default()
    }
}

fn check_response(bytes: &[u8]) -> Result<(), Error> {
    let response = PublishEventsResponse::parse_from_bytes(bytes)?;
    if let Some(error) = response.error.first() {
        let message = format!(
            "Listening event rejected: index {}, reason {}",
            error.index, error.reason
        );
        return Err(if error.transient {
            Error::unavailable(message)
        } else {
            Error::failed_precondition(message)
        });
    }
    Ok(())
}

async fn publish_with_retry<F, Fut>(
    body: &[u8],
    delay: Duration,
    mut publish: F,
) -> Result<(), Error>
where
    F: FnMut(Vec<u8>) -> Fut,
    Fut: std::future::Future<Output = Result<Vec<u8>, Error>>,
{
    for attempt in 0..3 {
        let response = publish(body.to_vec()).await?;
        match check_response(&response) {
            Err(error) if error.kind == crate::error::ErrorKind::Unavailable && attempt < 2 => {
                tokio::time::sleep(delay * (1 << attempt)).await;
            }
            result => return result,
        }
    }
    unreachable!()
}

fn validate_audio_id(audio_id: &[u8]) -> Result<(), Error> {
    if audio_id.is_empty() {
        return Err(Error::failed_precondition(
            "Listening metadata has no audio identity",
        ));
    }
    Ok(())
}

/// Serializes completed listens using Spotify's event transport. Construct one
/// reporter per player and call it from an asynchronous task, never an audio sink.
pub struct ListeningReporter {
    session: Session,
    app_session_id: Vec<u8>,
    sequence_id: Vec<u8>,
    sequence: i64,
    clock: Instant,
}

impl ListeningReporter {
    pub fn new(session: Session) -> Self {
        let mut sequence_id = vec![0; 20];
        rand::rng().fill_bytes(&mut sequence_id);
        Self {
            session,
            app_session_id: Uuid::new_v4().as_bytes().to_vec(),
            sequence_id,
            sequence: 0,
            clock: Instant::now(),
        }
    }

    /// Reports only nonempty Spotify playback. The payload and sequence identity
    /// are retained across transport retries so a retry cannot become a new play.
    pub async fn report(&mut self, report: &PlaybackReport) -> Result<(), Error> {
        if report.played.is_zero() {
            return Ok(());
        }
        if !matches!(
            report.uri,
            SpotifyUri::Track { .. } | SpotifyUri::Episode { .. }
        ) {
            return Err(Error::invalid_argument(
                "Only Spotify audio can be reported",
            ));
        }
        let data = self
            .session
            .spclient()
            .get_metadata(
                protocol::extension_kind::ExtensionKind::AUDIO_FILES,
                &report.uri,
            )
            .await?;
        let files =
            protocol::audio_files_extension::AudioFilesExtensionResponse::parse_from_bytes(&data)?;
        validate_audio_id(&files.audio_id)?;
        let mut message = stream_message(report, &files.audio_id);
        message.player_session_id = Some(HEXLOWER.encode(&self.app_session_id));
        self.sequence += 1;
        let mut envelope = EventEnvelope {
            event_name: "RawCoreStream".into(),
            sequence_id: self.sequence_id.clone(),
            sequence_number: self.sequence,
            ..Default::default()
        };
        fn fragment<M: Message>(name: &str, message: M) -> Result<EventFragment, Error> {
            Ok(EventFragment {
                name: name.into(),
                data: message.write_to_bytes()?,
                ..Default::default()
            })
        }
        envelope.event_fragment = vec![
            fragment("message", message)?,
            fragment(
                "context_client_id",
                protocol::context_client_id::ClientId {
                    value: HEXLOWER
                        .decode(self.session.client_id().as_bytes())
                        .map_err(Error::invalid_argument)?,
                    ..Default::default()
                },
            )?,
            fragment(
                "context_installation_id",
                protocol::context_installation_id::InstallationId {
                    value: Sha1::digest(self.session.device_id().as_bytes())[..16].to_vec(),
                    ..Default::default()
                },
            )?,
            fragment(
                "context_application_desktop",
                protocol::context_application_desktop::ApplicationDesktop {
                    version_string: REPORTING_VERSION.into(),
                    version_code: REPORTING_VERSION_CODE,
                    session_id: self.app_session_id.clone(),
                    ..Default::default()
                },
            )?,
            fragment(
                "context_device_desktop",
                protocol::context_device_desktop::DeviceDesktop {
                    platform_type: config::OS.into(),
                    device_manufacturer: "librespot".into(),
                    device_model: "librespot".into(),
                    device_id: self.session.device_id().into(),
                    os_version: config::os_version(),
                    ..Default::default()
                },
            )?,
            fragment(
                "context_time",
                protocol::context_time::Time {
                    value: timestamp(report.ended_at),
                    ..Default::default()
                },
            )?,
            fragment(
                "context_monotonic_clock",
                protocol::context_monotonic_clock::MonotonicClock {
                    id: 1,
                    value: self.clock.elapsed().as_millis().min(i64::MAX as u128) as i64,
                    ..Default::default()
                },
            )?,
            fragment(
                "context_sdk",
                protocol::context_sdk::Sdk {
                    version_name: REPORTING_SDK.into(),
                    type_: "cpp".into(),
                    ..Default::default()
                },
            )?,
            EventFragment {
                name: "context_client_context_id".into(),
                ..Default::default()
            },
        ];
        let request = PublishEventsRequest {
            event: vec![envelope],
            ..Default::default()
        };
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&request.write_to_bytes()?)?;
        let body = encoder.finish()?;
        let session = &self.session;
        publish_with_retry(&body, Duration::from_secs(1), |body| async move {
            Ok(session
                .spclient()
                .publish_listening_events(&body)
                .await?
                .to_vec())
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use protobuf::Message;

    #[tokio::test]
    async fn transient_event_rejection_retries_the_same_payload() {
        let mut bodies = Vec::new();
        publish_with_retry(b"serialized event", Duration::ZERO, |body| {
            bodies.push(body.to_vec());
            std::future::ready(Ok(if bodies.len() == 1 {
                vec![0x0a, 0x04, 0x10, 0x01, 0x18, 0x02]
            } else {
                vec![]
            }))
        })
        .await
        .unwrap();
        assert_eq!(bodies, vec![b"serialized event".to_vec(); 2]);
    }

    #[tokio::test]
    async fn permanent_event_rejection_is_not_retried() {
        let mut attempts = 0;
        let result = publish_with_retry(b"event", Duration::ZERO, |_| {
            attempts += 1;
            std::future::ready(Ok(vec![0x0a, 0x02, 0x18, 0x02]))
        })
        .await;
        assert!(result.is_err());
        assert_eq!(attempts, 1);
    }

    #[test]
    fn missing_audio_identity_is_rejected() {
        assert!(validate_audio_id(&[]).is_err());
        assert!(validate_audio_id(&[1; 16]).is_ok());
    }

    #[test]
    fn successful_http_response_with_event_error_is_rejected() {
        assert!(check_response(&[]).is_ok());
        // PublishEventsResponse.error[0]: index = 0, reason = 2.
        assert!(check_response(&[0x0a, 0x02, 0x18, 0x02]).is_err());
    }

    #[test]
    fn end_report_preserves_rendered_time_and_audio_identity_on_the_wire() {
        let report = PlaybackReport {
            uri: SpotifyUri::from_uri("spotify:track:72AZ3V52rs9NfgMNhALxln").unwrap(),
            file_id: FileId([0x17; 20]),
            audio_format: "Vorbis 320 kbps".into(),
            context_uri: Some("spotify:album:example".into()),
            started_at: SystemTime::UNIX_EPOCH + Duration::from_millis(1_700_000_000_000),
            ended_at: SystemTime::UNIX_EPOCH + Duration::from_millis(1_700_000_180_000),
            played: Duration::from_millis(32_125),
            reason: EndReason::EndPlay,
        };
        let encoded = stream_message(&report, &[0xab; 16])
            .write_to_bytes()
            .unwrap();
        let decoded = RawCoreStream::parse_from_bytes(&encoded).unwrap();
        assert_eq!(decoded.ms_played(), 32_125);
        assert_eq!(decoded.ms_played_nominal(), 32_125);
        assert_eq!(decoded.media_id(), &[0x17; 20]);
        assert_eq!(decoded.audio_id(), b"abababababababababababababababab");
        assert_eq!(decoded.content_uri(), report.uri.to_string());
        assert_eq!(decoded.play_context(), "spotify:album:example");
        assert_eq!(decoded.playback_start_time(), 1_700_000_000_000);
        assert_eq!(decoded.end_timestamp(), 1_700_000_180_000);
        assert_eq!(decoded.reason_end(), "endplay");
        assert_eq!(decoded.parent_playback_id(), &[0; 16]);
        assert_eq!(decoded.playback_id().len(), 16);
    }
}
