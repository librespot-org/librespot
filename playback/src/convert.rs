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

    // Denormalize and dither
    pub fn scale(&mut self, sample: f32, factor: i64) -> f32 {
        let dither = match self.ditherer {
            Some(ref mut d) => d.noise(),
            None => 0.0,
        };

        // From the many float to int conversion methods available, match what
        // the reference Vorbis implementation uses: sample * 32768 (for 16 bit)
        let int_value = sample * factor as f32 + dither;

        // Casting float to integer rounds towards zero by default, i.e. it
        // truncates, and that generates larger error than rounding half up.
        int_value.round()
    }

    // Special case for samples packed in a word of greater bit depth (e.g.
    // S24): clamp between min and max to ensure that the most significant
    // byte is zero. Otherwise, dithering may cause an overflow. This is not
    // necessary for other formats, because casting to integer will saturate
    // to the bounds of the primitive.
    pub fn clamping_scale(&mut self, sample: f32, factor: i64) -> f32 {
        let int_value = self.scale(sample, factor);

        // In two's complement, there are more negative than positive values.
        let min = -factor as f32;
        let max = (factor - 1) as f32;

        if int_value < min {
            return min;
        } else if int_value > max {
            return max;
        }
        int_value
    }

    pub fn f32_to_s32(&mut self, samples: &[f32]) -> Vec<i32> {
        samples
            .iter()
            .map(|sample| self.scale(*sample, 0x80000000) as i32)
            .collect()
    }

    // S24 is 24-bit PCM packed in an upper 32-bit word
    pub fn f32_to_s24(&mut self, samples: &[f32]) -> Vec<i32> {
        samples
            .iter()
            .map(|sample| self.clamping_scale(*sample, 0x800000) as i32)
            .collect()
    }

    // S24_3 is 24-bit PCM in a 3-byte array
    pub fn f32_to_s24_3(&mut self, samples: &[f32]) -> Vec<i24> {
        samples
            .iter()
            .map(|sample| {
                // Not as DRY as calling f32_to_s24 first, but this saves iterating
                // over all samples twice.
                let int_value = self.clamping_scale(*sample, 0x800000) as i32;
                i24::from_s24(int_value)
            })
            .collect()
    }

    pub fn f32_to_s16(&mut self, samples: &[f32]) -> Vec<i16> {
        samples
            .iter()
            .map(|sample| self.scale(*sample, 0x8000) as i16)
            .collect()
    }
}
