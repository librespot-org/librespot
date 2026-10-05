//! Track metadata, as exposed by the `Metadata` property of the MPRIS player interface.
//!
//! See <https://www.freedesktop.org/wiki/Specifications/mpris-spec/metadata/>.

use std::collections::HashMap;

use data_encoding::HEXLOWER;
use time::format_description::well_known::Iso8601;
use zvariant::{ObjectPath, OwnedValue};

use librespot::{
    core::{SpotifyUri, date::Date},
    metadata::audio::UniqueFields,
};

/// Object path used when there is no current track.
const NO_TRACK: &str = "/org/mpris/MediaPlayer2/TrackList/NoTrack";

/// Returns the D-Bus object path identifying a track in the `mpris:trackid` metadata entry.
///
/// Object paths may only contain `[A-Za-z0-9_]` elements, so ids which are not plain base62 (e.g.
/// local files) are hex encoded, behind a `_` prefix to avoid any collision with base62 ids.
pub(super) fn track_object_path(uri: &SpotifyUri) -> String {
    let id = uri.to_id();
    if !id.is_empty() && id.bytes().all(|b| b.is_ascii_alphanumeric()) {
        format!("/org/librespot/track/{id}")
    } else {
        format!("/org/librespot/track/_{}", HEXLOWER.encode(id.as_bytes()))
    }
}

/// MPRIS-specific metadata
#[derive(Default, Clone)]
pub(super) struct MprisMetadata {
    /// D-Bus path: A unique identity for this track within the context of an MPRIS object (eg: tracklist).
    pub(super) track_id: Option<SpotifyUri>,
    /// 64-bit integer: The duration of the track in microseconds.
    pub(super) length: Option<i64>,
    /// URI: The location of an image representing the track or album. Clients should not assume this will continue to exist when the media player stops giving out the URL.
    pub(super) art_url: Option<String>,
}

/// Common audio properties from the Xesam specification
#[derive(Default, Clone)]
pub(super) struct XesamMetadata {
    /// String: The album name.
    pub(super) album: Option<String>,
    /// List of Strings: The album artist(s).
    pub(super) album_artist: Option<Vec<String>>,
    /// List of Strings: The track artist(s).
    pub(super) artist: Option<Vec<String>>,
    /// String: The track lyrics.
    pub(super) as_text: Option<String>,
    /// Integer: The speed of the music, in beats per minute.
    pub(super) audio_bpm: Option<u32>,
    /// Float: An automatically-generated rating, based on things such as how often it has been played. This should be in the range 0.0 to 1.0.
    pub(super) auto_rating: Option<f64>,
    /// List of Strings: A (list of) freeform comment(s).
    pub(super) comment: Option<Vec<String>>,
    /// List of Strings: The composer(s) of the track.
    pub(super) composer: Option<Vec<String>>,
    /// Date/Time: When the track was created. Usually only the year component will be useful.
    pub(super) content_created: Option<Date>,
    /// Integer: The disc number on the album that this track is from.
    pub(super) disc_number: Option<i32>,
    /// Date/Time: When the track was first played.
    pub(super) first_used: Option<Date>,
    /// List of Strings: The genre(s) of the track.
    pub(super) genre: Option<Vec<String>>,
    /// Date/Time: When the track was last played.
    pub(super) last_used: Option<Date>,
    /// List of Strings: The lyricist(s) of the track.
    pub(super) lyricist: Option<Vec<String>>,
    /// String: The track title.
    pub(super) title: Option<String>,
    /// Integer: The track number on the album disc.
    pub(super) track_number: Option<i32>,
    /// URI: The location of the media file.
    pub(super) url: Option<String>,
    /// Integer: The number of times the track has been played.
    pub(super) use_count: Option<u16>,
    /// Float: A user-specified rating. This should be in the range 0.0 to 1.0.
    pub(super) user_rating: Option<f64>,
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
            UniqueFields::Local {
                artists,
                album,
                album_artists,
                number,
                disc_number,
                path,
            } => {
                xesam.artist = artists.map(|artists| vec![artists]);
                xesam.album_artist = album_artists.map(|album_artists| vec![album_artists]);
                xesam.album = album;
                xesam.track_number = number.map(|number| number as i32);
                xesam.disc_number = disc_number.map(|disc_number| disc_number as i32);
                xesam.url = Some(format!("file://{}", path.display()));
            }
        }

        xesam
    }
}

#[derive(Default, Clone)]
pub(super) struct Metadata {
    pub(super) mpris: MprisMetadata,
    pub(super) xesam: XesamMetadata,
}

impl TryInto<HashMap<String, OwnedValue>> for Metadata {
    type Error = zvariant::Error;

    fn try_into(self) -> Result<HashMap<String, OwnedValue>, Self::Error> {
        let mut meta: HashMap<String, OwnedValue> = HashMap::new();

        let track_id = self
            .mpris
            .track_id
            .as_ref()
            .map(track_object_path)
            .unwrap_or_else(|| NO_TRACK.to_string());
        meta.insert(
            String::from("mpris:trackid"),
            ObjectPath::try_from(track_id)?.into(),
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

#[cfg(test)]
mod tests {
    use super::*;

    fn track_id(metadata: Metadata) -> String {
        let meta: HashMap<String, OwnedValue> = metadata.try_into().unwrap();
        let track_id = meta
            .get("mpris:trackid")
            .expect("mpris:trackid is mandatory");
        ObjectPath::try_from(track_id.try_clone().unwrap())
            .unwrap()
            .to_string()
    }

    #[test]
    fn no_track() {
        assert_eq!(track_id(Metadata::default()), NO_TRACK);
    }

    #[test]
    fn spotify_track() {
        let uri = SpotifyUri::from_uri("spotify:track:4GKJALTXTwhZgLPp0usd5C").unwrap();
        let mut metadata = Metadata::default();
        metadata.mpris.track_id = Some(uri);

        assert_eq!(
            track_id(metadata),
            "/org/librespot/track/4GKJALTXTwhZgLPp0usd5C"
        );
    }

    #[test]
    fn local_track() {
        let uri = SpotifyUri::from_uri(
            "spotify:local:David+Wise:Donkey+Kong+Country%3A+Tropical+Freeze:Snomads+Island:127",
        )
        .unwrap();
        let path = track_object_path(&uri);

        assert!(path.starts_with("/org/librespot/track/_"));
        assert!(ObjectPath::try_from(path).is_ok());
    }
}
