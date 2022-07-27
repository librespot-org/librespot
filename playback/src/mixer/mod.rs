use crate::config::VolumeCtrl;

pub mod mappings;
use self::mappings::MappedCtrl;

pub struct NoOpVolume;

pub trait Mixer: Send {
    fn open(config: MixerConfig) -> Self
    where
        Self: Sized;

    fn set_volume(&self, volume: u16);
    fn volume(&self) -> u16;

    fn get_soft_volume(&self) -> Box<dyn VolumeGetter + Send> {
        Box::new(NoOpVolume)
    }
}

pub trait VolumeGetter {
    fn attenuation_factor(&self) -> f64;
}

impl VolumeGetter for NoOpVolume {
    fn attenuation_factor(&self) -> f64 {
        1.0
    }
}

pub mod softmixer;
use self::softmixer::SoftMixer;

#[cfg(feature = "alsa-backend")]
pub mod alsamixer;
#[cfg(feature = "alsa-backend")]
use self::alsamixer::AlsaMixer;

#[derive(Debug, Clone)]
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

pub type MixerFn = fn(MixerConfig) -> Box<dyn Mixer>;

fn mk_sink<M: Mixer + 'static>(config: MixerConfig) -> Box<dyn Mixer> {
    Box::new(M::open(config))
}

pub const MIXERS: &[(&str, MixerFn)] = &[
    (SoftMixer::NAME, mk_sink::<SoftMixer>), // default goes first
    #[cfg(feature = "alsa-backend")]
    (AlsaMixer::NAME, mk_sink::<AlsaMixer>),
];

pub fn find(name: Option<&str>) -> Option<MixerFn> {
    if let Some(name) = name {
        MIXERS
            .iter()
            .find(|mixer| name == mixer.0)
            .map(|mixer| mixer.1)
    } else {
        MIXERS.first().map(|mixer| mixer.1)
    }
}
