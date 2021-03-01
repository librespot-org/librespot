#![allow(clippy::unused_io_amount, clippy::too_many_arguments)]

#[macro_use]
extern crate log;

mod decrypt;
mod fetch;

use cfg_if::cfg_if;

#[cfg(any(
    all(feature = "with-lewton", feature = "with-tremor"),
    all(feature = "with-vorbis", feature = "with-tremor"),
    all(feature = "with-lewton", feature = "with-vorbis")
))]
compile_error!("Cannot use two decoders at the same time.");

cfg_if! {
    if #[cfg(feature = "with-lewton")] {
        mod lewton_decoder;
        pub use lewton_decoder::{VorbisDecoder, VorbisError};
    } else if #[cfg(any(feature = "with-tremor", feature = "with-vorbis"))] {
        mod libvorbis_decoder;
        pub use crate::libvorbis_decoder::{VorbisDecoder, VorbisError};
    } else {
        compile_error!("Must choose a vorbis decoder.");
    }
}

mod passthrough_decoder;
pub use passthrough_decoder::{PassthroughDecoder, PassthroughError};

mod range_set;

pub use decrypt::AudioDecrypt;
pub use fetch::{AudioFile, StreamLoaderController};
pub use fetch::{
    READ_AHEAD_BEFORE_PLAYBACK_ROUNDTRIPS, READ_AHEAD_BEFORE_PLAYBACK_SECONDS,
    READ_AHEAD_DURING_PLAYBACK_ROUNDTRIPS, READ_AHEAD_DURING_PLAYBACK_SECONDS,
};
use std::fmt;

pub enum AudioPacket {
    Samples(Vec<i16>),
    OggData(Vec<u8>),
}

impl AudioPacket {
    pub fn samples(&self) -> &[i16] {
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
}

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
        AudioError::VorbisError(err)
    }
}

impl From<PassthroughError> for AudioError {
    fn from(err: PassthroughError) -> AudioError {
        AudioError::PassthroughError(err)
    }
}

pub trait AudioDecoder {
    fn seek(&mut self, ms: i64) -> Result<(), AudioError>;
    fn next_packet(&mut self) -> Result<Option<AudioPacket>, AudioError>;
}
