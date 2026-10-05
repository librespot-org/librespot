//! The `org.mpris.MediaPlayer2` root interface.
//!
//! See <https://specifications.freedesktop.org/mpris-spec/latest/Media_Player.html>.

use log::debug;

pub(super) struct MprisService {
    pub(super) identity: String,
    pub(super) desktop_entry: Option<String>,
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
