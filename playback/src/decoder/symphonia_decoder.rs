use std::{io, time::Duration};

use symphonia::core::{
    audio::{Channels, SampleBuffer},
    codecs::{Decoder, DecoderOptions},
    errors::Error,
    formats::{FormatOptions, SeekMode, SeekTo},
    io::{MediaSource, MediaSourceStream, MediaSourceStreamOptions},
    meta::{MetadataOptions, StandardTagKey, Value},
    probe::{Hint, ProbeResult},
};

use super::{AudioDecoder, AudioPacket, AudioPacketPosition, DecoderError, DecoderResult};

use crate::{
    NUM_CHANNELS, PAGES_PER_MS, SAMPLE_RATE, player::NormalisationData, resample::Resampler,
    symphonia_util,
};

pub struct SymphoniaDecoder {
    probe_result: ProbeResult,
    decoder: Box<dyn Decoder>,
    sample_buffer: Option<SampleBuffer<f64>>,
    /// The left and right gain of each decoded channel, when the source is
    /// not already stereo.
    stereo_gains: Option<Vec<[f64; 2]>>,
    /// The packet as stereo, reused between packets.
    stereo: Vec<f64>,
    resampler: Option<Resampler>,
    resampled_start_ms: Option<u32>,
    resampled_frames: u64,
}

#[derive(Default)]
pub(crate) struct LocalFileMetadata {
    pub name: Option<String>,
    pub language: Option<String>,
    pub album: Option<String>,
    pub artists: Option<String>,
    pub album_artists: Option<String>,
    pub number: Option<u32>,
    pub disc_number: Option<u32>,
}

impl SymphoniaDecoder {
    /// Decode audio at 8,000 through 384,000 Hz in any channel layout, as
    /// 44.1 kHz stereo: mono is played on both sides, more channels are mixed
    /// down, and other rates are resampled. Bounding input rates limits
    /// per-packet expansion and filter resources.
    pub fn new<R>(input: R, hint: Hint) -> DecoderResult<Self>
    where
        R: MediaSource + 'static,
    {
        let mss_opts = MediaSourceStreamOptions {
            buffer_len: librespot_audio::AudioFetchParams::get().minimum_download_size,
        };
        let mss = MediaSourceStream::new(Box::new(input), mss_opts);

        let format_opts = FormatOptions {
            enable_gapless: true,
            ..Default::default()
        };
        let metadata_opts: MetadataOptions = Default::default();

        let probe_result =
            symphonia::default::get_probe().format(&hint, mss, &format_opts, &metadata_opts)?;

        let format = &probe_result.format;

        let track = format.default_track().ok_or_else(|| {
            DecoderError::SymphoniaDecoder("Could not retrieve default track".into())
        })?;

        let decoder_opts: DecoderOptions = Default::default();

        let decoder = symphonia::default::get_codecs().make(&track.codec_params, &decoder_opts)?;

        let rate = decoder.codec_params().sample_rate.ok_or_else(|| {
            DecoderError::SymphoniaDecoder("Could not retrieve sample rate".into())
        })?;

        if !(8000..=384000).contains(&rate) {
            return Err(DecoderError::SymphoniaDecoder(format!(
                "Unsupported sample rate: {rate}"
            )));
        }

        let channels = decoder.codec_params().channels.ok_or_else(|| {
            DecoderError::SymphoniaDecoder("Could not retrieve channel configuration".into())
        })?;
        if channels.count() == 0 {
            return Err(DecoderError::SymphoniaDecoder(format!(
                "Unsupported number of channels: {channels}"
            )));
        }
        let stereo_gains = (channels != (Channels::FRONT_LEFT | Channels::FRONT_RIGHT))
            .then(|| stereo_gains(channels));

        Ok(Self {
            probe_result,
            decoder,
            // We set the sample buffer when decoding the first full packet,
            // whose duration is also the ideal sample buffer size.
            sample_buffer: None,
            stereo_gains,
            stereo: Vec::new(),
            resampler: Resampler::new(rate, SAMPLE_RATE, NUM_CHANNELS as usize),
            resampled_start_ms: None,
            resampled_frames: 0,
        })
    }

    pub fn normalisation_data(&mut self) -> Option<NormalisationData> {
        let metadata = symphonia_util::get_latest_metadata(&mut self.probe_result)?;
        let tags = metadata.current()?.tags();

        if tags.is_empty() {
            None
        } else {
            let mut data = NormalisationData::default();

            for tag in tags {
                if let Value::Float(value) = tag.value {
                    match tag.std_key {
                        Some(StandardTagKey::ReplayGainAlbumGain) => data.album_gain_db = value,
                        Some(StandardTagKey::ReplayGainAlbumPeak) => data.album_peak = value,
                        Some(StandardTagKey::ReplayGainTrackGain) => data.track_gain_db = value,
                        Some(StandardTagKey::ReplayGainTrackPeak) => data.track_peak = value,
                        _ => (),
                    }
                }
            }

            Some(data)
        }
    }

    pub(crate) fn local_file_metadata(&mut self) -> Option<LocalFileMetadata> {
        let metadata = symphonia_util::get_latest_metadata(&mut self.probe_result)?;
        let tags = metadata.current()?.tags();
        let mut metadata = LocalFileMetadata::default();

        for tag in tags {
            if let Value::String(value) = &tag.value {
                match tag.std_key {
                    // We could possibly use mem::take here to avoid cloning, but that risks leaving
                    // the audio item metadata in a bad state.
                    Some(StandardTagKey::TrackTitle) => metadata.name = Some(value.clone()),
                    Some(StandardTagKey::Language) => metadata.language = Some(value.clone()),
                    Some(StandardTagKey::Artist) => metadata.artists = Some(value.clone()),
                    Some(StandardTagKey::AlbumArtist) => {
                        metadata.album_artists = Some(value.clone())
                    }
                    Some(StandardTagKey::Album) => metadata.album = Some(value.clone()),
                    Some(StandardTagKey::TrackNumber) => {
                        metadata.number = match value.parse::<u32>() {
                            Ok(value) => Some(value),
                            Err(e) => {
                                warn!(
                                    "Failed to parse local file's track number tag '{value}': {e}"
                                );
                                None
                            }
                        }
                    }
                    Some(StandardTagKey::DiscNumber) => {
                        metadata.disc_number = match value.parse::<u32>() {
                            Ok(value) => Some(value),
                            Err(e) => {
                                warn!(
                                    "Failed to parse local file's disc number tag '{value}': {e}"
                                );
                                None
                            }
                        }
                    }
                    _ => (),
                }
            } else if let Value::UnsignedInt(value) = &tag.value {
                match tag.std_key {
                    Some(StandardTagKey::TrackNumber) => metadata.number = Some(*value as u32),
                    Some(StandardTagKey::DiscNumber) => metadata.disc_number = Some(*value as u32),
                    _ => (),
                }
            } else if let Value::SignedInt(value) = &tag.value {
                match tag.std_key {
                    Some(StandardTagKey::TrackNumber) => metadata.number = Some(*value as u32),
                    Some(StandardTagKey::DiscNumber) => metadata.disc_number = Some(*value as u32),
                    _ => (),
                }
            }
        }

        Some(metadata)
    }

    #[inline]
    fn ts_to_ms(&self, ts: u64) -> u32 {
        match self.decoder.codec_params().time_base {
            Some(time_base) => {
                let time = Duration::from(time_base.calc_time(ts));
                time.as_millis() as u32
            }
            // Fallback in the unexpected case that the format has no base time set.
            None => (ts as f64 * PAGES_PER_MS) as u32,
        }
    }

    fn resampled_packet(
        &mut self,
        samples: Vec<f64>,
        skipped: bool,
    ) -> Option<(AudioPacketPosition, AudioPacket)> {
        if samples.is_empty() {
            return None;
        }
        // The filter retains a few input frames between packets. Timestamp the
        // actual output, rather than the next container packet's input clock.
        let elapsed_ms = self.resampled_frames * 1000 / u64::from(SAMPLE_RATE);
        let position_ms = self
            .resampled_start_ms
            .unwrap_or_default()
            .saturating_add(elapsed_ms as u32);
        self.resampled_frames += (samples.len() / usize::from(NUM_CHANNELS)) as u64;
        Some((
            AudioPacketPosition {
                position_ms,
                skipped,
            },
            AudioPacket::Samples(samples),
        ))
    }
}

impl AudioDecoder for SymphoniaDecoder {
    fn seek(&mut self, position_ms: u32) -> Result<u32, DecoderError> {
        // "Saturate" the position_ms to the duration of the track if it exceeds it.
        let mut target = Duration::from_millis(position_ms.into());
        let codec_params = self.decoder.codec_params();
        if let (Some(time_base), Some(n_frames)) = (codec_params.time_base, codec_params.n_frames) {
            let duration = Duration::from(time_base.calc_time(n_frames));
            if target > duration {
                target = duration;
            }
        }

        // `track_id: None` implies the default track ID (of the container, not of Spotify).
        let seeked_to_ts = self.probe_result.format.seek(
            SeekMode::Accurate,
            SeekTo::Time {
                time: target.into(),
                track_id: None,
            },
        )?;

        // Seeking is a `FormatReader` operation, so the decoder cannot reliably
        // know when a seek took place. Reset it to avoid audio glitches.
        self.decoder.reset();
        if let Some(resampler) = &mut self.resampler {
            resampler.reset();
            self.resampled_start_ms = None;
            self.resampled_frames = 0;
        }

        Ok(self.ts_to_ms(seeked_to_ts.actual_ts))
    }

    fn next_packet(&mut self) -> DecoderResult<Option<(AudioPacketPosition, AudioPacket)>> {
        let mut skipped = false;

        loop {
            let packet = match self.probe_result.format.next_packet() {
                Ok(packet) => packet,
                Err(Error::IoError(err)) => {
                    if err.kind() == io::ErrorKind::UnexpectedEof {
                        let samples = self.resampler.as_mut().map(Resampler::finish);
                        return Ok(
                            samples.and_then(|samples| self.resampled_packet(samples, skipped))
                        );
                    } else {
                        return Err(DecoderError::SymphoniaDecoder(err.to_string()));
                    }
                }
                Err(err) => {
                    return Err(err.into());
                }
            };

            let position_ms = self.ts_to_ms(packet.ts());
            let packet_position = AudioPacketPosition {
                position_ms,
                skipped,
            };

            match self.decoder.decode(&packet) {
                Ok(decoded) => {
                    let sample_buffer = match self.sample_buffer.as_mut() {
                        Some(buffer) => buffer,
                        None => {
                            let spec = *decoded.spec();
                            let duration = decoded.capacity() as u64;
                            self.sample_buffer.insert(SampleBuffer::new(duration, spec))
                        }
                    };

                    sample_buffer.copy_interleaved_ref(decoded);
                    let samples = match &self.stereo_gains {
                        Some(gains) => {
                            mix_to_stereo(sample_buffer.samples(), gains, &mut self.stereo);
                            &self.stereo
                        }
                        None => sample_buffer.samples(),
                    };
                    if let Some(resampler) = &mut self.resampler {
                        self.resampled_start_ms.get_or_insert(position_ms);
                        let samples = resampler.process(samples);
                        if let Some(packet) = self.resampled_packet(samples, skipped) {
                            return Ok(Some(packet));
                        }
                    } else {
                        let samples = AudioPacket::Samples(samples.to_vec());
                        return Ok(Some((packet_position, samples)));
                    }
                }
                Err(Error::DecodeError(_)) => {
                    // The packet failed to decode due to corrupted or invalid data, get a new
                    // packet and try again.
                    warn!("Skipping malformed audio packet at {position_ms} ms");
                    if let Some(resampler) = &mut self.resampler {
                        resampler.reset();
                        self.resampled_start_ms = None;
                        self.resampled_frames = 0;
                    }
                    skipped = true;
                    continue;
                }
                Err(err) => return Err(err.into()),
            }
        }
    }
}

/// How much of each channel, in decoding order, goes to the left and the right.
///
/// Mono plays at full level on both sides. A wider layout follows the usual
/// downmix: front left and right go to their own side, the other channels on
/// one side go there 3 dB down, centre channels go to both 3 dB down, and the
/// low-frequency effects channel is left out. The gains are then scaled so
/// neither side can exceed full scale.
fn stereo_gains(channels: Channels) -> Vec<[f64; 2]> {
    use std::f64::consts::FRAC_1_SQRT_2;

    if channels.count() == 1 {
        return vec![[1.0, 1.0]];
    }
    let left = Channels::FRONT_LEFT_CENTRE
        | Channels::REAR_LEFT
        | Channels::SIDE_LEFT
        | Channels::TOP_FRONT_LEFT
        | Channels::TOP_REAR_LEFT
        | Channels::REAR_LEFT_CENTRE
        | Channels::FRONT_LEFT_WIDE
        | Channels::FRONT_LEFT_HIGH;
    let right = Channels::FRONT_RIGHT_CENTRE
        | Channels::REAR_RIGHT
        | Channels::SIDE_RIGHT
        | Channels::TOP_FRONT_RIGHT
        | Channels::TOP_REAR_RIGHT
        | Channels::REAR_RIGHT_CENTRE
        | Channels::FRONT_RIGHT_WIDE
        | Channels::FRONT_RIGHT_HIGH;
    let mut gains: Vec<[f64; 2]> = channels
        .iter()
        .map(|channel| {
            if channel == Channels::FRONT_LEFT {
                [1.0, 0.0]
            } else if channel == Channels::FRONT_RIGHT {
                [0.0, 1.0]
            } else if channel == Channels::LFE1 || channel == Channels::LFE2 {
                [0.0, 0.0]
            } else if left.contains(channel) {
                [FRAC_1_SQRT_2, 0.0]
            } else if right.contains(channel) {
                [0.0, FRAC_1_SQRT_2]
            } else {
                [FRAC_1_SQRT_2, FRAC_1_SQRT_2]
            }
        })
        .collect();
    let peak = (0..2)
        .map(|side| gains.iter().map(|gain| gain[side]).sum::<f64>())
        .fold(0.0, f64::max);
    if peak > 1.0 {
        for gain in gains.iter_mut().flatten() {
            *gain /= peak;
        }
    }
    gains
}

/// Mix interleaved frames of `gains.len()` channels into interleaved stereo.
fn mix_to_stereo(samples: &[f64], gains: &[[f64; 2]], stereo: &mut Vec<f64>) {
    stereo.clear();
    stereo.reserve(samples.len() / gains.len() * 2);
    for frame in samples.chunks_exact(gains.len()) {
        let (left, right) = frame
            .iter()
            .zip(gains)
            .fold((0.0, 0.0), |(left, right), (sample, gain)| {
                (left + sample * gain[0], right + sample * gain[1])
            });
        stereo.extend([left, right]);
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    fn decoder(rate: u32) -> SymphoniaDecoder {
        let input: &[u8] = match rate {
            44100 => include_bytes!("testdata/stereo-44100.flac"),
            48000 => include_bytes!("testdata/stereo-48000.flac"),
            _ => unreachable!(),
        };
        SymphoniaDecoder::new(Cursor::new(input.to_vec()), Hint::new()).unwrap()
    }

    fn decode_remaining(decoder: &mut SymphoniaDecoder) -> Vec<(u32, Vec<f64>)> {
        let mut packets = Vec::new();
        while let Some((position, packet)) = decoder.next_packet().unwrap() {
            assert!(!packet.is_empty());
            packets.push((position.position_ms, packet.samples().unwrap().to_vec()));
        }
        assert!(decoder.next_packet().unwrap().is_none());
        packets
    }

    #[test]
    fn native_rate_flac_keeps_the_existing_decode_path() {
        let mut decoder = decoder(44100);
        assert!(decoder.resampler.is_none());
        let packets = decode_remaining(&mut decoder);
        let samples: Vec<_> = packets
            .into_iter()
            .flat_map(|(_, samples)| samples)
            .collect();
        assert_eq!(samples.len(), 44100 * 2 * 2);
        assert!(samples.into_iter().all(|sample| sample == 0.0));
    }

    #[test]
    fn very_low_rate_flac_is_rejected_before_resampling() {
        let input = include_bytes!("testdata/stereo-1.flac");
        let result = SymphoniaDecoder::new(Cursor::new(input.to_vec()), Hint::new());
        assert!(matches!(
            result,
            Err(DecoderError::SymphoniaDecoder(message))
                if message == "Unsupported sample rate: 1"
        ));
    }

    #[test]
    fn resampled_flac_has_the_full_duration_and_continuous_output_timestamps() {
        let mut decoder = decoder(48000);
        assert!(decoder.resampler.is_some());
        let packets = decode_remaining(&mut decoder);
        let mut frames = 0;
        for (position_ms, samples) in &packets {
            assert_eq!(*position_ms as usize, frames * 1000 / SAMPLE_RATE as usize);
            assert!(
                samples
                    .iter()
                    .all(|sample| sample.is_finite() && sample.abs() < 0.3)
            );
            frames += samples.len() / 2;
        }
        // The fixture is exactly 12,000 frames, or 250 ms, at 48 kHz.
        assert_eq!(frames, 11025);
        assert!(
            packets
                .iter()
                .flat_map(|(_, samples)| samples)
                .any(|sample| sample.abs() > 0.2)
        );
    }

    #[test]
    fn seeking_discards_resampler_history_including_after_eof() {
        for read_to_end in [false, true] {
            let mut used = decoder(48000);
            if read_to_end {
                decode_remaining(&mut used);
            } else {
                used.next_packet().unwrap().unwrap();
            }
            let mut fresh = decoder(48000);
            let actual_ms = used.seek(100).unwrap();
            assert_eq!(actual_ms, fresh.seek(100).unwrap());
            let actual = decode_remaining(&mut used);
            assert!(!actual.is_empty());
            assert_eq!(actual[0].0, actual_ms);
            assert_eq!(actual, decode_remaining(&mut fresh));
        }
    }

    /// A 16-bit PCM WAV of `frames` frames, with `sample(frame, channel)` in
    /// each channel. A channel mask makes it WAVE_FORMAT_EXTENSIBLE.
    fn wav(
        rate: u32,
        channels: u16,
        mask: Option<u32>,
        frames: usize,
        sample: impl Fn(usize, usize) -> f64,
    ) -> Vec<u8> {
        let data_len = (frames * usize::from(channels) * 2) as u32;
        let fmt_len: u32 = if mask.is_some() { 40 } else { 16 };
        let mut out = Vec::new();
        out.extend(b"RIFF");
        out.extend((4 + 8 + fmt_len + 8 + data_len).to_le_bytes());
        out.extend(b"WAVEfmt ");
        out.extend(fmt_len.to_le_bytes());
        out.extend((if mask.is_some() { 0xFFFE_u16 } else { 1 }).to_le_bytes());
        out.extend(channels.to_le_bytes());
        out.extend(rate.to_le_bytes());
        out.extend((rate * u32::from(channels) * 2).to_le_bytes());
        out.extend((channels * 2).to_le_bytes());
        out.extend(16_u16.to_le_bytes());
        if let Some(mask) = mask {
            out.extend(22_u16.to_le_bytes());
            out.extend(16_u16.to_le_bytes());
            out.extend(mask.to_le_bytes());
            // KSDATAFORMAT_SUBTYPE_PCM
            out.extend([
                0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x80, 0x00, 0x00, 0xAA, 0x00, 0x38,
                0x9B, 0x71,
            ]);
        }
        out.extend(b"data");
        out.extend(data_len.to_le_bytes());
        for frame in 0..frames {
            for channel in 0..usize::from(channels) {
                let value = (sample(frame, channel) * 32767.0).round() as i16;
                out.extend(value.to_le_bytes());
            }
        }
        out
    }

    fn wav_decoder(bytes: Vec<u8>) -> SymphoniaDecoder {
        let mut hint = Hint::new();
        hint.with_extension("wav");
        SymphoniaDecoder::new(Cursor::new(bytes), hint).unwrap()
    }

    fn sine(hz: f64, seconds: f64) -> f64 {
        0.5 * (2.0 * std::f64::consts::PI * hz * seconds).sin()
    }

    /// Every output frame, as (left, right), and the packets' positions, after
    /// checking each packet starts where the previous one ended.
    fn decode_frames(decoder: &mut SymphoniaDecoder) -> (u32, Vec<[f64; 2]>) {
        let packets = decode_remaining(decoder);
        let start_ms = packets.first().map_or(0, |(position_ms, _)| *position_ms);
        let mut frames = Vec::new();
        for (position_ms, samples) in packets {
            let elapsed_ms = frames.len() as u64 * 1000 / u64::from(SAMPLE_RATE);
            assert!(
                (i64::from(position_ms) - i64::from(start_ms) - elapsed_ms as i64).abs() <= 1,
                "packet at {position_ms} ms after {elapsed_ms} ms of output from {start_ms} ms"
            );
            frames.extend(samples.chunks_exact(2).map(|frame| [frame[0], frame[1]]));
        }
        (start_ms, frames)
    }

    /// The largest difference between `side` of `frames` and `ideal`, at
    /// seconds from `start` on the source's clock, away from the filter's
    /// edges.
    fn worst_error(
        frames: &[[f64; 2]],
        side: usize,
        start: f64,
        ideal: impl Fn(f64) -> f64,
    ) -> f64 {
        // The library reports delay in whole frames, so its fractional
        // alignment may differ by up to one frame from the previous filter.
        // Fit only that bounded constant offset, then check all output samples.
        let error = |n: usize, shift: f64| {
            let t = start + (n as f64 + shift) / f64::from(SAMPLE_RATE);
            (frames[n][side] - ideal(t)).abs()
        };
        let shift = (-128..=128)
            .map(|step| f64::from(step) / 128.0)
            .min_by(|a, b| {
                let score = |shift| {
                    (256..frames.len() - 256)
                        .step_by(97)
                        .map(|n| error(n, shift).powi(2))
                        .sum::<f64>()
                };
                score(*a).total_cmp(&score(*b))
            })
            .unwrap();
        (256..frames.len() - 256)
            .map(|n| error(n, shift))
            .fold(0.0, f64::max)
    }

    #[test]
    fn a_48_khz_stereo_file_plays_at_44_1_khz_for_the_same_time() {
        let rate = 48000;
        let bytes = wav(rate, 2, None, rate as usize, |frame, channel| {
            let t = frame as f64 / f64::from(rate);
            sine(if channel == 0 { 440.0 } else { 880.0 }, t)
        });
        let mut decoder = wav_decoder(bytes);
        let (start_ms, frames) = decode_frames(&mut decoder);
        assert_eq!(start_ms, 0);
        let duration_ms = frames.len() as f64 * 1000.0 / f64::from(SAMPLE_RATE);
        assert!((duration_ms - 1000.0).abs() < 2.0, "{duration_ms} ms");
        assert!(worst_error(&frames, 0, 0.0, |t| sine(440.0, t)) < 0.001);
        assert!(worst_error(&frames, 1, 0.0, |t| sine(880.0, t)) < 0.001);
    }

    #[test]
    fn mono_files_play_on_both_sides_at_44_1_khz() {
        for rate in [96000, 22050, 44100] {
            let bytes = wav(rate, 1, None, rate as usize / 2, |frame, _| {
                sine(1000.0, frame as f64 / f64::from(rate))
            });
            let mut decoder = wav_decoder(bytes);
            assert_eq!(decoder.resampler.is_some(), rate != SAMPLE_RATE);
            let (_, frames) = decode_frames(&mut decoder);
            let duration_ms = frames.len() as f64 * 1000.0 / f64::from(SAMPLE_RATE);
            assert!(
                (duration_ms - 500.0).abs() < 2.0,
                "{rate} Hz: {duration_ms} ms"
            );
            assert!(frames.iter().all(|[left, right]| left == right));
            let error = worst_error(&frames, 0, 0.0, |t| sine(1000.0, t));
            assert!(error < 0.001, "{rate} Hz: off by {error}");
        }
    }

    #[test]
    fn surround_files_are_mixed_down_to_stereo() {
        use std::f64::consts::FRAC_1_SQRT_2;

        // 5.1: front left, front right, centre, LFE, rear left, rear right.
        let rate = 48000;
        let bytes = wav(rate, 6, Some(0x3F), rate as usize / 2, |frame, channel| {
            let t = frame as f64 / f64::from(rate);
            match channel {
                0 => sine(440.0, t),
                2 => sine(1000.0, t),
                3 => 0.5,
                _ => 0.0,
            }
        });
        let mut decoder = wav_decoder(bytes);
        let (_, frames) = decode_frames(&mut decoder);
        assert!((frames.len() as f64 * 1000.0 / 44100.0 - 500.0).abs() < 2.0);
        // Each side takes its front channel, the centre and its rear channel
        // 3 dB down, scaled so the three together cannot clip.
        let scale = 1.0 / (1.0 + 2.0 * FRAC_1_SQRT_2);
        let centre = FRAC_1_SQRT_2 * scale;
        let left = worst_error(&frames, 0, 0.0, |t| {
            scale * sine(440.0, t) + centre * sine(1000.0, t)
        });
        let right = worst_error(&frames, 1, 0.0, |t| centre * sine(1000.0, t));
        assert!(left < 0.001 && right < 0.001, "off by {left} and {right}");
    }

    #[test]
    fn downmix_gains_cannot_clip() {
        for channels in [
            Channels::FRONT_LEFT | Channels::FRONT_RIGHT | Channels::FRONT_CENTRE,
            Channels::from_bits(0x3F).unwrap(),
            Channels::from_bits(0x63F).unwrap(),
            Channels::FRONT_CENTRE,
        ] {
            let gains = stereo_gains(channels);
            assert_eq!(gains.len(), channels.count());
            for side in 0..2 {
                let sum: f64 = gains.iter().map(|gain| gain[side]).sum();
                assert!(sum <= 1.0 + 1e-12, "{channels}: {sum}");
                assert!(sum > 0.5, "{channels}: {sum}");
            }
        }
    }

    #[test]
    fn two_channels_without_a_stereo_layout_are_mixed_by_position() {
        let channels = Channels::FRONT_CENTRE | Channels::LFE1;
        let gains = stereo_gains(channels);
        let mut stereo = Vec::new();
        mix_to_stereo(&[0.5, 1.0, -0.5, -1.0], &gains, &mut stereo);
        assert_eq!(stereo[0], stereo[1]);
        assert_eq!(stereo[2], stereo[3]);
        assert!((stereo[0] - 0.5 * std::f64::consts::FRAC_1_SQRT_2).abs() < 1e-12);
        assert_eq!(stereo[0], -stereo[2]);
    }

    #[test]
    fn seeking_a_resampled_file_lands_on_the_source_position() {
        // A slow ramp, a quarter per second, so the audio shows where in the
        // source it comes from to within a millisecond.
        for (rate, channels) in [(48000, 2), (22050, 1), (96000, 1)] {
            let bytes = wav(rate, channels, None, 2 * rate as usize, |frame, _| {
                frame as f64 / f64::from(rate) / 4.0
            });
            let mut decoder = wav_decoder(bytes);
            decoder.next_packet().unwrap().unwrap();
            // The seek lands on the packet holding the target, and says where.
            let actual_ms = decoder.seek(1250).unwrap();
            assert!(
                (1150..=1250).contains(&actual_ms),
                "{rate} Hz: {actual_ms} ms"
            );
            let (start_ms, frames) = decode_frames(&mut decoder);
            assert_eq!(start_ms, actual_ms);
            let remaining_ms = frames.len() as f64 * 1000.0 / f64::from(SAMPLE_RATE);
            let expected_ms = 2000.0 - f64::from(actual_ms);
            assert!(
                (remaining_ms - expected_ms).abs() < 2.0,
                "{rate} Hz: {remaining_ms} ms"
            );
            let start = f64::from(actual_ms) / 1000.0;
            let error = worst_error(&frames, 0, start, |t| t / 4.0);
            assert!(error < 0.0005, "{rate} Hz: off by {error}");
            assert!(frames.iter().all(|[left, right]| left == right));
        }
    }
}
