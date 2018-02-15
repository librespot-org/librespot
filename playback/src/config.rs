use std::str::FromStr;
use core::spotify_id::SpotifyId;
use std::sync::mpsc::Sender;

#[derive(Clone, Copy, Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum Bitrate {
    Bitrate96,
    Bitrate160,
    Bitrate320,
}

impl FromStr for Bitrate {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "96" => Ok(Bitrate::Bitrate96),
            "160" => Ok(Bitrate::Bitrate160),
            "320" => Ok(Bitrate::Bitrate320),
            _ => Err(()),
        }
    }
}

impl Default for Bitrate {
    fn default() -> Bitrate {
        Bitrate::Bitrate160
    }
}

#[derive(Debug, Clone)]
pub enum PlayerEvent {
    Started {
        track_id: SpotifyId,
    },

    Changed {
        old_track_id: SpotifyId,
        new_track_id: SpotifyId,
    },

    Stopped {
        track_id: SpotifyId,
    }
}

#[derive(Clone, Debug)]
pub struct PlayerConfig {
    pub bitrate: Bitrate,
    pub event_sender : Option<Sender<PlayerEvent>>,
}

impl Default for PlayerConfig {
    fn default() -> PlayerConfig {
        PlayerConfig {
            bitrate: Bitrate::default(),
            event_sender: None,
        }
    }
}
