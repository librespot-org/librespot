use crate::config::VolumeCtrl;

pub mod mappings;
use self::mappings::MappedCtrl;

pub trait Mixer: Send {
    fn open(config: MixerConfig) -> Self
    where
        Self: Sized;

    fn set_volume(&self, volume: u16);
    fn volume(&self) -> u16;

    fn get_audio_filter(&self) -> Option<Box<dyn AudioFilter + Send>> {
        None
    }
}

pub trait AudioFilter {
    fn modify_stream(&self, data: &mut [f64]);
}

pub mod softmixer;
use self::softmixer::SoftMixer;

#[cfg(feature = "alsa-backend")]
pub mod alsamixer;
#[cfg(feature = "alsa-backend")]
use self::alsamixer::AlsaMixer;

#[derive(Debug, Clone)]
pub struct MixerConfig {
    pub card: String,
    pub control: String,
    pub index: u32,
    pub volume_ctrl: VolumeCtrl,
}

impl Default for MixerConfig {
    fn default() -> MixerConfig {
        MixerConfig {
            card: String::from("default"),
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

pub fn find(name: Option<&str>) -> Option<MixerFn> {
    match name {
        None | Some(SoftMixer::NAME) => Some(mk_sink::<SoftMixer>),
        #[cfg(feature = "alsa-backend")]
        Some(AlsaMixer::NAME) => Some(mk_sink::<AlsaMixer>),
        _ => None,
    }
}
