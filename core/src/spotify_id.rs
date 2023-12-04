use std::{
    convert::{TryFrom, TryInto},
    fmt,
    ops::Deref,
};

use thiserror::Error;

use crate::Error;

use librespot_protocol as protocol;

// re-export FileId for historic reasons, when it was part of this mod
pub use crate::FileId;

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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpotifyId {
    pub id: u128,
    pub item_type: SpotifyItemType,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum SpotifyIdError {
    #[error("ID cannot be parsed")]
    InvalidId,
    #[error("not a valid Spotify URI")]
    InvalidFormat,
    #[error("URI does not belong to Spotify")]
    InvalidRoot,
}

impl From<SpotifyIdError> for Error {
    fn from(err: SpotifyIdError) -> Self {
        Error::invalid_argument(err)
    }
}

pub type SpotifyIdResult = Result<SpotifyId, Error>;
pub type NamedSpotifyIdResult = Result<NamedSpotifyId, Error>;

const BASE62_DIGITS: &[u8; 62] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const BASE16_DIGITS: &[u8; 16] = b"0123456789abcdef";

impl SpotifyId {
    const SIZE: usize = 16;
    const SIZE_BASE16: usize = 32;
    const SIZE_BASE62: usize = 22;

    /// Returns whether this `SpotifyId` is for a playable audio item, if known.
    pub fn is_playable(&self) -> bool {
        matches!(
            self.item_type,
            SpotifyItemType::Episode | SpotifyItemType::Track
        )
    }

    /// Parses a base16 (hex) encoded [Spotify ID] into a `SpotifyId`.
    ///
    /// `src` is expected to be 32 bytes long and encoded using valid characters.
    ///
    /// [Spotify ID]: https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids
    pub fn from_base16(src: &str) -> SpotifyIdResult {
        let mut dst: u128 = 0;

        for c in src.as_bytes() {
            let p = match c {
                b'0'..=b'9' => c - b'0',
                b'a'..=b'f' => c - b'a' + 10,
                _ => return Err(SpotifyIdError::InvalidId.into()),
            } as u128;

            dst <<= 4;
            dst += p;
        }

        Ok(Self {
            id: dst,
            item_type: SpotifyItemType::Unknown,
        })
    }

    /// Parses a base62 encoded [Spotify ID] into a `u128`.
    ///
    /// `src` is expected to be 22 bytes long and encoded using valid characters.
    ///
    /// [Spotify ID]: https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids
    pub fn from_base62(src: &str) -> SpotifyIdResult {
        let mut dst: u128 = 0;

        for c in src.as_bytes() {
            let p = match c {
                b'0'..=b'9' => c - b'0',
                b'a'..=b'z' => c - b'a' + 10,
                b'A'..=b'Z' => c - b'A' + 36,
                _ => return Err(SpotifyIdError::InvalidId.into()),
            } as u128;

            dst *= 62;
            dst += p;
        }

        Ok(Self {
            id: dst,
            item_type: SpotifyItemType::Unknown,
        })
    }

    /// Creates a `u128` from a copy of `SpotifyId::SIZE` (16) bytes in big-endian order.
    ///
    /// The resulting `SpotifyId` will default to a `SpotifyItemType::Unknown`.
    pub fn from_raw(src: &[u8]) -> SpotifyIdResult {
        match src.try_into() {
            Ok(dst) => Ok(Self {
                id: u128::from_be_bytes(dst),
                item_type: SpotifyItemType::Unknown,
            }),
            Err(_) => Err(SpotifyIdError::InvalidId.into()),
        }
    }

    /// Parses a [Spotify URI] into a `SpotifyId`.
    ///
    /// `uri` is expected to be in the canonical form `spotify:{type}:{id}`, where `{type}`
    /// can be arbitrary while `{id}` is a 22-character long, base62 encoded Spotify ID.
    ///
    /// Note that this should not be used for playlists, which have the form of
    /// `spotify:playlist:{id}`.
    ///
    /// [Spotify URI]: https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids
    pub fn from_uri(src: &str) -> SpotifyIdResult {
        // Basic: `spotify:{type}:{id}`
        // Named: `spotify:user:{user}:{type}:{id}`
        // Local: `spotify:local:{artist}:{album_title}:{track_title}:{duration_in_seconds}`
        let mut parts = src.split(':');

        let scheme = parts.next().ok_or(SpotifyIdError::InvalidFormat)?;

        let item_type = {
            let next = parts.next().ok_or(SpotifyIdError::InvalidFormat)?;
            if next == "user" {
                let _username = parts.next().ok_or(SpotifyIdError::InvalidFormat)?;
                parts.next().ok_or(SpotifyIdError::InvalidFormat)?
            } else {
                next
            }
        };

        let id = parts.next().ok_or(SpotifyIdError::InvalidFormat)?;

        if scheme != "spotify" {
            return Err(SpotifyIdError::InvalidRoot.into());
        }

        let item_type = item_type.into();

        // Local files have a variable-length ID: https://developer.spotify.com/documentation/general/guides/local-files-spotify-playlists/
        // TODO: find a way to add this local file ID to SpotifyId.
        // One possible solution would be to copy the contents of `id` to a new String field in SpotifyId,
        // but then we would need to remove the derived Copy trait, which would be a breaking change.
        if item_type == SpotifyItemType::Local {
            return Ok(Self { item_type, id: 0 });
        }

        if id.len() != Self::SIZE_BASE62 {
            return Err(SpotifyIdError::InvalidId.into());
        }

        Ok(Self {
            item_type,
            ..Self::from_base62(id)?
        })
    }

    /// Returns the `SpotifyId` as a base16 (hex) encoded, `SpotifyId::SIZE_BASE16` (32)
    /// character long `String`.
    #[allow(clippy::wrong_self_convention)]
    pub fn to_base16(&self) -> Result<String, Error> {
        to_base16(&self.to_raw(), &mut [0u8; Self::SIZE_BASE16])
    }

    /// Returns the `SpotifyId` as a [canonically] base62 encoded, `SpotifyId::SIZE_BASE62` (22)
    /// character long `String`.
    ///
    /// [canonically]: https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids
    #[allow(clippy::wrong_self_convention)]
    pub fn to_base62(&self) -> Result<String, Error> {
        let mut dst = [0u8; 22];
        let mut i = 0;
        let n = self.id;

        // The algorithm is based on:
        // https://github.com/trezor/trezor-crypto/blob/c316e775a2152db255ace96b6b65ac0f20525ec0/base58.c
        //
        // We are not using naive division of self.id as it is an u128 and div + mod are software
        // emulated at runtime (and unoptimized into mul + shift) on non-128bit platforms,
        // making them very expensive.
        //
        // Trezor's algorithm allows us to stick to arithmetic on native registers making this
        // an order of magnitude faster. Additionally, as our sizes are known, instead of
        // dealing with the ID on a byte by byte basis, we decompose it into four u32s and
        // use 64-bit arithmetic on them for an additional speedup.
        for shift in &[96, 64, 32, 0] {
            let mut carry = (n >> shift) as u32 as u64;

            for b in &mut dst[..i] {
                carry += (*b as u64) << 32;
                *b = (carry % 62) as u8;
                carry /= 62;
            }

            while carry > 0 {
                dst[i] = (carry % 62) as u8;
                carry /= 62;
                i += 1;
            }
        }

        for b in &mut dst {
            *b = BASE62_DIGITS[*b as usize];
        }

        dst.reverse();

        String::from_utf8(dst.to_vec()).map_err(|_| SpotifyIdError::InvalidId.into())
    }

    /// Returns a copy of the `SpotifyId` as an array of `SpotifyId::SIZE` (16) bytes in
    /// big-endian order.
    #[allow(clippy::wrong_self_convention)]
    pub fn to_raw(&self) -> [u8; Self::SIZE] {
        self.id.to_be_bytes()
    }

    /// Returns the `SpotifyId` as a [Spotify URI] in the canonical form `spotify:{type}:{id}`,
    /// where `{type}` is an arbitrary string and `{id}` is a 22-character long, base62 encoded
    /// Spotify ID.
    ///
    /// If the `SpotifyId` has an associated type unrecognized by the library, `{type}` will
    /// be encoded as `unknown`.
    ///
    /// [Spotify URI]: https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids
    #[allow(clippy::wrong_self_convention)]
    pub fn to_uri(&self) -> Result<String, Error> {
        // 8 chars for the "spotify:" prefix + 1 colon + 22 chars base62 encoded ID  = 31
        // + unknown size item_type.
        let item_type: &str = self.item_type.into();
        let mut dst = String::with_capacity(31 + item_type.len());
        dst.push_str("spotify:");
        dst.push_str(item_type);
        dst.push(':');
        let base_62 = self.to_base62()?;
        dst.push_str(&base_62);

        Ok(dst)
    }
}

impl fmt::Debug for SpotifyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("SpotifyId")
            .field(&self.to_uri().unwrap_or_else(|_| "invalid uri".into()))
            .finish()
    }
}

impl fmt::Display for SpotifyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_uri().unwrap_or_else(|_| "invalid uri".into()))
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct NamedSpotifyId {
    pub inner_id: SpotifyId,
    pub username: String,
}

impl NamedSpotifyId {
    pub fn from_uri(src: &str) -> NamedSpotifyIdResult {
        let uri_parts: Vec<&str> = src.split(':').collect();

        // At minimum, should be `spotify:user:{username}:{type}:{id}`
        if uri_parts.len() < 5 {
            return Err(SpotifyIdError::InvalidFormat.into());
        }

        if uri_parts[0] != "spotify" {
            return Err(SpotifyIdError::InvalidRoot.into());
        }

        if uri_parts[1] != "user" {
            return Err(SpotifyIdError::InvalidFormat.into());
        }

        Ok(Self {
            inner_id: SpotifyId::from_uri(src)?,
            username: uri_parts[2].to_owned(),
        })
    }

    pub fn to_uri(&self) -> Result<String, Error> {
        let item_type: &str = self.inner_id.item_type.into();
        let mut dst = String::with_capacity(37 + self.username.len() + item_type.len());
        dst.push_str("spotify:user:");
        dst.push_str(&self.username);
        dst.push(':');
        dst.push_str(item_type);
        dst.push(':');
        let base_62 = self.to_base62()?;
        dst.push_str(&base_62);

        Ok(dst)
    }

    pub fn from_spotify_id(id: SpotifyId, username: &str) -> Self {
        Self {
            inner_id: id,
            username: username.to_owned(),
        }
    }
}

impl Deref for NamedSpotifyId {
    type Target = SpotifyId;
    fn deref(&self) -> &Self::Target {
        &self.inner_id
    }
}

impl fmt::Debug for NamedSpotifyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("NamedSpotifyId")
            .field(
                &self
                    .inner_id
                    .to_uri()
                    .unwrap_or_else(|_| "invalid id".into()),
            )
            .finish()
    }
}

impl fmt::Display for NamedSpotifyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            &self
                .inner_id
                .to_uri()
                .unwrap_or_else(|_| "invalid id".into()),
        )
    }
}

impl TryFrom<&[u8]> for SpotifyId {
    type Error = crate::Error;
    fn try_from(src: &[u8]) -> Result<Self, Self::Error> {
        Self::from_raw(src)
    }
}

impl TryFrom<&str> for SpotifyId {
    type Error = crate::Error;
    fn try_from(src: &str) -> Result<Self, Self::Error> {
        Self::from_base62(src)
    }
}

impl TryFrom<String> for SpotifyId {
    type Error = crate::Error;
    fn try_from(src: String) -> Result<Self, Self::Error> {
        Self::try_from(src.as_str())
    }
}

impl TryFrom<&Vec<u8>> for SpotifyId {
    type Error = crate::Error;
    fn try_from(src: &Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(src.as_slice())
    }
}

impl TryFrom<&protocol::spirc::TrackRef> for SpotifyId {
    type Error = crate::Error;
    fn try_from(track: &protocol::spirc::TrackRef) -> Result<Self, Self::Error> {
        match SpotifyId::from_raw(track.gid()) {
            Ok(mut id) => {
                id.item_type = SpotifyItemType::Track;
                Ok(id)
            }
            Err(_) => SpotifyId::from_uri(track.uri()),
        }
    }
}

impl TryFrom<&protocol::metadata::Album> for SpotifyId {
    type Error = crate::Error;
    fn try_from(album: &protocol::metadata::Album) -> Result<Self, Self::Error> {
        Ok(Self {
            item_type: SpotifyItemType::Album,
            ..Self::from_raw(album.gid())?
        })
    }
}

impl TryFrom<&protocol::metadata::Artist> for SpotifyId {
    type Error = crate::Error;
    fn try_from(artist: &protocol::metadata::Artist) -> Result<Self, Self::Error> {
        Ok(Self {
            item_type: SpotifyItemType::Artist,
            ..Self::from_raw(artist.gid())?
        })
    }
}

impl TryFrom<&protocol::metadata::Episode> for SpotifyId {
    type Error = crate::Error;
    fn try_from(episode: &protocol::metadata::Episode) -> Result<Self, Self::Error> {
        Ok(Self {
            item_type: SpotifyItemType::Episode,
            ..Self::from_raw(episode.gid())?
        })
    }
}

impl TryFrom<&protocol::metadata::Track> for SpotifyId {
    type Error = crate::Error;
    fn try_from(track: &protocol::metadata::Track) -> Result<Self, Self::Error> {
        Ok(Self {
            item_type: SpotifyItemType::Track,
            ..Self::from_raw(track.gid())?
        })
    }
}

impl TryFrom<&protocol::metadata::Show> for SpotifyId {
    type Error = crate::Error;
    fn try_from(show: &protocol::metadata::Show) -> Result<Self, Self::Error> {
        Ok(Self {
            item_type: SpotifyItemType::Show,
            ..Self::from_raw(show.gid())?
        })
    }
}

impl TryFrom<&protocol::metadata::ArtistWithRole> for SpotifyId {
    type Error = crate::Error;
    fn try_from(artist: &protocol::metadata::ArtistWithRole) -> Result<Self, Self::Error> {
        Ok(Self {
            item_type: SpotifyItemType::Artist,
            ..Self::from_raw(artist.artist_gid())?
        })
    }
}

impl TryFrom<&protocol::playlist4_external::Item> for SpotifyId {
    type Error = crate::Error;
    fn try_from(item: &protocol::playlist4_external::Item) -> Result<Self, Self::Error> {
        Ok(Self {
            item_type: SpotifyItemType::Track,
            ..Self::from_uri(item.uri())?
        })
    }
}

// Note that this is the unique revision of an item's metadata on a playlist,
// not the ID of that item or playlist.
impl TryFrom<&protocol::playlist4_external::MetaItem> for SpotifyId {
    type Error = crate::Error;
    fn try_from(item: &protocol::playlist4_external::MetaItem) -> Result<Self, Self::Error> {
        Self::try_from(item.revision())
    }
}

// Note that this is the unique revision of a playlist, not the ID of that playlist.
impl TryFrom<&protocol::playlist4_external::SelectedListContent> for SpotifyId {
    type Error = crate::Error;
    fn try_from(
        playlist: &protocol::playlist4_external::SelectedListContent,
    ) -> Result<Self, Self::Error> {
        Self::try_from(playlist.revision())
    }
}

// TODO: check meaning and format of this field in the wild. This might be a FileId,
// which is why we now don't create a separate `Playlist` enum value yet and choose
// to discard any item type.
impl TryFrom<&protocol::playlist_annotate3::TranscodedPicture> for SpotifyId {
    type Error = crate::Error;
    fn try_from(
        picture: &protocol::playlist_annotate3::TranscodedPicture,
    ) -> Result<Self, Self::Error> {
        Self::from_base62(picture.uri())
    }
}

pub fn to_base16(src: &[u8], buf: &mut [u8]) -> Result<String, Error> {
    let mut i = 0;
    for v in src {
        buf[i] = BASE16_DIGITS[(v >> 4) as usize];
        buf[i + 1] = BASE16_DIGITS[(v & 0x0f) as usize];
        i += 2;
    }

    String::from_utf8(buf.to_vec()).map_err(|_| SpotifyIdError::InvalidId.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ConversionCase {
        id: u128,
        kind: SpotifyItemType,
        uri: &'static str,
        base16: &'static str,
        base62: &'static str,
        raw: &'static [u8],
    }

    static CONV_VALID: [ConversionCase; 5] = [
        ConversionCase {
            id: 238762092608182713602505436543891614649,
            kind: SpotifyItemType::Track,
            uri: "spotify:track:5sWHDYs0csV6RS48xBl0tH",
            base16: "b39fe8081e1f4c54be38e8d6f9f12bb9",
            base62: "5sWHDYs0csV6RS48xBl0tH",
            raw: &[
                179, 159, 232, 8, 30, 31, 76, 84, 190, 56, 232, 214, 249, 241, 43, 185,
            ],
        },
        ConversionCase {
            id: 204841891221366092811751085145916697048,
            kind: SpotifyItemType::Track,
            uri: "spotify:track:4GNcXTGWmnZ3ySrqvol3o4",
            base16: "9a1b1cfbc6f244569ae0356c77bbe9d8",
            base62: "4GNcXTGWmnZ3ySrqvol3o4",
            raw: &[
                154, 27, 28, 251, 198, 242, 68, 86, 154, 224, 53, 108, 119, 187, 233, 216,
            ],
        },
        ConversionCase {
            id: 204841891221366092811751085145916697048,
            kind: SpotifyItemType::Episode,
            uri: "spotify:episode:4GNcXTGWmnZ3ySrqvol3o4",
            base16: "9a1b1cfbc6f244569ae0356c77bbe9d8",
            base62: "4GNcXTGWmnZ3ySrqvol3o4",
            raw: &[
                154, 27, 28, 251, 198, 242, 68, 86, 154, 224, 53, 108, 119, 187, 233, 216,
            ],
        },
        ConversionCase {
            id: 204841891221366092811751085145916697048,
            kind: SpotifyItemType::Show,
            uri: "spotify:show:4GNcXTGWmnZ3ySrqvol3o4",
            base16: "9a1b1cfbc6f244569ae0356c77bbe9d8",
            base62: "4GNcXTGWmnZ3ySrqvol3o4",
            raw: &[
                154, 27, 28, 251, 198, 242, 68, 86, 154, 224, 53, 108, 119, 187, 233, 216,
            ],
        },
        ConversionCase {
            id: 0,
            kind: SpotifyItemType::Local,
            uri: "spotify:local:0000000000000000000000",
            base16: "00000000000000000000000000000000",
            base62: "0000000000000000000000",
            raw: &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        },
    ];

    static CONV_INVALID: [ConversionCase; 3] = [
        ConversionCase {
            id: 0,
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
            id: 0,
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
            id: 0,
            kind: SpotifyItemType::Unknown,
            // Uri too short
            uri: "spotify:azb:aRS48xBl0tH",
            base16: "--------------------",
            base62: "....................",
            raw: &[
                // Invalid length.
                154, 27, 28, 251,
            ],
        },
    ];

    #[test]
    fn from_base62() {
        for c in &CONV_VALID {
            assert_eq!(SpotifyId::from_base62(c.base62).unwrap().id, c.id);
        }

        for c in &CONV_INVALID {
            assert!(SpotifyId::from_base62(c.base62).is_err(),);
        }
    }

    #[test]
    fn to_base62() {
        for c in &CONV_VALID {
            let id = SpotifyId {
                id: c.id,
                item_type: c.kind,
            };

            assert_eq!(id.to_base62().unwrap(), c.base62);
        }
    }

    #[test]
    fn from_base16() {
        for c in &CONV_VALID {
            assert_eq!(SpotifyId::from_base16(c.base16).unwrap().id, c.id);
        }

        for c in &CONV_INVALID {
            assert!(SpotifyId::from_base16(c.base16).is_err(),);
        }
    }

    #[test]
    fn to_base16() {
        for c in &CONV_VALID {
            let id = SpotifyId {
                id: c.id,
                item_type: c.kind,
            };

            assert_eq!(id.to_base16().unwrap(), c.base16);
        }
    }

    #[test]
    fn from_uri() {
        for c in &CONV_VALID {
            let actual = SpotifyId::from_uri(c.uri).unwrap();

            assert_eq!(actual.id, c.id);
            assert_eq!(actual.item_type, c.kind);
        }

        for c in &CONV_INVALID {
            assert!(SpotifyId::from_uri(c.uri).is_err());
        }
    }

    #[test]
    fn from_local_uri() {
        let actual = SpotifyId::from_uri("spotify:local:xyz:123").unwrap();

        assert_eq!(actual.id, 0);
        assert_eq!(actual.item_type, SpotifyItemType::Local);
    }

    #[test]
    fn from_named_uri() {
        let actual =
            NamedSpotifyId::from_uri("spotify:user:spotify:playlist:37i9dQZF1DWSw8liJZcPOI")
                .unwrap();

        assert_eq!(actual.id, 136159921382084734723401526672209703396);
        assert_eq!(actual.item_type, SpotifyItemType::Playlist);
        assert_eq!(actual.username, "spotify");
    }

    #[test]
    fn to_uri() {
        for c in &CONV_VALID {
            let id = SpotifyId {
                id: c.id,
                item_type: c.kind,
            };

            assert_eq!(id.to_uri().unwrap(), c.uri);
        }
    }

    #[test]
    fn from_raw() {
        for c in &CONV_VALID {
            assert_eq!(SpotifyId::from_raw(c.raw).unwrap().id, c.id);
        }

        for c in &CONV_INVALID {
            assert!(SpotifyId::from_raw(c.raw).is_err());
        }
    }
}
