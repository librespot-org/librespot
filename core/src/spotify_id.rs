use std::convert::TryInto;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpotifyAudioType {
    Track,
    Podcast,
    NonPlayable,
}

impl From<&str> for SpotifyAudioType {
    fn from(v: &str) -> Self {
        match v {
            "track" => SpotifyAudioType::Track,
            "episode" => SpotifyAudioType::Podcast,
            _ => SpotifyAudioType::NonPlayable,
        }
    }
}

impl Into<&str> for SpotifyAudioType {
    fn into(self) -> &'static str {
        match self {
            SpotifyAudioType::Track => "track",
            SpotifyAudioType::Podcast => "episode",
            SpotifyAudioType::NonPlayable => "unknown",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpotifyId {
    pub id: u128,
    pub audio_type: SpotifyAudioType,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SpotifyIdError;

const BASE62_DIGITS: &[u8; 62] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const BASE16_DIGITS: &[u8; 16] = b"0123456789abcdef";

impl SpotifyId {
    const SIZE: usize = 16;
    const SIZE_BASE16: usize = 32;
    const SIZE_BASE62: usize = 22;

    fn as_track(n: u128) -> SpotifyId {
        SpotifyId {
            id: n,
            audio_type: SpotifyAudioType::Track,
        }
    }

    /// Parses a base16 (hex) encoded [Spotify ID] into a `SpotifyId`.
    ///
    /// `src` is expected to be 32 bytes long and encoded using valid characters.
    ///
    /// [Spotify ID]: https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids
    pub fn from_base16(src: &str) -> Result<SpotifyId, SpotifyIdError> {
        let mut dst: u128 = 0;

        for c in src.as_bytes() {
            let p = match c {
                b'0'..=b'9' => c - b'0',
                b'a'..=b'f' => c - b'a' + 10,
                _ => return Err(SpotifyIdError),
            } as u128;

            dst <<= 4;
            dst += p;
        }

        Ok(SpotifyId::as_track(dst))
    }

    /// Parses a base62 encoded [Spotify ID] into a `SpotifyId`.
    ///
    /// `src` is expected to be 22 bytes long and encoded using valid characters.
    ///
    /// [Spotify ID]: https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids
    pub fn from_base62(src: &str) -> Result<SpotifyId, SpotifyIdError> {
        let mut dst: u128 = 0;

        for c in src.as_bytes() {
            let p = match c {
                b'0'..=b'9' => c - b'0',
                b'a'..=b'z' => c - b'a' + 10,
                b'A'..=b'Z' => c - b'A' + 36,
                _ => return Err(SpotifyIdError),
            } as u128;

            dst *= 62;
            dst += p;
        }

        Ok(SpotifyId::as_track(dst))
    }

    /// Creates a `SpotifyId` from a copy of `SpotifyId::SIZE` (16) bytes in big-endian order.
    ///
    /// The resulting `SpotifyId` will default to a `SpotifyAudioType::TRACK`.
    pub fn from_raw(src: &[u8]) -> Result<SpotifyId, SpotifyIdError> {
        match src.try_into() {
            Ok(dst) => Ok(SpotifyId::as_track(u128::from_be_bytes(dst))),
            Err(_) => Err(SpotifyIdError),
        }
    }

    /// Parses a [Spotify URI] into a `SpotifyId`.
    ///
    /// `uri` is expected to be in the canonical form `spotify:{type}:{id}`, where `{type}`
    /// can be arbitrary while `{id}` is a 22-character long, base62 encoded Spotify ID.
    ///
    /// [Spotify URI]: https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids
    pub fn from_uri(src: &str) -> Result<SpotifyId, SpotifyIdError> {
        // We expect the ID to be the last colon-delimited item in the URI.
        let b = src.as_bytes();
        let id_i = b.len() - SpotifyId::SIZE_BASE62;
        if b[id_i - 1] != b':' {
            return Err(SpotifyIdError);
        }

        let mut id = SpotifyId::from_base62(&src[id_i..])?;

        // Slice offset by 8 as we are skipping the "spotify:" prefix.
        id.audio_type = src[8..id_i - 1].into();

        Ok(id)
    }

    /// Returns the `SpotifyId` as a base16 (hex) encoded, `SpotifyId::SIZE_BASE62` (22)
    /// character long `String`.
    pub fn to_base16(&self) -> String {
        to_base16(&self.to_raw(), &mut [0u8; SpotifyId::SIZE_BASE16])
    }

    /// Returns the `SpotifyId` as a [canonically] base62 encoded, `SpotifyId::SIZE_BASE62` (22)
    /// character long `String`.
    ///
    /// [canonically]: https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids
    pub fn to_base62(&self) -> String {
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

        unsafe {
            // Safety: We are only dealing with ASCII characters.
            String::from_utf8_unchecked(dst.to_vec())
        }
    }

    /// Returns a copy of the `SpotifyId` as an array of `SpotifyId::SIZE` (16) bytes in
    /// big-endian order.
    pub fn to_raw(&self) -> [u8; SpotifyId::SIZE] {
        self.id.to_be_bytes()
    }

    /// Returns the `SpotifyId` as a [Spotify URI] in the canonical form `spotify:{type}:{id}`,
    /// where `{type}` is an arbitrary string and `{id}` is a 22-character long, base62 encoded
    /// Spotify ID.
    ///
    /// If the `SpotifyId` has an associated type unrecognized by the library, `{type}` will
    /// be encoded as `unknown`.
    ///
    /// [Spotify URI]: https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids
    pub fn to_uri(&self) -> String {
        // 8 chars for the "spotify:" prefix + 1 colon + 22 chars base62 encoded ID  = 31
        // + unknown size audio_type.
        let audio_type: &str = self.audio_type.into();
        let mut dst = String::with_capacity(31 + audio_type.len());
        dst.push_str("spotify:");
        dst.push_str(audio_type);
        dst.push(':');
        dst.push_str(&self.to_base62());

        dst
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileId(pub [u8; 20]);

impl FileId {
    pub fn to_base16(&self) -> String {
        to_base16(&self.0, &mut [0u8; 40])
    }
}

impl fmt::Debug for FileId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("FileId").field(&self.to_base16()).finish()
    }
}

impl fmt::Display for FileId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_base16())
    }
}

#[inline]
fn to_base16(src: &[u8], buf: &mut [u8]) -> String {
    let mut i = 0;
    for v in src {
        buf[i] = BASE16_DIGITS[(v >> 4) as usize];
        buf[i + 1] = BASE16_DIGITS[(v & 0x0f) as usize];
        i += 2;
    }

    unsafe {
        // Safety: We are only dealing with ASCII characters.
        String::from_utf8_unchecked(buf.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ConversionCase {
        id: u128,
        kind: SpotifyAudioType,
        uri: &'static str,
        base16: &'static str,
        base62: &'static str,
        raw: &'static [u8],
    }

    static CONV_VALID: [ConversionCase; 4] = [
        ConversionCase {
            id: 238762092608182713602505436543891614649,
            kind: SpotifyAudioType::Track,
            uri: "spotify:track:5sWHDYs0csV6RS48xBl0tH",
            base16: "b39fe8081e1f4c54be38e8d6f9f12bb9",
            base62: "5sWHDYs0csV6RS48xBl0tH",
            raw: &[
                179, 159, 232, 8, 30, 31, 76, 84, 190, 56, 232, 214, 249, 241, 43, 185,
            ],
        },
        ConversionCase {
            id: 204841891221366092811751085145916697048,
            kind: SpotifyAudioType::Track,
            uri: "spotify:track:4GNcXTGWmnZ3ySrqvol3o4",
            base16: "9a1b1cfbc6f244569ae0356c77bbe9d8",
            base62: "4GNcXTGWmnZ3ySrqvol3o4",
            raw: &[
                154, 27, 28, 251, 198, 242, 68, 86, 154, 224, 53, 108, 119, 187, 233, 216,
            ],
        },
        ConversionCase {
            id: 204841891221366092811751085145916697048,
            kind: SpotifyAudioType::Podcast,
            uri: "spotify:episode:4GNcXTGWmnZ3ySrqvol3o4",
            base16: "9a1b1cfbc6f244569ae0356c77bbe9d8",
            base62: "4GNcXTGWmnZ3ySrqvol3o4",
            raw: &[
                154, 27, 28, 251, 198, 242, 68, 86, 154, 224, 53, 108, 119, 187, 233, 216,
            ],
        },
        ConversionCase {
            id: 204841891221366092811751085145916697048,
            kind: SpotifyAudioType::NonPlayable,
            uri: "spotify:unknown:4GNcXTGWmnZ3ySrqvol3o4",
            base16: "9a1b1cfbc6f244569ae0356c77bbe9d8",
            base62: "4GNcXTGWmnZ3ySrqvol3o4",
            raw: &[
                154, 27, 28, 251, 198, 242, 68, 86, 154, 224, 53, 108, 119, 187, 233, 216,
            ],
        },
    ];

    static CONV_INVALID: [ConversionCase; 2] = [
        ConversionCase {
            id: 0,
            kind: SpotifyAudioType::NonPlayable,
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
            kind: SpotifyAudioType::NonPlayable,
            // Missing colon between ID and type.
            uri: "spotify:arbitrarywhatever5sWHDYs0csV6RS48xBl0tH",
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
            assert_eq!(SpotifyId::from_base62(c.base62), Err(SpotifyIdError));
        }
    }

    #[test]
    fn to_base62() {
        for c in &CONV_VALID {
            let id = SpotifyId {
                id: c.id,
                audio_type: c.kind,
            };

            assert_eq!(id.to_base62(), c.base62);
        }
    }

    #[test]
    fn from_base16() {
        for c in &CONV_VALID {
            assert_eq!(SpotifyId::from_base16(c.base16).unwrap().id, c.id);
        }

        for c in &CONV_INVALID {
            assert_eq!(SpotifyId::from_base16(c.base16), Err(SpotifyIdError));
        }
    }

    #[test]
    fn to_base16() {
        for c in &CONV_VALID {
            let id = SpotifyId {
                id: c.id,
                audio_type: c.kind,
            };

            assert_eq!(id.to_base16(), c.base16);
        }
    }

    #[test]
    fn from_uri() {
        for c in &CONV_VALID {
            let actual = SpotifyId::from_uri(c.uri).unwrap();

            assert_eq!(actual.id, c.id);
            assert_eq!(actual.audio_type, c.kind);
        }

        for c in &CONV_INVALID {
            assert_eq!(SpotifyId::from_uri(c.uri), Err(SpotifyIdError));
        }
    }

    #[test]
    fn to_uri() {
        for c in &CONV_VALID {
            let id = SpotifyId {
                id: c.id,
                audio_type: c.kind,
            };

            assert_eq!(id.to_uri(), c.uri);
        }
    }

    #[test]
    fn from_raw() {
        for c in &CONV_VALID {
            assert_eq!(SpotifyId::from_raw(c.raw).unwrap().id, c.id);
        }

        for c in &CONV_INVALID {
            assert_eq!(SpotifyId::from_raw(c.raw), Err(SpotifyIdError));
        }
    }
}
