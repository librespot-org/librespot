//! Per-playback metrics that drive the Spotify play-count events.
//!
//! Tracks the fields and intervals needed to construct the three event-service
//! messages (`NewSessionId`, `NewPlaybackId`, `TrackTransition`) ported from
//! librespot-java's `PlaybackMetrics` / `TrackTransitionEvent`.
//!
//! Spotify only counts a play when:
//! - end reason is [`EndReason::TrackDone`], and
//! - the listened intervals total above the per-surface threshold (commonly
//!   30 seconds).
//!
//! We always send the `TrackTransition` event regardless of whether the play
//! is expected to count — the server makes the decision.

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use librespot_core::event_service::{EventBuilder, EventType};

static TRANSITION_INCREMENTAL: AtomicU64 = AtomicU64::new(0);

/// How a playback was started or ended. Translates 1:1 to the strings the
/// server expects in the `reason_start` / `reason_end` slots of the
/// `TrackTransition` event.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum EndReason {
    /// Track played to completion. Required for the play to count.
    TrackDone,
    /// Decoder/playback error while playing.
    TrackError,
    /// User pressed skip-forward (next).
    FwdBtn,
    /// User pressed skip-back (previous).
    BackBtn,
    /// Player was stopped (Connect handoff, app closed, explicit stop).
    EndPlay,
    /// Play button (e.g. unpause from a stopped state).
    PlayBtn,
    /// Track was started by clicking a row (e.g. picking a track in a playlist).
    ClickRow,
    /// User logged out.
    Logout,
    /// Application loaded — app open / startup playback.
    AppLoad,
    /// Remote command from another Connect device.
    Remote,
}

impl EndReason {
    pub fn as_wire(self) -> &'static str {
        match self {
            EndReason::TrackDone => "trackdone",
            EndReason::TrackError => "trackerror",
            EndReason::FwdBtn => "fwdbtn",
            EndReason::BackBtn => "backbtn",
            EndReason::EndPlay => "endplay",
            EndReason::PlayBtn => "playbtn",
            EndReason::ClickRow => "clickrow",
            EndReason::Logout => "logout",
            EndReason::AppLoad => "appload",
            EndReason::Remote => "remote",
        }
    }
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

fn uuid_hex() -> String {
    uuid::Uuid::new_v4().simple().to_string()
}

/// Per-track metrics record. Built once on `TrackChanged` and consumed once
/// on `EndOfTrack`.
pub struct PlaybackMetrics {
    pub playback_id: String,
    pub track_uri: String,
    pub track_id_hex: String,
    pub context_uri: String,
    pub feature_version: String,
    pub referrer_identifier: String,
    pub timestamp_ms: u64,
    pub duration_ms: u32,
    pub bitrate: u32,
    pub encoding: String,

    pub source_start: String,
    pub reason_start: EndReason,
    pub source_end: String,
    pub reason_end: Option<EndReason>,

    intervals: Vec<(u32, u32)>,
    open_interval_start: Option<u32>,
}

impl PlaybackMetrics {
    pub fn new(
        playback_id: String,
        track_uri: String,
        track_id_hex: String,
        context_uri: String,
        feature_version: String,
        referrer_identifier: String,
        duration_ms: u32,
        bitrate: u32,
        encoding: String,
        source_start: String,
        reason_start: EndReason,
    ) -> Self {
        Self {
            playback_id,
            track_uri,
            track_id_hex,
            context_uri,
            feature_version,
            referrer_identifier,
            timestamp_ms: now_ms(),
            duration_ms,
            bitrate,
            encoding,
            source_start,
            reason_start,
            source_end: String::new(),
            reason_end: None,
            intervals: Vec::with_capacity(8),
            open_interval_start: None,
        }
    }

    pub fn generate_playback_id() -> String {
        uuid_hex()
    }

    pub fn start_interval(&mut self, position_ms: u32) {
        self.open_interval_start = Some(position_ms);
    }

    pub fn end_interval(&mut self, position_ms: u32) {
        if let Some(begin) = self.open_interval_start.take()
            && begin != position_ms
        {
            // Spotify expects begin <= end. Clamp to keep the body well-formed
            // even if a seek inverts the interval — in that case, just record
            // the start as a zero-length interval.
            let (lo, hi) = if begin < position_ms {
                (begin, position_ms)
            } else {
                (begin, begin)
            };
            self.intervals.push((lo, hi));
        }
    }

    pub fn ended_how(&mut self, reason: EndReason, source: &str) {
        self.reason_end = Some(reason);
        self.source_end = if source.is_empty() {
            "unknown".to_string()
        } else {
            source.to_string()
        };
    }

    pub fn first_value(&self) -> u32 {
        self.intervals.first().map(|&(b, _)| b).unwrap_or(0)
    }

    pub fn last_value(&self) -> u32 {
        self.intervals
            .last()
            .map(|&(_, e)| e)
            .unwrap_or(self.duration_ms)
    }
}

/// Build the `TrackTransition` event body. Fields are positional and
/// tab-separated. Layout mirrors librespot-java's `TrackTransitionEvent.build()`.
///
/// Many fields are detailed audio-pipeline metrics that this Rust port does
/// not currently track (`decryptTime`, `audioKeyTime`, `fadeOverlap`,
/// `decodedLength`, `size`, `preloadedAudioKey`). They are sent as `0`. This
/// has been observed to be acceptable to the server: counting depends on
/// `reason_end`, the played intervals and the track id, not on these
/// pipeline metrics.
pub fn build_track_transition_event(
    metrics: &PlaybackMetrics,
    device_id: &str,
    last_command_sent_by_device_id: &str,
) -> EventBuilder {
    let mut e = EventBuilder::new(EventType::TrackTransition);
    let incr = TRANSITION_INCREMENTAL.fetch_add(1, Ordering::Relaxed);
    let when = metrics.last_value();
    let first = metrics.first_value();

    e.append_int(incr as i64);
    e.append(device_id);
    e.append(&metrics.playback_id)
        .append("00000000000000000000000000000000");
    e.append(&metrics.source_start)
        .append(metrics.reason_start.as_wire());
    e.append(&metrics.source_end).append(
        metrics
            .reason_end
            .unwrap_or(EndReason::EndPlay)
            .as_wire(),
    );
    // decoded_length, size — pipeline metrics we don't track, default 0.
    e.append_char('0').append_char('0');
    // when/when — current playback position at end (= last interval end).
    e.append_int(when as i64).append_int(when as i64);
    e.append_int(metrics.duration_ms as i64);
    // decrypt_time, fade_overlap, '0', '0' — pipeline metrics we don't track.
    e.append_char('0')
        .append_char('0')
        .append_char('0')
        .append_char('0');
    // first_value_trigger ('1' if firstValue > 0 else '0'), then first_value.
    e.append_char(if first == 0 { '0' } else { '1' });
    e.append_int(first as i64);
    // '0', "-1", "context"
    e.append_char('0').append("-1").append("context");
    // audio_key_time '0' — pipeline metric we don't track.
    e.append_char('0').append_char('0');
    // preloaded_audio_key, then '0','0','0' — pipeline metrics we don't track.
    e.append_char('0')
        .append_char('0')
        .append_char('0')
        .append_char('0');
    // when/when — repeated.
    e.append_int(when as i64).append_int(when as i64);
    // '0', bitrate.
    e.append_char('0').append_int(metrics.bitrate as i64);
    // context_uri, encoding.
    e.append(&metrics.context_uri).append(&metrics.encoding);
    // hex track id, then empty trailing field.
    e.append(&metrics.track_id_hex).append("");
    // '0', timestamp, '0'.
    e.append_char('0').append_int(metrics.timestamp_ms as i64).append_char('0');
    // "context", referrer_identifier, feature_version.
    e.append("context")
        .append(&metrics.referrer_identifier)
        .append(&metrics.feature_version);
    // "com.spotify", transition ("none"), "none".
    e.append("com.spotify").append("none").append("none");
    // last_command_sent_by_device_id, "na", "none".
    e.append(last_command_sent_by_device_id)
        .append("na")
        .append("none");

    e
}

pub fn build_new_session_id_event(
    session_id: &str,
    context_uri: &str,
    context_size: u32,
    context_url: &str,
) -> EventBuilder {
    let mut e = EventBuilder::new(EventType::NewSessionId);
    e.append(session_id);
    e.append(context_uri);
    e.append(context_uri);
    e.append_int(now_ms() as i64);
    e.append("");
    e.append_int(context_size as i64);
    e.append(context_url);
    e
}

pub fn build_new_playback_id_event(session_id: &str, playback_id: &str) -> EventBuilder {
    let mut e = EventBuilder::new(EventType::NewPlaybackId);
    e.append(playback_id);
    e.append(session_id);
    e.append_int(now_ms() as i64);
    e
}

/// Generate a fresh session id (hex UUIDv4). Spirc emits `NewSessionId` with
/// this value once per Connect activation; subsequent `NewPlaybackId` events
/// link back to it.
pub fn generate_session_id() -> String {
    uuid_hex()
}
