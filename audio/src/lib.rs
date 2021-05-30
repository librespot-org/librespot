#![allow(clippy::too_many_arguments)]

#[macro_use]
extern crate log;

mod decrypt;
mod fetch;

mod range_set;

pub use decrypt::AudioDecrypt;
pub use fetch::{AudioFile, StreamLoaderController};
pub use fetch::{
    READ_AHEAD_BEFORE_PLAYBACK_ROUNDTRIPS, READ_AHEAD_BEFORE_PLAYBACK_SECONDS,
    READ_AHEAD_DURING_PLAYBACK_ROUNDTRIPS, READ_AHEAD_DURING_PLAYBACK_SECONDS,
};
