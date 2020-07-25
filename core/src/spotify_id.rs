use std;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpotifyAudioType {
    Track,
    Podcast,
    NonPlayable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpotifyId {
    pub id: u128,
    pub audio_type: SpotifyAudioType,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SpotifyIdError;

const BASE62_DIGITS: &'static [u8] =
    b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const BASE16_DIGITS: &'static [u8] = b"0123456789abcdef";

impl SpotifyId {
    fn as_track(n: u128) -> SpotifyId {
        SpotifyId {
            id: n.to_owned(),
            audio_type: SpotifyAudioType::Track,
        }
    }

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

        Ok(SpotifyId::as_track(n))
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
        Ok(SpotifyId::as_track(n))
    }

    pub fn from_raw(data: &[u8]) -> Result<SpotifyId, SpotifyIdError> {
        if data.len() != 16 {
            return Err(SpotifyIdError);
        };

        let mut arr: [u8; 16] = Default::default();
        arr.copy_from_slice(&data[0..16]);

        Ok(SpotifyId::as_track(u128::from_be_bytes(arr)))
    }

    pub fn from_uri(uri: &str) -> Result<SpotifyId, SpotifyIdError> {
        let parts = uri.split(":").collect::<Vec<&str>>();
        let gid = parts.last().unwrap();
        if uri.contains(":episode:") {
            let mut spotify_id = SpotifyId::from_base62(gid).unwrap();
            let _ = std::mem::replace(&mut spotify_id.audio_type, SpotifyAudioType::Podcast);
            Ok(spotify_id)
        } else if uri.contains(":track:") {
            SpotifyId::from_base62(gid)
        } else {
            // show/playlist/artist/album/??
            let mut spotify_id = SpotifyId::from_base62(gid).unwrap();
            let _ = std::mem::replace(&mut spotify_id.audio_type, SpotifyAudioType::NonPlayable);
            Ok(spotify_id)
        }
    }

    pub fn to_base16(&self) -> String {
        format!("{:032x}", self.id)
    }

    pub fn to_base62(&self) -> String {
        let &SpotifyId { id: mut n, .. } = self;

        let mut data = [0u8; 22];
        for i in 0..22 {
            data[21 - i] = BASE62_DIGITS[(n % 62) as usize];
            n /= 62;
        }

        std::str::from_utf8(&data).unwrap().to_owned()
    }

    pub fn to_uri(&self) -> String {
        match self.audio_type {
            SpotifyAudioType::Track => format!("spotify:track:{}", self.to_base62()),
            SpotifyAudioType::Podcast => format!("spotify:episode:{}", self.to_base62()),
            SpotifyAudioType::NonPlayable => format!("spotify:unknown:{}", self.to_base62()),
        }
    }

    pub fn to_raw(&self) -> [u8; 16] {
        self.id.to_be_bytes()
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
