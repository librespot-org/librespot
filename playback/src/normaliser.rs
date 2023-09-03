use crate::{
    config::{NormalisationMethod, NormalisationType, PlayerConfig},
    db_to_ratio,
    decoder::AudioPacket,
    mixer::VolumeGetter,
    player::NormalisationData,
    ratio_to_db, PCM_AT_0DBFS,
};

#[derive(PartialEq)]
struct DynamicNormalisation {
    threshold_db: f64,
    attack_cf: f64,
    release_cf: f64,
    knee_db: f64,
    integrator: f64,
    peak: f64,
}

impl DynamicNormalisation {
    fn new(config: &PlayerConfig) -> Self {
        // as_millis() has rounding errors (truncates)
        let attack = config
            .sample_rate
            .normalisation_coefficient_to_duration(config.normalisation_attack_cf)
            .as_secs_f64()
            * 1000.0;

        debug!("Normalisation Attack: {:.0} ms", attack);

        let release = config
            .sample_rate
            .normalisation_coefficient_to_duration(config.normalisation_release_cf)
            .as_secs_f64()
            * 1000.0;

        debug!("Normalisation Release: {:.0} ms", release);

        Self {
            threshold_db: config.normalisation_threshold_dbfs,
            attack_cf: config.normalisation_attack_cf,
            release_cf: config.normalisation_release_cf,
            knee_db: config.normalisation_knee_db,
            integrator: 0.0,
            peak: 0.0,
        }
    }

    fn stop(&mut self) {
        self.integrator = 0.0;
        self.peak = 0.0;
    }

    fn normalise(&mut self, mut samples: Vec<f64>, volume: f64, factor: f64) -> Vec<f64> {
        samples.iter_mut().for_each(|sample| {
            *sample *= factor;

            // Feedforward limiter in the log domain
            // After: Giannoulis, D., Massberg, M., & Reiss, J.D. (2012). Digital Dynamic
            // Range Compressor Design—A Tutorial and Analysis. Journal of The Audio
            // Engineering Society, 60, 399-408.

            // Some tracks have samples that are precisely 0.0. That's silence
            // and we know we don't need to limit that, in which we can spare
            // the CPU cycles.
            //
            // Also, calling `ratio_to_db(0.0)` returns `inf` and would get the
            // peak detector stuck. Also catch the unlikely case where a sample
            // is decoded as `NaN` or some other non-normal value.
            let limiter_db = if sample.is_normal() {
                // step 1-4: half-wave rectification and conversion into dB
                // and gain computer with soft knee and subtractor
                let bias_db = ratio_to_db(sample.abs()) - self.threshold_db;
                let knee_boundary_db = bias_db * 2.0;

                if knee_boundary_db < -self.knee_db {
                    0.0
                } else if knee_boundary_db.abs() <= self.knee_db {
                    // The textbook equation:
                    // ratio_to_db(sample.abs()) - (ratio_to_db(sample.abs()) - (bias_db + knee_db / 2.0).powi(2) / (2.0 * knee_db))
                    // Simplifies to:
                    // ((2.0 * bias_db) + knee_db).powi(2) / (8.0 * knee_db)
                    // Which in our case further simplifies to:
                    // (knee_boundary_db + knee_db).powi(2) / (8.0 * knee_db)
                    // because knee_boundary_db is 2.0 * bias_db.
                    (knee_boundary_db + self.knee_db).powi(2) / (8.0 * self.knee_db)
                } else {
                    // Textbook:
                    // ratio_to_db(sample.abs()) - threshold_db, which is already our bias_db.
                    bias_db
                }
            } else {
                0.0
            };

            // Spare the CPU unless (1) the limiter is engaged, (2) we
            // were in attack or (3) we were in release, and that attack/
            // release wasn't finished yet.
            if limiter_db > 0.0 || self.integrator > 0.0 || self.peak > 0.0 {
                // step 5: smooth, decoupled peak detector
                // Textbook:
                // release_cf * integrator + (1.0 - release_cf) * limiter_db
                // Simplifies to:
                // release_cf * integrator - release_cf * limiter_db + limiter_db
                self.integrator = limiter_db.max(
                    self.release_cf * self.integrator - self.release_cf * limiter_db + limiter_db,
                );
                // Textbook:
                // attack_cf * peak + (1.0 - attack_cf) * integrator
                // Simplifies to:
                // attack_cf * peak - attack_cf * integrator + integrator
                self.peak =
                    self.attack_cf * self.peak - self.attack_cf * self.integrator + self.integrator;

                // step 6: make-up gain applied later (volume attenuation)
                // Applying the standard normalisation factor here won't work,
                // because there are tracks with peaks as high as 6 dB above
                // the default threshold, so that would clip.

                // steps 7-8: conversion into level and multiplication into gain stage
                *sample *= db_to_ratio(-self.peak);
            }

            *sample *= volume
        });

        samples
    }
}

#[derive(PartialEq)]
enum Normalisation {
    None,
    Basic,
    Dynamic(DynamicNormalisation),
}

impl Normalisation {
    fn new(config: &PlayerConfig) -> Self {
        if !config.normalisation {
            Normalisation::None
        } else {
            debug!("Normalisation Type: {:?}", config.normalisation_type);
            debug!(
                "Normalisation Pregain: {:.1} dB",
                config.normalisation_pregain_db
            );

            debug!(
                "Normalisation Threshold: {:.1} dBFS",
                config.normalisation_threshold_dbfs
            );

            debug!("Normalisation Method: {:?}", config.normalisation_method);

            match config.normalisation_method {
                NormalisationMethod::Dynamic => {
                    Normalisation::Dynamic(DynamicNormalisation::new(config))
                }
                NormalisationMethod::Basic => Normalisation::Basic,
            }
        }
    }

    fn stop(&mut self) {
        if let Normalisation::Dynamic(ref mut d) = self {
            d.stop()
        }
    }

    fn normalise(&mut self, mut samples: Vec<f64>, volume: f64, factor: f64) -> Vec<f64> {
        use Normalisation::*;

        match self {
            None => {
                // We only care about volume.
                // We don't care about factor.
                // volume: 0.0 - 1.0
                if volume < 1.0 {
                    // for each sample: sample = sample * volume
                    samples.iter_mut().for_each(|sample| *sample *= volume);
                }

                samples
            }
            Basic => {
                // We care about both volume and factor.
                // volume: 0.0 - 1.0
                // factor: 0.0 - 1.0
                if volume < 1.0 || factor < 1.0 {
                    // for each sample: sample = sample * volume * factor
                    samples
                        .iter_mut()
                        .for_each(|sample| *sample *= volume * factor);
                }

                samples
            }
            // We don't care about anything, DynamicNormalisation does that for us.
            Dynamic(ref mut d) => d.normalise(samples, volume, factor),
        }
    }
}

pub struct Normaliser {
    normalisation: Normalisation,
    volume_getter: Box<dyn VolumeGetter>,
    normalisation_type: NormalisationType,
    pregain_db: f64,
    threshold_dbfs: f64,
    factor: f64,
}

impl Normaliser {
    pub fn new(config: &PlayerConfig, volume_getter: Box<dyn VolumeGetter>) -> Self {
        Self {
            normalisation: Normalisation::new(config),
            volume_getter,
            normalisation_type: config.normalisation_type,
            pregain_db: config.normalisation_pregain_db,
            threshold_dbfs: config.normalisation_threshold_dbfs,
            factor: 0.0,
        }
    }

    pub fn normalise(&mut self, samples: Vec<f64>) -> AudioPacket {
        let volume = self.volume_getter.attenuation_factor();

        AudioPacket::Samples(self.normalisation.normalise(samples, volume, self.factor))
    }

    pub fn stop(&mut self) {
        self.factor = 0.0;
        self.normalisation.stop();
    }

    pub fn update_normalisation_data(
        &mut self,
        auto_normalise_as_album: bool,
        data: NormalisationData,
    ) {
        // Normalisation::None doesn't use the factor,
        // so there is no need to waste the time calculating it.
        if self.normalisation != Normalisation::None {
            self.factor = self.get_factor(auto_normalise_as_album, data);
        }
    }

    fn get_factor(&self, auto_normalise_as_album: bool, data: NormalisationData) -> f64 {
        let (gain_db, gain_peak, norm_type) = match self.normalisation_type {
            NormalisationType::Album => (
                data.album_gain_db,
                data.album_peak,
                NormalisationType::Album,
            ),
            NormalisationType::Track => (
                data.track_gain_db,
                data.track_peak,
                NormalisationType::Track,
            ),
            NormalisationType::Auto => {
                if auto_normalise_as_album {
                    (
                        data.album_gain_db,
                        data.album_peak,
                        NormalisationType::Album,
                    )
                } else {
                    (
                        data.track_gain_db,
                        data.track_peak,
                        NormalisationType::Track,
                    )
                }
            }
        };

        // As per the ReplayGain 1.0 & 2.0 (proposed) spec:
        // https://wiki.hydrogenaud.io/index.php?title=ReplayGain_1.0_specification#Clipping_prevention
        // https://wiki.hydrogenaud.io/index.php?title=ReplayGain_2.0_specification#Clipping_prevention
        let normalisation_factor = if self.normalisation == Normalisation::Basic {
            // For Basic Normalisation, factor = min(ratio of (ReplayGain + PreGain), 1.0 / peak level).
            // https://wiki.hydrogenaud.io/index.php?title=ReplayGain_1.0_specification#Peak_amplitude
            // https://wiki.hydrogenaud.io/index.php?title=ReplayGain_2.0_specification#Peak_amplitude
            // We then limit that to 1.0 as not to exceed dBFS (0.0 dB).
            let factor = f64::min(
                db_to_ratio(gain_db + self.pregain_db),
                PCM_AT_0DBFS / gain_peak,
            );

            if factor > PCM_AT_0DBFS {
                info!(
                    "Lowering gain by {:.2} dB for the duration of this track to avoid potentially exceeding dBFS.",
                    ratio_to_db(factor)
                );

                PCM_AT_0DBFS
            } else {
                factor
            }
        } else {
            // For Dynamic Normalisation it's up to the player to decide,
            // factor = ratio of (ReplayGain + PreGain).
            // We then let the dynamic limiter handle gain reduction.
            let factor = db_to_ratio(gain_db + self.pregain_db);
            let threshold_ratio = db_to_ratio(self.threshold_dbfs);

            if factor > PCM_AT_0DBFS {
                let factor_db = gain_db + self.pregain_db;
                let limiting_db = factor_db + self.threshold_dbfs.abs();

                warn!(
                    "This track may exceed dBFS by {:.2} dB and be subject to {:.2} dB of dynamic limiting at it's peak.",
                    factor_db, limiting_db
                );
            } else if factor > threshold_ratio {
                let limiting_db = gain_db + self.pregain_db + self.threshold_dbfs.abs();

                info!(
                    "This track may be subject to {:.2} dB of dynamic limiting at it's peak.",
                    limiting_db
                );
            }

            factor
        };

        debug!("Normalisation Data: {:?}", data);
        debug!("Normalisation Type: {:?}", self.normalisation_type);
        debug!(
            "Calculated Normalisation Factor for {:?}: {:.2}%",
            norm_type,
            normalisation_factor * 100.0
        );

        normalisation_factor
    }
}
