use crate::dither::*;
use crate::shape_noise::*;
use zerocopy::AsBytes;

#[derive(AsBytes, Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct i24([u8; 3]);
impl i24 {
    fn pcm_from_i32(sample: i32) -> Self {
        // drop the least significant byte
        let [a, b, c, _d] = (sample >> 8).to_le_bytes();
        i24([a, b, c])
    }
}

pub struct Requantizer {
    ditherer: Box<dyn Ditherer>,
    noise_shaper: Box<dyn NoiseShaper>,
}

impl Requantizer {
    pub fn new(ditherer: Box<dyn Ditherer>, noise_shaper: Box<dyn NoiseShaper>) -> Self {
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
        convert_samples_to!($type, $samples, $requantizer, 0)
    };
    ($type: ident, $samples: expr, $requantizer: expr, $drop_bits: expr) => {
        $samples
            .iter()
            .map(|sample| {
                // Losslessly represent [-1.0, 1.0] to [$type::MIN, $type::MAX]
                // while maintaining DC linearity. There is nothing to be gained
                // by doing this in f64, as the significand of a f32 is 24 bits,
                // just like the maximum bit depth we are converting to.
                let mut int_value = *sample * (std::$type::MAX as f32 + 0.5) - 0.5;
                int_value = $requantizer.shaped_dither(int_value);

                // https://doc.rust-lang.org/nomicon/casts.html:
                // casting float to integer rounds towards zero, then saturates.
                // ideally ties round to even, but since it is extremely
                // unlikely that a float has *exactly* .5 as fraction, this
                // should be more than precise enough
                int_value as $type >> $drop_bits
            })
            .collect()
    };
}

pub fn to_s32(samples: &[f32], requantizer: &mut Requantizer) -> Vec<i32> {
    convert_samples_to!(i32, samples, requantizer)
}

// S24 is 24-bit PCM packed in an upper 32-bit word
pub fn to_s24(samples: &[f32], requantizer: &mut Requantizer) -> Vec<i32> {
    convert_samples_to!(i32, samples, requantizer, 8)
}

pub fn to_s24_3(samples: &[f32], requantizer: &mut Requantizer) -> Vec<i24> {
    // TODO - can we improve performance by passing this as a closure?
    to_s32(samples, requantizer)
        .iter()
        .map(|sample| i24::pcm_from_i32(*sample))
        .collect()
}

pub fn to_s16(samples: &[f32], requantizer: &mut Requantizer) -> Vec<i16> {
    convert_samples_to!(i16, samples, requantizer)
}
