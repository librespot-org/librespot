pub trait Mixer: Send {
    fn open() -> Self
    where
        Self: Sized;
    fn start(&self);
    fn stop(&self);
    fn set_volume(&self, volume: u16);
    fn volume(&self) -> u16;
    fn get_audio_filter(&self) -> Option<Box<AudioFilter + Send>> {
        None
    }
    fn set_metadata_pipe(&mut self, _metadata_pipe: Option<String>) {}
}

pub trait AudioFilter {
    fn modify_stream(&self, data: &mut [i16]);
}

pub mod softmixer;
use self::softmixer::SoftMixer;

pub mod pipemixer;
use self::pipemixer::PipeMixer;

fn mk_sink<M: Mixer + 'static>() -> Box<Mixer> {
    Box::new(M::open())
}

pub fn find<T: AsRef<str>>(name: Option<T>) -> Option<fn() -> Box<Mixer>> {
    match name.as_ref().map(AsRef::as_ref) {
        None | Some("softvol") => Some(mk_sink::<SoftMixer>),
        Some("pipe") => Some(mk_sink::<PipeMixer>),
        _ => None,
    }
}
