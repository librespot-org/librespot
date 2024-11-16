use librespot_core::dealer::protocol::SkipTo;
use librespot_protocol::player::Context;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct ResolveContext {
    context: Context,
    autoplay: bool,
}

impl ResolveContext {
    pub fn from_uri(uri: impl Into<String>, autoplay: bool) -> Self {
        Self {
            context: Context {
                uri: uri.into(),
                ..Default::default()
            },
            autoplay,
        }
    }

    pub fn from_context(context: Context, autoplay: bool) -> Self {
        Self { context, autoplay }
    }

    pub fn uri(&self) -> &str {
        &self.context.uri
    }

    pub fn autoplay(&self) -> bool {
        self.autoplay
    }
}

impl Display for ResolveContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "uri: {}, autoplay: {}", self.context.uri, self.autoplay)
    }
}

impl PartialEq for ResolveContext {
    fn eq(&self, other: &Self) -> bool {
        let eq_autoplay = self.autoplay == other.autoplay;
        let eq_context = self.context.uri == other.context.uri;

        eq_autoplay && eq_context
    }
}

impl From<ResolveContext> for Context {
    fn from(value: ResolveContext) -> Self {
        value.context
    }
}

#[derive(Debug)]
pub struct SpircLoadCommand {
    pub context_uri: String,
    /// Whether the given tracks should immediately start playing, or just be initially loaded.
    pub start_playing: bool,
    pub shuffle: bool,
    pub repeat: bool,
    pub repeat_track: bool,
    pub playing_track: PlayingTrack,
}

#[derive(Debug)]
pub enum PlayingTrack {
    Index(u32),
    Uri(String),
    Uid(String),
}

impl From<SkipTo> for PlayingTrack {
    fn from(value: SkipTo) -> Self {
        // order of checks is important, as the index can be 0, but still has an uid or uri provided,
        // so we only use the index as last resort
        if let Some(uri) = value.track_uri {
            PlayingTrack::Uri(uri)
        } else if let Some(uid) = value.track_uid {
            PlayingTrack::Uid(uid)
        } else {
            PlayingTrack::Index(value.track_index.unwrap_or_else(|| {
                warn!("SkipTo didn't provided any point to skip to, falling back to index 0");
                0
            }))
        }
    }
}

#[derive(Debug)]
pub(super) enum SpircPlayStatus {
    Stopped,
    LoadingPlay {
        position_ms: u32,
    },
    LoadingPause {
        position_ms: u32,
    },
    Playing {
        nominal_start_time: i64,
        preloading_of_next_track_triggered: bool,
    },
    Paused {
        position_ms: u32,
        preloading_of_next_track_triggered: bool,
    },
}
