use std::{collections::HashMap, sync::Arc};

use librespot_connect::spirc::Spirc;
use log::{debug, warn};
use thiserror::Error;
use time::format_description::well_known::Iso8601;
use tokio::sync::mpsc;
use zbus::connection;

use librespot::{
    core::Error,
    metadata::audio::UniqueFields,
    playback::player::{Player, PlayerEvent},
};

/// A playback state.
#[derive(Clone, Copy, Debug)]
enum PlaybackStatus {
    /// A track is currently playing.
    Playing,

    /// A track is currently paused.
    Paused,

    /// There is no track currently playing.
    Stopped,
}

impl zvariant::Type for PlaybackStatus {
    fn signature() -> zvariant::Signature<'static> {
        zvariant::Signature::try_from("s").unwrap()
    }
}

impl TryFrom<zvariant::Value<'_>> for PlaybackStatus {
    type Error = zvariant::Error;

    fn try_from(value: zvariant::Value<'_>) -> Result<Self, zvariant::Error> {
        if let zvariant::Value::Str(s) = value {
            match s.as_str() {
                "Playing" => Ok(Self::Playing),
                "Paused" => Ok(Self::Paused),
                "Stopped" => Ok(Self::Stopped),
                _ => Err(zvariant::Error::Message("invalid enum value".to_owned())),
            }
        } else {
            Err(zvariant::Error::IncorrectType)
        }
    }
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
#[derive(Clone, Copy, Debug)]
enum LoopStatus {
    /// The playback will stop when there are no more tracks to play
    None,

    /// The current track will start again from the begining once it has finished playing
    Track,

    /// The playback loops through a list of tracks
    Playlist,
}

impl zvariant::Type for LoopStatus {
    fn signature() -> zvariant::Signature<'static> {
        zvariant::Signature::try_from("s").unwrap()
    }
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

/// Unique track identifier.
///
/// If the media player implements the TrackList interface and allows
/// the same track to appear multiple times in the tracklist,
/// this must be unique within the scope of the tracklist.
///
/// Note that this should be a valid D-Bus object id, although clients
/// should not assume that any object is actually exported with any
/// interfaces at that path.
///
/// Media players may not use any paths starting with
/// `/org/mpris` unless explicitly allowed by this specification.
/// Such paths are intended to have special meaning, such as
/// `/org/mpris/MediaPlayer2/TrackList/NoTrack`
/// to indicate "no track".
///
/// This is a D-Bus object id as that is the definitive way to have
/// unique identifiers on D-Bus.  It also allows for future optional
/// expansions to the specification where tracks are exported to D-Bus
/// with an interface similar to org.gnome.UPnP.MediaItem2.
// type TrackId = ...;

// A playback rate
//
// This is a multiplier, so a value of 0.5 indicates that playback is
// happening at half speed, while 1.5 means that 1.5 seconds of "track time"
// is consumed every second.
type PlaybackRate = f64;

// Audio volume level
//
//     - 0.0 means mute.
//     - 1.0 is a sensible maximum volume level (ex: 0dB).
//
// Note that the volume may be higher than 1.0, although generally
// clients should not attempt to set it above 1.0.
type Volume = f64;

// Time in microseconds.
type TimeInUs = i64;

struct MprisService {}

#[zbus::interface(name = "org.mpris.MediaPlayer2")]
impl MprisService {
    // Brings the media player's user interface to the front using any appropriate mechanism
    // available.
    //
    // The media player may be unable to control how its user interface is displayed, or it may not
    // have a graphical user interface at all. In this case, the `CanRaise` property is `false` and
    // this method does nothing.
    async fn raise(&self) {
        debug!("org.mpris.MediaPlayer2::Raise");
    }

    // Causes the media player to stop running.
    //
    // The media player may refuse to allow clients to shut it down. In this case, the `CanQuit`
    // property is `false` and this method does nothing.
    //
    // Note: Media players which can be D-Bus activated, or for which there is no sensibly easy way
    // to terminate a running instance (via the main interface or a notification area icon for
    // example) should allow clients to use this method. Otherwise, it should not be needed.
    //
    // If the media player does not have a UI, this should be implemented.
    async fn quit(&self) {
        debug!("org.mpris.MediaPlayer2::Quit");
    }

    // If `false`, calling `Quit` will have no effect, and may raise a `NotSupported` error.  If
    // `true`, calling `Quit` will cause the media application to attempt to quit (although it may
    // still be prevented from quitting by the user, for example).
    #[zbus(property)]
    async fn can_quit(&self) -> bool {
        debug!("org.mpris.MediaPlayer2::CanQuit");
        false
    }

    // Whether the media player is occupying the fullscreen.
    //
    // This is typically used for videos.  A value of `true` indicates that the media player is
    // taking up the full screen.
    //
    // Media centre software may well have this value fixed to `true`
    //
    // If `CanSetFullscreen` is `true`, clients may set this property to `true` to tell the media
    // player to enter fullscreen mode, or to `false` to return to windowed mode.
    //
    // If `CanSetFullscreen` is `false`, then attempting to set this property should have no
    // effect, and may raise an error.  However, even if it is `true`, the media player may still
    // be unable to fulfil the request, in which case attempting to set this property will have no
    // effect (but should not raise an error).
    //
    // Rationale:
    //
    //     This allows remote control interfaces, such as LIRC or mobile devices like
    //     phones, to control whether a video is shown in fullscreen.
    #[zbus(property)]
    async fn fullscreen(&self) -> bool {
        debug!("org.mpris.MediaPlayer2::Fullscreen");
        false
    }

    #[zbus(property)]
    async fn set_fullscreen(&self, _value: bool) {
        debug!("org.mpris.MediaPlayer2::SetFullscreen");
    }

    // If `false`, attempting to set `Fullscreen` will have no effect, and may raise an error.  If
    // `true`, attempting to set `Fullscreen` will not raise an error, and (if it is different from
    // the current value) will cause the media player to attempt to enter or exit fullscreen mode.
    //
    // Note that the media player may be unable to fulfil the request. In this case, the value will
    // not change.  If the media player knows in advance that it will not be able to fulfil the
    // request, however, this property should be `false`.
    //
    // Rationale:
    //
    //     This allows clients to choose whether to display controls for entering
    //     or exiting fullscreen mode.
    #[zbus(property)]
    async fn can_set_fullscreen(&self) -> bool {
        debug!("org.mpris.MediaPlayer2::CanSetFullscreen");
        false
    }

    // If `false`, calling `Raise` will have no effect, and may raise a NotSupported error.  If
    // `true`, calling `Raise` will cause the media application to attempt to bring its user
    // interface to the front, although it may be prevented from doing so (by the window manager,
    // for example).
    #[zbus(property)]
    async fn can_raise(&self) -> bool {
        debug!("org.mpris.MediaPlayer2::CanRaise");
        false
    }

    // Indicates whether the `/org/mpris/MediaPlayer2` object implements the
    // `org.mpris.MediaPlayer2.TrackList` interface.
    #[zbus(property)]
    async fn has_track_list(&self) -> bool {
        debug!("org.mpris.MediaPlayer2::HasTrackList");
        // TODO: Eventually implement
        false
    }

    // A friendly name to identify the media player to users. This should usually match the name
    // found in .desktop files (eg: "VLC media player").
    #[zbus(property)]
    async fn identity(&self) -> String {
        debug!("org.mpris.MediaPlayer2::Identity");
        // TOOD: use name from config
        "Librespot".to_owned()
    }

    // The basename of an installed .desktop file which complies with the
    // [Desktop entry specification](http://standards.freedesktop.org/desktop-entry-spec/latest/)
    // with the `.desktop` extension stripped.
    //
    // Example: The desktop entry file is "/usr/share/applications/vlc.desktop", and this property
    // contains "vlc"
    //
    #[zbus(property)]
    async fn desktop_entry(&self) -> String {
        debug!("org.mpris.MediaPlayer2::DesktopEntry");
        // FIXME: The spec doesn't say anything about the case when there is no .desktop.
        // Is there any convention? Any value that common clients handle in a sane way?
        "".to_owned()
    }

    // The URI schemes supported by the media player.
    //
    // This can be viewed as protocols supported by the player in almost all cases.  Almost every
    // media player will include support for the `"file"` scheme.  Other common schemes are
    // `"http"` and `"rtsp"`.
    //
    // Note that URI schemes should be lower-case.
    //
    // Rationale:
    //
    //     This is important for clients to know when using the editing
    //     capabilities of the Playlist interface, for example.
    #[zbus(property)]
    async fn supported_uri_schemes(&self) -> Vec<String> {
        debug!("org.mpris.MediaPlayer2::SupportedUriSchemes");
        vec![]
    }

    // The mime-types supported by the media player.
    //
    // Mime-types should be in the standard format (eg: `audio/mpeg` or `application/ogg`).
    //
    // Rationale:
    //
    //     This is important for clients to know when using the editing
    //     capabilities of the Playlist interface, for example.
    #[zbus(property)]
    async fn supported_mime_types(&self) -> Vec<String> {
        debug!("org.mpris.MediaPlayer2::SupportedMimeTypes");
        vec![]
    }
}

struct MprisPlayerService {
    spirc: Option<Spirc>,
    repeat: bool,
    shuffle: bool,
    playback_status: PlaybackStatus,
    volume: f64,
    metadata: HashMap<String, zbus::zvariant::OwnedValue>,
}

// This interface implements the methods for querying and providing basic
// control over what is currently playing.
#[zbus::interface(name = "org.mpris.MediaPlayer2.Player")]
impl MprisPlayerService {
    /// Skips to the next track in the tracklist. If there is no next track (and endless playback
    /// and track repeat are both off), stop playback.
    ///
    /// If playback is paused or stopped, it remains that way.
    ///
    /// If self.can_go_next is `false`, attempting to call this method should have no effect.
    async fn next(&self) {
        if let Some(spirc) = &self.spirc {
            let _ = spirc.next();
        }
    }

    // Skips to the previous track in the tracklist.
    //
    // If there is no previous track (and endless playback and track repeat are both off), stop
    // playback.
    //
    // If playback is paused or stopped, it remains that way.
    //
    // If `self.can_go_previous` is `false`, attempting to call this method should have no effect.
    async fn previous(&self) {
        if let Some(spirc) = &self.spirc {
            let _ = spirc.prev();
        }
    }

    // Pauses playback.
    //
    // If playback is already paused, this has no effect.
    //
    // Calling Play after this should cause playback to start again from the same position.
    //
    // If `self.can_pause` is `false`, attempting to call this method should have no effect.
    async fn pause(&self) {
        if let Some(spirc) = &self.spirc {
            let _ = spirc.pause();
        }
    }

    // Pauses playback.
    //
    // If playback is already paused, resumes playback.
    //
    // If playback is stopped, starts playback.
    //
    // If `self.can_pause` is `false`, attempting to call this method should have no effect and
    // raise an error.
    async fn play_pause(&self) {
        // ignore for now
        // TODO: implement
    }

    // Stops playback.
    //
    // If playback is already stopped, this has no effect.
    //
    // Calling Play after this should cause playback to start again from the beginning of the
    // track.
    //
    // If `CanControl` is `false`, attempting to call this method should have no effect and raise
    // an error.
    async fn stop(&self) {
        if let Some(spirc) = &self.spirc {
            let _ = spirc.pause();
            let _ = spirc.set_position_ms(0);
        }
    }

    // Starts or resumes playback.
    //
    // If already playing, this has no effect.
    //
    // If paused, playback resumes from the current position.
    //
    // If there is no track to play, this has no effect.
    //
    // If `self.can_play` is `false`, attempting to call this method should have no effect.
    async fn play(&self) {
        if let Some(spirc) = &self.spirc {
            let _ = spirc.activate();
            let _ = spirc.play();
        }
    }

    // Seeks forward in the current track by the specified number of microseconds.
    //
    // A negative value seeks back. If this would mean seeking back further than the start of the
    // track, the position is set to 0.
    //
    // If the value passed in would mean seeking beyond the end of the track, acts like a call to
    // Next.
    //
    // If the `self.can_seek` property is `false`, this has no effect.
    //
    // Arguments:
    //
    // * `offset`: The number of microseconds to seek forward.
    async fn seek(&self, offset: TimeInUs) {
        if let Some(spirc) = &self.spirc {
            let _ = spirc.seek_offset((offset / 1000) as i32);
        }
    }

    // Sets the current track position in microseconds.
    //
    // If the Position argument is less than 0, do nothing.
    //
    // If the Position argument is greater than the track length, do nothing.
    //
    // If the `CanSeek` property is `false`, this has no effect.
    //
    // Rationale:
    //
    //     The reason for having this method, rather than making `self.position` writable, is to
    //     include the `track_id` argument to avoid race conditions where a client tries to seek to
    //     a position when the track has already changed.
    //
    // Arguments:
    //
    // * `track_id`: The currently playing track's identifier.
    //               If this does not match the id of the currently-playing track, the call is
    //               ignored as "stale".
    //               `/org/mpris/MediaPlayer2/TrackList/NoTrack` is _not_ a valid value for this
    //               argument.
    // * `position`: Track position in microseconds. This must be between 0 and `track_length`.
    async fn set_position(&self, _track_id: zbus::zvariant::ObjectPath<'_>, position: TimeInUs) {
        // FIXME: handle track_id
        if position < 0 {
            return;
        }
        if let Some(spirc) = &self.spirc {
            let _ = spirc.set_position_ms((position / 1000) as u32);
        }
    }

    // Opens the Uri given as an argument
    //
    // If the playback is stopped, starts playing
    //
    // If the uri scheme or the mime-type of the uri to open is not supported, this method does
    // nothing and may raise an error.  In particular, if the list of available uri schemes is
    // empty, this method may not be implemented.
    //
    // Clients should not assume that the Uri has been opened as soon as this method returns. They
    // should wait until the mpris:trackid field in the `Metadata` property changes.
    //
    // If the media player implements the TrackList interface, then the opened track should be made
    // part of the tracklist, the `org.mpris.MediaPlayer2.TrackList.TrackAdded` or
    // `org.mpris.MediaPlayer2.TrackList.TrackListReplaced` signal should be fired, as well as the
    // `org.freedesktop.DBus.Properties.PropertiesChanged` signal on the tracklist interface.
    //
    // Arguments:
    //
    // * `uri`: Uri of the track to load. Its uri scheme should be an element of the
    //          `org.mpris.MediaPlayer2.SupportedUriSchemes` property and the mime-type should
    //          match one of the elements of the `org.mpris.MediaPlayer2.SupportedMimeTypes`.
    async fn open_uri(&self, _uri: &str) -> zbus::fdo::Result<()> {
        Err(zbus::fdo::Error::NotSupported(
            "OpenUri not supported".to_owned(),
        ))
    }

    // The current playback status.
    //
    // May be "Playing", "Paused" or "Stopped".
    #[zbus(property(emits_changed_signal = "true"))]
    async fn playback_status(&self) -> PlaybackStatus {
        self.playback_status
    }

    // The current loop / repeat status
    //
    // May be:
    //  - "None" if the playback will stop when there are no more tracks to play
    //  - "Track" if the current track will start again from the begining once it has finished playing
    //  - "Playlist" if the playback loops through a list of tracks
    //
    // If `self.can_control` is `false`, attempting to set this property should have no effect and
    // raise an error.
    //
    #[zbus(property(emits_changed_signal = "true"))]
    async fn loop_status(&self) -> LoopStatus {
        if self.repeat {
            // FIXME: How does Spotify handle single track repeat?
            LoopStatus::Playlist
        } else {
            LoopStatus::None
        }
    }

    #[zbus(property)]
    async fn set_loop_status(&mut self, value: LoopStatus) -> zbus::fdo::Result<()> {
        // TODO: implement, notify change
        match value {
            LoopStatus::None => {
                if let Some(spirc) = &self.spirc {
                    let _ = spirc.repeat(false);
                }
            }
            LoopStatus::Track => {
                return Err(zbus::fdo::Error::NotSupported(
                    "Player control not implemented".to_owned(),
                ));
            }
            LoopStatus::Playlist => {
                if let Some(spirc) = &self.spirc {
                    let _ = spirc.repeat(true);
                }
            }
        }

        Ok(())
    }

    // The current playback rate.
    //
    // The value must fall in the range described by `MinimumRate` and `MaximumRate`, and must not
    // be 0.0.  If playback is paused, the `PlaybackStatus` property should be used to indicate
    // this.  A value of 0.0 should not be set by the client.  If it is, the media player should
    // act as though `Pause` was called.
    //
    // If the media player has no ability to play at speeds other than the normal playback rate,
    // this must still be implemented, and must return 1.0.  The `MinimumRate` and `MaximumRate`
    // properties must also be set to 1.0.
    //
    // Not all values may be accepted by the media player.  It is left to media player
    // implementations to decide how to deal with values they cannot use; they may either ignore
    // them or pick a "best fit" value. Clients are recommended to only use sensible fractions or
    // multiples of 1 (eg: 0.5, 0.25, 1.5, 2.0, etc).
    //
    // Rationale:
    //
    //     This allows clients to display (reasonably) accurate progress bars
    //     without having to regularly query the media player for the current
    //     position.
    #[zbus(property(emits_changed_signal = "true"))]
    async fn rate(&self) -> PlaybackRate {
        1.0
    }

    #[zbus(property)]
    async fn set_rate(&mut self, _value: PlaybackRate) {
        // ignore
    }

    // A value of `false` indicates that playback is progressing linearly through a playlist, while
    // `true` means playback is progressing through a playlist in some other order.
    //
    // If `CanControl` is `false`, attempting to set this property should have no effect and raise
    // an error.
    //
    #[zbus(property(emits_changed_signal = "true"))]
    async fn shuffle(&self) -> bool {
        self.shuffle
    }

    #[zbus(property)]
    async fn set_shuffle(&mut self, value: bool) {
        if let Some(spirc) = &self.spirc {
            let _ = spirc.shuffle(value);
        }
    }

    // The metadata of the current element.
    //
    // If there is a current track, this must have a "mpris:trackid" entry (of D-Bus type "o") at
    // the very least, which contains a D-Bus path that uniquely identifies this track.
    //
    // See the type documentation for more details.
    #[zbus(property(emits_changed_signal = "true"))]
    async fn metadata(
        &self,
    ) -> zbus::fdo::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>> {
        let meta = if self.metadata.is_empty() {
            let mut meta = HashMap::new();
            meta.insert(
                "mpris:trackid".to_owned(),
                zvariant::Str::from(" /org/mpris/MediaPlayer2/TrackList/NoTrack").into(),
            );
            meta
        } else {
            self.metadata
                .iter()
                .map(|(k, v)| (k.clone(), v.try_clone().unwrap()))
                .collect()
        };
        Ok(meta)
    }

    // The volume level.
    //
    // When setting, if a negative value is passed, the volume should be set to 0.0.
    //
    // If `CanControl` is `false`, attempting to set this property should have no effect and raise
    // an error.
    #[zbus(property(emits_changed_signal = "true"))]
    async fn volume(&self) -> Volume {
        self.volume
    }

    #[zbus(property)]
    async fn set_volume(&mut self, _value: Volume) -> zbus::fdo::Result<()> {
        // TODO: implement
        Err(zbus::fdo::Error::NotSupported(
            "Player control not implemented".to_owned(),
        ))
    }

    // The current track position in microseconds, between 0 and the 'mpris:length' metadata entry
    // (see Metadata).
    //
    // Note: If the media player allows it, the current playback position can be changed either the
    // SetPosition method or the Seek method on this interface.  If this is not the case, the
    // `CanSeek` property is false, and setting this property has no effect and can raise an error.
    //
    // If the playback progresses in a way that is inconstistant with the `Rate` property, the
    // `Seeked` signal is emited.
    #[zbus(property(emits_changed_signal = "false"))]
    async fn position(&self) -> zbus::fdo::Result<TimeInUs> {
        // todo!("fetch up-to-date position from player")
        Ok(0)
    }

    // Note that the `Position` property is not writable intentionally, see
    // the `set_position` method above.
    // #[zbus(property)]
    // async fn set_position(&self, _value: TimeInUs) -> zbus::fdo::Result<()> {
    //     // TODO: implement
    //     Err(zbus::fdo::Error::NotSupported("Player control not implemented".to_owned()))
    // }

    // The minimum value which the `Rate` property can take. Clients should not attempt to set the
    // `Rate` property below this value.
    //
    // Note that even if this value is 0.0 or negative, clients should not attempt to set the
    // `Rate` property to 0.0.
    //
    // This value should always be 1.0 or less.
    #[zbus(property(emits_changed_signal = "true"))]
    async fn minimum_rate(&self) -> PlaybackRate {
        // TODO: implement
        1.0
    }

    // The maximum value which the `Rate` property can take. Clients should not attempt to set the
    // `Rate` property above this value.
    //
    // This value should always be 1.0 or greater.
    #[zbus(property(emits_changed_signal = "true"))]
    async fn maximum_rate(&self) -> PlaybackRate {
        // TODO: implement
        1.0
    }

    // Whether the client can call the `Next` method on this interface and expect the current track
    // to change.
    //
    // If it is unknown whether a call to `Next` will be successful (for example, when streaming
    // tracks), this property should be set to `true`.
    //
    // If `CanControl` is `false`, this property should also be `false`.
    //
    // Rationale:
    //
    //     Even when playback can generally be controlled, there may not
    //     always be a next track to move to.
    #[zbus(property(emits_changed_signal = "true"))]
    async fn can_go_next(&self) -> bool {
        true
    }

    // Whether the client can call the `Previous` method on this interface and expect the current
    // track to change.
    //
    // If it is unknown whether a call to `Previous` will be successful (for example, when
    // streaming tracks), this property should be set to `true`.
    //
    // If `CanControl` is `false`, this property should also be `false`.
    //
    // Rationale:
    //
    //     Even when playback can generally be controlled, there may not
    //     always be a next previous to move to.
    #[zbus(property(emits_changed_signal = "true"))]
    async fn can_go_previous(&self) -> bool {
        true
    }

    // Whether playback can be started using `Play` or `PlayPause`.
    //
    // Note that this is related to whether there is a "current track": the value should not depend
    // on whether the track is currently paused or playing.  In fact, if a track is currently
    // playing (and `CanControl` is `true`), this should be `true`.
    //
    // If `CanControl` is `false`, this property should also be `false`.
    //
    // Rationale:
    //
    //     Even when playback can generally be controlled, it may not be
    //     possible to enter a "playing" state, for example if there is no
    //     "current track".
    #[zbus(property(emits_changed_signal = "true"))]
    async fn can_play(&self) -> bool {
        !self.metadata.is_empty()
    }

    // Whether playback can be paused using `Pause` or `PlayPause`.
    //
    // Note that this is an intrinsic property of the current track: its value should not depend on
    // whether the track is currently paused or playing.  In fact, if playback is currently paused
    // (and `CanControl` is `true`), this should be `true`.
    //
    //
    // If `CanControl` is `false`, this property should also be `false`.
    //
    // Rationale:
    //
    //     Not all media is pausable: it may not be possible to pause some
    //     streamed media, for example.
    #[zbus(property(emits_changed_signal = "true"))]
    async fn can_pause(&self) -> bool {
        !self.metadata.is_empty()
    }

    // Whether the client can control the playback position using `Seek` and `SetPosition`.  This
    // may be different for different tracks.
    //
    // If `CanControl` is `false`, this property should also be `false`.
    //
    // Rationale:
    //
    //     Not all media is seekable: it may not be possible to seek when
    //     playing some streamed media, for example.
    #[zbus(property(emits_changed_signal = "true"))]
    async fn can_seek(&self) -> bool {
        true
    }

    // Whether the media player may be controlled over this interface.
    //
    // This property is not expected to change, as it describes an intrinsic capability of the
    // implementation.
    //
    // If this is `false`, clients should assume that all properties on this interface are
    // read-only (and will raise errors if writing to them is attempted), no methods are
    // implemented and all other properties starting with "can_" are also `false`.
    //
    // Rationale:
    //
    //     This allows clients to determine whether to present and enable controls to the user in
    //     advance of attempting to call methods and write to properties.
    #[zbus(property(emits_changed_signal = "const"))]
    async fn can_control(&self) -> bool {
        true
    }

    // Indicates that the track position has changed in a way that is inconsistant with the current
    // playing state.
    //
    // When this signal is not received, clients should assume that:
    // - When playing, the position progresses according to the rate property.
    // - When paused, it remains constant.
    //
    // This signal does not need to be emitted when playback starts or when the track changes,
    // unless the track is starting at an unexpected position. An expected position would be the
    // last known one when going from Paused to Playing, and 0 when going from Stopped to Playing.
    //
    // Arguments:
    //
    // * `position`: The new position, in microseconds.
    #[zbus(signal)]
    async fn seeked(signal_ctxt: &zbus::SignalContext<'_>, position: TimeInUs) -> zbus::Result<()>;
    // FIXME: signal on appropriate player events!
}

#[derive(Debug, Error)]
pub enum MprisError {
    #[error("zbus error: {0}")]
    DbusError(zbus::Error),
}

impl From<MprisError> for Error {
    fn from(err: MprisError) -> Self {
        use MprisError::*;
        match err {
            DbusError(_) => Error::internal(err),
        }
    }
}

impl From<zbus::Error> for MprisError {
    fn from(err: zbus::Error) -> Self {
        Self::DbusError(err)
    }
}

enum MprisCommand {
    SetSpirc(Spirc),
    Quit,
}

pub struct MprisEventHandler {
    cmd_tx: mpsc::UnboundedSender<MprisCommand>,
    join_handle: tokio::task::JoinHandle<()>,
}

impl MprisEventHandler {
    pub async fn spawn(player: Arc<Player>) -> Result<MprisEventHandler, MprisError> {
        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();

        let mpris_service = MprisService {};
        let mpris_player_service = MprisPlayerService {
            spirc: None,
            // FIXME: obtain current values from Player
            repeat: false,
            shuffle: false,
            playback_status: PlaybackStatus::Stopped,
            volume: 1.0,
            metadata: HashMap::new(),
        };

        let connection = connection::Builder::session()?
            // FIXME: retry with "org.mpris.MediaPlayer2.librespot.instance<pid>"
            // on error
            .name("org.mpris.MediaPlayer2.librespot")?
            .serve_at("/org/mpris/MediaPlayer2", mpris_service)?
            .serve_at("/org/mpris/MediaPlayer2", mpris_player_service)?
            .build()
            .await?;

        let mpris_task = MprisTask {
            player,
            connection,
            cmd_rx,
        };

        let join_handle = tokio::spawn(mpris_task.run());

        Ok(MprisEventHandler {
            cmd_tx,
            join_handle,
        })
    }

    pub fn set_spirc(&self, spirc: Spirc) {
        let _ = self.cmd_tx.send(MprisCommand::SetSpirc(spirc));
    }

    pub async fn quit_and_join(self) {
        let _ = self.cmd_tx.send(MprisCommand::Quit);
        let _ = self.join_handle.await;
    }
}

struct MprisTask {
    player: Arc<Player>,
    connection: zbus::Connection,
    cmd_rx: mpsc::UnboundedReceiver<MprisCommand>,
}

impl MprisTask {
    async fn run(mut self) {
        let mut player_events = self.player.get_player_event_channel();

        loop {
            tokio::select! {
                Some(event) = player_events.recv() => {
                    if let Err(e) = self.handle_event(event).await {
                        warn!("Error handling PlayerEvent: {}", e);
                    }
                }

                cmd = self.cmd_rx.recv() => {
                    match cmd {
                        Some(MprisCommand::SetSpirc(spirc)) => {
                            // TODO: Update playback status, metadata, etc (?)
                            self.mpris_player_iface().await
                                .get_mut().await
                                .spirc = Some(spirc);

                        }
                        Some(MprisCommand::Quit) => break,

                        // Keep running if the cmd sender was dropped
                        None => (),
                    }
                }

                // If player_events yields None, shutdown
                else => break,
            }
        }

        debug!("Shutting down MprisTask ...");
    }

    #[allow(dead_code)]
    async fn mpris_iface(&self) -> zbus::object_server::InterfaceRef<MprisService> {
        self.connection
            .object_server()
            .interface::<_, MprisService>("/org/mpris/MediaPlayer2")
            .await
            .expect("iface missing on object server")
    }

    async fn mpris_player_iface(&self) -> zbus::object_server::InterfaceRef<MprisPlayerService> {
        self.connection
            .object_server()
            .interface::<_, MprisPlayerService>("/org/mpris/MediaPlayer2")
            .await
            .expect("iface missing on object server")
    }

    async fn handle_event(&self, event: PlayerEvent) -> zbus::Result<()> {
        match event {
            PlayerEvent::PlayRequestIdChanged { play_request_id: _ } => { },
            PlayerEvent::TrackChanged { audio_item } => {
                match audio_item.track_id.to_base62() {
                    Err(e) => {
                        warn!("PlayerEvent::TrackChanged: Invalid track id: {}", e)
                    }
                    Ok(track_id) => {
                        let iface_ref = self.mpris_player_iface().await;
                        let mut iface = iface_ref.get_mut().await;

                        let meta = &mut iface.metadata;
                        meta.clear();

                        let mut trackid = String::new();
                        trackid.push_str("/org/librespot/track/");
                        trackid.push_str(&track_id);
                        meta.insert(
                            "mpris:trackid".into(),
                            zvariant::ObjectPath::try_from(trackid).unwrap().into()
                        );

                        meta.insert(
                            "xesam:title".into(),
                            zvariant::Str::from(audio_item.name).into()
                        );

                        if audio_item.covers.is_empty() {
                            meta.remove("mpris:artUrl");
                        } else {
                            // TODO: Select image by size
                            let url = &audio_item .covers[0].url;
                            meta.insert(
                                "mpris.artUrl".into(),
                                zvariant::Str::from(url).into()
                            );
                        }

                        meta.insert(
                            "mpris:length".into(),
                            (audio_item.duration_ms as i64 * 1000).into(),
                        );

                        match audio_item.unique_fields {
                            UniqueFields::Track {
                                artists,
                                album,
                                album_date,
                                album_artists,
                                popularity: _,
                                number,
                                disc_number,
                            } => {
                                let artists = artists
                                    .0
                                    .into_iter()
                                    .map(|a| a.name)
                                    .collect::<Vec<String>>();
                                meta.insert(
                                    "xesam:artist".into(),
                                    // try_to_owned only fails if the Value contains file
                                    // descriptors, so the unwrap never panics here
                                    zvariant::Value::from(artists).try_to_owned().unwrap()
                                );


                                meta.insert(
                                    "xesam:albumArtist".into(),
                                    // try_to_owned only fails if the Value contains file
                                    // descriptors, so the unwrap never panics here
                                    zvariant::Value::from(&album_artists).try_to_owned().unwrap()
                                );

                                meta.insert(
                                    "xesam:album".into(),
                                    zvariant::Str::from(album).into()
                                );

                                meta.insert(
                                    "xesam:trackNumber".into(),
                                    (number as i32).into(),
                                );

                                meta.insert(
                                    "xesam:discNumber".into(),
                                    (disc_number as i32).into(),
                                );

                                meta.insert(
                                    "xesam:contentCreated".into(),
                                    zvariant::Str::from(album_date.0.format(&Iso8601::DATE).unwrap()).into()
                                );
                            }
                            UniqueFields::Episode {
                                description,
                                publish_time,
                                show_name,
                            } => {
                                meta.insert(
                                    "xesam:album".into(),
                                    zvariant::Str::from(show_name).into()
                                );

                                meta.insert(
                                    "xesam:comment".into(),
                                    zvariant::Str::from(description).into()
                                );

                                meta.insert(
                                    "xesam:contentCreated".into(),
                                    zvariant::Str::from(publish_time.0.format(&Iso8601::DATE).unwrap()).into()
                                );
                            }
                        }

                        iface.metadata_changed(iface_ref.signal_context()).await?;
                    }
                }
            }
            PlayerEvent::Stopped { track_id, .. } => match track_id.to_base62() {
                Err(e) => warn!("PlayerEvent::Stopped: Invalid track id: {}", e),
                Ok(track_id) => {
                    let iface_ref = self.mpris_player_iface().await;
                    let mut iface = iface_ref.get_mut().await;
                    let meta = &mut iface.metadata;

                    // TODO: Check if metadata changed, if so clear
                    let mut trackid = String::new();
                    trackid.push_str("/org/librespot/track/");
                    trackid.push_str(&track_id);
                    meta.insert("mpris:trackid".into(), zvariant::ObjectPath::try_from(trackid).unwrap().into());
                    iface.metadata_changed(iface_ref.signal_context()).await?;

                    iface.playback_status = PlaybackStatus::Stopped;
                    iface.playback_status_changed(iface_ref.signal_context()).await?;
                }
            },
            PlayerEvent::Playing {
                track_id,
                // position_ms,
                ..
            } => match track_id.to_base62() {
                Err(e) => warn!("PlayerEvent::Playing: Invalid track id: {}", e),
                Ok(track_id) => {
                    // TODO: update position
                    let iface_ref = self.mpris_player_iface().await;
                    let mut iface = iface_ref.get_mut().await;
                    let meta = &mut iface.metadata;

                    // TODO: Check if metadata changed, if so clear
                    let mut trackid = String::new();
                    trackid.push_str("/org/librespot/track/");
                    trackid.push_str(&track_id);
                    meta.insert("mpris:trackid".into(), zvariant::ObjectPath::try_from(trackid).unwrap().into());
                    iface.metadata_changed(iface_ref.signal_context()).await?;

                    iface.playback_status = PlaybackStatus::Playing;
                    iface.playback_status_changed(iface_ref.signal_context()).await?;
                }
            },
            PlayerEvent::Paused {
                track_id,
                // position_ms,
                ..
            } => match track_id.to_base62() {
                Err(e) => warn!("PlayerEvent::Paused: Invalid track id: {}", e),
                Ok(track_id) => {
                    // TODO: update position
                    let iface_ref = self.mpris_player_iface().await;
                    let mut iface = iface_ref.get_mut().await;
                    let meta = &mut iface.metadata;

                    // TODO: Check if metadata changed, if so clear
                    let mut trackid = String::new();
                    trackid.push_str("/org/librespot/track/");
                    trackid.push_str(&track_id);
                    meta.insert("mpris:trackid".into(), zvariant::ObjectPath::try_from(trackid).unwrap().into());
                    iface.metadata_changed(iface_ref.signal_context()).await?;

                    iface.playback_status = PlaybackStatus::Paused;
                    iface.playback_status_changed(iface_ref.signal_context()).await?;
                }
            },
            PlayerEvent::Loading { .. } => { },
            PlayerEvent::Preloading { .. } => { },
            PlayerEvent::TimeToPreloadNextTrack { .. } => { },
            PlayerEvent::EndOfTrack { track_id, .. } => match track_id.to_base62() {
                Err(e) => warn!("PlayerEvent::EndOfTrack: Invalid track id: {}", e),
                Ok(_id) => {
                    // TODO: ?
                }
            },
            PlayerEvent::Unavailable { .. } => { },
            PlayerEvent::VolumeChanged {
                // volume
                ..
            } => {
                // TODO: Handle volume
            },
            PlayerEvent::Seeked {
                track_id,
                // position_ms,
                ..
            } => match track_id.to_base62() {
                Err(e) => warn!("PlayerEvent::Seeked: Invalid track id: {}", e),
                Ok(track_id) => {
                    // TODO: Update position + track_id
                    let iface_ref = self.mpris_player_iface().await;
                    let mut iface = iface_ref.get_mut().await;
                    let meta = &mut iface.metadata;

                    // TODO: Check if metadata changed, if so clear
                    let mut trackid = String::new();
                    trackid.push_str("/org/librespot/track/");
                    trackid.push_str(&track_id);
                    meta.insert("mpris:trackid".into(), zvariant::ObjectPath::try_from(trackid).unwrap().into());
                    iface.metadata_changed(iface_ref.signal_context()).await?;
                }
            },
            PlayerEvent::PositionCorrection {
                track_id,
                // position_ms,
                ..
            } => match track_id.to_base62() {
                Err(e) => {
                    warn!("PlayerEvent::PositionCorrection: Invalid track id: {}", e)
                }
                Ok(_id) => {
                    // TODO: Update position + track_id
                }
            },
            PlayerEvent::SessionConnected { .. } => { },
            PlayerEvent::SessionDisconnected { .. } => { },
            PlayerEvent::SessionClientChanged { .. } => { },
            PlayerEvent::ShuffleChanged { shuffle } => {
                let iface_ref = self.mpris_player_iface().await;
                let mut iface = iface_ref.get_mut().await;
                iface.shuffle = shuffle;
                iface.shuffle_changed(iface_ref.signal_context()).await?;
            },
            PlayerEvent::RepeatChanged { repeat } => {
                let iface_ref = self.mpris_player_iface().await;
                let mut iface = iface_ref.get_mut().await;
                iface.repeat = repeat;
                iface.loop_status_changed(iface_ref.signal_context()).await?;
            },
            PlayerEvent::AutoPlayChanged { .. } => { },
            PlayerEvent::FilterExplicitContentChanged { .. } => { },
        }

        Ok(())
    }
}
