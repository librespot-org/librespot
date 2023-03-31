use std::io;

use symphonia::{
    core::{
        audio::SampleBuffer,
        codecs::{Decoder, DecoderOptions},
        errors::Error,
        formats::{FormatOptions, FormatReader, SeekMode, SeekTo},
        io::{MediaSource, MediaSourceStream, MediaSourceStreamOptions},
        meta::{StandardTagKey, Value},
        units::Time,
    },
    default::{
        codecs::{MpaDecoder, VorbisDecoder},
        formats::{MpaReader, OggReader},
    },
};

use super::{AudioDecoder, AudioPacket, AudioPacketPosition, DecoderError, DecoderResult};

use crate::{
    metadata::audio::{AudioFileFormat, AudioFiles},
    player::NormalisationData,
    NUM_CHANNELS, PAGES_PER_MS, SAMPLE_RATE,
};

pub struct SymphoniaDecoder {
    format: Box<dyn FormatReader>,
    decoder: Box<dyn Decoder>,
    sample_buffer: Option<SampleBuffer<f64>>,
}

impl SymphoniaDecoder {
    pub fn new<R>(input: R, file_format: AudioFileFormat) -> DecoderResult<Self>
    where
        R: MediaSource + 'static,
    {
        let mss_opts = MediaSourceStreamOptions {
            buffer_len: librespot_audio::MINIMUM_DOWNLOAD_SIZE,
        };
        let mss = MediaSourceStream::new(Box::new(input), mss_opts);

        let format_opts = FormatOptions {
            enable_gapless: true,
            ..Default::default()
        };

        let format: Box<dyn FormatReader> = if AudioFiles::is_ogg_vorbis(file_format) {
            Box::new(OggReader::try_new(mss, &format_opts)?)
        } else if AudioFiles::is_mp3(file_format) {
            Box::new(MpaReader::try_new(mss, &format_opts)?)
        } else {
            return Err(DecoderError::SymphoniaDecoder(format!(
                "Unsupported format: {file_format:?}"
            )));
        };

        let track = format.default_track().ok_or_else(|| {
            DecoderError::SymphoniaDecoder("Could not retrieve default track".into())
        })?;

        let decoder_opts: DecoderOptions = Default::default();
        let decoder: Box<dyn Decoder> = if AudioFiles::is_ogg_vorbis(file_format) {
            Box::new(VorbisDecoder::try_new(&track.codec_params, &decoder_opts)?)
        } else if AudioFiles::is_mp3(file_format) {
            Box::new(MpaDecoder::try_new(&track.codec_params, &decoder_opts)?)
        } else {
            return Err(DecoderError::SymphoniaDecoder(format!(
                "Unsupported decoder: {file_format:?}"
            )));
        };

        let rate = decoder.codec_params().sample_rate.ok_or_else(|| {
            DecoderError::SymphoniaDecoder("Could not retrieve sample rate".into())
        })?;
        if rate != SAMPLE_RATE {
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
            format,
            decoder,

            // We set the sample buffer when decoding the first full packet,
            // whose duration is also the ideal sample buffer size.
            sample_buffer: None,
        })
    }

    pub fn normalisation_data(&mut self) -> Option<NormalisationData> {
        let mut metadata = self.format.metadata();

        // Advance to the latest metadata revision.
        // None means we hit the latest.
        loop {
            if metadata.pop().is_none() {
                break;
            }
        }

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

    fn ts_to_ms(&self, ts: u64) -> u32 {
        let time_base = self.decoder.codec_params().time_base;
        let seeked_to_ms = match time_base {
            Some(time_base) => {
                let time = time_base.calc_time(ts);
                (time.seconds as f64 + time.frac) * 1000.
            }
            // Fallback in the unexpected case that the format has no base time set.
            None => ts as f64 * PAGES_PER_MS,
        };
        seeked_to_ms as u32
    }
}

impl AudioDecoder for SymphoniaDecoder {
    fn seek(&mut self, position_ms: u32) -> Result<u32, DecoderError> {
        let seconds = position_ms as u64 / 1000;
        let frac = (position_ms as f64 % 1000.) / 1000.;
        let time = Time::new(seconds, frac);

        // `track_id: None` implies the default track ID (of the container, not of Spotify).
        let seeked_to_ts = self.format.seek(
            SeekMode::Accurate,
            SeekTo::Time {
                time,
                track_id: None,
            },
        )?;

        // Seeking is a `FormatReader` operation, so the decoder cannot reliably
        // know when a seek took place. Reset it to avoid audio glitches.
        self.decoder.reset();

        Ok(self.ts_to_ms(seeked_to_ts.actual_ts))
    }

    fn next_packet(&mut self) -> DecoderResult<Option<(AudioPacketPosition, AudioPacket)>> {
        let mut skipped = false;

        loop {
            let packet = match self.format.next_packet() {
                Ok(packet) => packet,
                Err(Error::IoError(err)) => {
                    if err.kind() == io::ErrorKind::UnexpectedEof {
                        return Ok(None);
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
                    let samples = AudioPacket::Samples(sample_buffer.samples().to_vec());

                    return Ok(Some((packet_position, samples)));
                }
                Err(Error::DecodeError(_)) => {
                    // The packet failed to decode due to corrupted or invalid data, get a new
                    // packet and try again.
                    warn!("Skipping malformed audio packet at {position_ms} ms");
                    skipped = true;
                    continue;
                }
                Err(err) => return Err(err.into()),
            }
        }
    }
}
