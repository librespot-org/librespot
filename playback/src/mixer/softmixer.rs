use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use super::AudioFilter;
use super::{MappedCtrl, VolumeCtrl};
use super::{Mixer, MixerConfig};

#[derive(Clone)]
pub struct SoftMixer {
    // There is no AtomicF32, so we store the f32 as bits in a u32 field.
    // It's much faster than a Mutex<f32>.
    volume: Arc<AtomicU32>,
    volume_ctrl: VolumeCtrl,
}

impl Mixer for SoftMixer {
    fn open(config: MixerConfig) -> Self {
        let volume_ctrl = config.volume_ctrl;
        info!("Mixing with softvol and volume control: {:?}", volume_ctrl);

        Self {
            volume: Arc::new(AtomicU32::new(f32::to_bits(0.5))),
            volume_ctrl,
        }
    }

    fn volume(&self) -> u16 {
        let mapped_volume = f32::from_bits(self.volume.load(Ordering::Relaxed));
        self.volume_ctrl.from_mapped(mapped_volume)
    }

    fn set_volume(&self, volume: u16) {
        let mapped_volume = self.volume_ctrl.to_mapped(volume);
        self.volume
            .store(mapped_volume.to_bits(), Ordering::Relaxed)
    }

    fn get_audio_filter(&self) -> Option<Box<dyn AudioFilter + Send>> {
        Some(Box::new(SoftVolumeApplier {
            volume: self.volume.clone(),
        }))
    }
}

struct SoftVolumeApplier {
    volume: Arc<AtomicU32>,
}

impl AudioFilter for SoftVolumeApplier {
    fn modify_stream(&self, data: &mut [f32]) {
        let volume = f32::from_bits(self.volume.load(Ordering::Relaxed));
        if volume < 1.0 {
            for x in data.iter_mut() {
                *x = (*x as f64 * volume as f64) as f32;
            }
        }
    }
}
