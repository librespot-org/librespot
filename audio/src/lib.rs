#[macro_use]
extern crate futures;
#[macro_use]
extern crate log;

extern crate bit_set;
extern crate byteorder;
extern crate crypto;
extern crate num_bigint;
extern crate num_traits;
extern crate tempfile;

extern crate librespot_core as core;

mod decrypt;
mod fetch;

#[cfg(not(any(feature = "with-tremor", feature = "with-vorbis")))]
mod lewton_decoder;
#[cfg(any(feature = "with-tremor", feature = "with-vorbis"))]
mod libvorbis_decoder;

pub use decrypt::AudioDecrypt;
pub use fetch::{AudioFile, AudioFileOpen};
use std::error;
use std::fmt;

pub struct AudioPacket(Vec<i16>);

impl AudioPacket {
    pub fn data(&self) -> &[i16] {
        &self.0
    }

    pub fn data_mut(&mut self) -> &mut [i16] {
        &mut self.0
    }
}

#[cfg(not(any(feature = "with-tremor", feature = "with-vorbis")))]
pub use lewton_decoder::{VorbisDecoder, VorbisError};
#[cfg(any(feature = "with-tremor", feature = "with-vorbis"))]
pub use libvorbis_decoder::{VorbisDecoder, VorbisError};

#[derive(Debug)]
pub enum AudioError {
    VorbisError(VorbisError),
}

impl fmt::Display for AudioError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AudioError::VorbisError(err) => write!(f, "VorbisError({})", err),
        }
    }
}

impl error::Error for AudioError {
    fn description(&self) -> &str {
        match self {
            AudioError::VorbisError(err) => err.description(),
        }
    }
    fn cause(&self) -> Option<&error::Error> {
        match self {
            AudioError::VorbisError(err) => err.cause(),
        }
    }
}

pub trait AudioDecoder {
    fn seek(&mut self, ms: i64) -> Result<(), AudioError>;
    fn next_packet(&mut self) -> Result<Option<AudioPacket>, AudioError>;
}
