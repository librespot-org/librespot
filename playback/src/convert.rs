use crate::dither::{Ditherer, DithererBuilder};
use zerocopy::AsBytes;

#[derive(AsBytes, Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct i24([u8; 3]);
impl i24 {
    fn from_s24(sample: i32) -> Self {
        // trim the padding in the most significant byte
        let [a, b, c, _d] = sample.to_le_bytes();
        i24([a, b, c])
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

    const SCALE_S32: f64 = 2147483648.;
    const SCALE_S24: f64 = 8388608.;
    const SCALE_S16: f64 = 32768.;

    // Denormalize and dither
    pub fn scale(&mut self, sample: f64, factor: f64) -> f64 {
        // From the many float to int conversion methods available, match what
        // the reference Vorbis implementation uses: sample * 32768 (for 16 bit)
        let int_value = sample * factor;

        // https://doc.rust-lang.org/nomicon/casts.html: casting float to integer
        // rounds towards zero, then saturates. Ideally halves should round to even to
        // prevent any bias, but since it is extremely unlikely that a float has
        // *exactly* .5 as fraction, this should be more than precise enough.
        match self.ditherer {
            Some(ref mut dither) => int_value + dither.noise(),
            None => int_value,
        }
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
