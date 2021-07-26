use std::process::exit;
use std::thread;
use std::time::Duration;

use cpal::traits::{DeviceTrait, HostTrait};
use thiserror::Error;

use super::{Sink, SinkResult};
use crate::config::AudioFormat;
use crate::convert::Converter;
use crate::decoder::AudioPacket;
use crate::{NUM_CHANNELS, SAMPLE_RATE};

#[cfg(all(
    feature = "rodiojack-backend",
    not(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd"))
))]
compile_error!("Rodio JACK backend is currently only supported on linux.");

#[cfg(feature = "rodio-backend")]
pub fn mk_rodio(device: Option<String>, format: AudioFormat) -> Box<dyn Sink> {
    Box::new(open(cpal::default_host(), device, format))
}

#[cfg(feature = "rodiojack-backend")]
pub fn mk_rodiojack(device: Option<String>, format: AudioFormat) -> Box<dyn Sink> {
    Box::new(open(
        cpal::host_from_id(cpal::HostId::Jack).unwrap(),
        device,
        format,
    ))
}

#[derive(Debug, Error)]
pub enum RodioError {
    #[error("Rodio: no device available")]
    NoDeviceAvailable,
    #[error("Rodio: device \"{0}\" is not available")]
    DeviceNotAvailable(String),
    #[error("Rodio play error: {0}")]
    PlayError(#[from] rodio::PlayError),
    #[error("Rodio stream error: {0}")]
    StreamError(#[from] rodio::StreamError),
    #[error("Cannot get audio devices: {0}")]
    DevicesError(#[from] cpal::DevicesError),
}

pub struct RodioSink {
    rodio_sink: rodio::Sink,
    format: AudioFormat,
    _stream: rodio::OutputStream,
}

fn list_formats(device: &rodio::Device) {
    match device.default_output_config() {
        Ok(cfg) => {
            debug!("  Default config:");
            debug!("    {:?}", cfg);
        }
        Err(e) => {
            // Use loglevel debug, since even the output is only debug
            debug!("Error getting default rodio::Sink config: {}", e);
        }
    };

    match device.supported_output_configs() {
        Ok(mut cfgs) => {
            if let Some(first) = cfgs.next() {
                debug!("  Available configs:");
                debug!("    {:?}", first);
            } else {
                return;
            }

            for cfg in cfgs {
                debug!("    {:?}", cfg);
            }
        }
        Err(e) => {
            debug!("Error getting supported rodio::Sink configs: {}", e);
        }
    }
}

fn list_outputs(host: &cpal::Host) -> Result<(), cpal::DevicesError> {
    let mut default_device_name = None;

    if let Some(default_device) = host.default_output_device() {
        default_device_name = default_device.name().ok();
        println!(
            "Default Audio Device:\n  {}",
            default_device_name.as_deref().unwrap_or("[unknown name]")
        );

        list_formats(&default_device);

        println!("Other Available Audio Devices:");
    } else {
        warn!("No default device was found");
    }

    for device in host.output_devices()? {
        match device.name() {
            Ok(name) if Some(&name) == default_device_name.as_ref() => (),
            Ok(name) => {
                println!("  {}", name);
                list_formats(&device);
            }
            Err(e) => {
                warn!("Cannot get device name: {}", e);
                println!("   [unknown name]");
                list_formats(&device);
            }
        }
    }

    Ok(())
}

fn create_sink(
    host: &cpal::Host,
    device: Option<String>,
) -> Result<(rodio::Sink, rodio::OutputStream), RodioError> {
    let rodio_device = match device {
        Some(ask) if &ask == "?" => {
            let exit_code = match list_outputs(host) {
                Ok(()) => 0,
                Err(e) => {
                    error!("{}", e);
                    1
                }
            };
            exit(exit_code)
        }
        Some(device_name) => {
            host.output_devices()?
                .find(|d| d.name().ok().map_or(false, |name| name == device_name)) // Ignore devices for which getting name fails
                .ok_or(RodioError::DeviceNotAvailable(device_name))?
        }
        None => host
            .default_output_device()
            .ok_or(RodioError::NoDeviceAvailable)?,
    };

    let name = rodio_device.name().ok();
    info!(
        "Using audio device: {}",
        name.as_deref().unwrap_or("[unknown name]")
    );

    let (stream, handle) = rodio::OutputStream::try_from_device(&rodio_device)?;
    let sink = rodio::Sink::try_new(&handle)?;
    Ok((sink, stream))
}

pub fn open(host: cpal::Host, device: Option<String>, format: AudioFormat) -> RodioSink {
    info!(
        "Using Rodio sink with format {:?} and cpal host: {}",
        format,
        host.id().name()
    );

    if format != AudioFormat::S16 && format != AudioFormat::F32 {
        unimplemented!("Rodio currently only supports F32 and S16 formats");
    }

    let (sink, stream) = create_sink(&host, device).unwrap();

    debug!("Rodio sink was created");
    RodioSink {
        rodio_sink: sink,
        format,
        _stream: stream,
    }
}

impl Sink for RodioSink {
    fn write(&mut self, packet: &AudioPacket, converter: &mut Converter) -> SinkResult<()> {
        let samples = packet.samples();
        match self.format {
            AudioFormat::F32 => {
                let samples_f32: &[f32] = &converter.f64_to_f32(samples);
                let source = rodio::buffer::SamplesBuffer::new(
                    NUM_CHANNELS as u16,
                    SAMPLE_RATE,
                    samples_f32,
                );
                self.rodio_sink.append(source);
            }
            AudioFormat::S16 => {
                let samples_s16: &[i16] = &converter.f64_to_s16(samples);
                let source = rodio::buffer::SamplesBuffer::new(
                    NUM_CHANNELS as u16,
                    SAMPLE_RATE,
                    samples_s16,
                );
                self.rodio_sink.append(source);
            }
            _ => unreachable!(),
        };

        // Chunk sizes seem to be about 256 to 3000 ish items long.
        // Assuming they're on average 1628 then a half second buffer is:
        // 44100 elements --> about 27 chunks
        while self.rodio_sink.len() > 26 {
            // sleep and wait for rodio to drain a bit
            thread::sleep(Duration::from_millis(10));
        }
        Ok(())
    }
}

impl RodioSink {
    pub const NAME: &'static str = "rodio";
}
