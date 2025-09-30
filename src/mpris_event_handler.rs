use std::{collections::HashMap, process, sync::Arc, time::Instant};

use librespot_connect::Spirc;
use log::{debug, info, warn};
use thiserror::Error;
use time::format_description::well_known::Iso8601;
use tokio::sync::mpsc;
use zbus::connection;

use librespot::{
    core::date::Date,
    core::{Error, SpotifyUri},
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

// /// Unique track identifier.
// ///
// /// If the media player implements the TrackList interface and allows
// /// the same track to appear multiple times in the tracklist,
// /// this must be unique within the scope of the tracklist.
// ///
// /// Note that this should be a valid D-Bus object id, although clients
// /// should not assume that any object is actually exported with any
// /// interfaces at that path.
// ///
// /// Media players may not use any paths starting with
// /// `/org/mpris` unless explicitly allowed by this specification.
// /// Such paths are intended to have special meaning, such as
// /// `/org/mpris/MediaPlayer2/TrackList/NoTrack`
// /// to indicate "no track".
// ///
// /// This is a D-Bus object id as that is the definitive way to have
// /// unique identifiers on D-Bus.  It also allows for future optional
// /// expansions to the specification where tracks are exported to D-Bus
// /// with an interface similar to org.gnome.UPnP.MediaItem2.
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

struct MprisService {
    identity: String,
    desktop_entry: Option<String>,
}

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
        self.identity.clone()
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
        self.desktop_entry.clone().unwrap_or_default()
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

/// MPRIS-specific metadata
#[derive(Default, Clone)]
struct MprisMetadata {
    /// D-Bus path: A unique identity for this track within the context of an MPRIS object (eg: tracklist).
    track_id: Option<SpotifyUri>,
    /// 64-bit integer: The duration of the track in microseconds.
    length: Option<i64>,
    /// URI: The location of an image representing the track or album. Clients should not assume this will continue to exist when the media player stops giving out the URL.
    art_url: Option<String>,
}

/// Common audio properties from the Xesam specification
#[derive(Default, Clone)]
struct XesamMetadata {
    /// String: The album name.
    album: Option<String>,
    /// List of Strings: The album artist(s).
    album_artist: Option<Vec<String>>,
    /// List of Strings: The track artist(s).
    artist: Option<Vec<String>>,
    /// String: The track lyrics.
    as_text: Option<String>,
    /// Integer: The speed of the music, in beats per minute.
    audio_bpm: Option<u32>,
    /// Float: An automatically-generated rating, based on things such as how often it has been played. This should be in the range 0.0 to 1.0.
    auto_rating: Option<f64>,
    /// List of Strings: A (list of) freeform comment(s).
    comment: Option<Vec<String>>,
    /// List of Strings: The composer(s) of the track.
    composer: Option<Vec<String>>,
    /// Date/Time: When the track was created. Usually only the year component will be useful.
    content_created: Option<Date>,
    /// Integer: The disc number on the album that this track is from.
    disc_number: Option<i32>,
    /// Date/Time: When the track was first played.
    first_used: Option<Date>,
    /// List of Strings: The genre(s) of the track.
    genre: Option<Vec<String>>,
    /// Date/Time: When the track was last played.
    last_used: Option<Date>,
    /// List of Strings: The lyricist(s) of the track.
    lyricist: Option<Vec<String>>,
    /// String: The track title.
    title: Option<String>,
    /// Integer: The track number on the album disc.
    track_number: Option<i32>,
    /// URI: The location of the media file.
    url: Option<String>,
    /// Integer: The number of times the track has been played.
    use_count: Option<u16>,
    /// Float: A user-specified rating. This should be in the range 0.0 to 1.0.
    user_rating: Option<f64>,
}

impl From<UniqueFields> for XesamMetadata {
    fn from(value: UniqueFields) -> Self {
        let mut xesam = Self::default();

        match value {
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
                xesam.artist = Some(artists);
                xesam.album_artist = Some(album_artists);
                xesam.album = Some(album);
                xesam.track_number = Some(number as i32);
                xesam.disc_number = Some(disc_number as i32);
                xesam.content_created = Some(album_date);
            }
            UniqueFields::Episode {
                description,
                publish_time,
                show_name,
            } => {
                xesam.album = Some(show_name);
                xesam.comment = Some(vec![description]);
                xesam.content_created = Some(publish_time);
            }
        }

        xesam
    }
}

#[derive(Default, Clone)]
struct Metadata {
    mpris: MprisMetadata,
    xesam: XesamMetadata,
}

impl TryInto<HashMap<String, zbus::zvariant::OwnedValue>> for Metadata {
    type Error = zbus::Error;

    fn try_into(self) -> Result<HashMap<String, zbus::zvariant::OwnedValue>, Self::Error> {
        let mut meta: HashMap<String, zbus::zvariant::OwnedValue> = HashMap::new();

        let track_id = self.mpris.track_id.map(|track_id| {
            track_id
                .to_id()
                .map(|id| format!("/org/librespot/track/{id}"))
                .ok()
        });
        let track_id = track_id
            .flatten()
            .unwrap_or(" /org/mpris/MediaPlayer2/TrackList/NoTrack".to_string());
        meta.insert(
            String::from("mpris:trackId"),
            zvariant::ObjectPath::try_from(track_id)?.into(),
        );

        if let Some(length) = self.mpris.length {
            meta.insert(String::from("mpris:length"), length.into());
        }
        if let Some(art_url) = self.mpris.art_url {
            meta.insert(
                String::from("mpris:artUrl"),
                zvariant::Str::from(art_url).into(),
            );
        }

        if let Some(album) = self.xesam.album {
            meta.insert(
                String::from("xesam:album"),
                zvariant::Str::from(album).into(),
            );
        }
        if let Some(album_artist) = self.xesam.album_artist {
            meta.insert(
                String::from("xesam:albumArtist"),
                zvariant::Array::from(album_artist).try_into()?,
            );
        }
        if let Some(artist) = self.xesam.artist {
            meta.insert(
                String::from("xesam:artist"),
                zvariant::Array::from(artist).try_into()?,
            );
        }
        if let Some(as_text) = self.xesam.as_text {
            meta.insert(
                String::from("xesam:asText"),
                zvariant::Str::from(as_text).into(),
            );
        }
        if let Some(audio_bpm) = self.xesam.audio_bpm {
            meta.insert(String::from("xesam:audioBPM"), audio_bpm.into());
        }
        if let Some(auto_rating) = self.xesam.auto_rating {
            meta.insert(String::from("xesam:autoRating"), auto_rating.into());
        }
        if let Some(comment) = self.xesam.comment {
            meta.insert(
                String::from("xesam:comment"),
                zvariant::Array::from(comment).try_into()?,
            );
        }
        if let Some(composer) = self.xesam.composer {
            meta.insert(
                String::from("xesam:composer"),
                zvariant::Array::from(composer).try_into()?,
            );
        }
        if let Some(content_created) = self.xesam.content_created {
            meta.insert(
                String::from("xesam:contentCreated"),
                zvariant::Str::from(
                    content_created
                        .format(&Iso8601::DEFAULT)
                        .map_err(|err| zvariant::Error::Message(format!("{err}")))?,
                )
                .into(),
            );
        }
        if let Some(disc_number) = self.xesam.disc_number {
            meta.insert(String::from("xesam:discNumber"), disc_number.into());
        }
        if let Some(first_used) = self.xesam.first_used {
            meta.insert(
                String::from("xesam:firstUsed"),
                zvariant::Str::from(
                    first_used
                        .format(&Iso8601::DEFAULT)
                        .map_err(|err| zvariant::Error::Message(format!("{err}")))?,
                )
                .into(),
            );
        }
        if let Some(genre) = self.xesam.genre {
            meta.insert(
                String::from("xesam:genre"),
                zvariant::Array::from(genre).try_into()?,
            );
        }
        if let Some(last_used) = self.xesam.last_used {
            meta.insert(
                String::from("xesam:lastUsed"),
                zvariant::Str::from(
                    last_used
                        .format(&Iso8601::DEFAULT)
                        .map_err(|err| zvariant::Error::Message(format!("{err}")))?,
                )
                .into(),
            );
        }
        if let Some(lyricist) = self.xesam.lyricist {
            meta.insert(
                String::from("xesam:lyricist"),
                zvariant::Array::from(lyricist).try_into()?,
            );
        }
        if let Some(title) = self.xesam.title {
            meta.insert(
                String::from("xesam:title"),
                zvariant::Str::from(title).into(),
            );
        }
        if let Some(track_number) = self.xesam.track_number {
            meta.insert(String::from("xesam:trackNumber"), track_number.into());
        }
        if let Some(url) = self.xesam.url {
            meta.insert(String::from("xesam:url"), zvariant::Str::from(url).into());
        }
        if let Some(use_count) = self.xesam.use_count {
            meta.insert(String::from("xesam:useCount"), use_count.into());
        }
        if let Some(user_rating) = self.xesam.user_rating {
            meta.insert(String::from("xesam:userRating"), user_rating.into());
        }

        Ok(meta)
    }
}

struct Position {
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

struct MprisPlayerService {
    spirc: Option<Spirc>,
    repeat: LoopStatus,
    shuffle: bool,
    playback_status: PlaybackStatus,
    volume: u16,
    position: Option<Position>,
    metadata: Metadata,
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
        log::debug!("org.mpris.MediaPlayer2.Player::Next");
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
        log::debug!("org.mpris.MediaPlayer2.Player::Previous");
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
    async fn pause(&self) -> zbus::fdo::Result<()> {
        debug!("org.mpris.MediaPlayer2.Player::Pause");
        match (&self.spirc, &self.metadata.mpris.track_id) {
            (Some(spirc), Some(_)) => spirc
                .pause()
                .map_err(|err| zbus::fdo::Error::Failed(format!("{err}"))),
            (Some(_), None) => {
                zbus::fdo::Result::Err(zbus::fdo::Error::Failed(String::from("No track")))
            }
            _ => zbus::fdo::Result::Err(zbus::fdo::Error::Failed(String::from("Can't play/pause"))),
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
    async fn play_pause(&self) -> zbus::fdo::Result<()> {
        debug!("org.mpris.MediaPlayer2.Player::PlayPause");
        match (&self.spirc, &self.metadata.mpris.track_id) {
            (Some(spirc), Some(_)) => spirc
                .play_pause()
                .map_err(|err| zbus::fdo::Error::Failed(format!("{err}"))),
            (Some(_), None) => {
                zbus::fdo::Result::Err(zbus::fdo::Error::Failed(String::from("No track")))
            }
            _ => zbus::fdo::Result::Err(zbus::fdo::Error::Failed(String::from("Can't play/pause"))),
        }
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
        debug!("org.mpris.MediaPlayer2.Player::Stop");
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
    async fn play(&self) -> zbus::fdo::Result<()> {
        debug!("org.mpris.MediaPlayer2.Player::Play");
        if let Some(spirc) = &self.spirc {
            let _ = spirc.activate();
            let _ = spirc.play();
        }
        match (&self.spirc, &self.metadata.mpris.track_id) {
            (Some(spirc), Some(_)) => {
                let result: Result<(), Error> = (|| {
                    spirc.activate()?;
                    spirc.play()
                })();
                result.map_err(|err| zbus::fdo::Error::Failed(format!("{err}")))
            }
            (Some(_), None) => {
                zbus::fdo::Result::Err(zbus::fdo::Error::Failed(String::from("No track")))
            }
            _ => zbus::fdo::Result::Err(zbus::fdo::Error::Failed(String::from("Can't play/pause"))),
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
        debug!("org.mpris.MediaPlayer2.Player::Seek({offset:?})");
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
    async fn set_position(&self, track_id: zbus::zvariant::ObjectPath<'_>, position: TimeInUs) {
        debug!("org.mpris.MediaPlayer2.Player::SetPosition({track_id:?}, {position:?})");
        if position < 0 {
            return;
        }
        if let Some(spirc) = &self.spirc {
            let current_track_id = self.metadata.mpris.track_id.as_ref().and_then(|track_id| {
                track_id
                    .to_id()
                    .ok()
                    .map(|id| format!("/org/librespot/track/{id}"))
            });
            if current_track_id.as_deref() == Some(track_id.as_str()) {
                let _ = spirc.set_position_ms((position / 1000) as u32);
            } else {
                info!("SetPosition on wrong trackId, ignoring as stale");
            }
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
    async fn open_uri(&self, uri: &str) -> zbus::fdo::Result<()> {
        debug!("org.mpris.MediaPlayer2.Player::OpenUri({uri:?})");
        Err(zbus::fdo::Error::NotSupported(
            "OpenUri not supported".to_owned(),
        ))
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
    async fn set_loop_status(&mut self, value: LoopStatus) -> zbus::fdo::Result<()> {
        debug!("org.mpris.MediaPlayer2.Player::LoopStatus({value:?})");
        match value {
            LoopStatus::None => {
                if let Some(spirc) = &self.spirc {
                    let _ = spirc.repeat(false);
                    let _ = spirc.repeat_track(false);
                }
            }
            LoopStatus::Track => {
                if let Some(spirc) = &self.spirc {
                    let _ = spirc.repeat_track(true);
                }
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
    async fn set_shuffle(&mut self, value: bool) {
        debug!("org.mpris.MediaPlayer2.Player::Shuffle({value:?})");
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
        debug!("org.mpris.MediaPlayer2.Player::Metadata");
        self.metadata
            .clone()
            .try_into()
            .map_err(zbus::fdo::Error::ZBus)
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
    async fn set_volume(&mut self, value: Volume) -> zbus::fdo::Result<()> {
        debug!("org.mpris.MediaPlayer2.Player::Volume({value})");
        if let Some(spirc) = &self.spirc {
            // As of rust 1.45, cast is guaranteed to round to 0 and saturate.
            // MPRIS volume is expected to range between 0 and 1, see
            // https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Simple-Type:Volume
            let mapped_volume = (value * (u16::MAX as f64)).round() as u16;
            spirc
                .set_volume(mapped_volume)
                .map_err(|err| zbus::fdo::Error::Failed(format!("{err}")))?;
        }
        Ok(())
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
        debug!("org.mpris.MediaPlayer2.Player::Position");

        self.position
            .as_ref()
            .map(|position| {
                let corrected = (position.ms as u128)
                    .saturating_add(position.last_update.elapsed().as_millis());
                corrected as i64 * 1000
            })
            .ok_or(zbus::fdo::Error::Failed(String::from("Got no position")))
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
    async fn seeked(signal_ctxt: &zbus::SignalContext<'_>, position: TimeInUs) -> zbus::Result<()>;
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
    fn connection_builder<'a>(
        identity: &str,
        name: &str,
        desktop_entry: Option<&str>,
    ) -> zbus::Result<connection::Builder<'a>> {
        let mpris_service = MprisService {
            identity: identity.to_string(),
            desktop_entry: desktop_entry.map(|desktop_entry| desktop_entry.to_string()),
        };
        let mpris_player_service = MprisPlayerService {
            spirc: None,
            // Values are updated upon reception of first player state, right after MprisTask event
            // handler registration
            repeat: LoopStatus::None,
            shuffle: false,
            playback_status: PlaybackStatus::Stopped,
            volume: u16::MAX,
            position: None,
            metadata: Metadata::default(),
        };

        connection::Builder::session()?
            .name(name.to_string())?
            .serve_at("/org/mpris/MediaPlayer2", mpris_service)?
            .serve_at("/org/mpris/MediaPlayer2", mpris_player_service)
    }

    pub async fn spawn(
        player: Arc<Player>,
        name: &str,
        desktop_entry: Option<&str>,
    ) -> Result<MprisEventHandler, MprisError> {
        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();

        let connection =
            Self::connection_builder(name, "org.mpris.MediaPlayer2.librespot", desktop_entry)?
                .build()
                .await;
        let connection = match connection {
            Err(zbus::Error::NameTaken) => {
                let pid_name =
                    format!("org.mpris.MediaPlayer2.librespot.instance{}", process::id());
                warn!("zbus name taken, trying with pid specific name: {pid_name}");

                Self::connection_builder(name, &pid_name, desktop_entry)?
                    .build()
                    .await
            }
            _ => connection,
        }?;

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
                        warn!("Error handling PlayerEvent: {e}");
                    }
                }

                cmd = self.cmd_rx.recv() => {
                    match cmd {
                        Some(MprisCommand::SetSpirc(spirc)) => {
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
            PlayerEvent::PlayRequestIdChanged { play_request_id: _ } => {}
            PlayerEvent::TrackChanged { audio_item } => {
                let iface_ref = self.mpris_player_iface().await;
                let mut iface = iface_ref.get_mut().await;

                let meta = &mut iface.metadata;
                *meta = Metadata::default();

                meta.mpris.track_id = Some(audio_item.track_id);
                meta.xesam.title = Some(audio_item.name);

                // Choose biggest cover
                if let Some(url) = audio_item
                    .covers
                    .iter()
                    .max_by(|a, b| (a.size as u8).cmp(&(b.size as u8)))
                    .map(|cover| &cover.url)
                {
                    meta.mpris.art_url = Some(String::from(url));
                }

                meta.mpris.length = Some(audio_item.duration_ms as i64 * 1000);

                meta.xesam = audio_item.unique_fields.into();

                iface.metadata_changed(iface_ref.signal_context()).await?;
            }
            PlayerEvent::Stopped { track_id, .. } => {
                let iface_ref = self.mpris_player_iface().await;
                let mut iface = iface_ref.get_mut().await;
                let meta = &mut iface.metadata;

                if meta.mpris.track_id.as_ref() != Some(&track_id) {
                    *meta = Metadata::default();
                    meta.mpris.track_id = Some(track_id);
                    warn!("Missed TrackChanged event, metadata missing");
                    iface.metadata_changed(iface_ref.signal_context()).await?;
                }

                iface.playback_status = PlaybackStatus::Stopped;
                iface
                    .playback_status_changed(iface_ref.signal_context())
                    .await?;
            }
            PlayerEvent::Playing {
                track_id,
                position_ms,
                ..
            } => {
                let iface_ref = self.mpris_player_iface().await;
                let mut iface = iface_ref.get_mut().await;

                iface.position = Some(Position::from(position_ms));

                let meta = &mut iface.metadata;

                if meta.mpris.track_id.as_ref() != Some(&track_id) {
                    *meta = Metadata::default();
                    meta.mpris.track_id = Some(track_id);
                    warn!("Missed TrackChanged event, metadata missing");
                    iface.metadata_changed(iface_ref.signal_context()).await?;
                }

                iface.playback_status = PlaybackStatus::Playing;
                iface
                    .playback_status_changed(iface_ref.signal_context())
                    .await?;
            }
            PlayerEvent::Paused {
                track_id,
                position_ms,
                ..
            } => {
                let iface_ref = self.mpris_player_iface().await;
                let mut iface = iface_ref.get_mut().await;

                iface.position = Some(Position::from(position_ms));

                let meta = &mut iface.metadata;

                if meta.mpris.track_id.as_ref() != Some(&track_id) {
                    *meta = Metadata::default();
                    meta.mpris.track_id = Some(track_id);
                    warn!("Missed TrackChanged event, metadata missing");
                    iface.metadata_changed(iface_ref.signal_context()).await?;
                }

                iface.playback_status = PlaybackStatus::Paused;
                iface
                    .playback_status_changed(iface_ref.signal_context())
                    .await?;
            }
            PlayerEvent::Loading { .. } => {}
            PlayerEvent::Preloading { .. } => {}
            PlayerEvent::TimeToPreloadNextTrack { .. } => {}
            PlayerEvent::EndOfTrack { track_id, .. } => {
                let iface_ref = self.mpris_player_iface().await;
                let mut iface = iface_ref.get_mut().await;
                let meta = &mut iface.metadata;

                if meta.mpris.track_id.as_ref() == Some(&track_id) {
                    iface.position = meta
                        .mpris
                        .length
                        .map(|length| Position::from((length as f64 / 1000.) as u32));
                } else {
                    *meta = Metadata::default();
                    meta.mpris.track_id = Some(track_id);
                    warn!("Missed TrackChanged event, metadata missing");
                    iface.position = None;
                    iface.metadata_changed(iface_ref.signal_context()).await?;
                }
            }
            PlayerEvent::Unavailable { .. } => {}
            PlayerEvent::VolumeChanged { volume, .. } => {
                let iface_ref = self.mpris_player_iface().await;
                let mut iface = iface_ref.get_mut().await;
                if iface.volume != volume {
                    iface.volume = volume;
                    iface.volume_changed(iface_ref.signal_context()).await?;
                }
            }
            PlayerEvent::Seeked {
                track_id,
                position_ms,
                ..
            } => {
                let iface_ref = self.mpris_player_iface().await;
                let mut iface = iface_ref.get_mut().await;

                iface.position = Some(Position::from(position_ms));

                MprisPlayerService::seeked(iface_ref.signal_context(), position_ms as i64 * 1000)
                    .await?;

                let meta = &mut iface.metadata;
                if meta.mpris.track_id.as_ref() != Some(&track_id) {
                    *meta = Metadata::default();
                    meta.mpris.track_id = Some(track_id);
                    warn!("Missed TrackChanged event, metadata missing");
                    iface.metadata_changed(iface_ref.signal_context()).await?;
                }
            }
            PlayerEvent::PositionCorrection {
                track_id,
                position_ms,
                ..
            } => {
                let iface_ref = self.mpris_player_iface().await;
                let mut iface = iface_ref.get_mut().await;

                iface.position = Some(Position::from(position_ms));

                MprisPlayerService::seeked(iface_ref.signal_context(), position_ms as i64 * 1000)
                    .await?;

                let meta = &mut iface.metadata;
                if meta.mpris.track_id.as_ref() != Some(&track_id) {
                    *meta = Metadata::default();
                    meta.mpris.track_id = Some(track_id);
                    warn!("Missed TrackChanged event, metadata missing");
                    iface.metadata_changed(iface_ref.signal_context()).await?;
                }
            }
            PlayerEvent::PositionChanged {
                track_id,
                position_ms,
                ..
            } => {
                let iface_ref = self.mpris_player_iface().await;
                let mut iface = iface_ref.get_mut().await;

                iface.position = Some(Position::from(position_ms));

                MprisPlayerService::seeked(iface_ref.signal_context(), position_ms as i64 * 1000)
                    .await?;

                let meta = &mut iface.metadata;
                if meta.mpris.track_id.as_ref() != Some(&track_id) {
                    *meta = Metadata::default();
                    meta.mpris.track_id = Some(track_id);
                    warn!("Missed TrackChanged event, metadata missing");
                    iface.metadata_changed(iface_ref.signal_context()).await?;
                }
            }
            PlayerEvent::SessionConnected { .. } => {}
            PlayerEvent::SessionDisconnected { .. } => {}
            PlayerEvent::SessionClientChanged { .. } => {}
            PlayerEvent::ShuffleChanged { shuffle } => {
                let iface_ref = self.mpris_player_iface().await;
                let mut iface = iface_ref.get_mut().await;
                iface.shuffle = shuffle;
                iface.shuffle_changed(iface_ref.signal_context()).await?;
            }
            PlayerEvent::RepeatChanged { context, track } => {
                let iface_ref = self.mpris_player_iface().await;
                let mut iface = iface_ref.get_mut().await;
                if context {
                    iface.repeat = LoopStatus::Playlist;
                } else if track {
                    iface.repeat = LoopStatus::Track;
                } else {
                    iface.repeat = LoopStatus::None;
                }
                iface
                    .loop_status_changed(iface_ref.signal_context())
                    .await?;
            }
            PlayerEvent::AutoPlayChanged { .. } => {}
            PlayerEvent::FilterExplicitContentChanged { .. } => {}
        }

        Ok(())
    }
}
