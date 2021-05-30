use super::player::db_to_ratio;
use crate::convert::i24;
pub use crate::dither::{mk_ditherer, DithererBuilder, TriangularDitherer};

use std::convert::TryFrom;
use std::mem;
use std::str::FromStr;
use std::time::Duration;

#[derive(Clone, Copy, Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum Bitrate {
    Bitrate96,
    Bitrate160,
    Bitrate320,
}

impl TryFrom<&str> for Bitrate {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
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

impl TryFrom<&str> for AudioFormat {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s.to_uppercase().as_ref() {
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
            Self::F32 => mem::size_of::<f32>(),
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

impl TryFrom<&str> for NormalisationType {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_ref() {
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

impl TryFrom<&str> for NormalisationMethod {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_ref() {
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
    pub normalisation_attack: Duration,
    pub normalisation_release: Duration,
    pub normalisation_knee: f32,

    // pass function pointers so they can be lazily instantiated *after* spawning a thread
    // (thereby circumventing Send bounds that they might not satisfy)
    pub ditherer: Option<DithererBuilder>,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        Self {
            bitrate: Bitrate::default(),
            gapless: true,
            normalisation: false,
            normalisation_type: NormalisationType::default(),
            normalisation_method: NormalisationMethod::default(),
            normalisation_pregain: 0.0,
            normalisation_threshold: db_to_ratio(-1.0),
            normalisation_attack: Duration::from_millis(5),
            normalisation_release: Duration::from_millis(100),
            normalisation_knee: 1.0,
            passthrough: false,
            ditherer: Some(mk_ditherer::<TriangularDitherer>),
        }
    }
}

// fields are intended for volume control range in dB
#[derive(Clone, Copy, Debug)]
pub enum VolumeCtrl {
    Cubic(f32),
    Fixed,
    Linear,
    Log(f32),
}

impl FromStr for VolumeCtrl {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from_str_with_range(s, Self::DEFAULT_DB_RANGE)
    }
}

impl Default for VolumeCtrl {
    fn default() -> VolumeCtrl {
        VolumeCtrl::Log(Self::DEFAULT_DB_RANGE)
    }
}

impl VolumeCtrl {
    pub const MAX_VOLUME: u16 = u16::MAX;

    // Taken from: https://www.dr-lex.be/info-stuff/volumecontrols.html
    pub const DEFAULT_DB_RANGE: f32 = 60.0;

    pub fn try_from_str_with_range(s: &str, db_range: f32) -> Result<Self, <Self as FromStr>::Err> {
        use self::VolumeCtrl::*;
        match s.to_lowercase().as_ref() {
            "cubic" => Ok(Cubic(db_range)),
            "fixed" => Ok(Fixed),
            "linear" => Ok(Linear),
            "log" => Ok(Log(db_range)),
            _ => Err(()),
        }
    }
}
