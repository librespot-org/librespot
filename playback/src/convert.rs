use crate::dither::{Ditherer, DithererBuilder};
use zerocopy::AsBytes;

#[derive(AsBytes, Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct i24([u8; 3]);
impl i24 {
    fn from_s24(sample: i32) -> Self {
        // trim the padding in the most significant byte
        #[allow(unused_variables)]
        let [a, b, c, d] = sample.to_ne_bytes();
        #[cfg(target_endian = "little")]
        return Self([a, b, c]);
        #[cfg(target_endian = "big")]
        return Self([b, c, d]);
    }
}

pub struct Converter {
    ditherer: Option<Box<dyn Ditherer>>,
}

impl Converter {
    pub fn new(dither_config: Option<DithererBuilder>) -> Self {
        if let Some(ref ditherer_builder) = dither_config {
            let ditherer = (ditherer_builder)();
            info!("Converting with ditherer: {}", ditherer.name());
            Self {
                ditherer: Some(ditherer),
            }
        } else {
            Self { ditherer: None }
        }
    }

    /// To convert PCM samples from floating point normalized as `-1.0..=1.0`
    /// to 32-bit signed integer, multiply by 2147483648 (0x80000000) and
    /// saturate at the bounds of `i32`.
    const SCALE_S32: f64 = 2147483648.;

    /// To convert PCM samples from floating point normalized as `-1.0..=1.0`
    /// to 24-bit signed integer, multiply by 8388608 (0x800000) and saturate
    /// at the bounds of `i24`.
    const SCALE_S24: f64 = 8388608.;

    /// To convert PCM samples from floating point normalized as `-1.0..=1.0`
    /// to 16-bit signed integer, multiply by 32768 (0x8000) and saturate at
    /// the bounds of `i16`. When the samples were encoded using the same
    /// scaling factor, like the reference Vorbis encoder does, this makes
    /// conversions transparent.
    const SCALE_S16: f64 = 32768.;

    pub fn scale(&mut self, sample: f64, factor: f64) -> f64 {
        let dither = match self.ditherer {
            Some(ref mut d) => d.noise(),
            None => 0.0,
        };

        // From the many float to int conversion methods available, match what
        // the reference Vorbis implementation uses: sample * 32768 (for 16 bit)
        let int_value = sample * factor + dither;

        // Casting float to integer rounds towards zero by default, i.e. it
        // truncates, and that generates larger error than rounding to nearest.
        // Absolute lowest error is gained from rounding ties to even.
        math::round::half_to_even(int_value, 0)
    }

    // Special case for samples packed in a word of greater bit depth (e.g.
    // S24): clamp between min and max to ensure that the most significant
    // byte is zero. Otherwise, dithering may cause an overflow. This is not
    // necessary for other formats, because casting to integer will saturate
    // to the bounds of the primitive.
    pub fn clamping_scale(&mut self, sample: f64, factor: f64) -> f64 {
        let int_value = self.scale(sample, factor);

        // In two's complement, there are more negative than positive values.
        let min = -factor;
        let max = factor - 1.0;

        if int_value < min {
            return min;
        } else if int_value > max {
            return max;
        }
        int_value
    }

    pub fn f64_to_f32(&mut self, samples: &[f64]) -> Vec<f32> {
        samples.iter().map(|sample| *sample as f32).collect()
    }

    pub fn f64_to_s32(&mut self, samples: &[f64]) -> Vec<i32> {
        samples
            .iter()
            .map(|sample| self.scale(*sample, Self::SCALE_S32) as i32)
            .collect()
    }

    // S24 is 24-bit PCM packed in an upper 32-bit word
    pub fn f64_to_s24(&mut self, samples: &[f64]) -> Vec<i32> {
        samples
            .iter()
            .map(|sample| self.clamping_scale(*sample, Self::SCALE_S24) as i32)
            .collect()
    }

    // S24_3 is 24-bit PCM in a 3-byte array
    pub fn f64_to_s24_3(&mut self, samples: &[f64]) -> Vec<i24> {
        samples
            .iter()
            .map(|sample| {
                // Not as DRY as calling f32_to_s24 first, but this saves iterating
                // over all samples twice.
                let int_value = self.clamping_scale(*sample, Self::SCALE_S24) as i32;
                i24::from_s24(int_value)
            })
            .collect()
    }

    pub fn f64_to_s16(&mut self, samples: &[f64]) -> Vec<i16> {
        samples
            .iter()
            .map(|sample| self.scale(*sample, Self::SCALE_S16) as i16)
            .collect()
    }
}
