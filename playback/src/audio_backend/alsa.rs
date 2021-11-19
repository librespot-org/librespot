use super::{Open, Sink, SinkAsBytes, SinkError, SinkResult};
use crate::config::AudioFormat;
use crate::convert::Converter;
use crate::decoder::AudioPacket;
use crate::{NUM_CHANNELS, SAMPLE_RATE};
use alsa::device_name::HintIter;
use alsa::pcm::{Access, Format, HwParams, PCM};
use alsa::{Direction, ValueOr};
use std::cmp::min;
use std::process::exit;
use std::time::Duration;
use thiserror::Error;

// 0.5 sec buffer.
const PERIOD_TIME: Duration = Duration::from_millis(100);
const BUFFER_TIME: Duration = Duration::from_millis(500);

#[derive(Debug, Error)]
enum AlsaError {
    #[error("<AlsaSink> Device {device} Unsupported Format {alsa_format:?} ({format:?}), {e}")]
    UnsupportedFormat {
        device: String,
        alsa_format: Format,
        format: AudioFormat,
        e: alsa::Error,
    },

    #[error("<AlsaSink> Device {device} Unsupported Channel Count {channel_count}, {e}")]
    UnsupportedChannelCount {
        device: String,
        channel_count: u8,
        e: alsa::Error,
    },

    #[error("<AlsaSink> Device {device} Unsupported Sample Rate {samplerate}, {e}")]
    UnsupportedSampleRate {
        device: String,
        samplerate: u32,
        e: alsa::Error,
    },

    #[error("<AlsaSink> Device {device} Unsupported Access Type RWInterleaved, {e}")]
    UnsupportedAccessType { device: String, e: alsa::Error },

    #[error("<AlsaSink> Device {device} May be Invalid, Busy, or Already in Use, {e}")]
    PcmSetUp { device: String, e: alsa::Error },

    #[error("<AlsaSink> Failed to Drain PCM Buffer, {0}")]
    DrainFailure(alsa::Error),

    #[error("<AlsaSink> {0}")]
    OnWrite(alsa::Error),

    #[error("<AlsaSink> Hardware, {0}")]
    HwParams(alsa::Error),

    #[error("<AlsaSink> Software, {0}")]
    SwParams(alsa::Error),

    #[error("<AlsaSink> PCM, {0}")]
    Pcm(alsa::Error),

    #[error("<AlsaSink> Could Not Parse Ouput Name(s) and/or Description(s)")]
    Parsing,

    #[error("<AlsaSink>")]
    NotConnected,
}

impl From<AlsaError> for SinkError {
    fn from(e: AlsaError) -> SinkError {
        use AlsaError::*;
        let es = e.to_string();
        match e {
            DrainFailure(_) | OnWrite(_) => SinkError::OnWrite(es),
            PcmSetUp { .. } => SinkError::ConnectionRefused(es),
            NotConnected => SinkError::NotConnected(es),
            _ => SinkError::InvalidParams(es),
        }
    }
}

impl From<AudioFormat> for Format {
    fn from(f: AudioFormat) -> Format {
        use AudioFormat::*;
        match f {
            F64 => Format::float64(),
            F32 => Format::float(),
            S32 => Format::s32(),
            S24 => Format::s24(),
            S16 => Format::s16(),
            #[cfg(target_endian = "little")]
            S24_3 => Format::S243LE,
            #[cfg(target_endian = "big")]
            S24_3 => Format::S243BE,
        }
    }
}

pub struct AlsaSink {
    pcm: Option<PCM>,
    format: AudioFormat,
    device: String,
    period_buffer: Vec<u8>,
}

fn list_compatible_devices() -> SinkResult<()> {
    println!("\n\n\tCompatible alsa device(s):\n");
    println!("\t------------------------------------------------------\n");

    let i = HintIter::new_str(None, "pcm").map_err(|_| AlsaError::Parsing)?;

    for a in i {
        if let Some(Direction::Playback) = a.direction {
            let name = a.name.ok_or(AlsaError::Parsing)?;
            let desc = a.desc.ok_or(AlsaError::Parsing)?;

            if let Ok(pcm) = PCM::new(&name, Direction::Playback, false) {
                if let Ok(hwp) = HwParams::any(&pcm) {
                    // Only show devices that support
                    // 2 ch 44.1 Interleaved.
                    if hwp.set_access(Access::RWInterleaved).is_ok()
                        && hwp.set_rate(SAMPLE_RATE, ValueOr::Nearest).is_ok()
                        && hwp.set_channels(NUM_CHANNELS as u32).is_ok()
                    {
                        println!("\tDevice:\n\n\t\t{}\n", name);
                        println!("\tDescription:\n\n\t\t{}\n", desc.replace("\n", "\n\t\t"));

                        let mut supported_formats = vec![];

                        for f in &[
                            AudioFormat::S16,
                            AudioFormat::S24,
                            AudioFormat::S24_3,
                            AudioFormat::S32,
                            AudioFormat::F32,
                            AudioFormat::F64,
                        ] {
                            if hwp.test_format(Format::from(*f)).is_ok() {
                                supported_formats.push(format!("{:?}", f));
                            }
                        }

                        println!(
                            "\tSupported Format(s):\n\n\t\t{}\n",
                            supported_formats.join(" ")
                        );
                        println!("\t------------------------------------------------------\n");
                    }
                };
            }
        }
    }

    Ok(())
}

fn open_device(dev_name: &str, format: AudioFormat) -> SinkResult<(PCM, usize)> {
    let pcm = PCM::new(dev_name, Direction::Playback, false).map_err(|e| AlsaError::PcmSetUp {
        device: dev_name.to_string(),
        e,
    })?;

    let bytes_per_period = {
        let hwp = HwParams::any(&pcm).map_err(AlsaError::HwParams)?;

        hwp.set_access(Access::RWInterleaved)
            .map_err(|e| AlsaError::UnsupportedAccessType {
                device: dev_name.to_string(),
                e,
            })?;

        let alsa_format = Format::from(format);

        hwp.set_format(alsa_format)
            .map_err(|e| AlsaError::UnsupportedFormat {
                device: dev_name.to_string(),
                alsa_format,
                format,
                e,
            })?;

        hwp.set_rate(SAMPLE_RATE, ValueOr::Nearest).map_err(|e| {
            AlsaError::UnsupportedSampleRate {
                device: dev_name.to_string(),
                samplerate: SAMPLE_RATE,
                e,
            }
        })?;

        hwp.set_channels(NUM_CHANNELS as u32)
            .map_err(|e| AlsaError::UnsupportedChannelCount {
                device: dev_name.to_string(),
                channel_count: NUM_CHANNELS,
                e,
            })?;

        hwp.set_buffer_time_near(BUFFER_TIME.as_micros() as u32, ValueOr::Nearest)
            .map_err(AlsaError::HwParams)?;

        hwp.set_period_time_near(PERIOD_TIME.as_micros() as u32, ValueOr::Nearest)
            .map_err(AlsaError::HwParams)?;

        pcm.hw_params(&hwp).map_err(AlsaError::Pcm)?;

        let swp = pcm.sw_params_current().map_err(AlsaError::Pcm)?;

        // Don't assume we got what we wanted. Ask to make sure.
        let frames_per_period = hwp.get_period_size().map_err(AlsaError::HwParams)?;

        let frames_per_buffer = hwp.get_buffer_size().map_err(AlsaError::HwParams)?;

        swp.set_start_threshold(frames_per_buffer - frames_per_period)
            .map_err(AlsaError::SwParams)?;

        pcm.sw_params(&swp).map_err(AlsaError::Pcm)?;

        trace!("Frames per Buffer: {:?}", frames_per_buffer);
        trace!("Frames per Period: {:?}", frames_per_period);

        // Let ALSA do the math for us.
        pcm.frames_to_bytes(frames_per_period) as usize
    };

    trace!("Period Buffer size in bytes: {:?}", bytes_per_period);

    Ok((pcm, bytes_per_period))
}

impl Open for AlsaSink {
    fn open(device: Option<String>, format: AudioFormat) -> Self {
        let name = match device.as_deref() {
            Some("?") => match list_compatible_devices() {
                Ok(_) => {
                    exit(0);
                }
                Err(e) => {
                    error!("{}", e);
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
    fn start(&mut self) -> SinkResult<()> {
        if self.pcm.is_none() {
            let (pcm, bytes_per_period) = open_device(&self.device, self.format)?;
            self.pcm = Some(pcm);

            if self.period_buffer.capacity() != bytes_per_period {
                self.period_buffer = Vec::with_capacity(bytes_per_period);
            }

            // Should always match the "Period Buffer size in bytes: " trace! message.
            trace!(
                "Period Buffer capacity: {:?}",
                self.period_buffer.capacity()
            );
        }

        Ok(())
    }

    fn stop(&mut self) -> SinkResult<()> {
        // Zero fill the remainder of the period buffer and
        // write any leftover data before draining the actual PCM buffer.
        self.period_buffer.resize(self.period_buffer.capacity(), 0);
        self.write_buf()?;

        let pcm = self.pcm.take().ok_or(AlsaError::NotConnected)?;

        pcm.drain().map_err(AlsaError::DrainFailure)?;

        Ok(())
    }

    sink_as_bytes!();
}

impl SinkAsBytes for AlsaSink {
    fn write_bytes(&mut self, data: &[u8]) -> SinkResult<()> {
        let mut start_index = 0;
        let data_len = data.len();
        let capacity = self.period_buffer.capacity();

        loop {
            let data_left = data_len - start_index;
            let space_left = capacity - self.period_buffer.len();
            let data_to_buffer = min(data_left, space_left);
            let end_index = start_index + data_to_buffer;

            self.period_buffer
                .extend_from_slice(&data[start_index..end_index]);

            if self.period_buffer.len() == capacity {
                self.write_buf()?;
            }

            if end_index == data_len {
                break Ok(());
            }

            start_index = end_index;
        }
    }
}

impl AlsaSink {
    pub const NAME: &'static str = "alsa";

    fn write_buf(&mut self) -> SinkResult<()> {
        let pcm = self.pcm.as_mut().ok_or(AlsaError::NotConnected)?;

        if let Err(e) = pcm.io_bytes().writei(&self.period_buffer) {
            // Capture and log the original error as a warning, and then try to recover.
            // If recovery fails then forward that error back to player.
            warn!(
                "Error writing from AlsaSink buffer to PCM, trying to recover, {}",
                e
            );

            pcm.try_recover(e, false).map_err(AlsaError::OnWrite)?
        }

        self.period_buffer.clear();
        Ok(())
    }
}
