// Local-file playback originally adapted from spiraljng/librespot commit
// a4bc693865a2a88ed056b2a0c7f75cb2e6b695a0 (MIT), by arlo (@spiraljng).
// Channel and spectral regression coverage contributed by Carmine Paolino (@crmne).

//! Streaming sample rate conversion into the player's fixed rate.
//!
//! Rubato owns the filter and interpolation. This adapter batches interleaved
//! decoder packets, removes the library's startup delay and flushes its tail
//! without extending the source duration. Native-rate audio bypasses it.

use audioadapter_buffers::direct::InterleavedSlice;
use rubato::{Async, FixedAsync, Resampler as _, SincInterpolationParameters, WindowFunction};

/// A fixed input block keeps buffering independent of container packet sizes.
const CHUNK_FRAMES: usize = 2048;

pub struct Resampler {
    inner: Async<f64>,
    from_hz: u32,
    to_hz: u32,
    channels: usize,
    input: Vec<f64>,
    output: Vec<f64>,
    delay_left: usize,
    input_frames: u64,
    output_frames: u64,
    finished: bool,
}

impl Resampler {
    /// `None` when the rates agree or the format is outside the decoder's limits.
    pub fn new(from_hz: u32, to_hz: u32, channels: usize) -> Option<Self> {
        if from_hz == to_hz
            || !(8000..=384000).contains(&from_hz)
            || !(8000..=384000).contains(&to_hz)
            || channels == 0
        {
            return None;
        }
        let ratio = f64::from(to_hz) / f64::from(from_hz);
        // Preserve a narrow transition band at high input rates. Rubato selects
        // the cutoff for this window and rounds the filter length to SIMD width.
        // Its 128-phase table stays bounded even for uncommon rates like 48,001 Hz.
        let taps = (256.0 / ratio.min(1.0)).ceil() as usize;
        let inner = Async::new_sinc(
            ratio,
            1.0,
            &SincInterpolationParameters::new(taps, WindowFunction::BlackmanHarris2),
            CHUNK_FRAMES,
            channels,
            FixedAsync::Input,
        )
        .expect("valid fixed resampling parameters");
        // The first generated sample is already one output step into the
        // stream. Keep the residual alignment within one output frame.
        let delay_left = inner.output_delay().saturating_sub(1);
        let output = vec![0.0; inner.output_frames_max() * channels];
        Some(Self {
            inner,
            from_hz,
            to_hz,
            channels,
            input: Vec::with_capacity(CHUNK_FRAMES * channels),
            output,
            delay_left,
            input_frames: 0,
            output_frames: 0,
            finished: false,
        })
    }

    /// Discard filter history and pending audio after seeking or corrupt input.
    pub fn reset(&mut self) {
        self.inner.reset();
        self.input.clear();
        self.delay_left = self.inner.output_delay().saturating_sub(1);
        self.input_frames = 0;
        self.output_frames = 0;
        self.finished = false;
    }

    /// Convert complete interleaved frames, retaining at most one input block.
    pub fn process(&mut self, mut samples: &[f64]) -> Vec<f64> {
        debug_assert_eq!(samples.len() % self.channels, 0);
        debug_assert!(!self.finished);
        self.input_frames += (samples.len() / self.channels) as u64;
        let mut out = Vec::new();
        let block_samples = CHUNK_FRAMES * self.channels;
        while !samples.is_empty() {
            let count = (block_samples - self.input.len()).min(samples.len());
            self.input.extend_from_slice(&samples[..count]);
            samples = &samples[count..];
            if self.input.len() == block_samples {
                self.convert(&mut out);
            }
        }
        out
    }

    /// Flush with silence, emitting exactly the source duration, once.
    pub fn finish(&mut self) -> Vec<f64> {
        if self.finished {
            return Vec::new();
        }
        self.finished = true;
        let mut out = Vec::new();
        while self.output_frames < self.total_output_frames() {
            self.input.resize(CHUNK_FRAMES * self.channels, 0.0);
            self.convert(&mut out);
        }
        out
    }

    fn total_output_frames(&self) -> u64 {
        (self.input_frames * u64::from(self.to_hz)).div_ceil(u64::from(self.from_hz))
    }

    /// Buffers are sized from Rubato's limits and always contain one full block.
    fn convert(&mut self, out: &mut Vec<f64>) {
        let input = InterleavedSlice::new(&self.input, self.channels, CHUNK_FRAMES)
            .expect("complete interleaved input block");
        let output_frames = self.output.len() / self.channels;
        let mut output = InterleavedSlice::new_mut(&mut self.output, self.channels, output_frames)
            .expect("preallocated interleaved output block");
        let (_, produced) = self
            .inner
            .process_into_buffer(&input, &mut output, None)
            .expect("buffers sized to Rubato's input and output limits");
        let skip = self.delay_left.min(produced);
        self.delay_left -= skip;
        let available = produced - skip;
        let remaining = self.total_output_frames() - self.output_frames;
        let frames = remaining.min(available as u64) as usize;
        out.extend_from_slice(&self.output[skip * self.channels..(skip + frames) * self.channels]);
        self.output_frames += frames as u64;
        self.input.clear();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    /// A stereo tone at half scale.
    fn tone(hz: f64, rate: u32, frames: usize) -> Vec<f64> {
        (0..frames)
            .flat_map(|i| {
                let sample = 0.5 * (2.0 * PI * hz * i as f64 / f64::from(rate)).sin();
                [sample, sample]
            })
            .collect()
    }

    fn check(from: u32, to: u32, tolerance: f64) {
        let mut resampler = Resampler::new(from, to, 2).unwrap();
        let mut out = resampler.process(&tone(1000.0, from, from as usize));
        out.extend(resampler.finish());
        let frames = out.len() / 2;
        assert_eq!(frames, to as usize);
        // Rubato reports delay in whole output frames. Allow at most one frame
        // of residual phase, then compare every sample to catch pitch or gain
        // errors without assuming the old filter's exact fractional alignment.
        let ideal = |n: usize, shift: f64| {
            0.5 * (2.0 * PI * 1000.0 * (n as f64 + shift) / f64::from(to)).sin()
        };
        let shift = (-128..=128)
            .map(|step| f64::from(step) / 128.0)
            .min_by(|a, b| {
                let error = |shift| {
                    (200..frames - 200)
                        .step_by(97)
                        .map(|n| (out[2 * n] - ideal(n, shift)).powi(2))
                        .sum::<f64>()
                };
                error(*a).total_cmp(&error(*b))
            })
            .unwrap();
        let mut worst = 0.0f64;
        for n in 200..frames - 200 {
            worst = worst.max((out[2 * n] - ideal(n, shift)).abs());
            assert_eq!(out[2 * n], out[2 * n + 1]);
        }
        assert!(worst < tolerance, "{from} to {to}: off by {worst}");
    }

    #[test]
    fn the_same_rate_needs_nothing() {
        assert!(Resampler::new(44100, 44100, 2).is_none());
    }

    #[test]
    fn a_tone_keeps_its_pitch_and_level_when_downsampling() {
        check(48000, 44100, 0.005);
    }

    #[test]
    fn a_tone_keeps_its_pitch_and_level_when_upsampling() {
        check(22050, 44100, 0.02);
    }

    #[test]
    fn chunking_makes_no_difference() {
        let input = tone(440.0, 48000, 20_000);
        let mut resampler = Resampler::new(48000, 44100, 2).unwrap();
        let mut whole = resampler.process(&input);
        whole.extend(resampler.finish());
        let mut chunked = Resampler::new(48000, 44100, 2).unwrap();
        let mut out = Vec::new();
        let mut at = 0;
        for size in [2, 14, 200, 3256, 6, 1000, 2, 8000].iter().cycle() {
            if at >= input.len() {
                break;
            }
            let end = (at + size).min(input.len());
            out.extend(chunked.process(&input[at..end]));
            at = end;
        }
        out.extend(chunked.finish());
        assert_eq!(out, whole);
    }

    #[test]
    fn finishing_short_inputs_preserves_their_duration() {
        for (from, to) in [(48000, 44100), (22050, 44100), (96000, 44100)] {
            for frames in 0..128 {
                let mut resampler = Resampler::new(from, to, 2).unwrap();
                let mut output = resampler.process(&vec![0.25; frames * 2]);
                output.extend(resampler.finish());
                let expected = (frames * to as usize).div_ceil(from as usize);
                assert_eq!(output.len(), expected * 2, "{frames} frames at {from}");
                assert!(resampler.finish().is_empty());
            }
        }
    }

    #[test]
    fn reset_discards_audio_before_a_seek_and_reopens_a_finished_stream() {
        let after_seek = tone(880.0, 48000, 1000);
        let mut resampler = Resampler::new(48000, 44100, 2).unwrap();
        resampler.process(&tone(440.0, 48000, 2000));
        resampler.finish();
        resampler.reset();
        let mut output = resampler.process(&after_seek);
        output.extend(resampler.finish());

        let mut fresh = Resampler::new(48000, 44100, 2).unwrap();
        let mut expected = fresh.process(&after_seek);
        expected.extend(fresh.finish());
        assert_eq!(output, expected);
    }

    #[test]
    fn channels_do_not_mix() {
        let input: Vec<_> = tone(440.0, 48000, 1000)
            .chunks_exact(2)
            .flat_map(|frame| [frame[0], 0.0])
            .collect();
        let mut resampler = Resampler::new(48000, 44100, 2).unwrap();
        let mut output = resampler.process(&input);
        output.extend(resampler.finish());
        assert!(output.chunks_exact(2).all(|frame| frame[1] == 0.0));
        assert!(output.chunks_exact(2).any(|frame| frame[0].abs() > 0.1));
    }

    /// The output's RMS level over its steady middle, after `from` frames of
    /// a stereo tone at `hz`.
    fn steady_rms(hz: f64, from: u32) -> f64 {
        let mut resampler = Resampler::new(from, 44100, 2).unwrap();
        let mut out = resampler.process(&tone(hz, from, from as usize));
        out.extend(resampler.finish());
        let middle = &out[2000..out.len() - 2000];
        (middle.iter().map(|sample| sample * sample).sum::<f64>() / middle.len() as f64).sqrt()
    }

    #[test]
    fn the_passband_is_flat_and_tones_above_the_output_nyquist_limit_do_not_alias() {
        // A half-scale tone has an RMS level of 0.354. Each of these would fold
        // back into the audible band if the filter let it through.
        for (hz, from) in [(23_500.0, 48000), (25_000.0, 96000), (30_000.0, 192_000)] {
            let level = steady_rms(hz, from);
            let db = 20.0 * (level / 0.354).log10();
            assert!(db < -75.0, "{hz} Hz at {from} Hz leaks at {db:.1} dB");
        }
        for from in [22050, 48000, 96000, 192_000, 384_000] {
            for hz in [1000.0, 10_000.0, 20_000.0] {
                if hz < f64::from(from) * 0.45 {
                    let level = steady_rms(hz, from);
                    let db = 20.0 * (level / 0.354).log10();
                    assert!(
                        db.abs() < 0.05,
                        "{hz} Hz at {from} Hz comes out at {db:.2} dB"
                    );
                }
            }
        }
    }

    #[test]
    fn high_rates_downsample_accurately() {
        check(96000, 44100, 0.005);
        check(192_000, 44100, 0.005);
        check(384_000, 44100, 0.005);
    }

    #[test]
    fn uncommon_rates_use_the_same_bounded_buffers() {
        let common = Resampler::new(48000, 44100, 2).unwrap();
        let uncommon = Resampler::new(48001, 44100, 2).unwrap();
        assert_eq!(common.input.capacity(), uncommon.input.capacity());
        assert_eq!(common.output.len(), uncommon.output.len());
        check(48001, 44100, 0.005);
        check(8001, 44100, 0.02);
    }
}
