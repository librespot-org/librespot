#[macro_use]
extern crate log;

mod decrypt;
mod fetch;

mod range_set;

pub use decrypt::AudioDecrypt;
pub use fetch::{AudioFile, AudioFileError, StreamLoaderController};
pub use fetch::{MINIMUM_DOWNLOAD_SIZE, READ_AHEAD_BEFORE_PLAYBACK, READ_AHEAD_DURING_PLAYBACK};
