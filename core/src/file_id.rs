use std::fmt;

use librespot_protocol as protocol;

use crate::{spotify_id::to_base16, Error};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileId(pub [u8; 20]);

impl FileId {
    pub fn from_raw(src: &[u8]) -> FileId {
        let mut dst = [0u8; 20];
        dst.clone_from_slice(src);
        FileId(dst)
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_base16(&self) -> Result<String, Error> {
        to_base16(&self.0, &mut [0u8; 40])
    }
}

impl fmt::Debug for FileId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("FileId").field(&self.to_base16()).finish()
    }
}

impl fmt::Display for FileId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_base16().unwrap_or_default())
    }
}

impl From<&[u8]> for FileId {
    fn from(src: &[u8]) -> Self {
        Self::from_raw(src)
    }
}
impl From<&protocol::metadata::Image> for FileId {
    fn from(image: &protocol::metadata::Image) -> Self {
        Self::from(image.file_id())
    }
}

impl From<&protocol::metadata::AudioFile> for FileId {
    fn from(file: &protocol::metadata::AudioFile) -> Self {
        Self::from(file.file_id())
    }
}

impl From<&protocol::metadata::VideoFile> for FileId {
    fn from(video: &protocol::metadata::VideoFile) -> Self {
        Self::from(video.file_id())
    }
}
