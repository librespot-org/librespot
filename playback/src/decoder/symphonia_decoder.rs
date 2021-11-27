use super::{AudioDecoder, AudioPacket, DecoderError, DecoderResult};

use crate::audio::AudioFile;

use symphonia::core::audio::{AudioBufferRef, Channels};
use symphonia::core::codecs::Decoder;
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::{FormatReader, SeekMode, SeekTo};
use symphonia::core::io::{MediaSource, MediaSourceStream};
use symphonia::core::units::TimeStamp;
use symphonia::default::{codecs::VorbisDecoder, formats::OggReader};

use std::io::{Read, Seek, SeekFrom};

impl<R> MediaSource for FileWithConstSize<R>
where
    R: Read + Seek + Send,
{
    fn is_seekable(&self) -> bool {
        true
    }

    fn byte_len(&self) -> Option<u64> {
        Some(self.len())
    }
}

pub struct FileWithConstSize<T> {
    stream: T,
    len: u64,
}

impl<T> FileWithConstSize<T> {
    pub fn len(&self) -> u64 {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T> FileWithConstSize<T>
where
    T: Seek,
{
    pub fn new(mut stream: T) -> Self {
        stream.seek(SeekFrom::End(0)).unwrap();
        let len = stream.stream_position().unwrap();
        stream.seek(SeekFrom::Start(0)).unwrap();
        Self { stream, len }
    }
}

impl<T> Read for FileWithConstSize<T>
where
    T: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.stream.read(buf)
    }
}

impl<T> Seek for FileWithConstSize<T>
where
    T: Seek,
{
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.stream.seek(pos)
    }
}

pub struct SymphoniaDecoder {
    track_id: u32,
    decoder: Box<dyn Decoder>,
    format: Box<dyn FormatReader>,
    position: TimeStamp,
}

impl SymphoniaDecoder {
    pub fn new<R>(input: R) -> DecoderResult<Self>
    where
    R: Read + Seek,
    {
        let mss_opts = Default::default();
        let mss = MediaSourceStream::new(Box::new(FileWithConstSize::new(input)), mss_opts);

        let format_opts = Default::default();
        let format = OggReader::try_new(mss, &format_opts).map_err(|e| DecoderError::SymphoniaDecoder(e.to_string()))?;

        let track = format.default_track().unwrap();
        let decoder_opts = Default::default();
        let decoder = VorbisDecoder::try_new(&track.codec_params, &decoder_opts)?;

        Ok(Self {
            track_id: track.id,
            decoder: Box::new(decoder),
            format: Box::new(format),
            position: 0,
        })
    }
}

impl AudioDecoder for SymphoniaDecoder {
    fn seek(&mut self, absgp: u64) -> DecoderResult<()> {
        let seeked_to = self.format.seek(
            SeekMode::Accurate,
            SeekTo::Time {
                time: absgp, // TODO : move to Duration
                track_id: Some(self.track_id),
            },
        )?;
        self.position = seeked_to.actual_ts;
        // TODO : Ok(self.position)
        Ok(())
    }

    fn next_packet(&mut self) -> DecoderResult<Option<AudioPacket>> {
        let packet = match self.format.next_packet() {
            Ok(packet) => packet,
            Err(e) => {
                log::error!("format error: {}", err);
                return Err(DecoderError::SymphoniaDecoder(e.to_string())),
            }
        };
        match self.decoder.decode(&packet) {
            Ok(audio_buf) => {
                self.position += packet.frames() as TimeStamp;
                Ok(Some(packet))
            }
            // TODO: Handle non-fatal decoding errors and retry.
            Err(e) => 
                return Err(DecoderError::SymphoniaDecoder(e.to_string())),
        }
    }
}
