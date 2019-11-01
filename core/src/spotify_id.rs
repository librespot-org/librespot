use std;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SpotifyId(u128);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SpotifyIdError;

const BASE62_DIGITS: &'static [u8] =
    b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const BASE16_DIGITS: &'static [u8] = b"0123456789abcdef";

impl SpotifyId {
    pub fn from_base16(id: &str) -> Result<SpotifyId, SpotifyIdError> {
        let data = id.as_bytes();

        let mut n = 0u128;
        for c in data {
            let d = match BASE16_DIGITS.iter().position(|e| e == c) {
                None => return Err(SpotifyIdError),
                Some(x) => x as u128,
            };
            n = n * 16;
            n = n + d;
        }

        Ok(SpotifyId(n))
    }

    pub fn from_base62(id: &str) -> Result<SpotifyId, SpotifyIdError> {
        let data = id.as_bytes();

        let mut n = 0u128;
        for c in data {
            let d = match BASE62_DIGITS.iter().position(|e| e == c) {
                None => return Err(SpotifyIdError),
                Some(x) => x as u128,
            };
            n = n * 62;
            n = n + d;
        }

        Ok(SpotifyId(n))
    }

    pub fn from_raw(data: &[u8]) -> Result<SpotifyId, SpotifyIdError> {
        if data.len() != 16 {
            return Err(SpotifyIdError);
        };

        let mut arr: [u8; 16] = Default::default();
        arr.copy_from_slice(&data[0..16]);

        Ok(SpotifyId(u128::from_be_bytes(arr)))
    }

    pub fn to_base16(&self) -> String {
        format!("{:032x}", self.0)
    }

    pub fn to_base62(&self) -> String {
        let &SpotifyId(mut n) = self;

        let mut data = [0u8; 22];
        for i in 0..22 {
            data[21 - i] = BASE62_DIGITS[(n % 62) as usize];
            n /= 62;
        }

        std::str::from_utf8(&data).unwrap().to_owned()
    }

    pub fn to_raw(&self) -> [u8; 16] {
        self.0.to_be_bytes()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileId(pub [u8; 20]);

impl FileId {
    pub fn to_base16(&self) -> String {
        self.0
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<String>>()
            .concat()
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
