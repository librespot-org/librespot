use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Deref;

use librespot_core::file_id::FileId;
use librespot_protocol as protocol;

use protocol::metadata::AudioFile as AudioFileMessage;

pub use protocol::metadata::AudioFile_Format as AudioFileFormat;

#[derive(Debug, Clone)]
pub struct AudioFiles(pub HashMap<AudioFileFormat, FileId>);

impl Deref for AudioFiles {
    type Target = HashMap<AudioFileFormat, FileId>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&[AudioFileMessage]> for AudioFiles {
    fn from(files: &[AudioFileMessage]) -> Self {
        let audio_files = files
            .iter()
            .map(|file| (file.get_format(), FileId::from(file.get_file_id())))
            .collect();

        AudioFiles(audio_files)
    }
}
