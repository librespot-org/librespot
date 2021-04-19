use rand::rngs::ThreadRng;
use rand_distr::{Distribution, Normal, Triangular, Uniform};
use std::fmt;

// Dithering lowers digital-to-analog conversion ("requantization") error,
// lowering distortion and replacing it with a constant, fixed noise level,
// which is more pleasant to the ear than the distortion. Doing so can with
// a noise-shaped dither can increase the dynamic range of 96 dB CD-quality
// audio to a perceived 120 dB.
//
// Guidance: experts can configure many different configurations of ditherers
// and noise shapers. For the rest of us:
//
//  * Don't dither or shape noise on S32 or F32. On F32 it's not supported
//    anyway (there are no rounding errors due to integer conversions) and on
//    S32 the noise level is so far down that it is simply inaudible.
//
//  * Generally use high pass dithering (hp) without noise shaping. Depending
//    on personal preference you may use Gaussian dithering (gauss) instead;
//    it's not as good objectively, but it may be preferred subjectively if
//    you are looking for a more "analog" sound.
//
//  * On power-constrained hardware, use the fraction saving noise shaper
//    instead of dithering. Performance-wise, this is not necessary even on a
//    Raspberry Pi Zero, but if you're on batteries...
//
// Implementation note: we save the handle to ThreadRng so it doesn't require
// a lookup on each call (which is on each sample!). This is ~2.5x as fast.
// Downside is that it is not Send so we cannot move it around player threads.
//
pub trait Ditherer {
    fn new() -> Self
    where
        Self: Sized;
    fn name(&self) -> &'static str;
    fn noise(&mut self, sample: f32) -> f32;
}

impl dyn Ditherer {
    pub fn default() -> fn() -> Box<Self> {
        mk_ditherer::<HighPassDitherer>
    }
}

impl fmt::Display for dyn Ditherer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

pub struct NoDithering {}
impl Ditherer for NoDithering {
    fn new() -> Self {
        Self {}
    }

    fn name(&self) -> &'static str {
        "None"
    }

    fn noise(&mut self, _sample: f32) -> f32 {
        0.0
    }
}

// "True" white noise (refer to Gaussian for analog source hiss). Advantages:
// least CPU-intensive dither, lowest signal-to-noise ratio. Disadvantage:
// highest perceived loudness, suffers from intermodulation distortion unless
// you are using this for subtractive dithering, which you most likely are not,
// and is not supported by any of the librespot backends. Guidance: use some
// other ditherer unless you know what you're doing.
pub struct RectangularDitherer {
    cached_rng: ThreadRng,
    distribution: Uniform<f32>,
}

impl Ditherer for RectangularDitherer {
    fn new() -> Self {
        Self {
            cached_rng: rand::thread_rng(),
            // 1 LSB peak-to-peak needed to linearize the response:
            distribution: Uniform::new_inclusive(-0.5, 0.5),
        }
    }

    fn name(&self) -> &'static str {
        "Rectangular"
    }

    fn noise(&mut self, _sample: f32) -> f32 {
        self.distribution.sample(&mut self.cached_rng)
    }
}

// Like Rectangular, but with lower error and OK to use for the default case
// of non-subtractive dithering such as to the librespot backends.
pub struct StochasticDitherer {
    cached_rng: ThreadRng,
    distribution: Uniform<f32>,
}

impl Ditherer for StochasticDitherer {
    fn new() -> Self {
        Self {
            cached_rng: rand::thread_rng(),
            distribution: Uniform::new(0.0, 1.0),
        }
    }

    fn name(&self) -> &'static str {
        "Stochastic"
    }

    fn noise(&mut self, sample: f32) -> f32 {
        let fract = sample.fract();
        if self.distribution.sample(&mut self.cached_rng) <= fract {
            1.0 - fract
        } else {
            fract * -1.0
        }
    }
}

// Higher level than Rectangular. Advantages: superior to Rectangular as it
// does not suffer from modulation noise effects. Disadvantage: more CPU-
// expensive. Guidance: all-round recommendation to reduce quantization noise,
// even on 24-bit output.
pub struct TriangularDitherer {
    cached_rng: ThreadRng,
    distribution: Triangular<f32>,
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

    fn noise(&mut self, _sample: f32) -> f32 {
        self.distribution.sample(&mut self.cached_rng)
    }
}

// Like Triangular, but with higher noise power and more like phono hiss.
// Guidance: theoretically less optimal, but an alternative to Triangular
// if a more analog sound is sought after.
pub struct GaussianDitherer {
    cached_rng: ThreadRng,
    distribution: Normal<f32>,
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

    fn noise(&mut self, _sample: f32) -> f32 {
        self.distribution.sample(&mut self.cached_rng)
    }
}

// Like Triangular, but with a high-pass filter. Advantages: comparably less
// perceptible noise, less CPU-intensive. Disadvantage: this acts like a FIR
// filter with weights [1.0, -1.0], and is superseded by noise shapers.
// Guidance: better than Triangular if not doing other noise shaping.
pub struct HighPassDitherer {
    previous_noise: f32,
    cached_rng: ThreadRng,
    distribution: Uniform<f32>,
}

impl Ditherer for HighPassDitherer {
    fn new() -> Self {
        Self {
            previous_noise: 0.0,
            cached_rng: rand::thread_rng(),
            distribution: Uniform::new_inclusive(-0.5, 0.5), // 1 LSB +/- 1 LSB (previous) = 2 LSB
        }
    }

    fn name(&self) -> &'static str {
        "High Pass"
    }

    fn noise(&mut self, _sample: f32) -> f32 {
        let new_noise = self.distribution.sample(&mut self.cached_rng);
        let high_passed_noise = new_noise - self.previous_noise;
        self.previous_noise = new_noise;
        high_passed_noise
    }
}

pub fn mk_ditherer<D: Ditherer + 'static>() -> Box<dyn Ditherer> {
    Box::new(D::new())
}

pub type DithererBuilder = fn() -> Box<dyn Ditherer>;

pub const DITHERERS: &[(&str, DithererBuilder)] = &[
    ("none", mk_ditherer::<NoDithering>),
    ("rect", mk_ditherer::<RectangularDitherer>),
    ("sto", mk_ditherer::<StochasticDitherer>),
    ("tri", mk_ditherer::<TriangularDitherer>),
    ("gauss", mk_ditherer::<GaussianDitherer>),
    ("hp", mk_ditherer::<HighPassDitherer>),
];

pub fn find_ditherer(name: Option<String>) -> Option<fn() -> Box<dyn Ditherer>> {
    match name {
        Some(name) => DITHERERS
            .iter()
            .find(|ditherer| name == ditherer.0)
            .map(|ditherer| ditherer.1),
        _ => Some(mk_ditherer::<NoDithering>),
    }
}
