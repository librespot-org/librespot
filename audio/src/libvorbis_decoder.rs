#[cfg(not(feature = "with-tremor"))] extern crate vorbis;
#[cfg(feature = "with-tremor")] extern crate tremor as vorbis;

use std::io::{Read, Seek};
use std::fmt;
use std::error;

pub struct VorbisDecoder<R: Read + Seek>(vorbis::Decoder<R>);
pub struct VorbisPacket(vorbis::Packet);
pub struct VorbisError(vorbis::VorbisError);

impl <R> VorbisDecoder<R>
    where R: Read + Seek
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

    pub fn next_packet(&mut self) -> Result<Option<VorbisPacket>, VorbisError> {
        loop {
            match self.0.packets().next() {
                Some(Ok(packet)) => return Ok(Some(VorbisPacket(packet))),
                None => return Ok(None),

                Some(Err(vorbis::VorbisError::Hole)) => (),
                Some(Err(err)) => return Err(err.into()),
            }
        }
    }
}

impl VorbisPacket {
    pub fn data(&self) -> &[i16] {
        &self.0.data
    }

    pub fn data_mut(&mut self) -> &mut [i16] {
        &mut self.0.data
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
