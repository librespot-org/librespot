use std::{io, time::Duration};

use symphonia::core::{
    audio::SampleBuffer,
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
    /// Decode stereo audio at 8,000 through 384,000 Hz, resampling as needed.
    /// Bounding input rates limits per-packet expansion and filter resources.
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
        if channels.count() != NUM_CHANNELS as usize {
            return Err(DecoderError::SymphoniaDecoder(format!(
                "Unsupported number of channels: {channels}"
            )));
        }

        Ok(Self {
            probe_result,
            decoder,
            // We set the sample buffer when decoding the first full packet,
            // whose duration is also the ideal sample buffer size.
            sample_buffer: None,
            resampler: Resampler::new(rate, SAMPLE_RATE, channels.count()),
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
                    if let Some(resampler) = &mut self.resampler {
                        self.resampled_start_ms.get_or_insert(position_ms);
                        let samples = resampler.process(sample_buffer.samples());
                        if let Some(packet) = self.resampled_packet(samples, skipped) {
                            return Ok(Some(packet));
                        }
                    } else {
                        let samples = AudioPacket::Samples(sample_buffer.samples().to_vec());
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
}
