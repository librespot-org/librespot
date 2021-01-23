use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use librespot_core::volume::Volume;

use super::AudioFilter;
use super::{Mixer, MixerConfig};

#[derive(Clone)]
pub struct SoftMixer {
    volume: Arc<AtomicUsize>,
}

impl Mixer for SoftMixer {
    fn open(_: Option<MixerConfig>) -> SoftMixer {
        SoftMixer {
            volume: Arc::new(AtomicUsize::new(0xFFFF)),
        }
    }
    fn start(&self) {}
    fn stop(&self) {}
    fn volume(&self) -> u16 {
        self.volume.load(Ordering::Relaxed) as u16
    }
    fn set_volume(&self, volume: Volume) {
        self.volume.store(volume.0 as usize, Ordering::Relaxed);
    }
    fn get_audio_filter(&self) -> Option<Box<dyn AudioFilter + Send>> {
        Some(Box::new(SoftVolumeApplier {
            volume: self.volume.clone(),
        }))
    }
}

struct SoftVolumeApplier {
    volume: Arc<AtomicUsize>,
}

impl AudioFilter for SoftVolumeApplier {
    fn modify_stream(&self, data: &mut [i16]) {
        let volume = self.volume.load(Ordering::Relaxed) as u16;
        if volume != 0xFFFF {
            for x in data.iter_mut() {
                *x = (*x as i32 * volume as i32 / 0xFFFF) as i16;
            }
        }
    }
}
