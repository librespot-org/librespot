use std::borrow::Cow;

pub mod softmixer;

pub trait Mixer {
    fn init(&self);
    fn start(&self);
    fn stop(&self);
    fn set_volume(&self, volume: u16);
    fn volume(&self) -> u16;
    fn get_stream_editor(&self) -> Option<Box<StreamEditor + Send>>
    {
        None
    }
}

pub trait StreamEditor {
  fn modify_stream<'a>(&self, data: &'a [i16]) -> Cow<'a, [i16]>;
}

pub fn find(s: &str) -> Option<Box<Mixer + Send>> {
  match s {
    "SoftMixer" => Some(Box::new(softmixer::SoftMixer::new())),
    _ => None,
  }
}