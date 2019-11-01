use super::AudioFilter;
use super::{Mixer, MixerConfig};
use std::error::Error;

use alsa;

#[derive(Clone)]
pub struct AlsaMixer {
    config: MixerConfig,
}

impl AlsaMixer {
    fn map_volume(&self, set_volume: Option<u16>) -> Result<(u16), Box<dyn Error>> {
        let mixer = alsa::mixer::Mixer::new(&self.config.card, false)?;
        let sid = alsa::mixer::SelemId::new(&*self.config.mixer, self.config.index);

        let selem = mixer.find_selem(&sid).expect(
            format!(
                "Couldn't find simple mixer control for {}",
                self.config.mixer
            )
            .as_str(),
        );
        let (min, max) = selem.get_playback_volume_range();
        let range = (max - min) as f64;

        let new_vol: u16;

        if let Some(vol) = set_volume {
            let alsa_volume: i64 = ((vol as f64 / 0xFFFF as f64) * range) as i64 + min;
            debug!("Mapping volume {:?} ->> alsa {:?}", vol, alsa_volume);
            selem
                .set_playback_volume_all(alsa_volume)
                .expect("Couldn't set alsa volume");
            new_vol = vol;
        } else {
            let cur_vol = selem
                .get_playback_volume(alsa::mixer::SelemChannelId::mono())
                .expect("Couldn't get current volume");
            new_vol = (((cur_vol - min) as f64 / range) * 0xFFFF as f64) as u16;
            debug!("Mapping volume {:?} <<- alsa {:?}", new_vol, cur_vol);
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
        AlsaMixer { config: config }
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
