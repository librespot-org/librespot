mod decrypt;
mod fetch;

#[cfg(not(any(feature = "with-tremor", feature = "with-vorbis")))]
mod lewton_decoder;
#[cfg(any(feature = "with-tremor", feature = "with-vorbis"))]
mod libvorbis_decoder;

mod range_set;

pub use decrypt::AudioDecrypt;
pub use fetch::{AudioFile, AudioFileOpen, StreamLoaderController};
pub use fetch::{
    READ_AHEAD_BEFORE_PLAYBACK_ROUNDTRIPS, READ_AHEAD_BEFORE_PLAYBACK_SECONDS,
    READ_AHEAD_DURING_PLAYBACK_ROUNDTRIPS, READ_AHEAD_DURING_PLAYBACK_SECONDS,
};

#[cfg(not(any(feature = "with-tremor", feature = "with-vorbis")))]
pub use crate::lewton_decoder::{VorbisDecoder, VorbisError, VorbisPacket};
#[cfg(any(feature = "with-tremor", feature = "with-vorbis"))]
pub use libvorbis_decoder::{VorbisDecoder, VorbisError, VorbisPacket};
