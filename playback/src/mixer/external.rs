use super::{Mixer, MixerConfig};
use librespot_core::Error;
use portable_atomic::AtomicU16;
use std::sync::atomic::Ordering;

pub struct ExternalMixer {
    volume: AtomicU16,
}

impl Mixer for ExternalMixer {
    fn open(_config: MixerConfig) -> Result<Self, Error> {
        info!("Mixing with external volume control");

        Ok(Self {
            volume: AtomicU16::new(u16::MAX / 2),
        })
    }

    fn volume(&self) -> u16 {
        self.volume.load(Ordering::Relaxed)
    }

    fn set_volume(&self, volume: u16) {
        self.volume.store(volume, Ordering::Relaxed);
    }
}

impl ExternalMixer {
    pub const NAME: &'static str = "external";
}

#[cfg(test)]
mod tests {
    use super::ExternalMixer;
    use crate::mixer::{Mixer, MixerConfig};

    #[test]
    fn external_mixer_tracks_requested_volume() {
        let mixer = ExternalMixer::open(MixerConfig::default()).unwrap();

        for volume in [0, u16::MAX / 2, u16::MAX] {
            mixer.set_volume(volume);
            assert_eq!(mixer.volume(), volume);
        }
    }

    #[test]
    fn external_mixer_never_attenuates_audio() {
        let mixer = ExternalMixer::open(MixerConfig::default()).unwrap();

        for volume in [0, u16::MAX / 2, u16::MAX] {
            mixer.set_volume(volume);
            assert_eq!(mixer.get_soft_volume().attenuation_factor(), 1.0);
        }
    }
}
