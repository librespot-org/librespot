use super::Mixer;
use super::StreamEditor;
use std::borrow::Cow;
use std::sync::{Arc, RwLock};

pub struct SoftMixer {
  volume: Arc<RwLock<u16>>
}

impl SoftMixer {
    pub fn new() -> SoftMixer {
        SoftMixer {
            volume: Arc::new(RwLock::new(0xFFFF))
        }
    }
}

impl Mixer for SoftMixer {
    fn init(&self) {
    }
    fn start(&self) {
    }
    fn stop(&self) {
    }
    fn volume(&self) -> u16 {
        *self.volume.read().unwrap()
    }
    fn set_volume(&self, volume: u16) {
        *self.volume.write().unwrap() = volume;
    }
    fn get_stream_editor(&self) -> Option<Box<StreamEditor + Send>> {
        let vol = self.volume.clone();
        Some(Box::new(SoftVolumeApplier { get_volume: Box::new(move || *vol.read().unwrap() ) }))
    }
}

struct SoftVolumeApplier {
  get_volume: Box<Fn() -> u16 + Send>
}

impl StreamEditor for SoftVolumeApplier {
    fn modify_stream<'a>(&self, data: &'a [i16]) -> Cow<'a, [i16]> {
        let volume = (self.get_volume)();
        if volume == 0xFFFF {
            Cow::Borrowed(data)
        } else {
            Cow::Owned(data.iter()
                        .map(|&x| {
                            (x as i32
                                * volume as i32
                                / 0xFFFF) as i16
                        })
                        .collect())
        }
    }
}