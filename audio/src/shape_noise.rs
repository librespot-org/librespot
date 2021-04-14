const NUM_CHANNELS: usize = 2;

// Noise shapers take the noise caused by errors in the digital-to-analog
// conversion process ("requantizing") plus any added dither, and change the
// frequency curve of that noise so that it is less perceptible to our ears.
// Like dithering, this process can generate more noise than there was in the
// first place ("noise power") but still make it more pleasant to listen to.
// Noise-shaped dithers can improve the dynamic range of 96 dB CD-quality audio
// to a perceived 120 dB.
//
// Guidance: take care to only use noise shaping when you are sure no further
// dithering and/or noise shaping is done further down the line -- like most
// delta-sigma DACs do. Exception to this rule is the Fraction Saver, which
// should be OK to use even on such DACs (but in this case, disable dithering
// in librespot -- you should do one or the other, not both).
//
// As for the more powerful noise shapers, it's a personal trade-off between
// absolute noise power and perceived noise. Co-incidentally, the lower the
// perceived noise, the higher the absolute power, and the higher the CPU-
// usage. All shapers have something going for them.
//
pub trait NoiseShaper {
    fn new() -> Self
    where
        Self: Sized;
    fn shape(&mut self, sample: f32, noise: f32) -> f32;
}

pub struct NoShaping {}
impl NoiseShaper for NoShaping {
    fn new() -> Self {
        // Doing nothing here means that the requantizer will simply cast to
        // integer. For Rust the default behavior then is to round towards
        // zero.
        debug!("Noise Shaper: None (rounding towards zero)");
        Self {}
    }

    fn shape(&mut self, sample: f32, noise: f32) -> f32 {
        sample + noise
    }
}

// First-order noise shaping. Advantages: negligible performance hit, infinite
// signal-to-noise ration at DC, lowered signal-to-noise ratio for low
// frequencies and slightly higher signal-to-noise ration for frequencies
// above Nyquist/2, kills certain limit cycles in subsequent IIR filters,
// safe to use on oversampling DACs. Disadvantage: higher perceived loudness
// than other options (but still less so than without shaping!). Guidance:
// there are very little reasons not to use this shaper, even on power-
// constrained hardware. If you do, best to set dithering to none.
pub struct FractionSaver {
    active_channel: usize,
    previous_fractions: [f32; NUM_CHANNELS],
}

impl NoiseShaper for FractionSaver {
    fn new() -> Self {
        debug!("Noise Shaper: Fraction Saver");
        Self {
            active_channel: 0,
            previous_fractions: [0.0; NUM_CHANNELS],
        }
    }

    fn shape(&mut self, sample: f32, noise: f32) -> f32 {
        let sample_with_fraction = sample + noise + self.previous_fractions[self.active_channel];
        self.previous_fractions[self.active_channel] = sample_with_fraction.fract();
        self.active_channel = self.active_channel ^ 1;
        sample_with_fraction.floor()
    }
}

// Higher-order noise shapers. Advantage: lower perceived loudness than first-
// order noise-shaping. Disadvantage: higher CPU-usage, higher absolute noise
// level, not to be used when there will be dithering or noise shaping further
// down the chain, as is the case with most delta-sigma DACs. Guidance: use
// on non-oversampling DACs, or other DACs that do not do dithering or noise
// shaping themselves.
macro_rules! fir_shaper {
    ($name: ident, $description: tt, $taps: expr, $weights: expr) => {
        pub struct $name {
            fir: FIR,
        }

        impl NoiseShaper for $name {
            fn new() -> Self {
                debug!(
                    "Noise Shaper: {}, {} taps",
                    $description,
                    Self::WEIGHTS.len()
                );
                Self {
                    fir: FIR::new(&Self::WEIGHTS),
                }
            }

            fn shape(&mut self, sample: f32, noise: f32) -> f32 {
                self.fir.shape(sample, noise)
            }
        }

        impl $name {
            const WEIGHTS: [f32; $taps] = $weights;
        }
    };
}

// 14.34 dB improvement in E-weighted noise at the expense of 12.19 dB higher
// noise power (unweighted) by pushing most noise into the spectrum above
// 15 kHz, meaning the noise is less audible. Guidance: this is a good
// cost/benefit trade-off. Widely used in other audio software.
fir_shaper!(
    Lipshitz5,
    "Lipshitz improved E-weighted",
    5,
    [2.033, -2.165, 1.959, -1.590, 0.6149]
);

// 18.32 dB improvement in E-weighted noise but at the expense of 23.1 dB
// higher noise power (unweighted). Approaches noise power levels of vinyl,
// but with lower perceived loudness. Guidance: certainly still acceptable
// noise levels, subject to personal preference. Arguably superseded by
// Wannamaker9.
fir_shaper!(
    Lipshitz9,
    "Lipshitz improved E-weighted",
    9,
    [2.847, -4.685, 6.214, -7.184, 6.639, -5.032, 3.263, -1.632, 0.4191]
);

// 10.47 dB improvement in F-weighted noise at the expense of 6.64 dB higher
// noise power (unweighted). Like Lipshitz, but with a refinement in psycho-
// acoustic levels. Spreads most noise into the spectrum above 10 kHz.
// Guidance: a less precise, but lower absolute noise alternative to Lipshitz5.
fir_shaper!(
    Wannamaker3,
    "Wannamaker F-weighted",
    3,
    [1.623, -0.982, 0.109]
);

// 16.8 dB improvement in F-weighted noise at the expense of 18.4 dB higher
// noise power (unweighted). Pushes most noise into the spectrum above 15 kHz.
// Guidance: refinement over Lipshitz9. This is what SoX uses.
fir_shaper!(
    Wannamaker9,
    "Wannamaker F-weighted",
    9,
    [2.412, -3.370, 3.937, -4.174, 3.353, -2.205, 1.281, -0.569, 0.0847]
);

// 16.7 dB improvement in F-weighted noise at the expense of 17.3 dB higher
// noise power (unweighted). This is close to the theoretical limit curve
// but with the highest CPU usage. Guidance: better than Wannamaker9 if you
// can suffer the performance hit.
fir_shaper!(
    Wannamaker24,
    "Wannamaker F-weighted",
    24,
    [
        2.391510, -3.284444, 3.679506, -3.635044, 2.524185, -1.146701, 0.115354, 0.513745,
        -0.749277, 0.512386, -0.188997, -0.043705, 0.149843, -0.151186, 0.076302, -0.012070,
        -0.021127, 0.025232, -0.016121, 0.004453, 0.000876, -0.001799, 0.000774, -0.000128
    ]
);

struct FIR {
    taps: usize,
    weights: Vec<f32>,
    active_channel: usize,
    error_buffer: Vec<f64>,
    buffer_index: usize,
}

impl FIR {
    fn new(weights: &[f32]) -> Self {
        let taps = weights.len();
        Self {
            taps,
            weights: weights.to_vec(),
            active_channel: 0,
            error_buffer: vec![0.0; taps * NUM_CHANNELS],
            buffer_index: 0,
        }
    }

    fn shape(&mut self, sample: f32, noise: f32) -> f32 {
        // apply FIR filter
        let mut sample_with_shaped_noise = sample as f64;
        for index in 0..self.taps {
            sample_with_shaped_noise = sample_with_shaped_noise + self.weighted_error(index);
        }

        let dithered_sample = (sample_with_shaped_noise + noise as f64).round();

        // store error and roll buffer -- this is a slight hack to increment
        // buffer_index only if we are moving from channel 1 to channel 0,
        // that is, just handled both the left and right channels
        self.buffer_index = (self.buffer_index + self.active_channel) % self.taps;
        let index = self.index_at_samples_ago(0);
        self.error_buffer[index] = sample_with_shaped_noise - dithered_sample;
        self.active_channel = self.active_channel ^ 1;

        dithered_sample as f32
    }
}

impl FIR {
    fn index_at_samples_ago(&self, errors_ago: usize) -> usize {
        ((self.buffer_index + self.taps - errors_ago) % self.taps) + self.taps * self.active_channel
    }

    fn weighted_error(&self, index: usize) -> f64 {
        self.error_buffer[self.index_at_samples_ago(index)] * self.weights[index] as f64
    }
}

pub fn mk_noise_shaper<S: NoiseShaper + 'static>() -> Box<dyn NoiseShaper> {
    Box::new(S::new())
}

pub const NOISE_SHAPERS: &'static [(&'static str, fn() -> Box<dyn NoiseShaper>)] = &[
    ("none", mk_noise_shaper::<NoShaping>),
    ("fract", mk_noise_shaper::<FractionSaver>),
    ("iew5", mk_noise_shaper::<Lipshitz5>),
    ("iew9", mk_noise_shaper::<Lipshitz9>),
    ("fw3", mk_noise_shaper::<Wannamaker3>),
    ("fw9", mk_noise_shaper::<Wannamaker9>),
    ("fw24", mk_noise_shaper::<Wannamaker24>),
];

pub fn find_noise_shaper(name: Option<String>) -> Option<fn() -> Box<dyn NoiseShaper>> {
    match name {
        Some(name) => NOISE_SHAPERS
            .iter()
            .find(|noise_shaper| name == noise_shaper.0)
            .map(|noise_shaper| noise_shaper.1),
        _ => None,
    }
}
