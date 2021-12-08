use super::{AudioDecoder, AudioPacket, DecoderError, DecoderResult};

use lewton::audio::AudioReadError::AudioIsHeader;
use lewton::inside_ogg::OggStreamReader;
use lewton::samples::InterleavedSamples;
use lewton::OggReadError::NoCapturePatternFound;
use lewton::VorbisError::{BadAudio, OggError};

use std::io::{Read, Seek};

pub struct VorbisDecoder<R: Read + Seek>(OggStreamReader<R>);

impl<R> VorbisDecoder<R>
where
    R: Read + Seek,
{
    pub fn new(input: R) -> DecoderResult<VorbisDecoder<R>> {
        let reader =
            OggStreamReader::new(input).map_err(|e| DecoderError::LewtonDecoder(e.to_string()))?;
        Ok(VorbisDecoder(reader))
    }
}

impl<R> AudioDecoder for VorbisDecoder<R>
where
    R: Read + Seek,
{
    fn seek(&mut self, absgp: u64) -> DecoderResult<()> {
        self.0
            .seek_absgp_pg(absgp)
            .map_err(|e| DecoderError::LewtonDecoder(e.to_string()))?;
        Ok(())
    }

    fn next_packet(&mut self) -> DecoderResult<Option<AudioPacket>> {
        loop {
            match self.0.read_dec_packet_generic::<InterleavedSamples<f32>>() {
                Ok(Some(packet)) => return Ok(Some(AudioPacket::samples_from_f32(packet.samples))),
                Ok(None) => return Ok(None),
                Err(BadAudio(AudioIsHeader)) => (),
                Err(OggError(NoCapturePatternFound)) => (),
                Err(e) => return Err(DecoderError::LewtonDecoder(e.to_string())),
            }
        }
    }
}
