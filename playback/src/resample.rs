// Adapted from spiraljng/librespot commit a4bc693865a2a88ed056b2a0c7f75cb2e6b695a0 (MIT).
// Original local-file resampling work by arlo (@spiraljng).

//! Sample rate conversion into the player's rate.
//!
//! Spotify streams arrive at [`crate::SAMPLE_RATE`] and the playback pipeline is
//! built around it, but a local file can carry any rate. Converting here
//! lets those files play instead of being rejected, and keeps every
//! downstream stage (sink, mixer, position tracking) on one rate.
//!
//! It uses a polyphase windowed-sinc filter and computes only the taps
//! needed for each output sample.

use std::f64::consts::PI;

/// Input samples each output sample is made from. Sixty-four give a
/// passband flat to about 19 kHz and images under the window's floor.
const TAPS: usize = 64;

pub struct Resampler {
    up: usize,
    down: usize,
    channels: usize,
    /// `up` phases of `TAPS` coefficients each, normalised to unit gain.
    taps: Vec<f64>,
    /// Interleaved input frames still in reach: the tail of what came
    /// before, then whatever has not produced its outputs yet.
    input: Vec<f64>,
    /// The frame in `input` the next output sample sits on or just after.
    next: usize,
    /// How far past `next` the output sits, in steps of `1 / up`.
    phase: usize,
    input_frames: u64,
    output_frames: u64,
    finished: bool,
}

impl Resampler {
    /// `None` when the rates agree and nothing needs doing.
    pub fn new(from_hz: u32, to_hz: u32, channels: usize) -> Option<Self> {
        if from_hz == to_hz || from_hz == 0 || to_hz == 0 || channels == 0 {
            return None;
        }
        let divisor = gcd(from_hz, to_hz);
        let up = (to_hz / divisor) as usize;
        let down = (from_hz / divisor) as usize;
        let half = TAPS / 2;
        Some(Self {
            up,
            down,
            channels,
            taps: kernel(up, down),
            input: vec![0.0; (half - 1) * channels],
            next: half - 1,
            phase: 0,
            input_frames: 0,
            output_frames: 0,
            finished: false,
        })
    }

    /// Clear the filter history after seeking or skipping corrupted input.
    pub fn reset(&mut self) {
        self.input.clear();
        self.input.resize((TAPS / 2 - 1) * self.channels, 0.0);
        self.next = TAPS / 2 - 1;
        self.phase = 0;
        self.input_frames = 0;
        self.output_frames = 0;
        self.finished = false;
    }

    /// Converts complete interleaved frames, retaining the lookahead needed
    /// by the symmetric filter until the following chunk arrives.
    pub fn process(&mut self, samples: &[f64]) -> Vec<f64> {
        debug_assert_eq!(samples.len() % self.channels, 0);
        debug_assert!(!self.finished);
        self.input_frames += (samples.len() / self.channels) as u64;
        self.input.extend_from_slice(samples);
        self.convert(None)
    }

    /// Flush the filter's lookahead with silence, without extending the audio's
    /// duration. Repeated calls produce no additional samples.
    pub fn finish(&mut self) -> Vec<f64> {
        if self.finished {
            return Vec::new();
        }
        self.finished = true;
        let output_frames = (self.input_frames * self.up as u64).div_ceil(self.down as u64);
        self.input
            .resize(self.input.len() + TAPS / 2 * self.channels, 0.0);
        self.convert(Some(output_frames))
    }

    fn convert(&mut self, output_limit: Option<u64>) -> Vec<f64> {
        let half = TAPS / 2;
        let frames = self.input.len() / self.channels;
        let mut out = Vec::new();
        while self.next + half < frames
            && output_limit.is_none_or(|limit| self.output_frames < limit)
        {
            let taps = &self.taps[self.phase * TAPS..(self.phase + 1) * TAPS];
            let start = (self.next + 1 - half) * self.channels;
            for channel in 0..self.channels {
                let sum: f64 = taps
                    .iter()
                    .enumerate()
                    .map(|(k, tap)| self.input[start + k * self.channels + channel] * tap)
                    .sum();
                out.push(sum);
            }
            let position = self.phase + self.down;
            self.next += position / self.up;
            self.phase = position % self.up;
            self.output_frames += 1;
        }
        // Keep only the frames the next output still reaches back to.
        let keep_from = (self.next + 1 - half).min(frames);
        self.input.drain(..keep_from * self.channels);
        self.next -= keep_from;
        out
    }
}

/// The taps for every phase: a sinc cut just under the lower of the two
/// Nyquist limits, under a Blackman window, each phase scaled to unit
/// gain so a steady level comes out at the level it went in.
fn kernel(up: usize, down: usize) -> Vec<f64> {
    let half = (TAPS / 2) as f64;
    let cutoff = 0.475 * (up as f64 / down as f64).min(1.0);
    let mut taps = Vec::with_capacity(up * TAPS);
    for phase in 0..up {
        let offset = phase as f64 / up as f64;
        let start = taps.len();
        for k in 0..TAPS {
            let u = offset + half - 1.0 - k as f64;
            let x = u / half;
            let window = 0.42 + 0.5 * (PI * x).cos() + 0.08 * (2.0 * PI * x).cos();
            let angle = 2.0 * PI * cutoff * u;
            let sinc = if u == 0.0 { 1.0 } else { angle.sin() / angle };
            taps.push(2.0 * cutoff * sinc * window);
        }
        let sum: f64 = taps[start..].iter().sum();
        for tap in &mut taps[start..] {
            *tap /= sum;
        }
    }
    taps
}

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 { a } else { gcd(b, a % b) }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A stereo tone at half scale.
    fn tone(hz: f64, rate: u32, frames: usize) -> Vec<f64> {
        (0..frames)
            .flat_map(|i| {
                let sample = 0.5 * (2.0 * PI * hz * i as f64 / f64::from(rate)).sin();
                [sample, sample]
            })
            .collect()
    }

    /// Where output frame `n` sits on the input's clock, in input frames:
    /// the filter is symmetric about its output, so there is no delay.
    fn input_time(resampler: &Resampler, n: usize) -> f64 {
        n as f64 * resampler.down as f64 / resampler.up as f64
    }

    fn check(from: u32, to: u32, tolerance: f64) {
        let mut resampler = Resampler::new(from, to, 2).unwrap();
        let mut out = resampler.process(&tone(1000.0, from, from as usize));
        out.extend(resampler.finish());
        let frames = out.len() / 2;
        assert_eq!(frames, to as usize);
        let mut worst = 0.0f64;
        for n in 200..frames - 200 {
            let t = input_time(&resampler, n) / f64::from(from);
            let ideal = 0.5 * (2.0 * PI * 1000.0 * t).sin();
            worst = worst.max((out[2 * n] - ideal).abs());
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
}
