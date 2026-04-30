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

#[cfg(test)]
mod tests {
    //! Byte-format regression tests. The wire format of these events is not
    //! part of any public Spotify spec — it was reverse-engineered by
    //! librespot-java's maintainer (devgianlu, see librespot-org/librespot
    //! discussion #626). These tests pin the exact field layout we send so
    //! that future refactors don't silently break play-count reporting.
    //!
    //! The fixtures here mirror the hand-traced output of librespot-java's
    //! `TrackTransitionEvent.build()`, `NewSessionIdEvent.build()` and
    //! `NewPlaybackIdEvent.build()` for known input. We compare with `|`
    //! substituted for the `0x09` tab delimiter for readability.
    use super::*;
    use std::sync::atomic::Ordering;

    fn fixed_metrics() -> PlaybackMetrics {
        let mut m = PlaybackMetrics::new(
            "11111111111111111111111111111111".to_string(),
            "spotify:track:4uLU6hMCjMI75M1A2tKUQC".to_string(),
            "23a82593c0204e9c952a37094c83fb3a".to_string(),
            "spotify:playlist:foo".to_string(),
            "16.18.0".to_string(),
            "spotify_desktop".to_string(),
            213_000,
            160,
            "vorbis".to_string(),
            "harmony".to_string(),
            EndReason::ClickRow,
        );
        // Pin timestamp so the snapshot is deterministic.
        m.timestamp_ms = 1_700_000_000_000;
        m.start_interval(0);
        m.end_interval(45_000);
        m.ended_how(EndReason::TrackDone, "harmony");
        m
    }

    #[test]
    fn new_session_id_layout() {
        let e = build_new_session_id_event(
            "ssss-ssss",
            "spotify:playlist:foo",
            42,
            "context://spotify:playlist:foo",
        );
        let body = EventBuilder::debug_string(&e.into_bytes());
        // The 4th positional field is now-millis at build time, which is
        // wall-clock; we only assert the static structure around it.
        let parts: Vec<&str> = body.split('|').collect();
        assert_eq!(parts[0], "557");
        assert_eq!(parts[1], "3");
        assert_eq!(parts[2], "ssss-ssss");
        assert_eq!(parts[3], "spotify:playlist:foo");
        assert_eq!(parts[4], "spotify:playlist:foo");
        assert!(parts[5].chars().all(|c| c.is_ascii_digit()));
        assert_eq!(parts[6], "");
        assert_eq!(parts[7], "42");
        assert_eq!(parts[8], "context://spotify:playlist:foo");
        assert_eq!(parts.len(), 9);
    }

    #[test]
    fn new_playback_id_layout() {
        let e = build_new_playback_id_event("sess-id", "play-id");
        let body = EventBuilder::debug_string(&e.into_bytes());
        let parts: Vec<&str> = body.split('|').collect();
        assert_eq!(parts[0], "558");
        assert_eq!(parts[1], "1");
        assert_eq!(parts[2], "play-id");
        assert_eq!(parts[3], "sess-id");
        assert!(parts[4].chars().all(|c| c.is_ascii_digit()));
        assert_eq!(parts.len(), 5);
    }

    #[test]
    fn track_transition_layout_matches_java() {
        // Pin the global incremental counter to a known value for this test.
        // We can't reset an atomic without races against other tests in the
        // same binary, so we instead capture the current value and compute
        // the expected one rather than asserting a literal.
        let pre = TRANSITION_INCREMENTAL.load(Ordering::Relaxed);

        let m = fixed_metrics();
        let e = build_track_transition_event(&m, "device-A", "device-B");
        let body = EventBuilder::debug_string(&e.into_bytes());
        let p: Vec<&str> = body.split('|').collect();

        // Header: type + sub_type.
        assert_eq!(p[0], "12");
        assert_eq!(p[1], "38");
        // Then incremental counter — exact value depends on test order, but
        // it must be the value that was current when build was called.
        assert_eq!(p[2], pre.to_string());
        assert_eq!(p[3], "device-A");
        assert_eq!(p[4], "11111111111111111111111111111111");
        assert_eq!(p[5], "00000000000000000000000000000000");
        assert_eq!(p[6], "harmony"); // source_start
        assert_eq!(p[7], "clickrow"); // reason_start
        assert_eq!(p[8], "harmony"); // source_end
        assert_eq!(p[9], "trackdone"); // reason_end
        assert_eq!(p[10], "0"); // decoded_length
        assert_eq!(p[11], "0"); // size
        assert_eq!(p[12], "45000"); // when
        assert_eq!(p[13], "45000"); // when
        assert_eq!(p[14], "213000"); // duration
        assert_eq!(p[15], "0"); // decrypt_time
        assert_eq!(p[16], "0"); // fade_overlap
        assert_eq!(p[17], "0");
        assert_eq!(p[18], "0");
        assert_eq!(p[19], "0"); // first_value_trigger (intervals start at 0)
        assert_eq!(p[20], "0"); // first_value
        assert_eq!(p[21], "0");
        assert_eq!(p[22], "-1");
        assert_eq!(p[23], "context");
        assert_eq!(p[24], "0"); // audio_key_time
        assert_eq!(p[25], "0");
        assert_eq!(p[26], "0"); // preloaded_audio_key
        assert_eq!(p[27], "0");
        assert_eq!(p[28], "0");
        assert_eq!(p[29], "0");
        assert_eq!(p[30], "45000"); // when (repeat)
        assert_eq!(p[31], "45000"); // when (repeat)
        assert_eq!(p[32], "0");
        assert_eq!(p[33], "160"); // bitrate
        assert_eq!(p[34], "spotify:playlist:foo"); // context_uri
        assert_eq!(p[35], "vorbis"); // encoding
        assert_eq!(p[36], "23a82593c0204e9c952a37094c83fb3a"); // hex track id
        assert_eq!(p[37], ""); // trailing empty field
        assert_eq!(p[38], "0");
        assert_eq!(p[39], "1700000000000"); // timestamp
        assert_eq!(p[40], "0");
        assert_eq!(p[41], "context");
        assert_eq!(p[42], "spotify_desktop"); // referrer_identifier
        assert_eq!(p[43], "16.18.0"); // feature_version
        assert_eq!(p[44], "com.spotify");
        assert_eq!(p[45], "none"); // transition
        assert_eq!(p[46], "none");
        assert_eq!(p[47], "device-B"); // last_command_sent_by_device_id
        assert_eq!(p[48], "na");
        assert_eq!(p[49], "none");
        assert_eq!(p.len(), 50);
    }

    #[test]
    fn end_reason_wire_strings() {
        // Spotify only counts a play when reason_end is "trackdone". This
        // pins the wire strings — drift here would silently break royalty
        // crediting.
        assert_eq!(EndReason::TrackDone.as_wire(), "trackdone");
        assert_eq!(EndReason::TrackError.as_wire(), "trackerror");
        assert_eq!(EndReason::FwdBtn.as_wire(), "fwdbtn");
        assert_eq!(EndReason::BackBtn.as_wire(), "backbtn");
        assert_eq!(EndReason::EndPlay.as_wire(), "endplay");
        assert_eq!(EndReason::PlayBtn.as_wire(), "playbtn");
        assert_eq!(EndReason::ClickRow.as_wire(), "clickrow");
        assert_eq!(EndReason::Logout.as_wire(), "logout");
        assert_eq!(EndReason::AppLoad.as_wire(), "appload");
        assert_eq!(EndReason::Remote.as_wire(), "remote");
    }

    #[test]
    fn intervals_track_listened_time() {
        let mut m = fixed_metrics();
        // Reset intervals from fixture, then construct a paused-mid-play
        // scenario: play 0..15s, pause, resume from 15s, play to 50s.
        m = PlaybackMetrics::new(
            m.playback_id,
            m.track_uri,
            m.track_id_hex,
            m.context_uri,
            m.feature_version,
            m.referrer_identifier,
            m.duration_ms,
            m.bitrate,
            m.encoding,
            m.source_start,
            m.reason_start,
        );
        m.start_interval(0);
        m.end_interval(15_000);
        m.start_interval(15_000);
        m.end_interval(50_000);
        // last_value reports the latest interval-end (drives the `when` field).
        assert_eq!(m.last_value(), 50_000);
        assert_eq!(m.first_value(), 0);
    }

    #[test]
    fn type_ids_match_librespot_java() {
        // Wire IDs are NOT documented; pinning them prevents silent drift.
        assert_eq!(EventType::NewSessionId.ids(), ("557", "3"));
        assert_eq!(EventType::NewPlaybackId.ids(), ("558", "1"));
        assert_eq!(EventType::TrackTransition.ids(), ("12", "38"));
    }
}
