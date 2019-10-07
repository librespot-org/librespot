use byteorder::{BigEndian, ByteOrder};
use extprim::u128::u128;
use std;
use std::fmt;
use spotify_id::SpotifyId::Track;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SpotifyId {
    Track(u128),
    Episode(u128),
    NonPlayable(u128),
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SpotifyIdError;

const BASE62_DIGITS: &'static [u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const BASE16_DIGITS: &'static [u8] = b"0123456789abcdef";

impl SpotifyId {
    pub fn from_base16(id: &str, podcast: bool) -> Result<SpotifyId, SpotifyIdError> {
        let data = id.as_bytes();

        let mut n: u128 = u128::zero();
        for c in data {
            let d = match BASE16_DIGITS.iter().position(|e| e == c) {
                None => return Err(SpotifyIdError),
                Some(x) => x as u64,
            };
            n = n * u128::new(16);
            n = n + u128::new(d);
        }
        if podcast {
            Ok(SpotifyId::Episode(n))

        }
        else {
            Ok(SpotifyId::Track(n))
        }
    }

    pub fn from_base62(id: &str) -> Result<SpotifyId, SpotifyIdError> {
        let data = id.as_bytes();

        let mut n: u128 = u128::zero();
        for c in data {
            let d = match BASE62_DIGITS.iter().position(|e| e == c) {
                None => {
                    warn!("Got an error decoding");
                    return Err(SpotifyIdError)
                },
                Some(x) => x as u64,
            };
            n = n * u128::new(62);
            n = n + u128::new(d);
        }
        Ok(SpotifyId::Track(n))
    }

    // Shouldn't be used, from_rawURI should be the only usage
    pub fn from_raw(data: &[u8]) -> Result<SpotifyId, SpotifyIdError> {
        debug!("Called from_raw()");
        if data.len() != 16 {
            return Err(SpotifyIdError);
        };

        let high = BigEndian::read_u64(&data[0..8]);
        let low = BigEndian::read_u64(&data[8..16]);
        Ok(SpotifyId::Track(u128::from_parts(high, low)))
    }

    // for episode support
    // extract the spotify id from the uri
    pub fn from_rawURI(data: &str) -> Result<SpotifyId, SpotifyIdError> {
        let parts = data.split(":");
        let vec = parts.collect::<Vec<&str>>();
        let uri = vec.last().unwrap();
        let spotify_type = vec[1];
        let id = SpotifyId::from_base62(uri);
        if spotify_type == "episode" {
            let n = match id.unwrap() {
                SpotifyId::Track(n) => n,
                SpotifyId::NonPlayable(n) => n,
                SpotifyId::Episode(n) => n,
            };
            Ok(SpotifyId::Episode(n))
        }
        else {
            id
        }
    }

    pub fn to_base16(&self) -> String {
        // This code seems idiotic, but should do the job
        // Need someone with rust expertise to do better
        let n = match self {
            SpotifyId::Episode(ref number) => number,
            SpotifyId::NonPlayable(ref number)=> number,
            SpotifyId::Track(ref number) => number
        };

        let mut data = [0u8; 32];
        for i in 0..32 {
            data[31 - i] = BASE16_DIGITS[(n.wrapping_shr(4 * i as u32).low64() & 0xF) as usize];
        }

        std::str::from_utf8(&data).unwrap().to_owned()
    }

    pub fn to_base62(&self) -> String {
        let mut n = match self {
            SpotifyId::Episode(ref number) => *number,
            SpotifyId::NonPlayable(ref number)=> *number,
            SpotifyId::Track(ref number) => *number
        };

        let mut data = [0u8; 22];
        let sixty_two = u128::new(62);
        for i in 0..22 {
            data[21 - i] = BASE62_DIGITS[(n % sixty_two).low64() as usize];
            n /= sixty_two;
        }

        std::str::from_utf8(&data).unwrap().to_owned()
    }

    pub fn to_raw(&self) -> [u8; 16] {
        let ref n = match self {
            SpotifyId::Episode(ref number) => number,
            SpotifyId::NonPlayable(ref number)=> number,
            SpotifyId::Track(ref number) => number
        };

        let mut data = [0u8; 16];

        BigEndian::write_u64(&mut data[0..8], n.high64());
        BigEndian::write_u64(&mut data[8..16], n.low64());

        data
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
