use super::{Open, Sink, SinkAsBytes, SinkError, SinkResult};
use crate::config::AudioFormat;
use crate::convert::Converter;
use crate::decoder::AudioPacket;
use crate::{NUM_CHANNELS, SAMPLE_RATE};
use alsa::device_name::HintIter;
use alsa::pcm::{Access, Format, Frames, HwParams, PCM};
use alsa::{Direction, ValueOr};
use std::process::exit;
use thiserror::Error;

const OPTIMAL_BUFFER_SIZE: Frames = SAMPLE_RATE as Frames / 2;
const OPTIMAL_PERIOD_SIZE: Frames = SAMPLE_RATE as Frames / 10;
const OPTIMAL_NUM_PERIODS: Frames = 5;
const MIN_NUM_PERIODS: Frames = 2;

const COMMON_SAMPLE_RATES: [u32; 14] = [
    8000, 11025, 16000, 22050, 44100, 48000, 88200, 96000, 176400, 192000, 352800, 384000, 705600,
    768000,
];

const FORMATS: [AudioFormat; 6] = [
    AudioFormat::S16,
    AudioFormat::S24,
    AudioFormat::S24_3,
    AudioFormat::S32,
    AudioFormat::F32,
    AudioFormat::F64,
];

#[derive(Debug, Error)]
enum AlsaError {
    #[error("<AlsaSink> Device {device} Unsupported Format {alsa_format} ({format:?}), {e}, Supported Format(s): {supported_formats:?}")]
    UnsupportedFormat {
        device: String,
        alsa_format: Format,
        format: AudioFormat,
        supported_formats: Vec<String>,
        e: alsa::Error,
    },

    #[error("<AlsaSink> Device {device} Unsupported Channel Count {channel_count}, {e}, Supported Channel Count(s): {supported_channel_counts:?}")]
    UnsupportedChannelCount {
        device: String,
        channel_count: u8,
        supported_channel_counts: Vec<u32>,
        e: alsa::Error,
    },

    #[error("<AlsaSink> Device {device} Unsupported Sample Rate {samplerate}, {e}, Supported Sample Rate(s): {supported_rates:?}")]
    UnsupportedSampleRate {
        device: String,
        samplerate: u32,
        supported_rates: Vec<u32>,
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

    #[error("<AlsaSink> Could Not Parse Output Name(s) and/or Description(s), {0}")]
    Parsing(alsa::Error),
}

impl From<AlsaError> for SinkError {
    fn from(e: AlsaError) -> SinkError {
        use AlsaError::*;
        let es = e.to_string();
        match e {
            DrainFailure(_) | OnWrite(_) => SinkError::OnWrite(es),
            PcmSetUp { .. } => SinkError::ConnectionRefused(es),
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
            S24_3 => Format::s24_3(),
            S16 => Format::s16(),
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
    let i = HintIter::new_str(None, "pcm").map_err(AlsaError::Parsing)?;

    println!("\n\n\tCompatible alsa device(s):\n");
    println!("\t------------------------------------------------------\n");

    for a in i {
        if let Some(Direction::Playback) = a.direction {
            if let Some(name) = a.name {
                // surround* outputs throw:
                // ALSA lib pcm_route.c:877:(find_matching_chmap) Found no matching channel map
                if name.contains(':') && !name.starts_with("surround") {
                    if let Ok(pcm) = PCM::new(&name, Direction::Playback, false) {
                        if let Ok(hwp) = HwParams::any(&pcm) {
                            // Only show devices that support
                            // 2 ch 44.1 Interleaved.

                            if hwp.set_access(Access::RWInterleaved).is_ok()
                                && hwp.set_rate(SAMPLE_RATE, ValueOr::Nearest).is_ok()
                                && hwp.set_channels(NUM_CHANNELS as u32).is_ok()
                            {
                                let supported_formats: Vec<String> = FORMATS
                                    .iter()
                                    .filter_map(|f| {
                                        if hwp.test_format((*f).into()).is_ok() {
                                            Some(format!("{f:?}"))
                                        } else {
                                            None
                                        }
                                    })
                                    .collect();

                                if !supported_formats.is_empty() {
                                    println!("\tDevice:\n\n\t\t{name}\n");

                                    println!(
                                        "\tDescription:\n\n\t\t{}\n",
                                        a.desc.unwrap_or_default().replace('\n', "\n\t\t")
                                    );

                                    println!(
                                        "\tSupported Format(s):\n\n\t\t{}\n",
                                        supported_formats.join(" ")
                                    );

                                    println!(
                                        "\t------------------------------------------------------\n"
                                    );
                                }
                            }
                        };
                    }
                }
            }
        }
    }

    Ok(())
}

impl Open for AlsaSink {
    fn open(device: Option<String>, format: AudioFormat) -> Self {
        let name = match device.as_deref() {
            Some("?") => match list_compatible_devices() {
                Ok(_) => {
                    exit(0);
                }
                Err(e) => {
                    error!("{e}");
                    exit(1);
                }
            },
            Some(device) => device,
            None => "default",
        }
        .to_string();

        info!("Using AlsaSink with format: {format:?}");

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
            self.open_device()?;
        }

        Ok(())
    }

    fn stop(&mut self) -> SinkResult<()> {
        // Zero fill the remainder of the period buffer and
        // write any leftover data before draining the actual PCM buffer.
        self.period_buffer.resize(self.period_buffer.capacity(), 0);
        self.write_buf()?;

        if let Some(pcm) = self.pcm.take() {
            pcm.drain().map_err(AlsaError::DrainFailure)?;
        }

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
            let data_to_buffer = data_left.min(space_left);
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

    fn set_period_and_buffer_size(hwp: &HwParams) -> bool {
        let period_size = match hwp.set_period_size_near(OPTIMAL_PERIOD_SIZE, ValueOr::Nearest) {
            Ok(period_size) => {
                if period_size > 0 {
                    trace!("Closest Supported Period Size to Optimal ({OPTIMAL_PERIOD_SIZE}): {period_size}");
                    period_size
                } else {
                    trace!("Error getting Period Size, Period Size must be greater than 0, falling back to the device's default Buffer parameters");
                    0
                }
            }
            Err(e) => {
                trace!("Error getting Period Size: {e}, falling back to the device's default Buffer parameters");
                0
            }
        };

        if period_size > 0 {
            let buffer_size = match hwp
                .set_buffer_size_near((period_size * OPTIMAL_NUM_PERIODS).max(OPTIMAL_BUFFER_SIZE))
            {
                Ok(buffer_size) => {
                    if buffer_size >= period_size * MIN_NUM_PERIODS {
                        trace!("Closest Supported Buffer Size to Optimal ({OPTIMAL_BUFFER_SIZE}): {buffer_size}");
                        buffer_size
                    } else {
                        trace!("Error getting Buffer Size, Buffer Size must be at least {period_size} * {MIN_NUM_PERIODS}, falling back to the device's default Buffer parameters");
                        0
                    }
                }
                Err(e) => {
                    trace!("Error getting Buffer Size: {e}, falling back to the device's default Buffer parameters");
                    0
                }
            };

            return buffer_size > 0;
        }

        false
    }

    fn open_device(&mut self) -> SinkResult<()> {
        let pcm = PCM::new(&self.device, Direction::Playback, false).map_err(|e| {
            AlsaError::PcmSetUp {
                device: self.device.clone(),
                e,
            }
        })?;

        {
            let hwp = HwParams::any(&pcm).map_err(AlsaError::HwParams)?;

            hwp.set_access(Access::RWInterleaved).map_err(|e| {
                AlsaError::UnsupportedAccessType {
                    device: self.device.clone(),
                    e,
                }
            })?;

            let alsa_format = self.format.into();

            hwp.set_format(alsa_format).map_err(|e| {
                let supported_formats = FORMATS
                    .iter()
                    .filter_map(|f| {
                        if hwp.test_format((*f).into()).is_ok() {
                            Some(format!("{f:?}"))
                        } else {
                            None
                        }
                    })
                    .collect();

                AlsaError::UnsupportedFormat {
                    device: self.device.clone(),
                    alsa_format,
                    format: self.format,
                    supported_formats,
                    e,
                }
            })?;

            hwp.set_rate(SAMPLE_RATE, ValueOr::Nearest).map_err(|e| {
                let supported_rates = (hwp.get_rate_min().unwrap_or_default()
                    ..=hwp.get_rate_max().unwrap_or_default())
                    .filter(|r| COMMON_SAMPLE_RATES.contains(r) && hwp.test_rate(*r).is_ok())
                    .collect();

                AlsaError::UnsupportedSampleRate {
                    device: self.device.clone(),
                    samplerate: SAMPLE_RATE,
                    supported_rates,
                    e,
                }
            })?;

            hwp.set_channels(NUM_CHANNELS as u32).map_err(|e| {
                let supported_channel_counts = (hwp.get_channels_min().unwrap_or_default()
                    ..=hwp.get_channels_max().unwrap_or_default())
                    .filter(|c| hwp.test_channels(*c).is_ok())
                    .collect();

                AlsaError::UnsupportedChannelCount {
                    device: self.device.clone(),
                    channel_count: NUM_CHANNELS,
                    supported_channel_counts,
                    e,
                }
            })?;

            // Calculate a buffer and period size as close
            // to optimal as possible.

            // hwp continuity is very important.
            let hwp_clone = hwp.clone();

            if Self::set_period_and_buffer_size(&hwp_clone) {
                pcm.hw_params(&hwp_clone).map_err(AlsaError::Pcm)?;
            } else {
                pcm.hw_params(&hwp).map_err(AlsaError::Pcm)?;
            }

            let hwp = pcm.hw_params_current().map_err(AlsaError::Pcm)?;

            // Don't assume we got what we wanted. Ask to make sure.
            let buffer_size = hwp.get_buffer_size().map_err(AlsaError::HwParams)?;

            let period_size = hwp.get_period_size().map_err(AlsaError::HwParams)?;

            let swp = pcm.sw_params_current().map_err(AlsaError::Pcm)?;

            swp.set_start_threshold(buffer_size - period_size)
                .map_err(AlsaError::SwParams)?;

            pcm.sw_params(&swp).map_err(AlsaError::Pcm)?;

            if buffer_size != OPTIMAL_BUFFER_SIZE {
                trace!("A Buffer Size of {buffer_size} Frames is Suboptimal");

                if buffer_size < OPTIMAL_BUFFER_SIZE {
                    trace!("A smaller than necessary Buffer Size can lead to Buffer underruns (audio glitches) and high CPU usage.");
                } else {
                    trace!("A larger than necessary Buffer Size can lead to perceivable latency (lag).");
                }
            }

            let optimal_period_size = buffer_size / OPTIMAL_NUM_PERIODS;

            if period_size != optimal_period_size {
                trace!("A Period Size of {period_size} Frames is Suboptimal");

                if period_size < optimal_period_size {
                    trace!("A smaller than necessary Period Size relative to Buffer Size can lead to high CPU usage.");
                } else {
                    trace!("A larger than necessary Period Size relative to Buffer Size can lessen Buffer underrun (audio glitch) protection.");
                }
            }

            // Let ALSA do the math for us.
            let bytes_per_period = pcm.frames_to_bytes(period_size) as usize;

            trace!("Period Buffer size in bytes: {bytes_per_period}");

            self.period_buffer = Vec::with_capacity(bytes_per_period);
        }

        self.pcm = Some(pcm);

        Ok(())
    }

    fn write_buf(&mut self) -> SinkResult<()> {
        if let Some(pcm) = self.pcm.as_mut() {
            if let Err(e) = pcm.io_bytes().writei(&self.period_buffer) {
                // Capture and log the original error as a warning, and then try to recover.
                // If recovery fails then forward that error back to player.
                warn!("Error writing from AlsaSink Buffer to PCM, trying to recover, {e}");

                pcm.try_recover(e, false).map_err(AlsaError::OnWrite)?;
            }
        }

        self.period_buffer.clear();

        Ok(())
    }
}
