use super::AudioFilter;
use super::{Mixer, MixerConfig};
use std;
use std::error::Error;

use alsa;

#[derive(Clone)]
struct AlsaMixerVolumeParams {
    min: i64,
    max: i64,
    range: f64,
    min_db: alsa::mixer::MilliBel,
    max_db: alsa::mixer::MilliBel,
}

#[derive(Clone)]
pub struct AlsaMixer {
    config: MixerConfig,
    params: AlsaMixerVolumeParams,
}

impl AlsaMixer {
    fn pvol<T>(&self, vol: T, min: T, max: T) -> f64
    where
        T: std::ops::Sub + Copy,
        f64: std::convert::From<<T as std::ops::Sub>::Output>,
    {
        f64::from(vol - min) / f64::from(max - min)
    }

    fn init_mixer(mut config: MixerConfig) -> Result<AlsaMixer, Box<Error>> {
        let mixer = alsa::mixer::Mixer::new(&config.card, false)?;
        let sid = alsa::mixer::SelemId::new(&config.mixer, config.index);

        let selem = mixer.find_selem(&sid).expect("Couldn't find SelemId");
        let (min, max) = selem.get_playback_volume_range();
        let (min_db, max_db) = selem.get_playback_db_range();

        info!(
            "Alsa min: {} ({:?}[dB]) -- max: {} ({:?}[dB])",
            min, min_db, max, max_db
        );

        if config.mapped_volume && (max_db - min_db <= alsa::mixer::MilliBel(24)) {
            warn!(
                "Switching to linear volume mapping, control range: {:?}",
                max_db - min_db
            );
            config.mapped_volume = false;
        } else {
            info!("Using Alsa mapped volume: dB range: {:?}", max_db - min_db);
        }

        Ok(AlsaMixer {
            config: config,
            params: AlsaMixerVolumeParams {
                min: min,
                max: max,
                range: (max - min) as f64,
                min_db: min_db,
                max_db: max_db,
            },
        })
    }

    fn map_volume(&self, set_volume: Option<u16>) -> Result<(u16), Box<Error>> {
        let mixer = alsa::mixer::Mixer::new(&self.config.card, false)?;
        let sid = alsa::mixer::SelemId::new(&*self.config.mixer, self.config.index);

        let selem = mixer.find_selem(&sid).expect("Couldn't find SelemId");
        let cur_vol = selem
            .get_playback_volume(alsa::mixer::SelemChannelId::mono())
            .expect("Couldn't get current volume");
        let cur_vol_db = selem
            .get_playback_vol_db(alsa::mixer::SelemChannelId::mono())
            .expect("Couldn't get current volume");

        let new_vol: u16;
        debug!("Current alsa volume: {}[i64] {:?}", cur_vol, cur_vol_db);

        if let Some(vol) = set_volume {
            let alsa_volume = if self.config.mapped_volume {
                ((self.pvol(vol, 0x0000, 0xFFFF)).log10() * 6000.0).floor() as i64 + self.params.max
            } else {
                ((vol as f64 / 0xFFFF as f64) * self.params.range) as i64 + self.params.min
            };
            debug!(
                "Maping volume [{:.3}%] {:?} [u16] ->> Alsa [{:.3}%] {:?} [i64]",
                self.pvol(vol, 0x0000, 0xFFFF) * 100.0,
                vol,
                self.pvol(
                    alsa_volume as f64,
                    self.params.min as f64,
                    self.params.max as f64
                ) * 100.0,
                alsa_volume
            );
            selem
                .set_playback_volume_all(alsa_volume)
                .expect("Couldn't set alsa volume");
            new_vol = vol; // Meh
        } else {
            new_vol =
                (((cur_vol - self.params.min) as f64 / self.params.range) * 0xFFFF as f64) as u16;
            debug!(
                "Maping volume [{:.3}%] {:?} [u16] <<- Alsa [{:.3}%] {:?} [i64]",
                self.pvol(new_vol, 0x0000, 0xFFFF),
                new_vol,
                self.pvol(
                    cur_vol as f64,
                    self.params.min as f64,
                    self.params.max as f64
                ),
                cur_vol
            );
        }

        Ok(new_vol)
    }
}

impl Mixer for AlsaMixer {
    fn open(config: Option<MixerConfig>) -> AlsaMixer {
        let config = config.unwrap_or_default();
        info!(
            "Setting up new mixer: card:{} mixer:{} index:{}",
            config.card, config.mixer, config.index
        );
        AlsaMixer::init_mixer(config).expect("Error setting up mixer!")
    }

    fn start(&self) {}

    fn stop(&self) {}

    fn volume(&self) -> u16 {
        match self.map_volume(None) {
            Ok(vol) => vol,
            Err(e) => {
                error!("Error getting volume for <{}>, {:?}", self.config.card, e);
                0
            }
        }
    }

    fn set_volume(&self, volume: u16) {
        match self.map_volume(Some(volume)) {
            Ok(_) => (),
            Err(e) => error!("Error setting volume for <{}>, {:?}", self.config.card, e),
        }
    }

    fn get_audio_filter(&self) -> Option<Box<dyn AudioFilter + Send>> {
        None
    }
}
