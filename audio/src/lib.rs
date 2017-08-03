#[macro_use] extern crate log;
#[macro_use] extern crate futures;

extern crate bit_set;
extern crate byteorder;
extern crate crypto;
extern crate num_traits;
extern crate num_bigint;
extern crate tempfile;

extern crate librespot_core as core;

mod fetch;
mod decrypt;

#[cfg(not(feature = "with-lewton"))]
mod libvorbis_decoder;
#[cfg(feature = "with-lewton")]
mod lewton_decoder;

pub use fetch::{AudioFile, AudioFileOpen};
pub use decrypt::AudioDecrypt;

#[cfg(not(feature = "with-lewton"))]
pub use libvorbis_decoder::{VorbisDecoder, VorbisPacket, VorbisError};
#[cfg(feature = "with-lewton")]
pub use lewton_decoder::{VorbisDecoder, VorbisPacket, VorbisError};
