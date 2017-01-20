use std::borrow::Cow;

pub mod softmixer;

pub trait Mixer {
    fn init(&mut self);
    fn inuse(&mut self);
    fn release(&mut self);
    fn set(&mut self, volume: u16);
    fn volume(&self) -> u16;
    fn apply_volume<'a>(&mut self, data: &'a [i16]) -> Cow<'a, [i16]> {
        Cow::Borrowed(data)
    }
}