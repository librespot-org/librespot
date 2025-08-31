use crate::{Error, SpotifyId};
use std::fmt;
use thiserror::Error;

use librespot_protocol as protocol;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpotifyItemType {
    Album,
    Artist,
    Episode,
    Playlist,
    Show,
    Track,
    Local,
    Unknown,
}

impl From<&str> for SpotifyItemType {
    fn from(v: &str) -> Self {
        match v {
            "album" => Self::Album,

            "artist" => Self::Artist,
            "episode" => Self::Episode,
            "playlist" => Self::Playlist,
            "show" => Self::Show,
            "track" => Self::Track,
            "local" => Self::Local,
            _ => Self::Unknown,
        }
    }
}

impl From<SpotifyItemType> for &str {
    fn from(item_type: SpotifyItemType) -> &'static str {
        match item_type {
            SpotifyItemType::Album => "album",
            SpotifyItemType::Artist => "artist",
            SpotifyItemType::Episode => "episode",
            SpotifyItemType::Playlist => "playlist",
            SpotifyItemType::Show => "show",
            SpotifyItemType::Track => "track",
            SpotifyItemType::Local => "local",
            _ => "unknown",
        }
    }
}

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
        id: SpotifyId,
    },
}

impl SpotifyUri {
    /// Returns whether this `SpotifyUri` is for a playable audio item, if known.
    pub fn is_playable(&self) -> bool {
        matches!(self, SpotifyUri::Episode { .. } | SpotifyUri::Track { .. })
    }

    /// Gets the item type of this URI as a [SpotifyItemType] value.
    pub fn item_type(&self) -> SpotifyItemType {
        match &self {
            SpotifyUri::Album { .. } => SpotifyItemType::Album,
            SpotifyUri::Artist { .. } => SpotifyItemType::Artist,
            SpotifyUri::Episode { .. } => SpotifyItemType::Episode,
            SpotifyUri::Playlist { .. } => SpotifyItemType::Playlist,
            SpotifyUri::Show { .. } => SpotifyItemType::Show,
            SpotifyUri::Track { .. } => SpotifyItemType::Track,
            SpotifyUri::Local { .. } => SpotifyItemType::Local,
            SpotifyUri::Unknown { .. } => SpotifyItemType::Unknown,
        }
    }

    /// Gets the name of this URI. The resource name is the component of the URI that identifies
    /// the resource after its type label. If `self` is a named ID, the user will be omitted.
    pub fn to_name(&self) -> Result<String, Error> {
        match &self {
            SpotifyUri::Album { id }
            | SpotifyUri::Artist { id }
            | SpotifyUri::Episode { id }
            | SpotifyUri::Playlist { id, .. }
            | SpotifyUri::Show { id }
            | SpotifyUri::Track { id }
            | SpotifyUri::Unknown { id } => id.to_base62(),
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
    /// Note that this should not be used for playlists, which have the form of
    /// `spotify:playlist:{id}`.
    ///
    /// Spotify URI: https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids
    pub fn from_uri(src: &str) -> SpotifyUriResult {
        // Basic: `spotify:{type}:{id}`
        // Named: `spotify:user:{user}:{type}:{id}`
        // Local: `spotify:local:{artist}:{album_title}:{track_title}:{duration_in_seconds}`
        let mut parts = src.split(':');

        let scheme = parts.next().ok_or(SpotifyUriError::InvalidFormat)?;
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

        if scheme != "spotify" {
            return Err(SpotifyUriError::InvalidRoot.into());
        }

        let item_type = item_type.into();

        match item_type {
            SpotifyItemType::Album => Ok(Self::Album {
                id: SpotifyId::from_base62(name)?,
            }),
            SpotifyItemType::Artist => Ok(Self::Artist {
                id: SpotifyId::from_base62(name)?,
            }),
            SpotifyItemType::Episode => Ok(Self::Episode {
                id: SpotifyId::from_base62(name)?,
            }),
            SpotifyItemType::Playlist => Ok(Self::Playlist {
                id: SpotifyId::from_base62(name)?,
                user: username,
            }),
            SpotifyItemType::Show => Ok(Self::Show {
                id: SpotifyId::from_base62(name)?,
            }),
            SpotifyItemType::Track => Ok(Self::Track {
                id: SpotifyId::from_base62(name)?,
            }),
            SpotifyItemType::Local => Ok(Self::Local {
                artist: "unimplemented".to_owned(),
                album_title: "unimplemented".to_owned(),
                track_title: "unimplemented".to_owned(),
                duration: Default::default(),
            }),
            SpotifyItemType::Unknown => Ok(Self::Unknown {
                id: SpotifyId::from_base62(name)?,
            }),
        }
    }

    /// Parses a base16 (hex) encoded Spotify ID into a `SpotifyUri`.
    ///
    /// `src` is expected to be 32 bytes long and encoded using valid characters.
    ///
    /// This method cannot be used to create local file IDs; passing in `SpotifyItemType::Local`
    /// will always yield an error.
    ///
    /// Spotify URI: https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids
    pub fn from_base16(src: &str, item_type: SpotifyItemType) -> SpotifyUriResult {
        let id = SpotifyId::from_base16(src)?;

        Self::from_id_and_type(id, item_type)
    }

    /// Parses a base62 encoded [Spotify ID] into a `u128`.
    ///
    /// `src` is expected to be 22 bytes long and encoded using valid characters.
    ///
    /// This method cannot be used to create local file IDs; passing in `SpotifyItemType::Local`
    /// will always yield an error.
    ///
    /// Spotify URI: https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids
    pub fn from_base62(src: &str, item_type: SpotifyItemType) -> SpotifyUriResult {
        let id = SpotifyId::from_base62(src)?;

        Self::from_id_and_type(id, item_type)
    }

    /// Creates a `u128` from a copy of `SpotifyId::SIZE` (16) bytes in big-endian order.
    ///
    /// The resulting `SpotifyId` will have a type based on the value of `item_type`.
    ///
    /// This method cannot be used to create local file IDs; passing in `SpotifyItemType::Local`
    /// will always yield an error.
    ///
    /// [Spotify URI]: https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids
    pub fn from_raw(src: &[u8], item_type: SpotifyItemType) -> SpotifyUriResult {
        let id = SpotifyId::from_raw(src)?;

        Self::from_id_and_type(id, item_type)
    }

    fn from_id_and_type(id: SpotifyId, item_type: SpotifyItemType) -> SpotifyUriResult {
        match item_type {
            SpotifyItemType::Album => Ok(Self::Album { id }),
            SpotifyItemType::Artist => Ok(Self::Artist { id }),
            SpotifyItemType::Episode => Ok(Self::Episode { id }),
            SpotifyItemType::Playlist => Ok(Self::Playlist { id, user: None }),
            SpotifyItemType::Show => Ok(Self::Show { id }),
            SpotifyItemType::Track => Ok(Self::Track { id }),
            SpotifyItemType::Unknown => Ok(Self::Unknown { id }),
            SpotifyItemType::Local => Err(SpotifyUriError::InvalidFormat.into()),
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
        let item_type: &str = self.item_type().into();
        let name = self.to_name()?;

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
    /// [SpotifyUri::to_name], which this implementation forwards to.
    #[deprecated(since = "0.8.0", note = "use to_name instead")]
    pub fn to_base62(&self) -> Result<String, Error> {
        self.to_name()
    }
}

impl fmt::Debug for SpotifyUri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("SpotifyUri")
            .field(&self.to_uri().unwrap_or_else(|_| "invalid uri".into()))
            .finish()
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
            id: SpotifyId::try_from(item.revision())?,
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
            id: SpotifyId::try_from(playlist.revision())?,
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
            id: SpotifyId::from_base62(picture.uri())?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ConversionCase {
        parsed: SpotifyUri,
        kind: SpotifyItemType,
        uri: &'static str,
        base16: &'static str,
        base62: &'static str,
        raw: &'static [u8],
    }

    static CONV_VALID: [ConversionCase; 4] = [
        ConversionCase {
            parsed: SpotifyUri::Track {
                id: SpotifyId {
                    id: 238762092608182713602505436543891614649,
                },
            },
            kind: SpotifyItemType::Track,
            uri: "spotify:track:5sWHDYs0csV6RS48xBl0tH",
            base16: "b39fe8081e1f4c54be38e8d6f9f12bb9",
            base62: "5sWHDYs0csV6RS48xBl0tH",
            raw: &[
                179, 159, 232, 8, 30, 31, 76, 84, 190, 56, 232, 214, 249, 241, 43, 185,
            ],
        },
        ConversionCase {
            parsed: SpotifyUri::Track {
                id: SpotifyId {
                    id: 204841891221366092811751085145916697048,
                },
            },
            kind: SpotifyItemType::Track,
            uri: "spotify:track:4GNcXTGWmnZ3ySrqvol3o4",
            base16: "9a1b1cfbc6f244569ae0356c77bbe9d8",
            base62: "4GNcXTGWmnZ3ySrqvol3o4",
            raw: &[
                154, 27, 28, 251, 198, 242, 68, 86, 154, 224, 53, 108, 119, 187, 233, 216,
            ],
        },
        ConversionCase {
            parsed: SpotifyUri::Episode {
                id: SpotifyId {
                    id: 204841891221366092811751085145916697048,
                },
            },
            kind: SpotifyItemType::Episode,
            uri: "spotify:episode:4GNcXTGWmnZ3ySrqvol3o4",
            base16: "9a1b1cfbc6f244569ae0356c77bbe9d8",
            base62: "4GNcXTGWmnZ3ySrqvol3o4",
            raw: &[
                154, 27, 28, 251, 198, 242, 68, 86, 154, 224, 53, 108, 119, 187, 233, 216,
            ],
        },
        ConversionCase {
            parsed: SpotifyUri::Show {
                id: SpotifyId {
                    id: 204841891221366092811751085145916697048,
                },
            },
            kind: SpotifyItemType::Show,
            uri: "spotify:show:4GNcXTGWmnZ3ySrqvol3o4",
            base16: "9a1b1cfbc6f244569ae0356c77bbe9d8",
            base62: "4GNcXTGWmnZ3ySrqvol3o4",
            raw: &[
                154, 27, 28, 251, 198, 242, 68, 86, 154, 224, 53, 108, 119, 187, 233, 216,
            ],
        },
    ];

    static CONV_INVALID: [ConversionCase; 5] = [
        ConversionCase {
            parsed: SpotifyUri::Unknown {
                id: SpotifyId { id: 0 },
            },
            kind: SpotifyItemType::Unknown,
            // Invalid ID in the URI.
            uri: "spotify:arbitrarywhatever:5sWHDYs0Bl0tH",
            base16: "ZZZZZ8081e1f4c54be38e8d6f9f12bb9",
            base62: "!!!!!Ys0csV6RS48xBl0tH",
            raw: &[
                // Invalid length.
                154, 27, 28, 251, 198, 242, 68, 86, 154, 224, 5, 3, 108, 119, 187, 233, 216, 255,
            ],
        },
        ConversionCase {
            parsed: SpotifyUri::Unknown {
                id: SpotifyId { id: 0 },
            },
            kind: SpotifyItemType::Unknown,
            // Missing colon between ID and type.
            uri: "spotify:arbitrarywhatever5sWHDYs0csV6RS48xBl0tH",
            base16: "--------------------",
            base62: "....................",
            raw: &[
                // Invalid length.
                154, 27, 28, 251,
            ],
        },
        ConversionCase {
            parsed: SpotifyUri::Unknown {
                id: SpotifyId { id: 0 },
            },
            kind: SpotifyItemType::Unknown,
            // Uri too short
            uri: "spotify:azb:aRS48xBl0tH",
            // too long, should return error but not panic overflow
            base16: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            // too long, should return error but not panic overflow
            base62: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            raw: &[
                // Invalid length.
                154, 27, 28, 251,
            ],
        },
        ConversionCase {
            parsed: SpotifyUri::Unknown {
                id: SpotifyId { id: 0 },
            },
            kind: SpotifyItemType::Unknown,
            // Uri too short
            uri: "spotify:azb:aRS48xBl0tH",
            base16: "--------------------",
            // too short to encode a 128 bits int
            base62: "aa",
            raw: &[
                // Invalid length.
                154, 27, 28, 251,
            ],
        },
        ConversionCase {
            parsed: SpotifyUri::Unknown {
                id: SpotifyId { id: 0 },
            },
            kind: SpotifyItemType::Unknown,
            uri: "cleary invalid uri",
            base16: "--------------------",
            // too high of a value, this would need a 132 bits int
            base62: "ZZZZZZZZZZZZZZZZZZZZZZ",
            raw: &[
                // Invalid length.
                154, 27, 28, 251,
            ],
        },
    ];

    #[test]
    fn from_base62() {
        for c in &CONV_VALID {
            assert_eq!(SpotifyUri::from_base62(c.base62, c.kind).unwrap(), c.parsed,);
        }

        for c in &CONV_INVALID {
            assert!(SpotifyId::from_base62(c.base62).is_err(),);
        }
    }

    #[test]
    fn to_name() {
        for c in &CONV_VALID {
            assert_eq!(c.parsed.to_name().unwrap(), c.base62);
        }
    }

    #[test]
    fn from_base16() {
        for c in &CONV_VALID {
            assert_eq!(SpotifyUri::from_base16(c.base16, c.kind).unwrap(), c.parsed);
        }

        for c in &CONV_INVALID {
            assert!(SpotifyId::from_base16(c.base16).is_err(),);
        }
    }

    #[test]
    fn from_uri() {
        for c in &CONV_VALID {
            let actual = SpotifyUri::from_uri(c.uri).unwrap();

            assert_eq!(actual, c.parsed);
            assert_eq!(actual.item_type(), c.kind);
        }

        for c in &CONV_INVALID {
            assert!(SpotifyUri::from_uri(c.uri).is_err());
        }
    }

    #[test]
    fn from_local_uri() {
        let actual = SpotifyUri::from_uri("spotify:local:xyz:123").unwrap();

        assert_eq!(
            actual,
            SpotifyUri::Local {
                artist: "unimplemented".to_owned(),
                album_title: "unimplemented".to_owned(),
                track_title: "unimplemented".to_owned(),
                duration: Default::default(),
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

        assert_eq!(actual.item_type(), SpotifyItemType::Playlist);
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
    fn from_raw() {
        for c in &CONV_VALID {
            assert_eq!(SpotifyUri::from_raw(c.raw, c.kind).unwrap(), c.parsed);
        }

        for c in &CONV_INVALID {
            assert!(SpotifyUri::from_raw(c.raw, c.kind).is_err());
        }
    }
}
