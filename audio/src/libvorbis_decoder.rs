#[cfg(feature = "with-tremor")]
extern crate librespot_tremor as vorbis;
#[cfg(not(feature = "with-tremor"))]
extern crate vorbis;

use super::{AudioDecoder, AudioError, AudioPacket};
use std::error;
use std::fmt;
use std::io::{Read, Seek};

pub struct VorbisDecoder<R: Read + Seek>(vorbis::Decoder<R>);
pub struct VorbisError(vorbis::VorbisError);

impl<R> VorbisDecoder<R>
where
    R: Read + Seek,
{
    pub fn new(input: R) -> Result<VorbisDecoder<R>, VorbisError> {
        Ok(VorbisDecoder(vorbis::Decoder::new(input)?))
    }
}

impl<R> AudioDecoder for VorbisDecoder<R>
where
    R: Read + Seek,
{
    #[cfg(not(feature = "with-tremor"))]
    fn seek(&mut self, ms: i64) -> Result<(), AudioError> {
        self.0.time_seek(ms as f64 / 1000f64)?;
        Ok(())
    }

    #[cfg(feature = "with-tremor")]
    fn seek(&mut self, ms: i64) -> Result<(), AudioError> {
        self.0.time_seek(ms)?;
        Ok(())
    }

    fn next_packet(&mut self) -> Result<Option<AudioPacket>, AudioError> {
        loop {
            match self.0.packets().next() {
                Some(Ok(packet)) => return Ok(Some(AudioPacket::Samples(packet.data))),
                None => return Ok(None),

                Some(Err(vorbis::VorbisError::Hole)) => (),
                Some(Err(err)) => return Err(err.into()),
            }
        }
    }
}

impl From<vorbis::VorbisError> for VorbisError {
    fn from(err: vorbis::VorbisError) -> VorbisError {
        VorbisError(err)
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
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        error::Error::source(&self.0)
    }
}

impl From<vorbis::VorbisError> for AudioError {
    fn from(err: vorbis::VorbisError) -> AudioError {
        AudioError::VorbisError(VorbisError(err))
    }
}
