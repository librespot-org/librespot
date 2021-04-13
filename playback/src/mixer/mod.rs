pub trait Mixer: Send {
    fn open(_: Option<MixerConfig>) -> Self
    where
        Self: Sized;
    fn start(&self);
    fn stop(&self);
    fn set_volume(&self, volume: u16);
    fn volume(&self) -> u16;
    fn get_audio_filter(&self) -> Option<Box<dyn AudioFilter + Send>> {
        None
    }
}

pub trait AudioFilter {
    fn modify_stream(&self, data: &mut [f32]);
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
    pub mapped_volume: bool,
    pub decrease_left_channel: i64,
    pub decrease_right_channel: i64,
}

impl Default for MixerConfig {
    fn default() -> MixerConfig {
        MixerConfig {
            card: String::from("default"),
            mixer: String::from("PCM"),
            index: 0,
            mapped_volume: true,
            decrease_left_channel: 0,
            decrease_right_channel: 0,
        }
    }
}

pub mod softmixer;
use self::softmixer::SoftMixer;

type MixerFn = fn(Option<MixerConfig>) -> Box<dyn Mixer>;

fn mk_sink<M: Mixer + 'static>(device: Option<MixerConfig>) -> Box<dyn Mixer> {
    Box::new(M::open(device))
}

pub fn find<T: AsRef<str>>(name: Option<T>) -> Option<MixerFn> {
    match name.as_ref().map(AsRef::as_ref) {
        None | Some("softvol") => Some(mk_sink::<SoftMixer>),
        #[cfg(feature = "alsa-backend")]
        Some("alsa") => Some(mk_sink::<AlsaMixer>),
        _ => None,
    }
}
