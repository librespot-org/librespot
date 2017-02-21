pub trait Mixer {
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

pub mod softmixer;
use self::softmixer::SoftMixer;

pub fn find<T: AsRef<str>>(name: Option<T>) -> Option<Box<Mixer + Send>> {
    match name.as_ref().map(AsRef::as_ref) {
        None | Some("softvol") => Some(Box::new(SoftMixer::new())),
        _ => None,
    }
}
