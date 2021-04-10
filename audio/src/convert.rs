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

// Losslessly represent [-1.0, 1.0] to [$type::MIN, $type::MAX] while maintaining DC linearity.
macro_rules! convert_samples_to {
    ($type: ident, $samples: expr) => {
        convert_samples_to!($type, $samples, 0)
    };
    ($type: ident, $samples: expr, $drop_bits: expr) => {
        $samples
            .iter()
            .map(|sample| {
                // Losslessly represent [-1.0, 1.0] to [$type::MIN, $type::MAX]
                // while maintaining DC linearity. There is nothing to be gained
                // by doing this in f64, as the significand of a f32 is 24 bits,
                // just like the maximum bit depth we are converting to.
                let int_value = *sample * (std::$type::MAX as f32 + 0.5) - 0.5;

                // Casting floats to ints truncates by default, which results
                // in larger quantization error than rounding arithmetically.
                // Flooring is faster, but again with larger error.
                int_value.round() as $type >> $drop_bits
            })
            .collect()
    };
}

pub struct SamplesConverter {}
impl SamplesConverter {
    pub fn to_s32(samples: &[f32]) -> Vec<i32> {
        convert_samples_to!(i32, samples)
    }

    pub fn to_s24(samples: &[f32]) -> Vec<i32> {
        convert_samples_to!(i32, samples, 8)
    }

    pub fn to_s24_3(samples: &[f32]) -> Vec<i24> {
        Self::to_s32(samples)
            .iter()
            .map(|sample| i24::pcm_from_i32(*sample))
            .collect()
    }

    pub fn to_s16(samples: &[f32]) -> Vec<i16> {
        convert_samples_to!(i16, samples)
    }
}
