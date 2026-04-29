//! Spotify event-service client.
//!
//! Posts tab-delimited event records to `hm://event-service/v1/events` over
//! Mercury. The server uses these events to drive listening history,
//! play-count statistics and royalty-eligible stream accounting. Without
//! these events, plays performed via this client are not credited to the
//! user's account or the artist.
//!
//! Ported from librespot-java's `EventService` / `EventBuilder`. Three event
//! types are needed for a play to register:
//!
//! 1. `NewSessionId` — once when a Spotify Connect session starts.
//! 2. `NewPlaybackId` — once per track playback.
//! 3. `TrackTransition` — when the track ends. The server only counts the
//!    play when the end reason is `trackdone` and the played duration is
//!    above the per-surface threshold (commonly 30 s).
//!
//! Wire format is **not** protobuf. Each event is a single byte string of
//! tab (`0x09`) separated UTF-8 fields. The first field is the numeric event
//! id, the second is a sub-id, and the remaining fields are positional and
//! event-specific.

use std::time::{SystemTime, UNIX_EPOCH};

use crate::{Error, session::Session};

/// Mercury URI for event reporting.
pub const EVENT_SERVICE_URI: &str = "hm://event-service/v1/events";

/// Identifies an event type to the server. Each variant maps to a fixed
/// `(id, sub_id)` pair that must appear as the first two fields of the
/// serialized event body.
#[derive(Copy, Clone, Debug)]
pub enum EventType {
    /// Emitted once per Spotify Connect session as the playback session
    /// identifier is established. Carries the context URI and size.
    NewSessionId,
    /// Emitted at the start of every track playback. Carries the per-track
    /// playback id and links it to the current session id.
    NewPlaybackId,
    /// Emitted at the end of every track playback. The event that drives
    /// play-count crediting; the server checks the listened intervals,
    /// the end reason and the context to decide whether the play counts.
    TrackTransition,
}

impl EventType {
    fn ids(self) -> (&'static str, &'static str) {
        match self {
            EventType::NewSessionId => ("557", "3"),
            EventType::NewPlaybackId => ("558", "1"),
            EventType::TrackTransition => ("12", "38"),
        }
    }
}

/// Tab-delimited event body builder. Mirrors `EventService.EventBuilder` from
/// librespot-java.
pub struct EventBuilder {
    body: Vec<u8>,
}

impl EventBuilder {
    pub fn new(event_type: EventType) -> Self {
        let (id, sub_id) = event_type.ids();
        let mut s = Self {
            body: Vec::with_capacity(256),
        };
        s.body.extend_from_slice(id.as_bytes());
        s.append(sub_id);
        s
    }

    /// Append a UTF-8 string field, prefixed with a tab delimiter.
    pub fn append(&mut self, s: &str) -> &mut Self {
        self.body.push(0x09);
        self.body.extend_from_slice(s.as_bytes());
        self
    }

    /// Append a numeric field as decimal text.
    pub fn append_int(&mut self, n: i64) -> &mut Self {
        self.append(&n.to_string())
    }

    /// Append a single-character field (e.g. `'0'`, `'1'`).
    pub fn append_char(&mut self, c: char) -> &mut Self {
        self.body.push(0x09);
        let mut buf = [0u8; 4];
        let s = c.encode_utf8(&mut buf);
        self.body.extend_from_slice(s.as_bytes());
        self
    }

    /// Append a possibly-null string field. `None` produces an empty field.
    pub fn append_opt(&mut self, s: Option<&str>) -> &mut Self {
        self.body.push(0x09);
        if let Some(s) = s {
            self.body.extend_from_slice(s.as_bytes());
        }
        self
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.body
    }

    /// Render with `|` for the tab separator. For diagnostic logging only.
    pub fn debug_string(body: &[u8]) -> String {
        body.iter()
            .map(|&b| if b == 0x09 { '|' } else { b as char })
            .collect()
    }
}

/// Send a built event to Spotify's event service. Fire-and-forget: the
/// returned `Result` only reports request submission, not server outcome.
pub fn send_event(session: &Session, builder: EventBuilder) -> Result<(), Error> {
    let body = builder.into_bytes();
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0);

    let user_fields = vec![
        ("Accept-Language".to_string(), b"en".to_vec()),
        (
            "X-ClientTimeStamp".to_string(),
            ts.to_string().into_bytes(),
        ),
    ];

    let debug_body = if log::log_enabled!(log::Level::Debug) {
        Some(EventBuilder::debug_string(&body))
    } else {
        None
    };

    let fut = session
        .mercury()
        .send_with_fields(EVENT_SERVICE_URI.to_string(), body, user_fields)?;

    tokio::spawn(async move {
        match fut.await {
            Ok(resp) => {
                if let Some(b) = debug_body {
                    log::debug!(
                        "event-service: status={} body={}",
                        resp.status_code,
                        b
                    );
                }
            }
            Err(e) => log::warn!("event-service: send failed: {e}"),
        }
    });

    Ok(())
}
