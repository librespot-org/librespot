use super::{Open, Sink};
use crate::audio::AudioPacket;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::mem;
use std::slice;

pub struct StdoutSink(Box<dyn Write>);

impl Open for StdoutSink {
    fn open(path: Option<String>) -> StdoutSink {
        if let Some(path) = path {
            let file = OpenOptions::new().write(true).open(path).unwrap();
            StdoutSink(Box::new(file))
        } else {
            StdoutSink(Box::new(io::stdout()))
        }
    }
}

impl Sink for StdoutSink {
    fn start(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn stop(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn write(&mut self, packet: &AudioPacket) -> io::Result<()> {
        let data: &[u8] = match packet {
            AudioPacket::Samples(data) => unsafe {
                slice::from_raw_parts(
                    data.as_ptr() as *const u8,
                    data.len() * mem::size_of::<i16>(),
                )
            },
            AudioPacket::OggData(data) => data,
        };

        self.0.write_all(data)?;
        self.0.flush()?;

        Ok(())
    }
}
