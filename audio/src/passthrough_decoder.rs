use super::{AudioDecoder, AudioError, AudioPacket};
use std::error;
use std::fmt;
use std::io::{Error, Read, Seek};
use std::mem;
use std::slice;

pub struct PassthroughDecoder<R: Read + Seek>(R);

impl<R: Read + Seek> PassthroughDecoder<R> {
    pub fn new(input: R) -> Result<Self, PassthroughError> {
        Ok(PassthroughDecoder(input))
    }
}

impl<R: Read + Seek> AudioDecoder for PassthroughDecoder<R> {
    fn seek(&mut self, _: i64) -> Result<(), AudioError> {
        Err(PassthroughError::SeekNotAllowed)?
    }
    fn next_packet(&mut self) -> Result<Option<AudioPacket>, AudioError> {
        let mut data = [0u8; 8192];
        let mut rd = 0;
        while rd < data.len() {
            let thisrd = self.0.read(&mut data[rd..])?;
            rd += thisrd;
            if thisrd == 0 {
                break;
            }
        }
		
		if rd == 0 {
			return Ok(None);
		}
		
        // This will insert an extraneous 0 byte at EOF if the file size was not an even number of bytes
        let s16 = mem::size_of::<i16>(); // Yes, I know it is always 2
        rd += (s16 - (rd % s16)) % s16;
        let data16 = unsafe { slice::from_raw_parts(data.as_ptr() as *const i16, rd / s16) };
        Ok(Some(AudioPacket(Vec::<i16>::from(data16))))
    }
}

#[derive(Debug)]
pub enum PassthroughError {
    SeekNotAllowed,
    IOError(Error),
}

impl From<PassthroughError> for AudioError {
    fn from(err: PassthroughError) -> AudioError {
        AudioError::PassthroughError(err)
    }
}

impl From<Error> for AudioError {
    fn from(err: Error) -> AudioError {
        AudioError::PassthroughError(PassthroughError::IOError(err))
    }
}

impl fmt::Display for PassthroughError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

impl error::Error for PassthroughError {
    fn description(&self) -> &str {
        match self {
            PassthroughError::SeekNotAllowed => {
                "Seeking is not yet implemented for the pass-through decoder"
            }
            PassthroughError::IOError(err) => err.description(),
        }
    }
    fn cause(&self) -> Option<&error::Error> {
        match self {
            PassthroughError::SeekNotAllowed => None,
            PassthroughError::IOError(err) => err.cause(),
        }
    }
}
