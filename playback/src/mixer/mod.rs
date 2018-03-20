pub trait Mixer: Send {
    fn open(Option<String>) -> Self
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

pub mod softmixer;
use self::softmixer::SoftMixer;

fn mk_sink<M: Mixer + 'static>(device: Option<String>) -> Box<Mixer> {
    Box::new(M::open(device))
}

pub fn find<T: AsRef<str>>(name: Option<T>) -> Option<fn(Option<String>) -> Box<Mixer>> {
    match name.as_ref().map(AsRef::as_ref) {
        None | Some("softvol") => Some(mk_sink::<SoftMixer>),
        #[cfg(feature = "alsa-backend")]
        Some("alsa") => Some(mk_sink::<AlsaMixer>),
        _ => None,
    }
}
