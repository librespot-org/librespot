use std::convert::TryFrom;
use std::fmt;

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
pub mod sample_pipeline;

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

#[derive(Copy, Clone, Debug, Default)]
pub enum CommonSampleRates {
    #[default]
    Hz8000,
    Hz11025,
    Hz16000,
    Hz22050,
    Hz44100,
    Hz48000,
    Hz88200,
    Hz96000,
    Hz176400,
    Hz192000,
    Hz352800,
    Hz384000,
    Hz705600,
    Hz768000,
}

impl TryFrom<u32> for CommonSampleRates {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        use CommonSampleRates::*;

        match value {
            8000 => Ok(Hz8000),
            11025 => Ok(Hz11025),
            16000 => Ok(Hz16000),
            22050 => Ok(Hz22050),
            44100 => Ok(Hz44100),
            48000 => Ok(Hz48000),
            88200 => Ok(Hz88200),
            96000 => Ok(Hz96000),
            176400 => Ok(Hz176400),
            192000 => Ok(Hz192000),
            352800 => Ok(Hz352800),
            384000 => Ok(Hz384000),
            705600 => Ok(Hz705600),
            768000 => Ok(Hz768000),
            _ => Err(()),
        }
    }
}

impl fmt::Display for CommonSampleRates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use CommonSampleRates::*;

        let rate_str = match self {
            Hz8000 => "8kHz",
            Hz11025 => "11.025kHz",
            Hz16000 => "16kHz",
            Hz22050 => "22.05kHz",
            Hz44100 => "44.1kHz",
            Hz48000 => "48kHz",
            Hz88200 => "88.2kHz",
            Hz96000 => "96kHz",
            Hz176400 => "176.4kHz",
            Hz192000 => "192kHz",
            Hz352800 => "352.8kHz",
            Hz384000 => "384kHz",
            Hz705600 => "705.6kHz",
            Hz768000 => "768kHz",
        };

        write!(f, "{}", rate_str)
    }
}

impl IntoIterator for CommonSampleRates {
    type Item = CommonSampleRates;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        use CommonSampleRates::*;

        vec![
            Hz8000, Hz11025, Hz16000, Hz22050, Hz44100, Hz48000, Hz88200, Hz96000, Hz176400,
            Hz192000, Hz352800, Hz384000, Hz705600, Hz768000,
        ]
        .into_iter()
    }
}

impl CommonSampleRates {
    pub fn as_u32(&self) -> u32 {
        use CommonSampleRates::*;

        match self {
            Hz8000 => 8000,
            Hz11025 => 11025,
            Hz16000 => 16000,
            Hz22050 => 22050,
            Hz44100 => 44100,
            Hz48000 => 48000,
            Hz88200 => 88200,
            Hz96000 => 96000,
            Hz176400 => 176400,
            Hz192000 => 192000,
            Hz352800 => 352800,
            Hz384000 => 384000,
            Hz705600 => 705600,
            Hz768000 => 768000,
        }
    }

    pub fn contains(&self, rate: u32) -> bool {
        self.into_iter().any(|r| r.as_u32() == rate)
    }
}
