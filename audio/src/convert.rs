use crate::dither::Ditherer;
use crate::shape_noise::NoiseShaper;
use zerocopy::AsBytes;

#[derive(AsBytes, Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct i24([u8; 3]);
impl i24 {
    pub const MIN: i32 = -8388608;
    pub const MAX: i32 = 8388607;

    fn from_s24(sample: i32) -> Self {
        // trim the padding in the most significant byte
        let [a, b, c, _d] = sample.to_le_bytes();
        i24([a, b, c])
    }
}

pub struct Requantizer {
    ditherer: Box<dyn Ditherer>,
    noise_shaper: Box<dyn NoiseShaper>,
}

impl Requantizer {
    pub fn new(ditherer: Box<dyn Ditherer>, noise_shaper: Box<dyn NoiseShaper>) -> Self {
        info!(
            "Requantizing with ditherer: {} and noise shaper: {}",
            ditherer, noise_shaper
        );
        Self {
            ditherer,
            noise_shaper,
        }
    }

    pub fn shaped_dither(&mut self, sample: f32) -> f32 {
        let noise = self.ditherer.noise(sample);
        self.noise_shaper.shape(sample, noise)
    }
}

macro_rules! convert_samples_to {
    ($type: ident, $samples: expr, $requantizer: expr) => {
        convert_samples_to!($type, $samples, $requantizer, $type)
    };
    ($type: ident, $samples: expr, $requantizer: expr, $scale: ident) => {
        $samples
            .iter()
            .map(|sample| {
                // Losslessly represent [-1.0, 1.0] to [$type::MIN, $type::MAX]
                // while maintaining DC linearity. There is nothing to be gained
                // by doing this in f64, as the significand of a f32 is 24 bits,
                // just like the maximum bit depth we are converting to.
                let mut int_value = *sample * ($scale::MAX as f32 + 0.5) - 0.5;
                int_value = $requantizer.shaped_dither(int_value);

                // Special case for output formats with a range smaller than
                // their primitive allows (i.e. S24: 24-bit samples in a 32-bit
                // word): clamp between MIN and MAX to ensure that the most
                // significant byte is zero. Otherwise, dithering may cause an
                // overflow. Don't care in other cases as casting to primitives
                // will saturate correctly (see below).
                if ($scale::MAX as $type) < std::$type::MAX {
                    let min = $scale::MIN as f32;
                    let max = $scale::MAX as f32;

                    if int_value < min {
                        int_value = min;
                    } else if int_value > max {
                        int_value = max;
                    }
                }

                // https://doc.rust-lang.org/nomicon/casts.html:
                // casting float to integer rounds towards zero, then saturates.
                // Ideally halves should round to even to prevent any bias, but
                // since it is extremely unlikely that a float has *exactly* .5
                // as fraction, this should be more than precise enough.
                int_value as $type
            })
            .collect()
    };
}

pub fn to_s32(samples: &[f32], requantizer: &mut Requantizer) -> Vec<i32> {
    convert_samples_to!(i32, samples, requantizer)
}

// S24 is 24-bit PCM packed in an upper 32-bit word
pub fn to_s24(samples: &[f32], requantizer: &mut Requantizer) -> Vec<i32> {
    convert_samples_to!(i32, samples, requantizer, i24)
}

pub fn to_s24_3(samples: &[f32], requantizer: &mut Requantizer) -> Vec<i24> {
    // TODO - can we improve performance by passing this as a closure?
    to_s24(samples, requantizer)
        .iter()
        .map(|sample| i24::from_s24(*sample))
        .collect()
}

pub fn to_s16(samples: &[f32], requantizer: &mut Requantizer) -> Vec<i16> {
    convert_samples_to!(i16, samples, requantizer)
}
