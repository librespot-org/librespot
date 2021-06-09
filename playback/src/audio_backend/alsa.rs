use super::{Open, Sink, SinkAsBytes};
use crate::config::AudioFormat;
use crate::convert::Converter;
use crate::decoder::AudioPacket;
use crate::{NUM_CHANNELS, SAMPLE_RATE};
use alsa::device_name::HintIter;
use alsa::pcm::{Access, Format, HwParams, PCM};
use alsa::{Direction, Error, ValueOr};
use std::cmp::min;
use std::ffi::CString;
use std::io;
use std::process::exit;
use std::time::Duration;

// 125 ms Period time * 4 periods = 0.5 sec buffer.
const PERIOD_TIME: Duration = Duration::from_millis(125);
const NUM_PERIODS: u32 = 4;

pub struct AlsaSink {
    pcm: Option<PCM>,
    format: AudioFormat,
    device: String,
    period_buffer: Vec<u8>,
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

fn open_device(dev_name: &str, format: AudioFormat) -> Result<(PCM, usize), Box<Error>> {
    let pcm = PCM::new(dev_name, Direction::Playback, false)?;
    let alsa_format = match format {
        AudioFormat::F64 => Format::float64(),
        AudioFormat::F32 => Format::float(),
        AudioFormat::S32 => Format::s32(),
        AudioFormat::S24 => Format::s24(),
        AudioFormat::S16 => Format::s16(),

        #[cfg(target_endian = "little")]
        AudioFormat::S24_3 => Format::S243LE,
        #[cfg(target_endian = "big")]
        AudioFormat::S24_3 => Format::S243BE,
    };

    let bytes_per_period = {
        let hwp = HwParams::any(&pcm)?;
        hwp.set_access(Access::RWInterleaved)?;
        hwp.set_format(alsa_format)?;
        hwp.set_rate(SAMPLE_RATE, ValueOr::Nearest)?;
        hwp.set_channels(NUM_CHANNELS as u32)?;
        // Deal strictly in time and periods.
        hwp.set_periods(NUM_PERIODS, ValueOr::Nearest)?;
        hwp.set_period_time_near(PERIOD_TIME.as_micros() as u32, ValueOr::Nearest)?;
        pcm.hw_params(&hwp)?;

        let swp = pcm.sw_params_current()?;
        // Don't assume we got what we wanted.
        // Ask to make sure.
        let frames_per_period = hwp.get_period_size()?;

        swp.set_start_threshold(hwp.get_buffer_size()? - frames_per_period)?;
        pcm.sw_params(&swp)?;

        // Let ALSA do the math for us.
        pcm.frames_to_bytes(frames_per_period) as usize
    };

    Ok((pcm, bytes_per_period))
}

impl Open for AlsaSink {
    fn open(device: Option<String>, format: AudioFormat) -> Self {
        info!("Using Alsa sink with format: {:?}", format);

        let name = match device.as_deref() {
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
            format,
            device: name,
            period_buffer: vec![],
        }
    }
}

impl Sink for AlsaSink {
    fn start(&mut self) -> io::Result<()> {
        if self.pcm.is_none() {
            let pcm = open_device(&self.device, self.format);
            match pcm {
                Ok((p, bytes_per_period)) => {
                    self.pcm = Some(p);
                    self.period_buffer = Vec::with_capacity(bytes_per_period);
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
                self.period_buffer.capacity() - self.period_buffer.len(),
                data.len() - processed_data,
            );
            self.period_buffer
                .extend_from_slice(&data[processed_data..processed_data + data_to_buffer]);
            processed_data += data_to_buffer;
            if self.period_buffer.len() == self.period_buffer.capacity() {
                self.write_buf();
                self.period_buffer.clear();
            }
        }

        Ok(())
    }
}

impl AlsaSink {
    pub const NAME: &'static str = "alsa";

    fn write_buf(&mut self) {
        let pcm = self.pcm.as_mut().unwrap();
        let io = pcm.io_bytes();
        match io.writei(&self.period_buffer) {
            Ok(_) => (),
            Err(err) => pcm.try_recover(err, false).unwrap(),
        };
    }
}
