use std::{mem, str::FromStr, time::Duration};

pub use crate::dither::{mk_ditherer, DithererBuilder, TriangularDitherer};
use crate::{convert::i24, RESAMPLER_INPUT_SIZE, SAMPLE_RATE};

// Reciprocals allow us to multiply instead of divide during interpolation.
const HZ48000_RESAMPLE_FACTOR_RECIPROCAL: f64 = SAMPLE_RATE as f64 / 48_000.0;
const HZ88200_RESAMPLE_FACTOR_RECIPROCAL: f64 = SAMPLE_RATE as f64 / 88_200.0;
const HZ96000_RESAMPLE_FACTOR_RECIPROCAL: f64 = SAMPLE_RATE as f64 / 96_000.0;

// sample rate * channels
const HZ44100_SAMPLES_PER_SECOND: f64 = 44_100.0 * 2.0;
const HZ48000_SAMPLES_PER_SECOND: f64 = 48_000.0 * 2.0;
const HZ88200_SAMPLES_PER_SECOND: f64 = 88_200.0 * 2.0;
const HZ96000_SAMPLES_PER_SECOND: f64 = 96_000.0 * 2.0;

// Given a RESAMPLER_INPUT_SIZE of 147 all of our output sizes work out
// to be integers, which is a very good thing. That means no fractional samples
// which translates to much better interpolation.
const HZ48000_INTERPOLATION_OUTPUT_SIZE: usize =
    (RESAMPLER_INPUT_SIZE as f64 * (1.0 / HZ48000_RESAMPLE_FACTOR_RECIPROCAL)) as usize;

const HZ88200_INTERPOLATION_OUTPUT_SIZE: usize =
    (RESAMPLER_INPUT_SIZE as f64 * (1.0 / HZ88200_RESAMPLE_FACTOR_RECIPROCAL)) as usize;

const HZ96000_INTERPOLATION_OUTPUT_SIZE: usize =
    (RESAMPLER_INPUT_SIZE as f64 * (1.0 / HZ96000_RESAMPLE_FACTOR_RECIPROCAL)) as usize;

pub const NUM_FIR_FILTER_TAPS: usize = 5;

// Blackman Window coefficients
const BLACKMAN_A0: f64 = 0.42;
const BLACKMAN_A1: f64 = 0.5;
const BLACKMAN_A2: f64 = 0.08;

// Constants for calculations
const TWO_TIMES_PI: f64 = 2.0 * std::f64::consts::PI;
const FOUR_TIMES_PI: f64 = 4.0 * std::f64::consts::PI;

#[derive(Clone, Copy, Debug, Default)]
pub enum InterpolationQuality {
    Low,
    Medium,
    #[default]
    High,
}

impl FromStr for InterpolationQuality {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InterpolationQuality::*;

        match s.to_lowercase().as_ref() {
            "low" => Ok(Low),
            "medium" => Ok(Medium),
            "high" => Ok(High),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for InterpolationQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use InterpolationQuality::*;

        match self {
            Low => write!(f, "Low"),
            Medium => write!(f, "Medium"),
            High => write!(f, "High"),
        }
    }
}

impl InterpolationQuality {
    pub fn get_interpolation_coefficients(&self, resample_factor_reciprocal: f64) -> Vec<f64> {
        let interpolation_coefficients_length = self.get_interpolation_coefficients_length();

        let mut coefficients = Vec::with_capacity(interpolation_coefficients_length);

        if interpolation_coefficients_length == 0 {
            warn!("InterpolationQuality::Low::get_interpolation_coefficients always returns an empty Vec<f64>");
            warn!("Linear Interpolation does not use coefficients");

            return coefficients;
        }

        let last_index = interpolation_coefficients_length as f64 - 1.0;

        let sinc_center = last_index * 0.5;

        let mut coefficient_sum = 0.0;

        coefficients.extend((0..interpolation_coefficients_length).map(
            |interpolation_coefficient_index| {
                let index_float = interpolation_coefficient_index as f64;
                let sample_index_fractional = (index_float * resample_factor_reciprocal).fract();

                let sample_index_fractional_sinc_weight = Self::sinc(sample_index_fractional);

                let fir_filter = Self::fir_filter(
                    index_float,
                    last_index,
                    sinc_center,
                    resample_factor_reciprocal,
                );

                let coefficient = sample_index_fractional_sinc_weight * fir_filter;

                coefficient_sum += coefficient;

                coefficient
            },
        ));

        coefficients
            .iter_mut()
            .for_each(|coefficient| *coefficient /= coefficient_sum);

        coefficients
    }

    pub fn get_fir_filter_coefficients(&self, resample_factor_reciprocal: f64) -> Vec<f64> {
        let mut coefficients = Vec::with_capacity(NUM_FIR_FILTER_TAPS);

        if self.get_interpolation_coefficients_length() != 0 {
            warn!("InterpolationQuality::Medium/High::get_fir_filter_coefficients always returns an empty Vec<f64>");
            warn!("The FIR Filter coefficients are a part of the Windowed Sinc Interpolation coefficients");

            return coefficients;
        }

        let last_index = NUM_FIR_FILTER_TAPS as f64 - 1.0;

        let sinc_center = last_index * 0.5;

        let mut coefficient_sum = 0.0;

        coefficients.extend(
            (0..NUM_FIR_FILTER_TAPS).map(|fir_filter_coefficient_index| {
                let coefficient = Self::fir_filter(
                    fir_filter_coefficient_index as f64,
                    last_index,
                    sinc_center,
                    resample_factor_reciprocal,
                );

                coefficient_sum += coefficient;

                coefficient
            }),
        );

        coefficients
            .iter_mut()
            .for_each(|coefficient| *coefficient /= coefficient_sum);

        coefficients
    }

    pub fn get_interpolation_coefficients_length(&self) -> usize {
        use InterpolationQuality::*;
        match self {
            Low => 0,
            Medium => 129,
            High => 257,
        }
    }

    fn sinc(x: f64) -> f64 {
        if x.abs() < f64::EPSILON {
            1.0
        } else {
            let pi_x = std::f64::consts::PI * x;
            pi_x.sin() / pi_x
        }
    }

    fn blackman(index: f64, last_index: f64) -> f64 {
        // Calculate the Blackman window function for the given center offset
        // w(n) = A0 - A1*cos(2πn / (N-1)) + A2*cos(4πn / (N-1)),
        // where n is the center offset, N is the window size,
        // and A0, A1, A2 are precalculated coefficients
        let two_pi_n = TWO_TIMES_PI * index;
        let four_pi_n = FOUR_TIMES_PI * index;

        BLACKMAN_A0 - BLACKMAN_A1 * (two_pi_n / last_index).cos()
            + BLACKMAN_A2 * (four_pi_n / last_index).cos()
    }

    fn fir_filter(
        index: f64,
        last_index: f64,
        sinc_center: f64,
        resample_factor_reciprocal: f64,
    ) -> f64 {
        // The resample_factor_reciprocal also happens to be our
        // anti-alias cutoff. In this case it represents the minimum
        // output bandwidth needed to fully represent the input.
        let adjusted_sinc_center_offset = (index - sinc_center) * resample_factor_reciprocal;

        let sinc_value = Self::sinc(adjusted_sinc_center_offset);
        let blackman_window_value = Self::blackman(index, last_index);

        sinc_value * blackman_window_value
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum SampleRate {
    #[default]
    Hz44100,
    Hz48000,
    Hz88200,
    Hz96000,
}

impl IntoIterator for SampleRate {
    type Item = SampleRate;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        use SampleRate::*;

        vec![Hz44100, Hz48000, Hz88200, Hz96000].into_iter()
    }
}

impl FromStr for SampleRate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SampleRate::*;

        let lowercase_input = s.to_lowercase();

        // Match against both the actual
        // stringified value and how most
        // humans would write a sample rate.
        match lowercase_input.as_str() {
            "hz44100" | "44100hz" | "44100" | "44.1khz" => Ok(Hz44100),
            "hz48000" | "48000hz" | "48000" | "48khz" => Ok(Hz48000),
            "hz88200" | "88200hz" | "88200" | "88.2khz" => Ok(Hz88200),
            "hz96000" | "96000hz" | "96000" | "96khz" => Ok(Hz96000),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for SampleRate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use SampleRate::*;

        match self {
            // Let's make these more human readable.
            // "Hz44100" is just awkward.
            Hz44100 => write!(f, "44.1kHz"),
            Hz48000 => write!(f, "48kHz"),
            Hz88200 => write!(f, "88.2kHz"),
            Hz96000 => write!(f, "96kHz"),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ResampleSpec {
    pub resample_factor_reciprocal: f64,
    pub interpolation_output_size: usize,
}

impl SampleRate {
    pub fn as_u32(&self) -> u32 {
        use SampleRate::*;

        match self {
            Hz44100 => 44100,
            Hz48000 => 48000,
            Hz88200 => 88200,
            Hz96000 => 96000,
        }
    }

    pub fn duration_to_normalisation_coefficient(&self, duration: Duration) -> f64 {
        let secs = duration.as_secs_f64();
        let ms = secs * 1000.0;

        if ms < 1.0 {
            warn!("Coefficient Duration: {:.0} ms, a Normalisation Attack/Release of < 1 ms will cause severe distortion", ms);
        }

        (-1.0 / (secs * self.samples_per_second())).exp()
    }

    pub fn normalisation_coefficient_to_duration(&self, coefficient: f64) -> Duration {
        let duration = Duration::from_secs_f64(-1.0 / coefficient.ln() / self.samples_per_second());

        let secs = duration.as_secs_f64();
        let ms = secs * 1000.0;

        if ms < 1.0 {
            warn!("Coefficient Duration: {:.0} ms, a Normalisation Attack/Release of < 1 ms will cause severe distortion", ms);
        }

        duration
    }

    fn samples_per_second(&self) -> f64 {
        use SampleRate::*;

        match self {
            Hz44100 => HZ44100_SAMPLES_PER_SECOND,
            Hz48000 => HZ48000_SAMPLES_PER_SECOND,
            Hz88200 => HZ88200_SAMPLES_PER_SECOND,
            Hz96000 => HZ96000_SAMPLES_PER_SECOND,
        }
    }

    pub fn get_resample_spec(&self) -> ResampleSpec {
        use SampleRate::*;

        match self {
            // Dummy values to satisfy
            // the match statement.
            // 44.1kHz will be bypassed.
            Hz44100 => {
                warn!("Resampling 44.1kHz to 44.1kHz is just a really CPU intensive no-op, you should not be doing it");

                ResampleSpec {
                    resample_factor_reciprocal: 1.0,
                    interpolation_output_size: RESAMPLER_INPUT_SIZE,
                }
            }
            Hz48000 => ResampleSpec {
                resample_factor_reciprocal: HZ48000_RESAMPLE_FACTOR_RECIPROCAL,
                interpolation_output_size: HZ48000_INTERPOLATION_OUTPUT_SIZE,
            },
            Hz88200 => ResampleSpec {
                resample_factor_reciprocal: HZ88200_RESAMPLE_FACTOR_RECIPROCAL,
                interpolation_output_size: HZ88200_INTERPOLATION_OUTPUT_SIZE,
            },
            Hz96000 => ResampleSpec {
                resample_factor_reciprocal: HZ96000_RESAMPLE_FACTOR_RECIPROCAL,
                interpolation_output_size: HZ96000_INTERPOLATION_OUTPUT_SIZE,
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum Bitrate {
    Bitrate96,
    #[default]
    Bitrate160,
    Bitrate320,
}

impl FromStr for Bitrate {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "96" => Ok(Self::Bitrate96),
            "160" => Ok(Self::Bitrate160),
            "320" => Ok(Self::Bitrate320),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum AudioFormat {
    F64,
    F32,
    S32,
    S24,
    S24_3,
    #[default]
    S16,
}

impl IntoIterator for AudioFormat {
    type Item = AudioFormat;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        use AudioFormat::*;

        vec![F64, F32, S32, S24, S24_3, S16].into_iter()
    }
}

impl FromStr for AudioFormat {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_ref() {
            "F64" => Ok(Self::F64),
            "F32" => Ok(Self::F32),
            "S32" => Ok(Self::S32),
            "S24" => Ok(Self::S24),
            "S24_3" => Ok(Self::S24_3),
            "S16" => Ok(Self::S16),
            _ => Err(()),
        }
    }
}

impl AudioFormat {
    // not used by all backends
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        match self {
            Self::F64 => mem::size_of::<f64>(),
            Self::F32 => mem::size_of::<f32>(),
            Self::S24_3 => mem::size_of::<i24>(),
            Self::S16 => mem::size_of::<i16>(),
            _ => mem::size_of::<i32>(), // S32 and S24 are both stored in i32
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum NormalisationType {
    Album,
    Track,
    #[default]
    Auto,
}

impl FromStr for NormalisationType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "album" => Ok(Self::Album),
            "track" => Ok(Self::Track),
            "auto" => Ok(Self::Auto),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum NormalisationMethod {
    Basic,
    #[default]
    Dynamic,
}

impl FromStr for NormalisationMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "basic" => Ok(Self::Basic),
            "dynamic" => Ok(Self::Dynamic),
            _ => Err(()),
        }
    }
}

#[derive(Clone)]
pub struct PlayerConfig {
    pub bitrate: Bitrate,
    pub gapless: bool,
    pub passthrough: bool,

    pub interpolation_quality: InterpolationQuality,
    pub sample_rate: SampleRate,

    pub normalisation: bool,
    pub normalisation_type: NormalisationType,
    pub normalisation_method: NormalisationMethod,
    pub normalisation_pregain_db: f64,
    pub normalisation_threshold_dbfs: f64,
    pub normalisation_attack_cf: f64,
    pub normalisation_release_cf: f64,
    pub normalisation_knee_db: f64,

    // pass function pointers so they can be lazily instantiated *after* spawning a thread
    // (thereby circumventing Send bounds that they might not satisfy)
    pub ditherer: Option<DithererBuilder>,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        Self {
            bitrate: Bitrate::default(),
            gapless: true,
            normalisation: false,
            interpolation_quality: InterpolationQuality::default(),
            sample_rate: SampleRate::default(),
            normalisation_type: NormalisationType::default(),
            normalisation_method: NormalisationMethod::default(),
            normalisation_pregain_db: 0.0,
            normalisation_threshold_dbfs: -2.0,
            // Dummy value. We can't use the default because
            // no matter what it's dependent on the sample rate.
            normalisation_attack_cf: 0.0,
            // Same with release.
            normalisation_release_cf: 0.0,
            normalisation_knee_db: 5.0,
            passthrough: false,
            ditherer: Some(mk_ditherer::<TriangularDitherer>),
        }
    }
}

// fields are intended for volume control range in dB
#[derive(Clone, Copy, Debug)]
pub enum VolumeCtrl {
    Cubic(f64),
    Fixed,
    Linear,
    Log(f64),
}

impl FromStr for VolumeCtrl {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_with_range(s, Self::DEFAULT_DB_RANGE)
    }
}

impl Default for VolumeCtrl {
    fn default() -> VolumeCtrl {
        VolumeCtrl::Log(Self::DEFAULT_DB_RANGE)
    }
}

impl VolumeCtrl {
    pub const MAX_VOLUME: u16 = u16::MAX;

    // Taken from: https://www.dr-lex.be/info-stuff/volumecontrols.html
    pub const DEFAULT_DB_RANGE: f64 = 60.0;

    pub fn from_str_with_range(s: &str, db_range: f64) -> Result<Self, <Self as FromStr>::Err> {
        use self::VolumeCtrl::*;
        match s.to_lowercase().as_ref() {
            "cubic" => Ok(Cubic(db_range)),
            "fixed" => Ok(Fixed),
            "linear" => Ok(Linear),
            "log" => Ok(Log(db_range)),
            _ => Err(()),
        }
    }
}
