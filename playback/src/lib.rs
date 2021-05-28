#[macro_use]
extern crate log;

use librespot_audio as audio;
use librespot_core as core;
use librespot_metadata as metadata;

pub mod audio_backend;
pub mod config;
mod convert;
mod decoder;
pub mod dither;
pub mod mixer;
pub mod player;

pub const SAMPLE_RATE: u32 = 44100;
pub const NUM_CHANNELS: u8 = 2;
pub const SAMPLES_PER_SECOND: u32 = SAMPLE_RATE as u32 * NUM_CHANNELS as u32;

pub const MILLIS: f32 = 1000.0;
