use super::{Open, Sink};
use alsa::device_name::HintIter;
use alsa::pcm::{Access, Format, HwParams, PCM};
use alsa::{Direction, Error, ValueOr};
use std::env;
use std::ffi::CString;
use std::io;
use std::process::exit;

pub struct AlsaSink(Option<PCM>, String);

fn list_outputs() {
    for t in &["pcm", "ctl", "hwdep"] {
        println!("{} devices:", t);
        let i = HintIter::new(None, &*CString::new(*t).unwrap()).unwrap();
        for a in i {
            if let Some(Direction::Playback) = a.direction {
                println!("{:#?}", a)
            }
        }
    }
}

fn open_device(dev_name: &str) -> Result<(PCM), Box<Error>> {
    let pcm = PCM::new(dev_name, Direction::Playback, false)?;
    // http://www.linuxjournal.com/article/6735?page=0,1#N0x19ab2890.0x19ba78d8
    // latency = period_size * periods / (rate * bytes_per_frame)
    // For 16 Bit stereo data, one frame has a length of four bytes.
    // 500ms  = buffer_size / (44100 * 4)
    // buffer_size_bytes = 0.5 * 44100 / 4
    // buffer_size_frames = 0.5 * 44100 = 22050
    {
        // Set hardware parameters: 44100 Hz / Stereo / 16 bit
        let hwp = HwParams::any(&pcm)?;

        hwp.set_access(Access::RWInterleaved)?;
        hwp.set_format(Format::s16())?;
        hwp.set_rate(44100, ValueOr::Nearest)?;
        hwp.set_channels(2)?;
        hwp.set_buffer_size_near(22050)?; // ~ 0.5s latency

        pcm.hw_params(&hwp)?;
    }

    Ok(pcm)
}

impl Open for AlsaSink {
    fn open(device: Option<String>) -> AlsaSink {
        info!("Using alsa sink");

        let name = match device.as_ref().map(AsRef::as_ref) {
            Some("?") => {
                println!("Listing available alsa outputs");
                list_outputs();
                exit(0)
            }
            Some(device) => device,
            None => "default",
        }.to_string();

        AlsaSink(None, name)
    }
}

impl Sink for AlsaSink {
    fn start(&mut self) -> io::Result<()> {
        if self.0.is_none() {
            let pcm = open_device(&self.1);
            match pcm {
                Ok(p) => self.0 = Some(p),
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
        {
            let pcm = self.0.as_ref().unwrap();
            pcm.drain().unwrap();
        }
        self.0 = None;
        Ok(())
    }

    fn write(&mut self, data: &[i16]) -> io::Result<()> {
        let pcm = self.0.as_mut().unwrap();
        let io = pcm.io_i16().unwrap();

        match io.writei(&data) {
            Ok(_) => (),
            Err(err) => pcm.try_recover(err, false).unwrap(),
        }

        Ok(())
    }
}
