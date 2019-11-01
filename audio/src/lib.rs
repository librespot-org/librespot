#[macro_use]
extern crate futures;
#[macro_use]
extern crate log;

extern crate bit_set;
extern crate byteorder;
extern crate bytes;
extern crate crypto;
extern crate num_bigint;
extern crate num_traits;
extern crate tempfile;
extern crate tokio;

extern crate librespot_core as core;

mod decrypt;
mod fetch;

#[cfg(not(any(feature = "with-tremor", feature = "with-vorbis")))]
mod lewton_decoder;
#[cfg(any(feature = "with-tremor", feature = "with-vorbis"))]
mod libvorbis_decoder;

mod range_set;

pub use decrypt::AudioDecrypt;
pub use fetch::{AudioFile, AudioFileOpen, StreamLoaderController};

#[cfg(not(any(feature = "with-tremor", feature = "with-vorbis")))]
pub use lewton_decoder::{VorbisDecoder, VorbisError, VorbisPacket};
#[cfg(any(feature = "with-tremor", feature = "with-vorbis"))]
pub use libvorbis_decoder::{VorbisDecoder, VorbisError, VorbisPacket};

