use super::{Open, Sink};
use alsa::{Access, Format, Mode, Stream, PCM};
use std::io;

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
        if self.0.is_none() {
            match PCM::open(
                &*self.1,
                Stream::Playback,
                Mode::Blocking,
                Format::Signed16,
                Access::Interleaved,
                2,
                44100,
            ) {
                Ok(f) => self.0 = Some(f),
                Err(e) => {
                    error!("Alsa error PCM open {}", e);
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        "Alsa error: PCM open failed",
                    ));
                }
            }
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
