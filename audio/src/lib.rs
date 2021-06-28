#![allow(clippy::unused_io_amount, clippy::too_many_arguments)]

#[macro_use]
extern crate log;

mod decrypt;
mod fetch;

mod range_set;

pub use decrypt::AudioDecrypt;
pub use fetch::{AudioFile, StreamLoaderController};
pub use fetch::{
    READ_AHEAD_BEFORE_PLAYBACK, READ_AHEAD_BEFORE_PLAYBACK_ROUNDTRIPS, READ_AHEAD_DURING_PLAYBACK,
    READ_AHEAD_DURING_PLAYBACK_ROUNDTRIPS,
};
