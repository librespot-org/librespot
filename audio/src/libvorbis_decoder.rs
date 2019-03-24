#[cfg(feature = "with-tremor")]
extern crate tremor as vorbis;
#[cfg(not(feature = "with-tremor"))]
extern crate vorbis;

use super::AudioPacket;
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

    #[cfg(not(feature = "with-tremor"))]
    pub fn seek(&mut self, ms: i64) -> Result<(), VorbisError> {
        self.0.time_seek(ms as f64 / 1000f64)?;
        Ok(())
    }

    #[cfg(feature = "with-tremor")]
    pub fn seek(&mut self, ms: i64) -> Result<(), VorbisError> {
        self.0.time_seek(ms)?;
        Ok(())
    }

    pub fn next_packet(&mut self) -> Result<Option<AudioPacket>, VorbisError> {
        loop {
            match self.0.packets().next() {
                Some(Ok(packet)) => return Ok(Some(AudioPacket(packet.data))),
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
    fn description(&self) -> &str {
        error::Error::description(&self.0)
    }

    fn cause(&self) -> Option<&error::Error> {
        error::Error::cause(&self.0)
    }
}
