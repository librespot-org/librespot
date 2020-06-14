use super::{Open, Sink};
use alsa::device_name::HintIter;
use alsa::pcm::{Access, Format, Frames, HwParams, PCM};
use alsa::{Direction, Error, ValueOr};
use std::cmp::min;
use std::ffi::CString;
use std::io;
use std::process::exit;

const PERIOD_SIZE: usize = 5512; // Period of roughly 125ms
const BUFFERED_PERIODS: usize = 4; // ~ 0.5s latency

pub struct AlsaSink {
    pcm: Option<PCM>,
    device: String,
    buffer: [i16; PERIOD_SIZE as usize],
    buffered_data: usize,
}

fn list_outputs() {
    for t in &["pcm", "ctl", "hwdep"] {
        println!("{} devices:", t);
        let i = HintIter::new(None, &*CString::new(*t).unwrap()).unwrap();
        for a in i {
            if let Some(Direction::Playback) = a.direction {
                // mimic aplay -L
                println!(
                    "{}\n\t{}\n",
                    a.name.unwrap(),
                    a.desc.unwrap().replace("\n", "\n\t")
                );
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
        hwp.set_period_size_near(PERIOD_SIZE as Frames, ValueOr::Nearest)?;
        hwp.set_buffer_size_near((PERIOD_SIZE * BUFFERED_PERIODS) as Frames)?;
        pcm.hw_params(&hwp)?;

        let swp = pcm.sw_params_current()?;
        swp.set_start_threshold(hwp.get_buffer_size()? - hwp.get_period_size()?)?;
        pcm.sw_params(&swp)?;
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
        }
        .to_string();

        AlsaSink {
            pcm: None,
            device: name,
            buffer: [0; PERIOD_SIZE],
            buffered_data: 0,
        }
    }
}

impl Sink for AlsaSink {
    fn start(&mut self) -> io::Result<()> {
        if self.pcm.is_none() {
            let pcm = open_device(&self.device);
            match pcm {
                Ok(p) => self.pcm = Some(p),
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
            let pcm = self.pcm.as_mut().unwrap();
            // Write any leftover data in the period buffer
            // before draining the actual buffer
            let io = pcm.io_i16().unwrap();
            match io.writei(&self.buffer[..self.buffered_data]) {
                Ok(_) => (),
                Err(err) => pcm.try_recover(err, false).unwrap(),
            }
            pcm.drain().unwrap();
        }
        self.pcm = None;
        self.buffered_data = 0;
        Ok(())
    }

    fn write(&mut self, data: &[i16]) -> io::Result<()> {
        let mut processed_data = 0;
        while processed_data < data.len() {
            let data_to_buffer = min(
                PERIOD_SIZE - self.buffered_data,
                data.len() - processed_data,
            );
            let buffer_slice =
                &mut self.buffer[self.buffered_data..self.buffered_data + data_to_buffer];
            buffer_slice.copy_from_slice(&data[processed_data..processed_data + data_to_buffer]);
            self.buffered_data += data_to_buffer;
            processed_data += data_to_buffer;
            if self.buffered_data == PERIOD_SIZE {
                self.buffered_data = 0;
                let pcm = self.pcm.as_mut().unwrap();
                let io = pcm.io_i16().unwrap();
                match io.writei(&self.buffer) {
                    Ok(_) => (),
                    Err(err) => pcm.try_recover(err, false).unwrap(),
                }
            }
        }

        Ok(())
    }
}
