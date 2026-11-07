//! Simple types of the MPRIS specification.
//!
//! See <https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html>.

use zvariant::Type;

/// A playback state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Type)]
#[zvariant(signature = "s")]
pub(super) enum PlaybackStatus {
    /// A track is currently playing.
    Playing,

    /// A track is currently paused.
    Paused,

    /// There is no track currently playing.
    Stopped,
}

impl From<PlaybackStatus> for zvariant::Value<'_> {
    fn from(value: PlaybackStatus) -> Self {
        let s = match value {
            PlaybackStatus::Playing => "Playing",
            PlaybackStatus::Paused => "Paused",
            PlaybackStatus::Stopped => "Stopped",
        };

        s.into()
    }
}

/// A repeat / loop status
#[derive(Clone, Copy, Debug, PartialEq, Eq, Type)]
#[zvariant(signature = "s")]
pub(super) enum LoopStatus {
    /// The playback will stop when there are no more tracks to play
    None,

    /// The current track will start again from the beginning once it has finished playing
    Track,

    /// The playback loops through a list of tracks
    Playlist,
}

impl TryFrom<zvariant::Value<'_>> for LoopStatus {
    type Error = zvariant::Error;

    fn try_from(value: zvariant::Value<'_>) -> Result<Self, zvariant::Error> {
        if let zvariant::Value::Str(s) = value {
            match s.as_str() {
                "None" => Ok(Self::None),
                "Track" => Ok(Self::Track),
                "Playlist" => Ok(Self::Playlist),
                _ => Err(zvariant::Error::Message("invalid enum value".to_owned())),
            }
        } else {
            Err(zvariant::Error::IncorrectType)
        }
    }
}

impl From<LoopStatus> for zvariant::Value<'_> {
    fn from(value: LoopStatus) -> Self {
        let s = match value {
            LoopStatus::None => "None",
            LoopStatus::Track => "Track",
            LoopStatus::Playlist => "Playlist",
        };

        s.into()
    }
}

/// A playback rate
///
/// This is a multiplier, so a value of 0.5 indicates that playback is
/// happening at half speed, while 1.5 means that 1.5 seconds of "track time"
/// is consumed every second.
pub(super) type PlaybackRate = f64;

/// Audio volume level
///
/// - 0.0 means mute.
/// - 1.0 is a sensible maximum volume level (ex: 0dB).
///
/// Note that the volume may be higher than 1.0, although generally
/// clients should not attempt to set it above 1.0.
pub(super) type Volume = f64;

/// Time in microseconds.
pub(super) type TimeInUs = i64;
