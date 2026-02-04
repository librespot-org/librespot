use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, time::Duration};

use crate::dither::DithererBuilder;
use crate::player::duration_to_coefficient;

#[derive(
    Clone,
    Copy,
    Debug,
    Hash,
    PartialOrd,
    Ord,
    PartialEq,
    Eq,
    Default,
    ValueEnum,
    Deserialize,
    Serialize,
)]
pub enum Bitrate {
    #[clap(name = "96")]
    Bitrate96,
    #[default]
    #[clap(name = "160")]
    Bitrate160,
    #[clap(name = "320")]
    Bitrate320,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Hash,
    PartialOrd,
    Ord,
    PartialEq,
    Eq,
    Default,
    ValueEnum,
    Deserialize,
    Serialize,
)]
#[clap(rename_all = "verbatim")]
pub enum AudioFormat {
    F64,
    F32,
    S32,
    S24,
    S24_3,
    #[default]
    S16,
}

#[cfg(any(
    feature = "gstreamer-backend",
    feature = "jackaudio-backend",
    feature = "sdl-backend"
))]
use std::mem;

#[cfg(any(
    feature = "gstreamer-backend",
    feature = "jackaudio-backend",
    feature = "sdl-backend"
))]
use crate::convert::i24;

#[cfg(any(
    feature = "gstreamer-backend",
    feature = "jackaudio-backend",
    feature = "sdl-backend"
))]
impl AudioFormat {
    pub fn size(&self) -> usize {
        match self {
            Self::F64 => mem::size_of::<f64>(),
            Self::F32 => mem::size_of::<f32>(),
            Self::S32 | Self::S24 => mem::size_of::<i32>(),
            Self::S24_3 => mem::size_of::<i24>(),
            Self::S16 => mem::size_of::<i16>(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, ValueEnum, Serialize, Deserialize)]
pub enum NormalisationType {
    Album,
    Track,
    #[default]
    Auto,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, ValueEnum, Serialize, Deserialize)]
pub enum NormalisationMethod {
    Basic,
    #[default]
    Dynamic,
}

#[derive(Clone)]
pub struct PlayerConfig {
    pub bitrate: Bitrate,
    pub gapless: bool,
    pub passthrough: bool,

    pub normalisation: bool,
    pub normalisation_type: NormalisationType,
    pub normalisation_method: NormalisationMethod,
    pub normalisation_pregain_db: f64,
    pub normalisation_threshold_dbfs: f64,
    pub normalisation_attack_cf: f64,
    pub normalisation_release_cf: f64,
    pub normalisation_knee_db: f64,

    pub local_file_directories: Vec<PathBuf>,

    // pass function pointers so they can be lazily instantiated *after* spawning a thread
    // (thereby circumventing Send bounds that they might not satisfy)
    pub ditherer_builder: DithererBuilder,
    /// Setting this will enable periodically sending events during playback informing about the playback position
    /// To consume the PlayerEvent::PositionChanged event, listen to events via `Player::get_player_event_channel()``
    pub position_update_interval: Option<Duration>,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        Self {
            bitrate: Bitrate::default(),
            gapless: true,
            normalisation: false,
            normalisation_type: NormalisationType::default(),
            normalisation_method: NormalisationMethod::default(),
            normalisation_pregain_db: PlayerConfig::DEFAULT_PREGAIN,
            normalisation_threshold_dbfs: PlayerConfig::DEFAULT_THRESHOLD,
            normalisation_attack_cf: duration_to_coefficient(Duration::from_millis(
                PlayerConfig::DEFAULT_ATTACK,
            )),
            normalisation_release_cf: duration_to_coefficient(Duration::from_millis(
                PlayerConfig::DEFAULT_RELEASE,
            )),
            normalisation_knee_db: PlayerConfig::DEFAULT_KNEE,
            passthrough: false,
            ditherer_builder: DithererBuilder::default(),
            position_update_interval: None,
            local_file_directories: Vec::new(),
        }
    }
}

impl PlayerConfig {
    pub const DEFAULT_PREGAIN: f64 = 0.0;
    pub const DEFAULT_THRESHOLD: f64 = -2.0;
    pub const DEFAULT_ATTACK: u64 = 5;
    pub const DEFAULT_RELEASE: u64 = 100;
    pub const DEFAULT_KNEE: f64 = 5.0;
}
