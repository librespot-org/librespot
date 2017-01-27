use std::borrow::Cow;
use self::softmixer::SoftMixer;

pub mod softmixer;

pub trait Mixer {
    fn init(&self);
    fn start(&self);
    fn stop(&self);
    fn set_volume(&self, volume: u16);
    fn volume(&self) -> u16;
    fn get_audio_filter(&self) -> Option<Box<AudioFilter + Send>> {
        None
    }
}

pub trait AudioFilter {
  fn modify_stream<'a>(&self, data: &'a [i16]) -> Cow<'a, [i16]>;
}

pub fn find<T: AsRef<str>>(name: Option<T>) -> Option<Box<Mixer + Send>> {
  match name {
    _ => Some(Box::new(SoftMixer::new())),
  }
}