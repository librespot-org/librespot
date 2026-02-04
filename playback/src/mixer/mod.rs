use librespot_core::Error;
use std::sync::Arc;

pub mod mappings;
use self::mappings::MappedCtrl;

use clap::ValueEnum;
use enum_assoc::Assoc;
use serde::{Deserialize, Serialize};

pub struct NoOpVolume;

/// Fields are intended for volume control range in dB
#[derive(Default, Clone, Copy, Debug, ValueEnum, Assoc, Serialize, Deserialize)]
#[func(pub fn build(&self, db_range: f64) -> VolumeCtrl)]
pub enum VolumeCtrlBuilder {
    #[assoc(build = VolumeCtrl::Fixed)]
    Fixed,
    #[assoc(build = VolumeCtrl::Linear)]
    Linear,
    #[assoc(build = VolumeCtrl::Cubic(db_range))]
    Cubic,
    #[default]
    #[assoc(build = VolumeCtrl::Log(db_range))]
    Log,
}

#[derive(Clone, Copy, Debug)]
pub enum VolumeCtrl {
    Fixed,
    Linear,
    Cubic(f64),
    Log(f64),
}

impl Default for VolumeCtrl {
    fn default() -> VolumeCtrl {
        VolumeCtrl::Log(Self::DEFAULT_DB_RANGE)
    }
}

impl VolumeCtrl {
    pub const MAX_VOLUME: u16 = u16::MAX;

    // Taken from: https://www.dr-lex.be/info-stuff/volumecontrols.html
    pub const DEFAULT_DB_RANGE: f64 = 60.0;
}

pub trait VolumeGetter {
    fn attenuation_factor(&self) -> f64;
}

impl VolumeGetter for NoOpVolume {
    #[inline]
    fn attenuation_factor(&self) -> f64 {
        1.0
    }
}

#[derive(Debug, Clone, derive_builder::Builder)]
#[builder(default)]
pub struct MixerConfig {
    pub device: String,
    pub control: String,
    pub index: u32,
    pub volume_ctrl: VolumeCtrl,
}

impl Default for MixerConfig {
    fn default() -> MixerConfig {
        MixerConfig {
            device: String::from("default"),
            control: String::from("PCM"),
            index: 0,
            volume_ctrl: VolumeCtrl::default(),
        }
    }
}

pub trait Mixer: Send + Sync {
    fn open(config: MixerConfig) -> Result<Self, Error>
    where
        Self: Sized;

    fn volume(&self) -> u16;
    fn set_volume(&self, volume: u16);

    fn get_soft_volume(&self) -> Box<dyn VolumeGetter + Send> {
        Box::new(NoOpVolume)
    }
}

fn mk_sink<M: Mixer + 'static>(config: MixerConfig) -> Result<Arc<dyn Mixer>, Error> {
    Ok(Arc::new(M::open(config)?))
}

pub mod softmixer;
use self::softmixer::SoftMixer;

#[cfg(feature = "alsa-backend")]
pub mod alsamixer;
#[cfg(feature = "alsa-backend")]
use self::alsamixer::AlsaMixer;

#[derive(Clone, Copy, Debug, Default, ValueEnum, Assoc, Serialize, Deserialize)]
#[func(pub fn build(&self, config: MixerConfig) -> Result<Arc<dyn Mixer>, Error>)]
#[func(pub fn is_alsa(&self) -> bool)]
pub enum MixerBuilder {
    #[assoc(build=mk_sink::<SoftMixer>(config))]
    #[assoc(is_alsa = false)]
    #[default]
    Softvol,
    #[cfg(feature = "alsa-backend")]
    #[assoc(build=mk_sink::<AlsaMixer>(config))]
    #[assoc(is_alsa = true)]
    Alsa,
}
