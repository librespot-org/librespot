use super::Mixer;
use std::borrow::Cow;

pub struct SoftMixer {
    volume: u16,
}

impl SoftMixer {
    pub fn new() -> SoftMixer {
        SoftMixer {
            volume: 0xFFFF
        }
    }
}

impl Mixer for SoftMixer {
    fn init(&mut self) {
    }
    
    fn inuse(&mut self) {
    }

    fn release(&mut self) {
    }

    fn set_volume(&mut self, volume: u16) {
        self.volume = volume;
    }

    fn volume(&self) -> u16 {
        self.volume
    }
    fn apply_volume<'a>(&mut self, data: &'a [i16]) -> Cow<'a, [i16]> {
        if self.volume == 0xFFFF {
            Cow::Borrowed(data)
        } else {
            Cow::Owned(data.iter()
                        .map(|&x| {
                            (x as i32
                                * self.volume as i32
                                / 0xFFFF) as i16
                        })
                        .collect())
        }
    }
}