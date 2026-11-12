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

/// Input frames each output frame is made from when the input rate is at or
/// below the output rate. A hundred and twenty-eight keep the passband flat
/// to 20 kHz and what would alias or image some 80 dB down. Downsampling
/// lengthens the filter by the rate ratio, so the transition band stays as
/// narrow in output hertz at 96 or 192 kHz as at 48 kHz.
const BASE_TAPS: usize = 128;

/// The most filter phases kept in the coefficient table. Common rate pairs
/// need at most 441 (8 kHz into 44.1 kHz). A rarer pair that needs more, such
/// as 48,001 Hz, which needs 44,100, interpolates between neighbouring phases
/// instead, so the table stays small whatever the input rate.
const MAX_PHASES: usize = 512;

pub struct Resampler {
    up: usize,
    down: usize,
    channels: usize,
    /// Coefficients in each phase.
    taps: usize,
    /// Phases in `table`. `up` when it fits under [`MAX_PHASES`], when every
    /// output position has a phase of its own.
    phases: usize,
    /// `phases` rows of `taps` coefficients, each normalised to unit gain,
    /// plus one more row when interpolating, for the last phase to lean on.
    table: Vec<f64>,
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
        let stretch = (down as f64 / up as f64).max(1.0);
        let taps = ((BASE_TAPS as f64 * stretch).ceil() as usize).next_multiple_of(2);
        let phases = up.min(MAX_PHASES);
        let mut resampler = Self {
            up,
            down,
            channels,
            taps,
            phases,
            table: kernel(up, down, taps, phases),
            input: Vec::new(),
            next: 0,
            phase: 0,
            input_frames: 0,
            output_frames: 0,
            finished: false,
        };
        resampler.reset();
        Some(resampler)
    }

    /// Clear the filter history after seeking or skipping corrupted input.
    pub fn reset(&mut self) {
        let half = self.taps / 2;
        self.input.clear();
        self.input.resize((half - 1) * self.channels, 0.0);
        self.next = half - 1;
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
            .resize(self.input.len() + self.taps / 2 * self.channels, 0.0);
        self.convert(Some(output_frames))
    }

    fn convert(&mut self, output_limit: Option<u64>) -> Vec<f64> {
        let half = self.taps / 2;
        let frames = self.input.len() / self.channels;
        let available = (frames.saturating_sub(self.next + half) as u64 * self.up as u64)
            .div_ceil(self.down as u64) as usize;
        let mut out = Vec::with_capacity(available * self.channels);
        while self.next + half < frames
            && output_limit.is_none_or(|limit| self.output_frames < limit)
        {
            let window = &self.input[(self.next + 1 - half) * self.channels..];
            if self.phases == self.up {
                let taps = self.row(self.phase);
                for channel in 0..self.channels {
                    out.push(dot(taps, window, channel, self.channels));
                }
            } else {
                // Sit between two of the table's phases and blend their outputs.
                let position = self.phase as u64 * self.phases as u64;
                let row = (position / self.up as u64) as usize;
                let fraction = (position % self.up as u64) as f64 / self.up as f64;
                let (before, after) = (self.row(row), self.row(row + 1));
                for channel in 0..self.channels {
                    let a = dot(before, window, channel, self.channels);
                    let b = dot(after, window, channel, self.channels);
                    out.push(a + (b - a) * fraction);
                }
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

    fn row(&self, row: usize) -> &[f64] {
        &self.table[row * self.taps..(row + 1) * self.taps]
    }
}

/// One channel of `input`, interleaved over `channels`, weighted by `taps`.
#[inline]
fn dot(taps: &[f64], input: &[f64], channel: usize, channels: usize) -> f64 {
    taps.iter()
        .zip(input[channel..].iter().step_by(channels))
        .map(|(tap, sample)| tap * sample)
        .sum()
}

/// The taps for every phase: a sinc cut just under the lower of the two
/// Nyquist limits, under a Blackman window, each phase scaled to unit
/// gain so a steady level comes out at the level it went in. When the table
/// holds fewer phases than `up`, one more row closes the last interval.
fn kernel(up: usize, down: usize, taps: usize, phases: usize) -> Vec<f64> {
    let half = (taps / 2) as f64;
    let cutoff = 0.475 * (up as f64 / down as f64).min(1.0);
    let rows = if phases == up { phases } else { phases + 1 };
    let mut table = Vec::with_capacity(rows * taps);
    for row in 0..rows {
        let offset = row as f64 / phases as f64;
        let start = table.len();
        for k in 0..taps {
            let u = offset + half - 1.0 - k as f64;
            let x = u / half;
            let window = 0.42 + 0.5 * (PI * x).cos() + 0.08 * (2.0 * PI * x).cos();
            let angle = 2.0 * PI * cutoff * u;
            let sinc = if u == 0.0 { 1.0 } else { angle.sin() / angle };
            table.push(2.0 * cutoff * sinc * window);
        }
        let sum: f64 = table[start..].iter().sum();
        for tap in &mut table[start..] {
            *tap /= sum;
        }
    }
    table
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
    fn uncommon_rates_interpolate_phases_from_a_bounded_table() {
        let resampler = Resampler::new(48001, 44100, 2).unwrap();
        assert_eq!(resampler.up, 44100);
        assert!(resampler.phases < resampler.up);
        assert!(resampler.table.len() <= (MAX_PHASES + 1) * resampler.taps);
        check(48001, 44100, 0.005);
        check(8001, 44100, 0.02);
    }
}
