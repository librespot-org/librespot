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

pub struct Converter {
    ditherer: Box<dyn Ditherer>,
    noise_shaper: Box<dyn NoiseShaper>,
}

impl Converter {
    pub fn new(ditherer: Box<dyn Ditherer>, noise_shaper: Box<dyn NoiseShaper>) -> Self {
        info!(
            "Converting with ditherer: {} and noise shaper: {}",
            ditherer, noise_shaper
        );
        Self {
            ditherer,
            noise_shaper,
        }
    }

    // Denormalize, dither and shape noise
    pub fn scale(&mut self, sample: f32, max: i32) -> f32 {
        // Losslessly represent [-1.0..1.0] as some signed integer value while
        // maintaining DC linearity. There is nothing to be gained by doing
        // this in f64, as the significand of a f32 is 24 bits, just like the
        // maximum bit depth we are converting to. Taken from:
        // http://blog.bjornroche.com/2009/12/int-float-int-its-jungle-out-there.html
        let int_value = sample * (max as f32 + 0.5) - 0.5;
        self.shaped_dither(int_value)
    }

    // Special case for samples packed in a word of greater bit depth (e.g.
    // S24): clamp between min and max to ensure that the most significant
    // byte is zero. Otherwise, dithering may cause an overflow. This is not
    // necessary for other formats, because casting to integer will saturate
    // to the bounds of the primitive.
    pub fn clamping_scale(&mut self, sample: f32, min: i32, max: i32) -> f32 {
        let int_value = self.scale(sample, max);

        let min = min as f32;
        let max = max as f32;
        if int_value < min {
            return min;
        } else if int_value > max {
            return max;
        }
        int_value
    }

    fn shaped_dither(&mut self, sample: f32) -> f32 {
        let noise = self.ditherer.noise(sample);
        self.noise_shaper.shape(sample, noise)
    }

    // https://doc.rust-lang.org/nomicon/casts.html: casting float to integer
    // rounds towards zero, then saturates. Ideally halves should round to even to
    // prevent any bias, but since it is extremely unlikely that a float has
    // *exactly* .5 as fraction, this should be more than precise enough.
    pub fn f32_to_s32(&mut self, samples: &[f32]) -> Vec<i32> {
        samples
            .iter()
            .map(|sample| self.scale(*sample, std::i32::MAX) as i32)
            .collect()
    }

    // S24 is 24-bit PCM packed in an upper 32-bit word
    pub fn f32_to_s24(&mut self, samples: &[f32]) -> Vec<i32> {
        samples
            .iter()
            .map(|sample| self.clamping_scale(*sample, i24::MIN, i24::MAX) as i32)
            .collect()
    }

    // S24_3 is 24-bit PCM in a 3-byte array
    pub fn f32_to_s24_3(&mut self, samples: &[f32]) -> Vec<i24> {
        samples
            .iter()
            .map(|sample| {
                // Not as DRY as calling f32_to_s24 first, but this saves iterating
                // over all samples twice.
                let int_value = self.clamping_scale(*sample, i24::MIN, i24::MAX) as i32;
                i24::from_s24(int_value)
            })
            .collect()
    }

    pub fn f32_to_s16(&mut self, samples: &[f32]) -> Vec<i16> {
        samples
            .iter()
            .map(|sample| self.scale(*sample, std::i16::MAX as i32) as i16)
            .collect()
    }
}
