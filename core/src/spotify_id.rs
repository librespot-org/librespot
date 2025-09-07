use std::fmt;

use thiserror::Error;

use crate::Error;

// re-export FileId for historic reasons, when it was part of this mod
pub use crate::FileId;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpotifyId {
    pub id: u128,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum SpotifyIdError {
    #[error("ID cannot be parsed")]
    InvalidId,
    #[error("not a valid Spotify ID")]
    InvalidFormat,
}

impl From<SpotifyIdError> for Error {
    fn from(err: SpotifyIdError) -> Self {
        Error::invalid_argument(err)
    }
}

pub type SpotifyIdResult = Result<SpotifyId, Error>;

const BASE62_DIGITS: &[u8; 62] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const BASE16_DIGITS: &[u8; 16] = b"0123456789abcdef";

impl SpotifyId {
    const SIZE: usize = 16;
    const SIZE_BASE16: usize = 32;
    const SIZE_BASE62: usize = 22;

    /// Parses a base16 (hex) encoded [Spotify ID] into a `SpotifyId`.
    ///
    /// `src` is expected to be 32 bytes long and encoded using valid characters.
    ///
    /// [Spotify ID]: https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids
    pub fn from_base16(src: &str) -> SpotifyIdResult {
        if src.len() != 32 {
            return Err(SpotifyIdError::InvalidId.into());
        }
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

        Ok(Self { id: dst })
    }

    /// Parses a base62 encoded [Spotify ID] into a `u128`.
    ///
    /// `src` is expected to be 22 bytes long and encoded using valid characters.
    ///
    /// [Spotify ID]: https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids
    pub fn from_base62(src: &str) -> SpotifyIdResult {
        if src.len() != Self::SIZE_BASE62 {
            return Err(SpotifyIdError::InvalidId.into());
        }
        let mut dst: u128 = 0;

        for c in src.as_bytes() {
            let p = match c {
                b'0'..=b'9' => c - b'0',
                b'a'..=b'z' => c - b'a' + 10,
                b'A'..=b'Z' => c - b'A' + 36,
                _ => return Err(SpotifyIdError::InvalidId.into()),
            } as u128;

            dst = dst.checked_mul(62).ok_or(SpotifyIdError::InvalidId)?;
            dst = dst.checked_add(p).ok_or(SpotifyIdError::InvalidId)?;
        }

        Ok(Self { id: dst })
    }

    /// Creates a `u128` from a copy of `SpotifyId::SIZE` (16) bytes in big-endian order.
    ///
    /// The resulting `SpotifyId` will default to a `SpotifyItemType::Unknown`.
    pub fn from_raw(src: &[u8]) -> SpotifyIdResult {
        match src.try_into() {
            Ok(dst) => Ok(Self {
                id: u128::from_be_bytes(dst),
            }),
            Err(_) => Err(SpotifyIdError::InvalidId.into()),
        }
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
}

impl fmt::Debug for SpotifyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("SpotifyId")
            .field(&self.to_base62().unwrap_or_else(|_| "invalid uri".into()))
            .finish()
    }
}

impl fmt::Display for SpotifyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_base62().unwrap_or_else(|_| "invalid uri".into()))
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
        base16: &'static str,
        base62: &'static str,
        raw: &'static [u8],
    }

    static CONV_VALID: [ConversionCase; 5] = [
        ConversionCase {
            id: 238762092608182713602505436543891614649,
            base16: "b39fe8081e1f4c54be38e8d6f9f12bb9",
            base62: "5sWHDYs0csV6RS48xBl0tH",
            raw: &[
                179, 159, 232, 8, 30, 31, 76, 84, 190, 56, 232, 214, 249, 241, 43, 185,
            ],
        },
        ConversionCase {
            id: 204841891221366092811751085145916697048,
            base16: "9a1b1cfbc6f244569ae0356c77bbe9d8",
            base62: "4GNcXTGWmnZ3ySrqvol3o4",
            raw: &[
                154, 27, 28, 251, 198, 242, 68, 86, 154, 224, 53, 108, 119, 187, 233, 216,
            ],
        },
        ConversionCase {
            id: 204841891221366092811751085145916697048,
            base16: "9a1b1cfbc6f244569ae0356c77bbe9d8",
            base62: "4GNcXTGWmnZ3ySrqvol3o4",
            raw: &[
                154, 27, 28, 251, 198, 242, 68, 86, 154, 224, 53, 108, 119, 187, 233, 216,
            ],
        },
        ConversionCase {
            id: 204841891221366092811751085145916697048,
            base16: "9a1b1cfbc6f244569ae0356c77bbe9d8",
            base62: "4GNcXTGWmnZ3ySrqvol3o4",
            raw: &[
                154, 27, 28, 251, 198, 242, 68, 86, 154, 224, 53, 108, 119, 187, 233, 216,
            ],
        },
        ConversionCase {
            id: 0,
            base16: "00000000000000000000000000000000",
            base62: "0000000000000000000000",
            raw: &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        },
    ];

    static CONV_INVALID: [ConversionCase; 5] = [
        ConversionCase {
            id: 0,
            base16: "ZZZZZ8081e1f4c54be38e8d6f9f12bb9",
            base62: "!!!!!Ys0csV6RS48xBl0tH",
            raw: &[
                // Invalid length.
                154, 27, 28, 251, 198, 242, 68, 86, 154, 224, 5, 3, 108, 119, 187, 233, 216, 255,
            ],
        },
        ConversionCase {
            id: 0,
            base16: "--------------------",
            base62: "....................",
            raw: &[
                // Invalid length.
                154, 27, 28, 251,
            ],
        },
        ConversionCase {
            id: 0,
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
            id: 0,
            base16: "--------------------",
            // too short to encode a 128 bits int
            base62: "aa",
            raw: &[
                // Invalid length.
                154, 27, 28, 251,
            ],
        },
        ConversionCase {
            id: 0,
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
            assert_eq!(SpotifyId::from_base62(c.base62).unwrap().id, c.id);
        }

        for c in &CONV_INVALID {
            assert!(SpotifyId::from_base62(c.base62).is_err(),);
        }
    }

    #[test]
    fn to_base62() {
        for c in &CONV_VALID {
            let id = SpotifyId { id: c.id };

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
            let id = SpotifyId { id: c.id };

            assert_eq!(id.to_base16().unwrap(), c.base16);
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
