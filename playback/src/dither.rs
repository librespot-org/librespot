use rand::SeedableRng;
use rand::rngs::SmallRng;
use rand_distr::{Distribution, Normal, Triangular, Uniform};

use clap::ValueEnum;
use enum_assoc::Assoc;
use serde::{Deserialize, Serialize};

use crate::NUM_CHANNELS;

// Dithering lowers digital-to-analog conversion ("requantization") error,
// linearizing output, lowering distortion and replacing it with a constant,
// fixed noise level, which is more pleasant to the ear than the distortion.
//
// Guidance:
//
//  * On S16, S24 and S24_3, the default is to use triangular dithering.
//    Depending on personal preference you may use Gaussian dithering instead;
//    it's not as good objectively, but it may be preferred subjectively if
//    you are looking for a more "analog" sound akin to tape hiss.
//
//  * Advanced users who know that they have a DAC without noise shaping have
//    a third option: high-passed dithering, which is like triangular dithering
//    except that it moves dithering noise up in frequency where it is less
//    audible. Note: 99% of DACs are of delta-sigma design with noise shaping,
//    so unless you have a multibit / R2R DAC, or otherwise know what you are
//    doing, this is not for you.
//
//  * Don't dither or shape noise on S32 or F32. On F32 it's not supported
//    anyway (there are no integer conversions and so no rounding errors) and
//    on S32 the noise level is so far down that it is simply inaudible even
//    after volume normalisation and control.
//

#[derive(Default, Debug, Clone, Copy, Assoc, ValueEnum, Serialize, Deserialize)]
#[func(pub fn build(&self) -> Option<Box<dyn Ditherer>>)]
pub enum DithererBuilder {
    #[default]
    #[assoc(build = Box::new(TriangularDitherer::new()))]
    Tpdf,
    #[assoc(build = Box::new(GaussianDitherer::new()))]
    Gpdf,
    #[assoc(build = Box::new(HighPassDitherer::new()))]
    TpdfHp,
    None,
}

pub trait Ditherer: Send + Sync {
    fn new() -> Self
    where
        Self: Sized;

    fn noise(&mut self) -> f64;
}

fn create_rng() -> SmallRng {
    SmallRng::from_os_rng()
}

pub struct TriangularDitherer {
    cached_rng: SmallRng,
    distribution: Triangular<f64>,
}

impl Ditherer for TriangularDitherer {
    fn new() -> Self {
        Self {
            cached_rng: create_rng(),
            // 2 LSB peak-to-peak needed to linearize the response:
            distribution: Triangular::new(-1.0, 1.0, 0.0).unwrap(),
        }
    }

    #[inline]
    fn noise(&mut self) -> f64 {
        self.distribution.sample(&mut self.cached_rng)
    }
}

pub struct GaussianDitherer {
    cached_rng: SmallRng,
    distribution: Normal<f64>,
}

impl Ditherer for GaussianDitherer {
    fn new() -> Self {
        Self {
            cached_rng: create_rng(),
            // For Gaussian to achieve equivalent decorrelation to triangular dithering, it needs
            // 3-4 dB higher amplitude than TPDF's optimal 0.408 LSB. If optimizing:
            // - minimum correlation: σ ≈ 0.58
            // - perceptual equivalence: σ ≈ 0.65
            // - worst-case performance: σ ≈ 0.70
            //
            // σ = 0.6 LSB is a reasonable compromise that balances mathematical theory with
            // empirical performance across various signal types.
            distribution: Normal::new(0.0, 0.6).unwrap(),
        }
    }

    #[inline]
    fn noise(&mut self) -> f64 {
        self.distribution.sample(&mut self.cached_rng)
    }
}

pub struct HighPassDitherer {
    active_channel: usize,
    previous_noises: [f64; NUM_CHANNELS as usize],
    cached_rng: SmallRng,
    distribution: Uniform<f64>,
}

impl Ditherer for HighPassDitherer {
    fn new() -> Self {
        Self {
            active_channel: 0,
            previous_noises: [0.0; NUM_CHANNELS as usize],
            cached_rng: create_rng(),
            // 1 LSB +/- 1 LSB (previous) = 2 LSB
            distribution: Uniform::new_inclusive(-0.5, 0.5)
                .expect("Failed to create uniform distribution"),
        }
    }

    // fn name(&self) -> &'static str {
    //     Self::NAME
    // }

    #[inline]
    fn noise(&mut self) -> f64 {
        let new_noise = self.distribution.sample(&mut self.cached_rng);
        let high_passed_noise = new_noise - self.previous_noises[self.active_channel];
        self.previous_noises[self.active_channel] = new_noise;
        self.active_channel ^= 1;
        high_passed_noise
    }
}
