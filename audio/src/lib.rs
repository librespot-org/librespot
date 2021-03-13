#[macro_use]
extern crate futures;
#[macro_use]
extern crate log;

extern crate aes_ctr;
extern crate bit_set;
extern crate byteorder;
extern crate bytes;
extern crate num_bigint;
extern crate num_traits;
extern crate tempfile;

extern crate librespot_core;

mod decrypt;
mod fetch;

#[cfg(not(any(feature = "with-tremor", feature = "with-vorbis")))]
mod lewton_decoder;
#[cfg(any(feature = "with-tremor", feature = "with-vorbis"))]
mod libvorbis_decoder;
mod passthrough_decoder;

mod range_set;

pub use decrypt::AudioDecrypt;
pub use fetch::{AudioFile, AudioFileOpen, StreamLoaderController};
pub use fetch::{
    READ_AHEAD_BEFORE_PLAYBACK_ROUNDTRIPS, READ_AHEAD_BEFORE_PLAYBACK_SECONDS,
    READ_AHEAD_DURING_PLAYBACK_ROUNDTRIPS, READ_AHEAD_DURING_PLAYBACK_SECONDS,
};
use std::fmt;

pub enum AudioPacket {
    Samples(Vec<f32>),
    OggData(Vec<u8>),
}

// Losslessly represent [-1.0, 1.0] to [$type::MIN, $type::MAX] while maintaining DC linearity.
macro_rules! convert_samples_to {
    ($type: ident, $samples: expr) => {
        $samples
            .iter()
            .map(|sample| {
                if *sample == 0.0 {
                    0 as $type
                } else {
                    (*sample as f64 * (std::$type::MAX as f64 + 0.5) - 0.5) as $type
                }
            })
            .collect()
    };
}

impl AudioPacket {
    pub fn samples(&self) -> &[f32] {
        match self {
            AudioPacket::Samples(s) => s,
            AudioPacket::OggData(_) => panic!("can't return OggData on samples"),
        }
    }

    pub fn oggdata(&self) -> &[u8] {
        match self {
            AudioPacket::Samples(_) => panic!("can't return samples on OggData"),
            AudioPacket::OggData(d) => d,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            AudioPacket::Samples(s) => s.is_empty(),
            AudioPacket::OggData(d) => d.is_empty(),
        }
    }

    pub fn f32_to_s32(samples: &[f32]) -> Vec<i32> {
        convert_samples_to!(i32, samples)
    }

    pub fn f32_to_s16(samples: &[f32]) -> Vec<i16> {
        convert_samples_to!(i16, samples)
    }
}

#[cfg(not(any(feature = "with-tremor", feature = "with-vorbis")))]
pub use crate::lewton_decoder::{VorbisDecoder, VorbisError};
#[cfg(any(feature = "with-tremor", feature = "with-vorbis"))]
pub use libvorbis_decoder::{VorbisDecoder, VorbisError};
pub use passthrough_decoder::{PassthroughDecoder, PassthroughError};

#[derive(Debug)]
pub enum AudioError {
    PassthroughError(PassthroughError),
    VorbisError(VorbisError),
}

impl fmt::Display for AudioError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AudioError::PassthroughError(err) => write!(f, "PassthroughError({})", err),
            AudioError::VorbisError(err) => write!(f, "VorbisError({})", err),
        }
    }
}

impl From<VorbisError> for AudioError {
    fn from(err: VorbisError) -> AudioError {
        AudioError::VorbisError(VorbisError::from(err))
    }
}

impl From<PassthroughError> for AudioError {
    fn from(err: PassthroughError) -> AudioError {
        AudioError::PassthroughError(PassthroughError::from(err))
    }
}

pub trait AudioDecoder {
    fn seek(&mut self, ms: i64) -> Result<(), AudioError>;
    fn next_packet(&mut self) -> Result<Option<AudioPacket>, AudioError>;
}
