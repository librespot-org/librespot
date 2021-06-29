use super::{Open, Sink, SinkAsBytes};
use crate::config::AudioFormat;
use crate::convert::Converter;
use crate::decoder::AudioPacket;
use crate::{NUM_CHANNELS, SAMPLE_RATE};
use alsa::device_name::HintIter;
use alsa::pcm::{Access, Format, HwParams, PCM};
use alsa::{Direction, ValueOr};
use std::cmp::min;
use std::io;
use std::process::exit;
use std::time::Duration;
use thiserror::Error;

// 0.5 sec buffer.
const PERIOD_TIME: Duration = Duration::from_millis(100);
const BUFFER_TIME: Duration = Duration::from_millis(500);

#[derive(Debug, Error)]
enum AlsaError {
    #[error("AlsaSink, device {device} may be invalid or busy, {err}")]
    PcmSetUp { device: String, err: alsa::Error },
    #[error("AlsaSink, device {device} unsupported access type RWInterleaved, {err}")]
    UnsupportedAccessType { device: String, err: alsa::Error },
    #[error("AlsaSink, device {device} unsupported format {format:?}, {err}")]
    UnsupportedFormat {
        device: String,
        format: AudioFormat,
        err: alsa::Error,
    },
    #[error("AlsaSink, device {device} unsupported sample rate {samplerate}, {err}")]
    UnsupportedSampleRate {
        device: String,
        samplerate: u32,
        err: alsa::Error,
    },
    #[error("AlsaSink, device {device} unsupported channel count {channel_count}, {err}")]
    UnsupportedChannelCount {
        device: String,
        channel_count: u8,
        err: alsa::Error,
    },
    #[error("AlsaSink Hardware Parameters Error, {0}")]
    HwParams(alsa::Error),
    #[error("AlsaSink Software Parameters Error, {0}")]
    SwParams(alsa::Error),
    #[error("AlsaSink PCM Error, {0}")]
    Pcm(alsa::Error),
}

pub struct AlsaSink {
    pcm: Option<PCM>,
    format: AudioFormat,
    device: String,
    period_buffer: Vec<u8>,
}

fn list_outputs() -> io::Result<()> {
    println!("Listing available Alsa outputs:");
    for t in &["pcm", "ctl", "hwdep"] {
        println!("{} devices:", t);
        let i = match HintIter::new_str(None, &t) {
            Ok(i) => i,
            Err(e) => {
                return Err(io::Error::new(io::ErrorKind::Other, e));
            }
        };
        for a in i {
            if let Some(Direction::Playback) = a.direction {
                // mimic aplay -L
                let name = a
                    .name
                    .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Could not parse name"))?;
                let desc = a
                    .desc
                    .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Could not parse desc"))?;
                println!("{}\n\t{}\n", name, desc.replace("\n", "\n\t"));
            }
        }
    }

    Ok(())
}

fn open_device(dev_name: &str, format: AudioFormat) -> Result<(PCM, usize), AlsaError> {
    let pcm = PCM::new(dev_name, Direction::Playback, false).map_err(|e| AlsaError::PcmSetUp {
        device: dev_name.to_string(),
        err: e,
    })?;

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
        let hwp = HwParams::any(&pcm).map_err(AlsaError::HwParams)?;
        hwp.set_access(Access::RWInterleaved)
            .map_err(|e| AlsaError::UnsupportedAccessType {
                device: dev_name.to_string(),
                err: e,
            })?;

        hwp.set_format(alsa_format)
            .map_err(|e| AlsaError::UnsupportedFormat {
                device: dev_name.to_string(),
                format,
                err: e,
            })?;

        hwp.set_rate(SAMPLE_RATE, ValueOr::Nearest).map_err(|e| {
            AlsaError::UnsupportedSampleRate {
                device: dev_name.to_string(),
                samplerate: SAMPLE_RATE,
                err: e,
            }
        })?;

        hwp.set_channels(NUM_CHANNELS as u32)
            .map_err(|e| AlsaError::UnsupportedChannelCount {
                device: dev_name.to_string(),
                channel_count: NUM_CHANNELS,
                err: e,
            })?;

        hwp.set_buffer_time_near(BUFFER_TIME.as_micros() as u32, ValueOr::Nearest)
            .map_err(AlsaError::HwParams)?;

        hwp.set_period_time_near(PERIOD_TIME.as_micros() as u32, ValueOr::Nearest)
            .map_err(AlsaError::HwParams)?;

        pcm.hw_params(&hwp).map_err(AlsaError::Pcm)?;

        let swp = pcm.sw_params_current().map_err(AlsaError::Pcm)?;

        // Don't assume we got what we wanted.
        // Ask to make sure.
        let frames_per_period = hwp.get_period_size().map_err(AlsaError::HwParams)?;

        let frames_per_buffer = hwp.get_buffer_size().map_err(AlsaError::HwParams)?;

        swp.set_start_threshold(frames_per_buffer - frames_per_period)
            .map_err(AlsaError::SwParams)?;

        pcm.sw_params(&swp).map_err(AlsaError::Pcm)?;

        // Let ALSA do the math for us.
        pcm.frames_to_bytes(frames_per_period) as usize
    };

    Ok((pcm, bytes_per_period))
}

impl Open for AlsaSink {
    fn open(device: Option<String>, format: AudioFormat) -> Self {
        let name = match device.as_deref() {
            Some("?") => match list_outputs() {
                Ok(_) => {
                    exit(0);
                }
                Err(err) => {
                    error!("Error listing Alsa outputs, {}", err);
                    exit(1);
                }
            },
            Some(device) => device,
            None => "default",
        }
        .to_string();

        info!("Using AlsaSink with format: {:?}", format);

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
            match open_device(&self.device, self.format) {
                Ok((pcm, bytes_per_period)) => {
                    self.pcm = Some(pcm);
                    self.period_buffer = Vec::with_capacity(bytes_per_period);
                }
                Err(e) => {
                    return Err(io::Error::new(io::ErrorKind::Other, e));
                }
            }
        }

        Ok(())
    }

    fn stop(&mut self) -> io::Result<()> {
        {
            // Write any leftover data in the period buffer
            // before draining the actual buffer
            self.write_bytes(&[])?;
            let pcm = self.pcm.as_mut().ok_or_else(|| {
                io::Error::new(io::ErrorKind::Other, "Error stopping AlsaSink, PCM is None")
            })?;
            pcm.drain().map_err(|e| {
                io::Error::new(
                    io::ErrorKind::Other,
                    format!("Error stopping AlsaSink {}", e),
                )
            })?
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
                self.write_buf()?;
                self.period_buffer.clear();
            }
        }

        Ok(())
    }
}

impl AlsaSink {
    pub const NAME: &'static str = "alsa";

    fn write_buf(&mut self) -> io::Result<()> {
        let pcm = self.pcm.as_mut().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::Other,
                "Error writing from AlsaSink buffer to PCM, PCM is None",
            )
        })?;
        let io = pcm.io_bytes();
        if let Err(err) = io.writei(&self.period_buffer) {
            // Capture and log the original error as a warning, and then try to recover.
            // If recovery fails then forward that error back to player.
            warn!(
                "Error writing from AlsaSink buffer to PCM, trying to recover {}",
                err
            );
            pcm.try_recover(err, false).map_err(|e| {
                io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "Error writing from AlsaSink buffer to PCM, recovery failed {}",
                        e
                    ),
                )
            })?
        }

        Ok(())
    }
}
