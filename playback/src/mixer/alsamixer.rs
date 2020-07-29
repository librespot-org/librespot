use super::AudioFilter;
use super::{Mixer, MixerConfig};
use std;
use std::error::Error;

use alsa;

const SND_CTL_TLV_DB_GAIN_MUTE: i64 = -9999999;

#[derive(Clone)]
struct AlsaMixerVolumeParams {
    min: i64,
    max: i64,
    range: f64,
    min_db: alsa::mixer::MilliBel,
    max_db: alsa::mixer::MilliBel,
    has_switch: bool,
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

    fn init_mixer(mut config: MixerConfig) -> Result<AlsaMixer, Box<dyn Error>> {
        let mixer = alsa::mixer::Mixer::new(&config.card, false)?;
        let sid = alsa::mixer::SelemId::new(&config.mixer, config.index);

        let selem = mixer.find_selem(&sid).expect(
            format!(
                "Couldn't find simple mixer control for {},{}",
                &config.mixer, &config.index,
            )
            .as_str(),
        );
        let (min, max) = selem.get_playback_volume_range();
        let (min_db, max_db) = selem.get_playback_db_range();
        let hw_mix = selem
            .get_playback_vol_db(alsa::mixer::SelemChannelId::mono())
            .is_ok();
        let has_switch = selem.has_playback_switch();
        if min_db != alsa::mixer::MilliBel(SND_CTL_TLV_DB_GAIN_MUTE) {
            warn!("Alsa min-db is not SND_CTL_TLV_DB_GAIN_MUTE!!");
        }
        info!(
            "Alsa Mixer info min: {} ({:?}[dB]) -- max: {} ({:?}[dB]) HW: {:?}",
            min, min_db, max, max_db, hw_mix
        );

        if config.mapped_volume && (max_db - min_db <= alsa::mixer::MilliBel(24)) {
            warn!(
                "Switching to linear volume mapping, control range: {:?}",
                max_db - min_db
            );
            config.mapped_volume = false;
        } else if !config.mapped_volume {
            info!("Using Alsa linear volume");
        }

        if min_db != alsa::mixer::MilliBel(SND_CTL_TLV_DB_GAIN_MUTE) {
            debug!("Alsa min-db is not SND_CTL_TLV_DB_GAIN_MUTE!!");
        }

        Ok(AlsaMixer {
            config: config,
            params: AlsaMixerVolumeParams {
                min: min,
                max: max,
                range: (max - min) as f64,
                min_db: min_db,
                max_db: max_db,
                has_switch: has_switch,
            },
        })
    }

    fn map_volume(&self, set_volume: Option<u16>) -> Result<u16, Box<dyn Error>> {
        let mixer = alsa::mixer::Mixer::new(&self.config.card, false)?;
        let sid = alsa::mixer::SelemId::new(&*self.config.mixer, self.config.index);

        let selem = mixer.find_selem(&sid).unwrap();
        let cur_vol = selem
            .get_playback_volume(alsa::mixer::SelemChannelId::mono())
            .expect("Couldn't get current volume");
        let cur_vol_db = selem
            .get_playback_vol_db(alsa::mixer::SelemChannelId::mono())
            .unwrap_or(alsa::mixer::MilliBel(-SND_CTL_TLV_DB_GAIN_MUTE));

        let mut new_vol: u16 = 0;
        trace!("Current alsa volume: {}{:?}", cur_vol, cur_vol_db);

        match set_volume {
            Some(vol) => {
                if self.params.has_switch {
                    let is_muted = selem
                        .get_playback_switch(alsa::mixer::SelemChannelId::mono())
                        .map(|b| b == 0)
                        .unwrap_or(false);
                    if vol == 0 {
                        debug!("Toggling mute::True");
                        selem.set_playback_switch_all(0).expect("Can't switch mute");

                        return Ok(vol);
                    } else if is_muted {
                        debug!("Toggling mute::False");
                        selem.set_playback_switch_all(1).expect("Can't reset mute");
                    }
                }

                if self.config.mapped_volume {
                    // Cubic mapping ala alsamixer
                    // https://linux.die.net/man/1/alsamixer
                    // In alsamixer, the volume is mapped to a value that is more natural for a
                    // human ear. The mapping is designed so that the position in the interval is
                    // proportional to the volume as a human ear would perceive it, i.e. the
                    // position is the cubic root of the linear sample multiplication factor. For
                    // controls with a small range (24 dB or less), the mapping is linear in the dB
                    // values so that each step has the same size visually. TODO
                    // TODO: Check if min is not mute!
                    let vol_db = (self.pvol(vol, 0x0000, 0xFFFF).log10() * 6000.0).floor() as i64
                        + self.params.max_db.0;
                    selem
                        .set_playback_db_all(alsa::mixer::MilliBel(vol_db), alsa::Round::Floor)
                        .expect("Couldn't set alsa dB volume");
                    debug!(
                        "Mapping volume [{:.3}%] {:?} [u16] ->> Alsa [{:.3}%] {:?} [dB] - {} [i64]",
                        self.pvol(vol, 0x0000, 0xFFFF) * 100.0,
                        vol,
                        self.pvol(
                            vol_db as f64,
                            self.params.min as f64,
                            self.params.max as f64
                        ) * 100.0,
                        vol_db as f64 / 100.0,
                        vol_db
                    );
                } else {
                    // Linear mapping
                    let alsa_volume =
                        ((vol as f64 / 0xFFFF as f64) * self.params.range) as i64 + self.params.min;
                    selem
                        .set_playback_volume_all(alsa_volume)
                        .expect("Couldn't set alsa raw volume");
                    debug!(
                        "Mapping volume [{:.3}%] {:?} [u16] ->> Alsa [{:.3}%] {:?} [i64]",
                        self.pvol(vol, 0x0000, 0xFFFF) * 100.0,
                        vol,
                        self.pvol(
                            alsa_volume as f64,
                            self.params.min as f64,
                            self.params.max as f64
                        ) * 100.0,
                        alsa_volume
                    );
                };
            }
            None => {
                new_vol = (((cur_vol - self.params.min) as f64 / self.params.range) * 0xFFFF as f64)
                    as u16;
                debug!(
                    "Mapping volume [{:.3}%] {:?} [u16] <<- Alsa [{:.3}%] {:?} [i64]",
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
