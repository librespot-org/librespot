//! The `org.mpris.MediaPlayer2.Player` interface.
//!
//! See <https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html>.

use std::{collections::HashMap, time::Instant};

use librespot_connect::Spirc;
use log::{debug, info};
use zbus::{fdo, object_server::SignalEmitter};
use zvariant::{ObjectPath, OwnedValue};

use super::{
    metadata::{Metadata, track_object_path},
    types::{LoopStatus, PlaybackRate, PlaybackStatus, TimeInUs, Volume},
};

/// Last known playback position.
pub(super) struct Position {
    ms: u32,
    last_update: Instant,
}

impl From<u32> for Position {
    fn from(value: u32) -> Self {
        Self {
            ms: value,
            last_update: Instant::now(),
        }
    }
}

pub(super) struct MprisPlayerService {
    pub(super) spirc: Option<Spirc>,
    pub(super) repeat: LoopStatus,
    pub(super) shuffle: bool,
    pub(super) playback_status: PlaybackStatus,
    pub(super) volume: u16,
    pub(super) position: Option<Position>,
    pub(super) metadata: Metadata,
}

impl MprisPlayerService {
    pub(super) fn new() -> Self {
        Self {
            spirc: None,
            // Values are updated upon reception of the first player events, right after the
            // MprisTask event handler registration
            repeat: LoopStatus::None,
            shuffle: false,
            playback_status: PlaybackStatus::Stopped,
            volume: u16::MAX,
            position: None,
            metadata: Metadata::default(),
        }
    }

    /// Current position in milliseconds.
    ///
    /// Player events only report the position when it changes in a way that is inconsistent with
    /// the playback rate, so it has to be extrapolated while playing.
    fn position_ms(&self) -> Option<u64> {
        self.position.as_ref().map(|position| {
            let elapsed_ms = match self.playback_status {
                PlaybackStatus::Playing => position.last_update.elapsed().as_millis() as u64,
                PlaybackStatus::Paused | PlaybackStatus::Stopped => 0,
            };
            let position_ms = (position.ms as u64).saturating_add(elapsed_ms);

            match self.metadata.mpris.length {
                Some(length_us) => position_ms.min(length_us as u64 / 1000),
                None => position_ms,
            }
        })
    }

    fn spirc(&self) -> fdo::Result<&Spirc> {
        self.spirc
            .as_ref()
            .ok_or_else(|| fdo::Error::Failed(String::from("Not connected")))
    }

    fn current_track(&self) -> fdo::Result<&Spirc> {
        let spirc = self.spirc()?;
        if self.metadata.mpris.track_id.is_none() {
            return Err(fdo::Error::Failed(String::from("No track")));
        }
        Ok(spirc)
    }
}

fn spirc_error(err: librespot::core::Error) -> fdo::Error {
    fdo::Error::Failed(err.to_string())
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
    async fn next(&self) -> fdo::Result<()> {
        debug!("org.mpris.MediaPlayer2.Player::Next");
        self.spirc()?.next().map_err(spirc_error)
    }

    // Skips to the previous track in the tracklist.
    //
    // If there is no previous track (and endless playback and track repeat are both off), stop
    // playback.
    //
    // If playback is paused or stopped, it remains that way.
    //
    // If `self.can_go_previous` is `false`, attempting to call this method should have no effect.
    async fn previous(&self) -> fdo::Result<()> {
        debug!("org.mpris.MediaPlayer2.Player::Previous");
        self.spirc()?.prev().map_err(spirc_error)
    }

    // Pauses playback.
    //
    // If playback is already paused, this has no effect.
    //
    // Calling Play after this should cause playback to start again from the same position.
    //
    // If `self.can_pause` is `false`, attempting to call this method should have no effect.
    async fn pause(&self) -> fdo::Result<()> {
        debug!("org.mpris.MediaPlayer2.Player::Pause");
        self.current_track()?.pause().map_err(spirc_error)
    }

    // Pauses playback.
    //
    // If playback is already paused, resumes playback.
    //
    // If playback is stopped, starts playback.
    //
    // If `self.can_pause` is `false`, attempting to call this method should have no effect and
    // raise an error.
    async fn play_pause(&self) -> fdo::Result<()> {
        debug!("org.mpris.MediaPlayer2.Player::PlayPause");
        self.current_track()?.play_pause().map_err(spirc_error)
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
    async fn stop(&self) -> fdo::Result<()> {
        debug!("org.mpris.MediaPlayer2.Player::Stop");
        let spirc = self.spirc()?;
        spirc.pause().map_err(spirc_error)?;
        spirc.set_position_ms(0).map_err(spirc_error)
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
    async fn play(&self) -> fdo::Result<()> {
        debug!("org.mpris.MediaPlayer2.Player::Play");
        let spirc = self.current_track()?;
        spirc.activate().map_err(spirc_error)?;
        spirc.play().map_err(spirc_error)
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
    async fn seek(&self, offset: TimeInUs) -> fdo::Result<()> {
        debug!("org.mpris.MediaPlayer2.Player::Seek({offset:?})");
        let spirc = self.current_track()?;
        let offset_ms = offset / 1000;

        // Seeking beyond the end of the track acts like a call to Next
        if let (Some(position_ms), Some(length_us)) =
            (self.position_ms(), self.metadata.mpris.length)
        {
            if (position_ms as i64).saturating_add(offset_ms) > length_us / 1000 {
                return spirc.next().map_err(spirc_error);
            }
        }

        let offset_ms = offset_ms.clamp(i32::MIN as i64, i32::MAX as i64) as i32;
        spirc.seek_offset(offset_ms).map_err(spirc_error)
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
    async fn set_position(&self, track_id: ObjectPath<'_>, position: TimeInUs) -> fdo::Result<()> {
        debug!("org.mpris.MediaPlayer2.Player::SetPosition({track_id:?}, {position:?})");
        let spirc = self.current_track()?;

        let length_us = self.metadata.mpris.length.unwrap_or(TimeInUs::MAX);
        if !(0..=length_us).contains(&position) {
            return Ok(());
        }

        let current_track_id = self.metadata.mpris.track_id.as_ref().map(track_object_path);
        if current_track_id.as_deref() != Some(track_id.as_str()) {
            info!("SetPosition on wrong trackId, ignoring as stale");
            return Ok(());
        }

        spirc
            .set_position_ms((position / 1000) as u32)
            .map_err(spirc_error)
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
    async fn open_uri(&self, uri: &str) -> fdo::Result<()> {
        debug!("org.mpris.MediaPlayer2.Player::OpenUri({uri:?})");
        Err(fdo::Error::NotSupported("OpenUri not supported".to_owned()))
    }

    // The current playback status.
    //
    // May be "Playing", "Paused" or "Stopped".
    #[zbus(property(emits_changed_signal = "true"))]
    async fn playback_status(&self) -> PlaybackStatus {
        debug!("org.mpris.MediaPlayer2.Player::PlaybackStatus");
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
        debug!("org.mpris.MediaPlayer2.Player::LoopStatus");
        self.repeat
    }

    #[zbus(property)]
    async fn set_loop_status(&mut self, value: LoopStatus) -> fdo::Result<()> {
        debug!("org.mpris.MediaPlayer2.Player::LoopStatus({value:?})");
        let spirc = self.spirc()?;
        match value {
            LoopStatus::None => {
                spirc.repeat(false).map_err(spirc_error)?;
                spirc.repeat_track(false).map_err(spirc_error)
            }
            LoopStatus::Track => spirc.repeat_track(true).map_err(spirc_error),
            LoopStatus::Playlist => {
                spirc.repeat(true).map_err(spirc_error)?;
                spirc.repeat_track(false).map_err(spirc_error)
            }
        }
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
        debug!("org.mpris.MediaPlayer2.Player::Rate");
        1.0
    }

    #[zbus(property)]
    async fn set_rate(&mut self, value: PlaybackRate) {
        debug!("org.mpris.MediaPlayer2.Player::Rate({value:?})");
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
        debug!("org.mpris.MediaPlayer2.Player::Shuffle");
        self.shuffle
    }

    #[zbus(property)]
    async fn set_shuffle(&mut self, value: bool) -> fdo::Result<()> {
        debug!("org.mpris.MediaPlayer2.Player::Shuffle({value:?})");
        self.spirc()?.shuffle(value).map_err(spirc_error)
    }

    // The metadata of the current element.
    //
    // If there is a current track, this must have a "mpris:trackid" entry (of D-Bus type "o") at
    // the very least, which contains a D-Bus path that uniquely identifies this track.
    //
    // See the type documentation for more details.
    #[zbus(property(emits_changed_signal = "true"))]
    async fn metadata(&self) -> fdo::Result<HashMap<String, OwnedValue>> {
        debug!("org.mpris.MediaPlayer2.Player::Metadata");
        self.metadata
            .clone()
            .try_into()
            .map_err(|err| fdo::Error::ZBus(zbus::Error::Variant(err)))
    }

    // The volume level.
    //
    // When setting, if a negative value is passed, the volume should be set to 0.0.
    //
    // If `CanControl` is `false`, attempting to set this property should have no effect and raise
    // an error.
    #[zbus(property(emits_changed_signal = "true"))]
    async fn volume(&self) -> Volume {
        debug!("org.mpris.MediaPlayer2.Player::Volume");
        self.volume as f64 / u16::MAX as f64
    }

    #[zbus(property)]
    async fn set_volume(&mut self, value: Volume) -> fdo::Result<()> {
        debug!("org.mpris.MediaPlayer2.Player::Volume({value})");
        // MPRIS volume is expected to range between 0 and 1, see
        // https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Simple-Type:Volume
        // The cast saturates, so negative values map to 0 and values above 1 to the maximum.
        let mapped_volume = (value * (u16::MAX as f64)).round() as u16;
        self.spirc()?.set_volume(mapped_volume).map_err(spirc_error)
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
    async fn position(&self) -> TimeInUs {
        debug!("org.mpris.MediaPlayer2.Player::Position");
        self.position_ms().unwrap_or_default() as TimeInUs * 1000
    }

    // The minimum value which the `Rate` property can take. Clients should not attempt to set the
    // `Rate` property below this value.
    //
    // Note that even if this value is 0.0 or negative, clients should not attempt to set the
    // `Rate` property to 0.0.
    //
    // This value should always be 1.0 or less.
    #[zbus(property(emits_changed_signal = "true"))]
    async fn minimum_rate(&self) -> PlaybackRate {
        debug!("org.mpris.MediaPlayer2.Player::MinimumRate");
        // Setting minimum and maximum rate to 1 disallow client to set rate.
        1.0
    }

    // The maximum value which the `Rate` property can take. Clients should not attempt to set the
    // `Rate` property above this value.
    //
    // This value should always be 1.0 or greater.
    #[zbus(property(emits_changed_signal = "true"))]
    async fn maximum_rate(&self) -> PlaybackRate {
        debug!("org.mpris.MediaPlayer2.Player::MaximumRate");
        // Setting minimum and maximum rate to 1 disallow client to set rate.
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
        debug!("org.mpris.MediaPlayer2.Player::CanGoNext");
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
        debug!("org.mpris.MediaPlayer2.Player::CanGoPrevious");
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
        debug!("org.mpris.MediaPlayer2.Player::CanPlay");
        self.metadata.mpris.track_id.is_some()
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
        debug!("org.mpris.MediaPlayer2.Player::CanPause");
        self.metadata.mpris.track_id.is_some()
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
        debug!("org.mpris.MediaPlayer2.Player::CanSeek");
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
        debug!("org.mpris.MediaPlayer2.Player::CanControl");
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
    pub(super) async fn seeked(
        signal_emitter: &SignalEmitter<'_>,
        position: TimeInUs,
    ) -> zbus::Result<()>;
}
