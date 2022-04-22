use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use super::SoftVolume;
use super::{MappedCtrl, VolumeCtrl};
use super::{Mixer, MixerConfig};

#[derive(Clone)]
pub struct SoftMixer {
    // There is no AtomicF64, so we store the f64 as bits in a u64 field.
    // It's much faster than a Mutex<f64>.
    volume: Arc<AtomicU64>,
    volume_ctrl: VolumeCtrl,
}

impl Mixer for SoftMixer {
    fn open(config: MixerConfig) -> Self {
        let volume_ctrl = config.volume_ctrl;
        info!("Mixing with softvol and volume control: {:?}", volume_ctrl);

        Self {
            volume: Arc::new(AtomicU64::new(f64::to_bits(0.5))),
            volume_ctrl,
        }
    }

    fn volume(&self) -> u16 {
        let mapped_volume = f64::from_bits(self.volume.load(Ordering::Relaxed));
        self.volume_ctrl.from_mapped(mapped_volume)
    }

    fn set_volume(&self, volume: u16) {
        let mapped_volume = self.volume_ctrl.to_mapped(volume);
        self.volume
            .store(mapped_volume.to_bits(), Ordering::Relaxed)
    }

    fn get_soft_volume(&self) -> Option<Box<dyn SoftVolume + Send>> {
        Some(Box::new(SoftVolumeApplier {
            volume: self.volume.clone(),
        }))
    }
}

impl SoftMixer {
    pub const NAME: &'static str = "softvol";
}

struct SoftVolumeApplier {
    volume: Arc<AtomicU64>,
}

impl SoftVolume for SoftVolumeApplier {
    fn attenuation_factor(&self) -> f64 {
        f64::from_bits(self.volume.load(Ordering::Relaxed))
    }
}
