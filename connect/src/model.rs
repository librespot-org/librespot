use librespot_core::dealer::protocol::SkipTo;
use std::ops::Deref;

/// Request for loading playback
#[derive(Debug)]
pub struct LoadRequest {
    pub(super) context_uri: String,
    pub(super) options: LoadRequestOptions,
}

impl Deref for LoadRequest {
    type Target = LoadRequestOptions;

    fn deref(&self) -> &Self::Target {
        &self.options
    }
}

/// The parameters for creating a load request
#[derive(Debug, Default)]
pub struct LoadRequestOptions {
    /// Whether the given tracks should immediately start playing, or just be initially loaded.
    pub start_playing: bool,
    /// Start the playback after a specified amount of time (milliseconds).
    pub seek_to: u32,
    /// Start the playback in shuffle mode.
    pub shuffle: bool,
    /// Start the playback in repeat mode.
    pub repeat: bool,
    /// Start the playback by repeating the first track.
    pub repeat_track: bool,
    /// Decides if the context or the autoplay of the context is played
    ///
    /// ## Remarks:
    /// If `true` is provided, the option values (`shuffle`, `repeat` and `repeat_track`) are ignored
    pub autoplay: bool,
    /// Decides the starting position in the given context.
    ///
    /// If the provided item doesn't exist or is out of range,
    /// the playback starts at the beginning of the context.
    ///
    /// ## Remarks:
    /// If `None` is provided and `shuffle` is `true`, a random track is played, otherwise the first
    pub playing_track: Option<PlayingTrack>,
}

impl LoadRequest {
    /// Create a load request from a `context_uri`
    ///
    /// For supported `context_uri` see [`SpClient::get_context`](librespot_core::spclient::SpClient::get_context)
    pub fn from_context_uri(context_uri: String, options: LoadRequestOptions) -> Self {
        Self {
            context_uri,
            options,
        }
    }
}

/// An item that represent a track to play
#[derive(Debug)]
pub enum PlayingTrack {
    /// Represent the track at a given index.
    Index(u32),
    /// Represent the uri of a track.
    Uri(String),
    /// Represent an internal identifier from spotify.
    ///
    /// ## Remarks:
    /// Is not intended for usage, but required for parsing of some connect messages.
    Uid(String),
}

impl TryFrom<SkipTo> for PlayingTrack {
    type Error = ();

    fn try_from(value: SkipTo) -> Result<Self, Self::Error> {
        // order of checks is important, as the index can be 0, but still has an uid or uri provided,
        // so we only use the index as last resort
        if let Some(uri) = value.track_uri {
            Ok(PlayingTrack::Uri(uri))
        } else if let Some(uid) = value.track_uid {
            Ok(PlayingTrack::Uid(uid))
        } else if let Some(index) = value.track_index {
            Ok(PlayingTrack::Index(index))
        } else {
            Err(())
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
