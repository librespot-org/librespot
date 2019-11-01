pub trait Mixer: Send {
    fn open(Option<MixerConfig>) -> Self
    where
        Self: Sized;
    fn start(&self);
    fn stop(&self);
    fn set_volume(&self, volume: u16);
    fn volume(&self) -> u16;
    fn get_audio_filter(&self) -> Option<Box<AudioFilter + Send>> {
        None
    }
}

pub trait AudioFilter {
    fn modify_stream(&self, data: &mut [i16]);
}

#[cfg(feature = "alsa-backend")]
pub mod alsamixer;
#[cfg(feature = "alsa-backend")]
use self::alsamixer::AlsaMixer;

#[derive(Debug, Clone)]
pub struct MixerConfig {
    pub card: String,
    pub mixer: String,
    pub index: u32,
}

impl Default for MixerConfig {
    fn default() -> MixerConfig { MixerConfig {
        card: String::from("default"),
        mixer: String::from("PCM"),
        index: 0,
        }
    }
}

pub mod softmixer;
use self::softmixer::SoftMixer;

fn mk_sink<M: Mixer + 'static>(device: Option<MixerConfig>) -> Box<Mixer> {
    Box::new(M::open(device))
}

pub fn find<T: AsRef<str>>(name: Option<T>) -> Option<fn(Option<MixerConfig>) -> Box<Mixer>> {
    match name.as_ref().map(AsRef::as_ref) {
        None | Some("softvol") => Some(mk_sink::<SoftMixer>),
        #[cfg(feature = "alsa-backend")]
        Some("alsa") => Some(mk_sink::<AlsaMixer>),
        _ => None,
    }
}
