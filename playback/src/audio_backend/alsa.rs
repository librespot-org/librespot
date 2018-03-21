use super::{Open, Sink};
use alsa::{Direction, Error, ValueOr};
use alsa::device_name::HintIter;
use std::ffi::{CStr, CString};
use alsa::pcm::{Access, Format, HwParams, PCM};
use std::io;
use std::process::exit;


pub struct AlsaSink(Option<PCM>, String);

fn list_outputs() {
    for t in &["pcm", "ctl", "rawmidi", "timer", "seq", "hwdep"] {
     println!("{} devices:", t);
     let i = HintIter::new(None, &*CString::new(*t).unwrap()).unwrap();
     for a in i { println!("  {:?}", a) }
 }
}

impl Open for AlsaSink {
   fn open(device: Option<String>) -> AlsaSink {
        info!("Using alsa sink");

        let name = match device.as_ref().map(AsRef::as_ref) {
            Some("?") => {
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
            let pcm = PCM::new(&*self.1, Direction::Playback, false).unwrap();
            {
                // Set hardware parameters: 44100 Hz / Stereo / 16 bit
                let hwp = HwParams::any(&pcm).unwrap();
                hwp.set_channels(2).unwrap();
                hwp.set_rate(44100, ValueOr::Nearest).unwrap();
                hwp.set_format(Format::s16()).unwrap();
                hwp.set_access(Access::RWInterleaved).unwrap();
                pcm.hw_params(&hwp).unwrap();
                println!("PCM status: {:?}, {:?}", pcm.state(), pcm.hw_params_current().unwrap())
                }
            PCM::prepare(&pcm).unwrap();

            self.0 = Some(pcm);
        }

        Ok(())
    }

    fn stop(&mut self) -> io::Result<()> {
        {
            let pcm = self.0.as_mut().unwrap();
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
            // Err(err) => println!("{:?}",err),
        }

        Ok(())
    }
}
