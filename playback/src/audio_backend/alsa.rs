use super::{Open, Sink};
use std::io;

use std::ffi::CString;
use alsa::{Direction, ValueOr};
use alsa::pcm::{PCM, HwParams, Format, Access};

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
            let pcm = PCM::open(&*CString::new(self.1.to_owned().into_bytes()).unwrap(),
                                Direction::Playback,
                                false).unwrap();
            {
                // Set hardware parameters: 44100 Hz / Stereo / 16 bit
                let hwp = HwParams::any(&pcm).unwrap();
                hwp.set_channels(2).unwrap();
                hwp.set_rate(44100, ValueOr::Nearest).unwrap();
                hwp.set_format(Format::s16()).unwrap();
                hwp.set_access(Access::RWInterleaved).unwrap();
                pcm.hw_params(&hwp).unwrap();
            }

            self.0 = Some(pcm);
        }

        Ok(())
    }

    fn stop(&mut self) -> io::Result<()> {
        self.0 = None;
        Ok(())
    }

    fn write(&mut self, data: &[i16]) -> io::Result<()> {
        let pcm = self.0.as_mut().unwrap();
        let io = pcm.io_i16().unwrap();

        match io.writei(&data) {
            Ok(_) => (),
            Err(err) => pcm.recover(err.code(), false).unwrap(),
        }

        Ok(())
    }
}
