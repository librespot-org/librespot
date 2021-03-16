use super::{Open, Sink, SinkAsBytes};
use crate::audio::AudioPacket;
use crate::config::AudioFormat;
use crate::player::{NUM_CHANNELS, SAMPLES_PER_SECOND, SAMPLE_RATE};
use alsa::device_name::HintIter;
use alsa::pcm::{Access, Format, Frames, HwParams, PCM};
use alsa::{Direction, Error, ValueOr};
use std::cmp::min;
use std::ffi::CString;
use std::io;
use std::process::exit;

const BUFFERED_LATENCY: f32 = 0.125; // seconds
const BUFFERED_PERIODS: Frames = 4;

pub struct AlsaSink {
    pcm: Option<PCM>,
    format: AudioFormat,
    device: String,
    buffer: Vec<u8>,
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

fn open_device(dev_name: &str, format: AudioFormat) -> Result<(PCM, Frames), Box<Error>> {
    let pcm = PCM::new(dev_name, Direction::Playback, false)?;
    let alsa_format = match format {
        AudioFormat::F32 => Format::float(),
        AudioFormat::S32 => Format::s32(),
        AudioFormat::S24 => Format::s24(),
        AudioFormat::S24_3 => Format::S243LE,
        AudioFormat::S16 => Format::s16(),
    };

    // http://www.linuxjournal.com/article/6735?page=0,1#N0x19ab2890.0x19ba78d8
    // latency = period_size * periods / (rate * bytes_per_frame)
    // For stereo samples encoded as 32-bit float, one frame has a length of eight bytes.
    let mut period_size = ((SAMPLES_PER_SECOND * format.size() as u32) as f32
        * (BUFFERED_LATENCY / BUFFERED_PERIODS as f32)) as Frames;

    // Set hardware parameters: 44100 Hz / stereo / 32-bit float or 16-bit signed integer
    {
        let hwp = HwParams::any(&pcm)?;
        hwp.set_access(Access::RWInterleaved)?;
        hwp.set_format(alsa_format)?;
        hwp.set_rate(SAMPLE_RATE, ValueOr::Nearest)?;
        hwp.set_channels(NUM_CHANNELS as u32)?;
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
    fn open(device: Option<String>, format: AudioFormat) -> AlsaSink {
        info!("Using Alsa sink with format: {:?}", format);

        let name = match device.as_ref().map(AsRef::as_ref) {
            Some("?") => {
                println!("Listing available Alsa outputs:");
                list_outputs();
                exit(0)
            }
            Some(device) => device,
            None => "default",
        }
        .to_string();

        Self {
            pcm: None,
            format: format,
            device: name,
            buffer: vec![],
        }
    }
}

impl Sink for AlsaSink {
    fn start(&mut self) -> io::Result<()> {
        if self.pcm.is_none() {
            let pcm = open_device(&self.device, self.format);
            match pcm {
                Ok((p, period_size)) => {
                    self.pcm = Some(p);
                    // Create a buffer for all samples for a full period
                    self.buffer = Vec::with_capacity(
                        period_size as usize * BUFFERED_PERIODS as usize * self.format.size(),
                    );
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
            // Write any leftover data in the period buffer
            // before draining the actual buffer
            self.write_bytes(&[]).expect("could not flush buffer");
            let pcm = self.pcm.as_mut().unwrap();
            pcm.drain().unwrap();
        }
        self.pcm = None;
        Ok(())
    }

    sink_as_bytes!();
}

impl SinkAsBytes for AlsaSink {
    fn write_bytes(&mut self, data: &[u8]) -> io::Result<()> {
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
                self.write_buf().expect("could not append to buffer");
                self.buffer.clear();
            }
        }

        Ok(())
    }
}

impl AlsaSink {
    fn write_buf(&mut self) -> io::Result<()> {
        let pcm = self.pcm.as_mut().unwrap();
        let io = pcm.io_bytes();
        match io.writei(&self.buffer) {
            Ok(_) => (),
            Err(err) => pcm.try_recover(err, false).unwrap(),
        };

        Ok(())
    }
}
