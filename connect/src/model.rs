use librespot_core::dealer::protocol::SkipTo;

#[derive(Debug)]
pub struct SpircLoadCommand {
    pub context_uri: String,
    /// Whether the given tracks should immediately start playing, or just be initially loaded.
    pub start_playing: bool,
    pub seek_to: u32,
    pub shuffle: bool,
    pub repeat: bool,
    pub repeat_track: bool,
    /// Decides the starting position in the given context
    ///
    /// ## Remarks:
    /// If none is provided and shuffle true, a random track is played, otherwise the first
    pub playing_track: Option<PlayingTrack>,
}

#[derive(Debug)]
pub enum PlayingTrack {
    Index(u32),
    Uri(String),
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
