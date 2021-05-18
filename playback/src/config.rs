use super::player::NormalisationData;
use crate::convert::i24;
pub use crate::dither::{Ditherer, DithererBuilder};
pub use crate::shape_noise::{NoiseShaper, NoiseShaperBuilder};

use std::convert::TryFrom;
use std::mem;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum Bitrate {
    Bitrate96,
    Bitrate160,
    Bitrate320,
}

impl FromStr for Bitrate {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "96" => Ok(Self::Bitrate96),
            "160" => Ok(Self::Bitrate160),
            "320" => Ok(Self::Bitrate320),
            _ => Err(()),
        }
    }
}

impl Default for Bitrate {
    fn default() -> Self {
        Self::Bitrate160
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum AudioFormat {
    F32,
    S32,
    S24,
    S24_3,
    S16,
}

impl TryFrom<&String> for AudioFormat {
    type Error = ();
    fn try_from(s: &String) -> Result<Self, Self::Error> {
        match s.to_uppercase().as_str() {
            "F32" => Ok(Self::F32),
            "S32" => Ok(Self::S32),
            "S24" => Ok(Self::S24),
            "S24_3" => Ok(Self::S24_3),
            "S16" => Ok(Self::S16),
            _ => Err(()),
        }
    }
}

impl Default for AudioFormat {
    fn default() -> Self {
        Self::S16
    }
}

impl AudioFormat {
    // not used by all backends
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        match self {
            Self::S24_3 => mem::size_of::<i24>(),
            Self::S16 => mem::size_of::<i16>(),
            _ => mem::size_of::<i32>(), // S32 and S24 are both stored in i32
        }
    }
}

#[derive(Clone, Debug)]
pub enum NormalisationType {
    Album,
    Track,
}

impl FromStr for NormalisationType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "album" => Ok(Self::Album),
            "track" => Ok(Self::Track),
            _ => Err(()),
        }
    }
}

impl Default for NormalisationType {
    fn default() -> Self {
        Self::Album
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum NormalisationMethod {
    Basic,
    Dynamic,
}

impl FromStr for NormalisationMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "basic" => Ok(Self::Basic),
            "dynamic" => Ok(Self::Dynamic),
            _ => Err(()),
        }
    }
}

impl Default for NormalisationMethod {
    fn default() -> Self {
        Self::Dynamic
    }
}

#[derive(Clone)]
pub struct PlayerConfig {
    pub bitrate: Bitrate,
    pub gapless: bool,
    pub passthrough: bool,

    pub normalisation: bool,
    pub normalisation_type: NormalisationType,
    pub normalisation_method: NormalisationMethod,
    pub normalisation_pregain: f32,
    pub normalisation_threshold: f32,
    pub normalisation_attack: f32,
    pub normalisation_release: f32,
    pub normalisation_knee: f32,

    // pass function pointers so they can be lazily instantiated *after* spawning a thread
    // (thereby circumventing Send bounds that they might not satisfy)
    pub ditherer: DithererBuilder,
    pub noise_shaper: NoiseShaperBuilder,
}

impl Default for PlayerConfig {
    fn default() -> PlayerConfig {
        PlayerConfig {
            bitrate: Bitrate::default(),
            normalisation: false,
            normalisation_type: NormalisationType::default(),
            normalisation_method: NormalisationMethod::default(),
            normalisation_pregain: 0.0,
            normalisation_threshold: NormalisationData::db_to_ratio(-1.0),
            normalisation_attack: 0.005,
            normalisation_release: 0.1,
            normalisation_knee: 1.0,
            gapless: true,
            passthrough: false,
            ditherer: <dyn Ditherer>::default(),
            noise_shaper: <dyn NoiseShaper>::default(),
        }
    }
}
