use crate::{Error, SpotifyId};
use std::{borrow::Cow, fmt, str::FromStr, time::Duration};
use thiserror::Error;

use librespot_protocol as protocol;

const SPOTIFY_ITEM_TYPE_ALBUM: &str = "album";
const SPOTIFY_ITEM_TYPE_ARTIST: &str = "artist";
const SPOTIFY_ITEM_TYPE_EPISODE: &str = "episode";
const SPOTIFY_ITEM_TYPE_PLAYLIST: &str = "playlist";
const SPOTIFY_ITEM_TYPE_SHOW: &str = "show";
const SPOTIFY_ITEM_TYPE_TRACK: &str = "track";
const SPOTIFY_ITEM_TYPE_LOCAL: &str = "local";
const SPOTIFY_ITEM_TYPE_UNKNOWN: &str = "unknown";

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum SpotifyUriError {
    #[error("not a valid Spotify URI")]
    InvalidFormat,
    #[error("URI does not belong to Spotify")]
    InvalidRoot,
}

impl From<SpotifyUriError> for Error {
    fn from(err: SpotifyUriError) -> Self {
        Error::invalid_argument(err)
    }
}

pub type SpotifyUriResult = Result<SpotifyUri, Error>;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum SpotifyUri {
    Album {
        id: SpotifyId,
    },
    Artist {
        id: SpotifyId,
    },
    Episode {
        id: SpotifyId,
    },
    Playlist {
        user: Option<String>,
        id: SpotifyId,
    },
    Show {
        id: SpotifyId,
    },
    Track {
        id: SpotifyId,
    },
    Local {
        artist: String,
        album_title: String,
        track_title: String,
        duration: std::time::Duration,
    },
    Unknown {
        kind: Cow<'static, str>,
        id: String,
    },
}

impl SpotifyUri {
    /// Returns whether this `SpotifyUri` is for a playable audio item, if known.
    pub fn is_playable(&self) -> bool {
        matches!(
            self,
            SpotifyUri::Episode { .. } | SpotifyUri::Track { .. } | SpotifyUri::Local { .. }
        )
    }

    /// Gets the item type of this URI as a static string
    pub fn item_type(&self) -> &'static str {
        match &self {
            SpotifyUri::Album { .. } => SPOTIFY_ITEM_TYPE_ALBUM,
            SpotifyUri::Artist { .. } => SPOTIFY_ITEM_TYPE_ARTIST,
            SpotifyUri::Episode { .. } => SPOTIFY_ITEM_TYPE_EPISODE,
            SpotifyUri::Playlist { .. } => SPOTIFY_ITEM_TYPE_PLAYLIST,
            SpotifyUri::Show { .. } => SPOTIFY_ITEM_TYPE_SHOW,
            SpotifyUri::Track { .. } => SPOTIFY_ITEM_TYPE_TRACK,
            SpotifyUri::Local { .. } => SPOTIFY_ITEM_TYPE_LOCAL,
            SpotifyUri::Unknown { .. } => SPOTIFY_ITEM_TYPE_UNKNOWN,
        }
    }

    /// Gets the ID of this URI. The resource ID is the component of the URI that identifies
    /// the resource after its type label. If `self` is a named ID, the user will be omitted.
    pub fn to_id(&self) -> Result<String, Error> {
        match &self {
            SpotifyUri::Album { id }
            | SpotifyUri::Artist { id }
            | SpotifyUri::Episode { id }
            | SpotifyUri::Playlist { id, .. }
            | SpotifyUri::Show { id }
            | SpotifyUri::Track { id } => id.to_base62(),
            SpotifyUri::Local {
                artist,
                album_title,
                track_title,
                duration,
            } => {
                let duration_secs = duration.as_secs();
                Ok(format!(
                    "{artist}:{album_title}:{track_title}:{duration_secs}"
                ))
            }
            SpotifyUri::Unknown { id, .. } => Ok(id.clone()),
        }
    }

    /// Parses a [Spotify URI] into a `SpotifyUri`.
    ///
    /// `uri` is expected to be in the canonical form `spotify:{type}:{id}`, where `{type}`
    /// can be arbitrary while `{id}` is in a format that varies based on the `{type}`:
    ///
    ///  - For most item types, a 22-character long, base62 encoded Spotify ID is expected.
    ///  - For local files, an arbitrary length string with the fields
    ///    `{artist}:{album_title}:{track_title}:{duration_in_seconds}` is expected.
    ///
    /// Spotify URI: https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids
    pub fn from_uri(src: &str) -> SpotifyUriResult {
        // Basic: `spotify:{type}:{id}`
        // Named: `spotify:user:{user}:{type}:{id}`
        // Local: `spotify:local:{artist}:{album_title}:{track_title}:{duration_in_seconds}`
        let mut parts = src.split(':');

        let scheme = parts.next().ok_or(SpotifyUriError::InvalidFormat)?;

        if scheme != "spotify" {
            return Err(SpotifyUriError::InvalidRoot.into());
        }

        let mut username: Option<String> = None;

        let item_type = {
            let next = parts.next().ok_or(SpotifyUriError::InvalidFormat)?;
            if next == "user" {
                username.replace(
                    parts
                        .next()
                        .ok_or(SpotifyUriError::InvalidFormat)?
                        .to_owned(),
                );
                parts.next().ok_or(SpotifyUriError::InvalidFormat)?
            } else {
                next
            }
        };

        let name = parts.next().ok_or(SpotifyUriError::InvalidFormat)?;

        match item_type {
            SPOTIFY_ITEM_TYPE_ALBUM => Ok(Self::Album {
                id: SpotifyId::from_base62(name)?,
            }),
            SPOTIFY_ITEM_TYPE_ARTIST => Ok(Self::Artist {
                id: SpotifyId::from_base62(name)?,
            }),
            SPOTIFY_ITEM_TYPE_EPISODE => Ok(Self::Episode {
                id: SpotifyId::from_base62(name)?,
            }),
            SPOTIFY_ITEM_TYPE_PLAYLIST => Ok(Self::Playlist {
                id: SpotifyId::from_base62(name)?,
                user: username,
            }),
            SPOTIFY_ITEM_TYPE_SHOW => Ok(Self::Show {
                id: SpotifyId::from_base62(name)?,
            }),
            SPOTIFY_ITEM_TYPE_TRACK => Ok(Self::Track {
                id: SpotifyId::from_base62(name)?,
            }),
            SPOTIFY_ITEM_TYPE_LOCAL => {
                let artist = name;
                let album_title = parts.next().ok_or(SpotifyUriError::InvalidFormat)?;
                let track_title = parts.next().ok_or(SpotifyUriError::InvalidFormat)?;
                let duration_secs = parts
                    .next()
                    .and_then(|f| u64::from_str(f).ok())
                    .ok_or(SpotifyUriError::InvalidFormat)?;

                Ok(Self::Local {
                    artist: artist.to_owned(),
                    album_title: album_title.to_owned(),
                    track_title: track_title.to_owned(),
                    duration: Duration::from_secs(duration_secs),
                })
            }
            _ => Ok(Self::Unknown {
                kind: item_type.to_owned().into(),
                id: name.to_owned(),
            }),
        }
    }

    /// Returns the `SpotifyUri` as a [Spotify URI] in the canonical form `spotify:{type}:{id}`,
    /// where `{type}` is an arbitrary string and `{id}` is a 22-character long, base62 encoded
    /// Spotify ID.
    ///
    /// If the `SpotifyUri` has an associated type unrecognized by the library, `{type}` will
    /// be encoded as `unknown`.
    ///
    /// If the `SpotifyUri` is named, it will be returned in the form
    /// `spotify:user:{user}:{type}:{id}`.
    ///
    /// [Spotify URI]: https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids
    pub fn to_uri(&self) -> Result<String, Error> {
        let item_type = self.item_type();
        let name = self.to_id()?;

        if let SpotifyUri::Playlist {
            id,
            user: Some(user),
        } = self
        {
            Ok(format!("spotify:user:{user}:{item_type}:{id}"))
        } else {
            Ok(format!("spotify:{item_type}:{name}"))
        }
    }

    /// Gets the name of this URI. The resource name is the component of the URI that identifies
    /// the resource after its type label. If `self` is a named ID, the user will be omitted.
    ///
    /// Deprecated: not all IDs can be represented in Base62, so this function has been renamed to
    /// [SpotifyUri::to_id], which this implementation forwards to.
    #[deprecated(since = "0.8.0", note = "use to_name instead")]
    pub fn to_base62(&self) -> Result<String, Error> {
        self.to_id()
    }
}

impl fmt::Debug for SpotifyUri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("SpotifyUri")
            .field(&self.to_uri().unwrap_or_else(|_| "invalid uri".into()))
            .finish()
    }
}

impl fmt::Display for SpotifyUri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_uri().unwrap_or_else(|_| "invalid uri".into()))
    }
}

impl TryFrom<&protocol::metadata::Album> for SpotifyUri {
    type Error = crate::Error;
    fn try_from(album: &protocol::metadata::Album) -> Result<Self, Self::Error> {
        Ok(Self::Album {
            id: SpotifyId::from_raw(album.gid())?,
        })
    }
}

impl TryFrom<&protocol::metadata::Artist> for SpotifyUri {
    type Error = crate::Error;
    fn try_from(artist: &protocol::metadata::Artist) -> Result<Self, Self::Error> {
        Ok(Self::Artist {
            id: SpotifyId::from_raw(artist.gid())?,
        })
    }
}

impl TryFrom<&protocol::metadata::Episode> for SpotifyUri {
    type Error = crate::Error;
    fn try_from(episode: &protocol::metadata::Episode) -> Result<Self, Self::Error> {
        Ok(Self::Episode {
            id: SpotifyId::from_raw(episode.gid())?,
        })
    }
}

impl TryFrom<&protocol::metadata::Track> for SpotifyUri {
    type Error = crate::Error;
    fn try_from(track: &protocol::metadata::Track) -> Result<Self, Self::Error> {
        Ok(Self::Track {
            id: SpotifyId::from_raw(track.gid())?,
        })
    }
}

impl TryFrom<&protocol::metadata::Show> for SpotifyUri {
    type Error = crate::Error;
    fn try_from(show: &protocol::metadata::Show) -> Result<Self, Self::Error> {
        Ok(Self::Show {
            id: SpotifyId::from_raw(show.gid())?,
        })
    }
}

impl TryFrom<&protocol::metadata::ArtistWithRole> for SpotifyUri {
    type Error = crate::Error;
    fn try_from(artist: &protocol::metadata::ArtistWithRole) -> Result<Self, Self::Error> {
        Ok(Self::Artist {
            id: SpotifyId::from_raw(artist.artist_gid())?,
        })
    }
}

impl TryFrom<&protocol::playlist4_external::Item> for SpotifyUri {
    type Error = crate::Error;
    fn try_from(item: &protocol::playlist4_external::Item) -> Result<Self, Self::Error> {
        Self::from_uri(item.uri())
    }
}

// Note that this is the unique revision of an item's metadata on a playlist,
// not the ID of that item or playlist.
impl TryFrom<&protocol::playlist4_external::MetaItem> for SpotifyUri {
    type Error = crate::Error;
    fn try_from(item: &protocol::playlist4_external::MetaItem) -> Result<Self, Self::Error> {
        Ok(Self::Unknown {
            kind: "MetaItem".into(),
            id: SpotifyId::try_from(item.revision())?.to_base62()?,
        })
    }
}

// Note that this is the unique revision of a playlist, not the ID of that playlist.
impl TryFrom<&protocol::playlist4_external::SelectedListContent> for SpotifyUri {
    type Error = crate::Error;
    fn try_from(
        playlist: &protocol::playlist4_external::SelectedListContent,
    ) -> Result<Self, Self::Error> {
        Ok(Self::Unknown {
            kind: "SelectedListContent".into(),
            id: SpotifyId::try_from(playlist.revision())?.to_base62()?,
        })
    }
}

// TODO: check meaning and format of this field in the wild. This might be a FileId,
// which is why we now don't create a separate `Playlist` enum value yet and choose
// to discard any item type.
impl TryFrom<&protocol::playlist_annotate3::TranscodedPicture> for SpotifyUri {
    type Error = crate::Error;
    fn try_from(
        picture: &protocol::playlist_annotate3::TranscodedPicture,
    ) -> Result<Self, Self::Error> {
        Ok(Self::Unknown {
            kind: "TranscodedPicture".into(),
            id: picture.uri().to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ConversionCase {
        parsed: SpotifyUri,
        uri: &'static str,
        base62: &'static str,
    }

    static CONV_VALID: [ConversionCase; 4] = [
        ConversionCase {
            parsed: SpotifyUri::Track {
                id: SpotifyId {
                    id: 238762092608182713602505436543891614649,
                },
            },
            uri: "spotify:track:5sWHDYs0csV6RS48xBl0tH",
            base62: "5sWHDYs0csV6RS48xBl0tH",
        },
        ConversionCase {
            parsed: SpotifyUri::Track {
                id: SpotifyId {
                    id: 204841891221366092811751085145916697048,
                },
            },
            uri: "spotify:track:4GNcXTGWmnZ3ySrqvol3o4",
            base62: "4GNcXTGWmnZ3ySrqvol3o4",
        },
        ConversionCase {
            parsed: SpotifyUri::Episode {
                id: SpotifyId {
                    id: 204841891221366092811751085145916697048,
                },
            },
            uri: "spotify:episode:4GNcXTGWmnZ3ySrqvol3o4",
            base62: "4GNcXTGWmnZ3ySrqvol3o4",
        },
        ConversionCase {
            parsed: SpotifyUri::Show {
                id: SpotifyId {
                    id: 204841891221366092811751085145916697048,
                },
            },
            uri: "spotify:show:4GNcXTGWmnZ3ySrqvol3o4",
            base62: "4GNcXTGWmnZ3ySrqvol3o4",
        },
    ];

    static CONV_INVALID: [ConversionCase; 5] = [
        ConversionCase {
            parsed: SpotifyUri::Track {
                id: SpotifyId { id: 0 },
            },
            // Invalid ID in the URI.
            uri: "spotify:track:5sWHDYs0Bl0tH",
            base62: "!!!!!Ys0csV6RS48xBl0tH",
        },
        ConversionCase {
            parsed: SpotifyUri::Track {
                id: SpotifyId { id: 0 },
            },
            // Missing colon between ID and type.
            uri: "spotify:arbitrarywhatever5sWHDYs0csV6RS48xBl0tH",
            base62: "....................",
        },
        ConversionCase {
            parsed: SpotifyUri::Track {
                id: SpotifyId { id: 0 },
            },
            // Uri too short
            uri: "spotify:track:aRS48xBl0tH",
            // too long, should return error but not panic overflow
            base62: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        },
        ConversionCase {
            parsed: SpotifyUri::Track {
                id: SpotifyId { id: 0 },
            },
            // Uri too short
            uri: "spotify:track:aRS48xBl0tH",
            // too short to encode a 128 bits int
            base62: "aa",
        },
        ConversionCase {
            parsed: SpotifyUri::Track {
                id: SpotifyId { id: 0 },
            },
            uri: "cleary invalid uri",
            // too high of a value, this would need a 132 bits int
            base62: "ZZZZZZZZZZZZZZZZZZZZZZ",
        },
    ];

    struct ItemTypeCase {
        uri: SpotifyUri,
        expected_type: &'static str,
    }

    static ITEM_TYPES: [ItemTypeCase; 6] = [
        ItemTypeCase {
            uri: SpotifyUri::Album {
                id: SpotifyId { id: 0 },
            },
            expected_type: "album",
        },
        ItemTypeCase {
            uri: SpotifyUri::Artist {
                id: SpotifyId { id: 0 },
            },
            expected_type: "artist",
        },
        ItemTypeCase {
            uri: SpotifyUri::Episode {
                id: SpotifyId { id: 0 },
            },
            expected_type: "episode",
        },
        ItemTypeCase {
            uri: SpotifyUri::Playlist {
                user: None,
                id: SpotifyId { id: 0 },
            },
            expected_type: "playlist",
        },
        ItemTypeCase {
            uri: SpotifyUri::Show {
                id: SpotifyId { id: 0 },
            },
            expected_type: "show",
        },
        ItemTypeCase {
            uri: SpotifyUri::Track {
                id: SpotifyId { id: 0 },
            },
            expected_type: "track",
        },
    ];

    #[test]
    fn to_id() {
        for c in &CONV_VALID {
            assert_eq!(c.parsed.to_id().unwrap(), c.base62);
        }
    }

    #[test]
    fn item_type() {
        for i in &ITEM_TYPES {
            assert_eq!(i.uri.item_type(), i.expected_type);
        }

        // These need to use methods that can't be used in the static context like to_owned() and
        // into().

        let local_file = SpotifyUri::Local {
            artist: "".to_owned(),
            album_title: "".to_owned(),
            track_title: "".to_owned(),
            duration: Default::default(),
        };

        assert_eq!(local_file.item_type(), "local");

        let unknown = SpotifyUri::Unknown {
            kind: "not used".into(),
            id: "".to_owned(),
        };

        assert_eq!(unknown.item_type(), "unknown");
    }

    #[test]
    fn from_uri() {
        for c in &CONV_VALID {
            let actual = SpotifyUri::from_uri(c.uri).unwrap();

            assert_eq!(actual, c.parsed);
        }

        for c in &CONV_INVALID {
            assert!(SpotifyUri::from_uri(c.uri).is_err());
        }
    }

    #[test]
    fn from_invalid_type_uri() {
        let actual =
            SpotifyUri::from_uri("spotify:arbitrarywhatever:5sWHDYs0csV6RS48xBl0tH").unwrap();

        assert_eq!(
            actual,
            SpotifyUri::Unknown {
                kind: "arbitrarywhatever".into(),
                id: "5sWHDYs0csV6RS48xBl0tH".to_owned()
            }
        )
    }

    #[test]
    fn from_local_uri() {
        let actual = SpotifyUri::from_uri(
            "spotify:local:David+Wise:Donkey+Kong+Country%3A+Tropical+Freeze:Snomads+Island:127",
        )
        .unwrap();

        assert_eq!(
            actual,
            SpotifyUri::Local {
                artist: "David+Wise".to_owned(),
                album_title: "Donkey+Kong+Country%3A+Tropical+Freeze".to_owned(),
                track_title: "Snomads+Island".to_owned(),
                duration: Duration::from_secs(127),
            }
        );
    }

    #[test]
    fn from_local_uri_missing_fields() {
        let actual = SpotifyUri::from_uri("spotify:local:::Snomads+Island:127").unwrap();

        assert_eq!(
            actual,
            SpotifyUri::Local {
                artist: "".to_owned(),
                album_title: "".to_owned(),
                track_title: "Snomads+Island".to_owned(),
                duration: Duration::from_secs(127),
            }
        );
    }

    #[test]
    fn from_named_uri() {
        let actual =
            SpotifyUri::from_uri("spotify:user:spotify:playlist:37i9dQZF1DWSw8liJZcPOI").unwrap();

        let SpotifyUri::Playlist { ref user, id } = actual else {
            panic!("wrong id type");
        };

        assert_eq!(*user, Some("spotify".to_owned()));
        assert_eq!(
            id,
            SpotifyId {
                id: 136159921382084734723401526672209703396
            },
        );
    }

    #[test]
    fn to_uri() {
        for c in &CONV_VALID {
            assert_eq!(c.parsed.to_uri().unwrap(), c.uri);
        }
    }

    #[test]
    fn to_named_uri() {
        let string = "spotify:user:spotify:playlist:37i9dQZF1DWSw8liJZcPOI";

        let actual =
            SpotifyUri::from_uri("spotify:user:spotify:playlist:37i9dQZF1DWSw8liJZcPOI").unwrap();

        assert_eq!(actual.to_uri().unwrap(), string);
    }
}
