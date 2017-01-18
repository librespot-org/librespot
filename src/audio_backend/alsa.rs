use super::{Open, Sink};
use std::io;
use alsa::{PCM, Stream, Mode, Format, Access};

pub struct AlsaSink(Option<PCM>, String);

impl Open for AlsaSink {
   fn open(device: Option<String>) -> AlsaSink {
        info!("Using alsa sink");

        let name = device.unwrap_or("default".to_string());

        AlsaSink(None, name)
    }
}

impl Sink for AlsaSink {
    fn start(&mut self) -> io::Result<()> {
        if self.0.is_some() {
        } else {
            self.0 = Some(PCM::open(&*self.1,
                                    Stream::Playback, Mode::Blocking,
                                    Format::Signed16, Access::Interleaved,
                                    2, 44100).ok().unwrap());
        }
        Ok(())
    }

    fn stop(&mut self) -> io::Result<()> {
        self.0 = None;
        Ok(())
    }

    fn write(&mut self, data: &[i16]) -> io::Result<()> {
        self.0.as_mut().unwrap().write_interleaved(&data).unwrap();
        Ok(())
    }
}
