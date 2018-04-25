extern crate lewton;

use self::lewton::inside_ogg::OggStreamReader;

use std::error;
use std::fmt;
use std::io::{Read, Seek};

pub struct VorbisDecoder<R: Read + Seek>(OggStreamReader<R>);
pub struct VorbisPacket(Vec<i16>);
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

    pub fn next_packet(&mut self) -> Result<Option<VorbisPacket>, VorbisError> {
        use self::lewton::audio::AudioReadError::AudioIsHeader;
        use self::lewton::OggReadError::NoCapturePatternFound;
        use self::lewton::VorbisError::BadAudio;
        use self::lewton::VorbisError::OggError;
        loop {
            match self.0.read_dec_packet_itl() {
                Ok(Some(packet)) => return Ok(Some(VorbisPacket(packet))),
                Ok(None) => return Ok(None),

                Err(BadAudio(AudioIsHeader)) => (),
                Err(OggError(NoCapturePatternFound)) => (),
                Err(err) => return Err(err.into()),
            }
        }
    }
}

impl VorbisPacket {
    pub fn data(&self) -> &[i16] {
        &self.0
    }

    pub fn data_mut(&mut self) -> &mut [i16] {
        &mut self.0
    }
}

impl From<lewton::VorbisError> for VorbisError {
    fn from(err: lewton::VorbisError) -> VorbisError {
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
