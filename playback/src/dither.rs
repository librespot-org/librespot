use rand::rngs::ThreadRng;
use rand_distr::{Distribution, Normal, Triangular, Uniform};
use std::fmt;

const NUM_CHANNELS: usize = 2;

// Dithering lowers digital-to-analog conversion ("requantization") error,
// linearizing output, lowering distortion and replacing it with a constant,
// fixed noise level, which is more pleasant to the ear than the distortion.
//
// Guidance:
//
//  * On S24, S24_3 and S24, the default is to use triangular dithering.
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
pub trait Ditherer {
    fn new() -> Self
    where
        Self: Sized;
    fn name(&self) -> &'static str;
    fn noise(&mut self) -> f64;
}

impl fmt::Display for dyn Ditherer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

// Implementation note: we save the handle to ThreadRng so it doesn't require
// a lookup on each call (which is on each sample!). This is ~2.5x as fast.
// Downside is that it is not Send so we cannot move it around player threads.
//

pub struct TriangularDitherer {
    cached_rng: ThreadRng,
    distribution: Triangular<f64>,
}

impl Ditherer for TriangularDitherer {
    fn new() -> Self {
        Self {
            cached_rng: rand::thread_rng(),
            // 2 LSB peak-to-peak needed to linearize the response:
            distribution: Triangular::new(-1.0, 1.0, 0.0).unwrap(),
        }
    }

    fn name(&self) -> &'static str {
        "Triangular"
    }

    fn noise(&mut self) -> f64 {
        self.distribution.sample(&mut self.cached_rng)
    }
}

pub struct GaussianDitherer {
    cached_rng: ThreadRng,
    distribution: Normal<f64>,
}

impl Ditherer for GaussianDitherer {
    fn new() -> Self {
        Self {
            cached_rng: rand::thread_rng(),
            // 1/2 LSB RMS needed to linearize the response:
            distribution: Normal::new(0.0, 0.5).unwrap(),
        }
    }

    fn name(&self) -> &'static str {
        "Gaussian"
    }

    fn noise(&mut self) -> f64 {
        self.distribution.sample(&mut self.cached_rng)
    }
}

pub struct HighPassDitherer {
    active_channel: usize,
    previous_noises: [f64; NUM_CHANNELS],
    cached_rng: ThreadRng,
    distribution: Uniform<f64>,
}

impl Ditherer for HighPassDitherer {
    fn new() -> Self {
        Self {
            active_channel: 0,
            previous_noises: [0.0; NUM_CHANNELS],
            cached_rng: rand::thread_rng(),
            distribution: Uniform::new_inclusive(-0.5, 0.5), // 1 LSB +/- 1 LSB (previous) = 2 LSB
        }
    }

    fn name(&self) -> &'static str {
        "Triangular, High Passed"
    }

    fn noise(&mut self) -> f64 {
        let new_noise = self.distribution.sample(&mut self.cached_rng);
        let high_passed_noise = new_noise - self.previous_noises[self.active_channel];
        self.previous_noises[self.active_channel] = new_noise;
        self.active_channel ^= 1;
        high_passed_noise
    }
}

pub fn mk_ditherer<D: Ditherer + 'static>() -> Box<dyn Ditherer> {
    Box::new(D::new())
}

pub type DithererBuilder = fn() -> Box<dyn Ditherer>;

pub fn find_ditherer(name: Option<String>) -> Option<DithererBuilder> {
    match name.as_deref() {
        Some("tpdf") => Some(mk_ditherer::<TriangularDitherer>),
        Some("gpdf") => Some(mk_ditherer::<GaussianDitherer>),
        Some("tpdf_hp") => Some(mk_ditherer::<HighPassDitherer>),
        _ => None,
    }
}
