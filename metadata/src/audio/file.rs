use std::{
    collections::HashMap,
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use librespot_core::FileId;

use librespot_protocol as protocol;
use protocol::metadata::AudioFile as AudioFileMessage;
pub use protocol::metadata::AudioFile_Format as AudioFileFormat;

use crate::util::impl_deref_wrapped;

#[derive(Debug, Clone, Default)]
pub struct AudioFiles(pub HashMap<AudioFileFormat, FileId>);

impl_deref_wrapped!(AudioFiles, HashMap<AudioFileFormat, FileId>);

impl AudioFiles {
    pub fn is_ogg_vorbis(format: AudioFileFormat) -> bool {
        matches!(
            format,
            AudioFileFormat::OGG_VORBIS_320
                | AudioFileFormat::OGG_VORBIS_160
                | AudioFileFormat::OGG_VORBIS_96
        )
    }

    pub fn is_mp3(format: AudioFileFormat) -> bool {
        matches!(
            format,
            AudioFileFormat::MP3_320
                | AudioFileFormat::MP3_256
                | AudioFileFormat::MP3_160
                | AudioFileFormat::MP3_96
                | AudioFileFormat::MP3_160_ENC
        )
    }

    pub fn is_flac(format: AudioFileFormat) -> bool {
        matches!(format, AudioFileFormat::FLAC_FLAC)
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
