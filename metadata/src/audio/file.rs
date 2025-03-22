use std::{
    collections::HashMap,
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use librespot_core::FileId;

use crate::util::impl_deref_wrapped;
use librespot_protocol as protocol;
use protocol::metadata::AudioFile as AudioFileMessage;

use librespot_protocol::metadata::audio_file::Format;
use protobuf::Enum;

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AudioFileFormat {
    OGG_VORBIS_96,   // 0
    OGG_VORBIS_160,  // 1
    OGG_VORBIS_320,  // 2
    MP3_256,         // 3
    MP3_320,         // 4
    MP3_160,         // 5
    MP3_96,          // 6
    MP3_160_ENC,     // 7
    AAC_24,          // 8
    AAC_48,          // 9
    FLAC_FLAC,       // 16
    XHE_AAC_24,      // 18
    XHE_AAC_16,      // 19
    XHE_AAC_12,      // 20
    FLAC_FLAC_24BIT, // 22
    // not defined in protobuf, but sometimes send
    AAC_160, // 10
    AAC_320, // 11
    MP4_128, // 12
    OTHER5,  // 13
}

impl TryFrom<i32> for AudioFileFormat {
    type Error = i32;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            10 => AudioFileFormat::AAC_160,
            11 => AudioFileFormat::AAC_320,
            12 => AudioFileFormat::MP4_128,
            13 => AudioFileFormat::OTHER5,
            _ => Format::from_i32(value).ok_or(value)?.into(),
        })
    }
}

impl From<Format> for AudioFileFormat {
    fn from(value: Format) -> Self {
        match value {
            Format::OGG_VORBIS_96 => AudioFileFormat::OGG_VORBIS_96,
            Format::OGG_VORBIS_160 => AudioFileFormat::OGG_VORBIS_160,
            Format::OGG_VORBIS_320 => AudioFileFormat::OGG_VORBIS_320,
            Format::MP3_256 => AudioFileFormat::MP3_256,
            Format::MP3_320 => AudioFileFormat::MP3_320,
            Format::MP3_160 => AudioFileFormat::MP3_160,
            Format::MP3_96 => AudioFileFormat::MP3_96,
            Format::MP3_160_ENC => AudioFileFormat::MP3_160_ENC,
            Format::AAC_24 => AudioFileFormat::AAC_24,
            Format::AAC_48 => AudioFileFormat::AAC_48,
            Format::FLAC_FLAC => AudioFileFormat::FLAC_FLAC,
            Format::XHE_AAC_24 => AudioFileFormat::XHE_AAC_24,
            Format::XHE_AAC_16 => AudioFileFormat::XHE_AAC_16,
            Format::XHE_AAC_12 => AudioFileFormat::XHE_AAC_12,
            Format::FLAC_FLAC_24BIT => AudioFileFormat::FLAC_FLAC_24BIT,
        }
    }
}

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
        let audio_files: HashMap<AudioFileFormat, FileId> = files
            .iter()
            .filter_map(|file| {
                let file_id = FileId::from(file.file_id());
                if let Some(format) = file.format {
                    match format.enum_value() {
                        Ok(f) => return Some((f.into(), file_id)),
                        Err(unknown) => {
                            trace!("Ignoring file <{}> with unknown format {unknown}", file_id);
                        }
                    }
                } else {
                    trace!("Ignoring file <{}> with unspecified format", file_id);
                }
                None
            })
            .collect();

        AudioFiles(audio_files)
    }
}
