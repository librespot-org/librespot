use std::{io, time::Duration};

use symphonia::{
    core::meta::{Metadata, MetadataOptions},
    core::probe::{Hint, ProbedMetadata},
    core::{
        audio::SampleBuffer,
        codecs::{Decoder, DecoderOptions},
        errors::Error,
        formats::{FormatOptions, FormatReader, SeekMode, SeekTo},
        io::{MediaSource, MediaSourceStream, MediaSourceStreamOptions},
        meta::{StandardTagKey, Value},
    },
    default::{
        codecs::{FlacDecoder, MpaDecoder, VorbisDecoder},
        formats::{FlacReader, MpaReader, OggReader},
    },
};

use super::{AudioDecoder, AudioPacket, AudioPacketPosition, DecoderError, DecoderResult};

use crate::{
    NUM_CHANNELS, PAGES_PER_MS, SAMPLE_RATE,
    metadata::audio::{AudioFileFormat, AudioFiles},
    player::NormalisationData,
};

pub struct SymphoniaDecoder {
    format: Box<dyn FormatReader>,
    decoder: Box<dyn Decoder>,
    sample_buffer: Option<SampleBuffer<f64>>,
    probed_metadata: Option<ProbedMetadata>,
}

#[derive(Default)]
pub(crate) struct LocalFileMetadata {
    pub name: String,
    pub language: String,
    pub album: String,
    pub artists: String,
    pub album_artists: String,
    pub number: u32,
    pub disc_number: u32,
}

impl SymphoniaDecoder {
    pub fn new<R>(input: R, file_format: AudioFileFormat) -> DecoderResult<Self>
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

        let format: Box<dyn FormatReader> = if AudioFiles::is_ogg_vorbis(file_format) {
            Box::new(OggReader::try_new(mss, &format_opts)?)
        } else if AudioFiles::is_mp3(file_format) {
            Box::new(MpaReader::try_new(mss, &format_opts)?)
        } else if AudioFiles::is_flac(file_format) {
            Box::new(FlacReader::try_new(mss, &format_opts)?)
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
        } else if AudioFiles::is_flac(file_format) {
            Box::new(FlacDecoder::try_new(&track.codec_params, &decoder_opts)?)
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

            probed_metadata: None,
        })
    }

    pub(crate) fn new_with_probe<R>(src: R, extension: Option<&str>) -> DecoderResult<Self>
    where
        R: MediaSource + 'static,
    {
        let mss = MediaSourceStream::new(Box::new(src), Default::default());

        let mut hint = Hint::new();

        if let Some(extension) = extension {
            hint.with_extension(extension);
        }

        let format_opts: FormatOptions = Default::default();
        let metadata_opts: MetadataOptions = Default::default();
        let decoder_opts: DecoderOptions = Default::default();

        let probed =
            symphonia::default::get_probe().format(&hint, mss, &format_opts, &metadata_opts)?;

        let format = probed.format;

        let track = format.default_track().ok_or_else(|| {
            DecoderError::SymphoniaDecoder("Could not retrieve default track".into())
        })?;

        let decoder = symphonia::default::get_codecs().make(&track.codec_params, &decoder_opts)?;

        let rate = decoder.codec_params().sample_rate.ok_or_else(|| {
            DecoderError::SymphoniaDecoder("Could not retrieve sample rate".into())
        })?;

        // TODO: The official client supports local files with sample rates other than 44,100 kHz.
        // To play these accurately, we need to either resample the input audio, or introduce a way
        // to change the player's current sample rate (likely by closing and re-opening the sink
        // with new parameters).
        if rate != SAMPLE_RATE {
            return Err(DecoderError::SymphoniaDecoder(format!(
                "Unsupported sample rate: {rate}. Local files must have a sample rate of {SAMPLE_RATE} Hz."
            )));
        }

        Ok(Self {
            format,
            decoder,
            sample_buffer: None,
            probed_metadata: Some(probed.metadata),
        })
    }

    pub fn normalisation_data(&mut self) -> Option<NormalisationData> {
        let metadata = self.metadata()?;
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
        let metadata = self.metadata()?;
        let tags = metadata.current()?.tags();
        let mut metadata = LocalFileMetadata::default();

        for tag in tags {
            if let Value::String(value) = &tag.value {
                match tag.std_key {
                    // We could possibly use mem::take here to avoid cloning, but that risks leaving
                    // the audio item metadata in a bad state.
                    Some(StandardTagKey::TrackTitle) => metadata.name = value.clone(),
                    Some(StandardTagKey::Language) => metadata.language = value.clone(),
                    Some(StandardTagKey::Artist) => metadata.artists = value.clone(),
                    Some(StandardTagKey::AlbumArtist) => metadata.album_artists = value.clone(),
                    Some(StandardTagKey::Album) => metadata.album = value.clone(),
                    Some(StandardTagKey::TrackNumber) => {
                        metadata.number = value.parse::<u32>().unwrap_or_default()
                    }
                    Some(StandardTagKey::DiscNumber) => {
                        metadata.disc_number = value.parse::<u32>().unwrap_or_default()
                    }
                    _ => (),
                }
            } else if let Value::UnsignedInt(value) = &tag.value {
                match tag.std_key {
                    Some(StandardTagKey::TrackNumber) => metadata.number = *value as u32,
                    Some(StandardTagKey::DiscNumber) => metadata.disc_number = *value as u32,
                    _ => (),
                }
            } else if let Value::SignedInt(value) = &tag.value {
                match tag.std_key {
                    Some(StandardTagKey::TrackNumber) => metadata.number = *value as u32,
                    Some(StandardTagKey::DiscNumber) => metadata.disc_number = *value as u32,
                    _ => (),
                }
            }
        }

        Some(metadata)
    }

    fn metadata(&mut self) -> Option<Metadata<'_>> {
        let mut metadata = self.format.metadata();

        // If we can't get metadata from the container, fall back to other tags found by probing.
        // Note that this is only relevant for local files.
        if metadata.current().is_none() {
            if let Some(ref mut probe_metadata) = self.probed_metadata {
                if let Some(inner_probe_metadata) = probe_metadata.get() {
                    metadata = inner_probe_metadata;
                }
            }
        }

        // Advance to the latest metadata revision.
        // None means we hit the latest.
        loop {
            if metadata.pop().is_none() {
                break;
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
        let seeked_to_ts = self.format.seek(
            SeekMode::Accurate,
            SeekTo::Time {
                time: target.into(),
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
