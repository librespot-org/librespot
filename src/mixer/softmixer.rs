use std::borrow::Cow;
use std::sync::{Arc, RwLock};

use spirc::UpdateMessageSender;
use spirc::UpdateMessage;

use super::Mixer;
use super::AudioFilter;

pub struct SoftMixer {
  volume: Arc<RwLock<u16>>,
  tx: Option<UpdateMessageSender>
}

impl SoftMixer {
    pub fn new() -> SoftMixer {
        SoftMixer {
            volume: Arc::new(RwLock::new(0xFFFF)),
            tx: None
        }
    }
}

impl Mixer for SoftMixer {
    fn init(&mut self, tx: UpdateMessageSender) {
        self.tx = Some(tx);
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
        let tx = self.tx.as_ref().expect("SoftMixer not initialized");
        tx.send(UpdateMessage).unwrap();
    }
    fn get_audio_filter(&self) -> Option<Box<AudioFilter + Send>> {
        let vol = self.volume.clone();
        let get_volume = Box::new(move || *vol.read().unwrap());
        Some(Box::new(SoftVolumeApplier { get_volume: get_volume }))
    }
}

struct SoftVolumeApplier {
  get_volume: Box<Fn() -> u16 + Send>
}

impl AudioFilter for SoftVolumeApplier {
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