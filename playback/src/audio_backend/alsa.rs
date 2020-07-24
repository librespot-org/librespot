use super::{Open, Sink};
use alsa::device_name::HintIter;
use alsa::pcm::{Access, Format, Frames, HwParams, PCM};
use alsa::{Direction, Error, ValueOr};
use std::cmp::min;
use std::ffi::CString;
use std::io;
use std::process::exit;

const PREFERED_PERIOD_SIZE: Frames = 5512; // Period of roughly 125ms
const BUFFERED_PERIODS: Frames = 4;

pub struct AlsaSink {
    pcm: Option<PCM>,
    device: String,
    buffer: Vec<i16>,
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

fn open_device(dev_name: &str) -> Result<(PCM, Frames), Box<Error>> {
    let pcm = PCM::new(dev_name, Direction::Playback, false)?;
    let mut period_size = PREFERED_PERIOD_SIZE;
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
        period_size = hwp.set_period_size_near(period_size, ValueOr::Greater)?;
        hwp.set_buffer_size_near(period_size * BUFFERED_PERIODS)?;
        pcm.hw_params(&hwp)?;

        let swp = pcm.sw_params_current()?;
        swp.set_start_threshold(hwp.get_buffer_size()? - hwp.get_period_size()?)?;
        pcm.sw_params(&swp)?;
    }

    Ok((pcm, period_size))
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
            buffer: vec![],
        }
    }
}

impl Sink for AlsaSink {
    fn start(&mut self) -> io::Result<()> {
        if self.pcm.is_none() {
            let pcm = open_device(&self.device);
            match pcm {
                Ok((p, period_size)) => {
                    self.pcm = Some(p);
                    // Create a buffer for all samples for a full period
                    self.buffer = Vec::with_capacity((period_size * 2) as usize);
                }
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
            match io.writei(&self.buffer[..]) {
                Ok(_) => (),
                Err(err) => pcm.try_recover(err, false).unwrap(),
            }
            pcm.drain().unwrap();
        }
        self.pcm = None;
        Ok(())
    }

    fn write(&mut self, data: &[i16]) -> io::Result<()> {
        let mut processed_data = 0;
        while processed_data < data.len() {
            let data_to_buffer = min(
                self.buffer.capacity() - self.buffer.len(),
                data.len() - processed_data,
            );
            self.buffer
                .extend_from_slice(&data[processed_data..processed_data + data_to_buffer]);
            processed_data += data_to_buffer;
            if self.buffer.len() == self.buffer.capacity() {
                let pcm = self.pcm.as_mut().unwrap();
                let io = pcm.io_i16().unwrap();
                match io.writei(&self.buffer) {
                    Ok(_) => (),
                    Err(err) => pcm.try_recover(err, false).unwrap(),
                }
                self.buffer.clear();
            }
        }

        Ok(())
    }
}
