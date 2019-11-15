extern crate lewton;

use self::lewton::inside_ogg::OggStreamReader;

use super::{AudioDecoder, AudioError, AudioPacket};
use std::error;
use std::fmt;
use std::io::{Read, Seek};

pub struct VorbisDecoder<R: Read + Seek>(OggStreamReader<R>);
pub struct VorbisError(lewton::VorbisError);

impl<R> VorbisDecoder<R>
where
    R: Read + Seek,
{
    pub fn new(input: R) -> Result<VorbisDecoder<R>, VorbisError> {
        Ok(VorbisDecoder(OggStreamReader::new(input)?))
    }

    pub fn seek(&mut self, ms: i64) -> Result<(), VorbisError> {
        let absgp = ms * 44100 / 1000;
        self.0.seek_absgp_pg(absgp as u64)?;
        Ok(())
    }

    pub fn next_packet(&mut self) -> Result<Option<AudioPacket>, VorbisError> {
        use self::lewton::audio::AudioReadError::AudioIsHeader;
        use self::lewton::OggReadError::NoCapturePatternFound;
        use self::lewton::VorbisError::BadAudio;
        use self::lewton::VorbisError::OggError;
        loop {
            match self.0.read_dec_packet_itl() {
                Ok(Some(packet)) => return Ok(Some(AudioPacket(packet))),
                Ok(None) => return Ok(None),

                Err(BadAudio(AudioIsHeader)) => (),
                Err(OggError(NoCapturePatternFound)) => (),
                Err(err) => return Err(err.into()),
            }
        }
    }
}

impl<R> AudioDecoder for VorbisDecoder<R>
where
    R: Read + Seek,
{
    fn seek(&mut self, ms: i64) -> Result<(), AudioError> {
        let absgp = ms * 44100 / 1000;
        self.0.seek_absgp_pg(absgp as u64)?;
        Ok(())
    }

    fn next_packet(&mut self) -> Result<Option<AudioPacket>, AudioError> {
        use self::lewton::audio::AudioReadError::AudioIsHeader;
        use self::lewton::OggReadError::NoCapturePatternFound;
        use self::lewton::VorbisError::BadAudio;
        use self::lewton::VorbisError::OggError;
        loop {
            match self.0.read_dec_packet_itl() {
                Ok(Some(packet)) => return Ok(Some(AudioPacket(packet))),
                Ok(None) => return Ok(None),

                Err(BadAudio(AudioIsHeader)) => (),
                Err(OggError(NoCapturePatternFound)) => (),
                Err(err) => return Err(err.into()),
            }
        }
    }
}

impl From<lewton::VorbisError> for VorbisError {
    fn from(err: lewton::VorbisError) -> VorbisError {
        VorbisError(err)
    }
}

impl From<lewton::VorbisError> for AudioError {
    fn from(err: lewton::VorbisError) -> AudioError {
        AudioError::VorbisError(VorbisError(err))
    }
}

impl fmt::Debug for VorbisError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Display for VorbisError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl error::Error for VorbisError {
    fn description(&self) -> &str {
        error::Error::description(&self.0)
    }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        error::Error::source(&self.0)
    }
}
