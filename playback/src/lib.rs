#[macro_use]
extern crate log;

use librespot_audio as audio;
use librespot_core as core;
use librespot_metadata as metadata;

pub mod audio_backend;
pub mod config;
pub mod convert;
pub mod decoder;
pub mod dither;
pub mod mixer;
pub mod normaliser;
pub mod player;
pub mod resampler;

pub const DB_VOLTAGE_RATIO: f64 = 20.0;
pub const PCM_AT_0DBFS: f64 = 1.0;
pub const RESAMPLER_INPUT_SIZE: usize = 147;
pub const SAMPLE_RATE: u32 = 44100;
pub const NUM_CHANNELS: u8 = 2;
pub const SAMPLES_PER_SECOND: u32 = SAMPLE_RATE * NUM_CHANNELS as u32;
pub const PAGES_PER_MS: f64 = SAMPLE_RATE as f64 / 1000.0;
pub const MS_PER_PAGE: f64 = 1000.0 / SAMPLE_RATE as f64;

pub fn db_to_ratio(db: f64) -> f64 {
    f64::powf(10.0, db / DB_VOLTAGE_RATIO)
}

pub fn ratio_to_db(ratio: f64) -> f64 {
    ratio.log10() * DB_VOLTAGE_RATIO
}
