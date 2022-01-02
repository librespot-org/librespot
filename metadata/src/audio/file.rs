use std::{collections::HashMap, fmt::Debug, ops::Deref};

use librespot_core::FileId;

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
            .filter_map(|file| {
                let file_id = FileId::from(file.get_file_id());
                if file.has_format() {
                    Some((file.get_format(), file_id))
                } else {
                    trace!("Ignoring file <{}> with unspecified format", file_id);
                    None
                }
            })
            .collect();

        AudioFiles(audio_files)
    }
}
