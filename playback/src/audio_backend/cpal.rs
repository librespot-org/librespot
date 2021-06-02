use std::process::exit;
use std::{io, thread, time};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Sample, StreamConfig};
use rtrb::{Consumer, RingBuffer};
use thiserror::Error;

use super::Sink;
use crate::config::AudioFormat;
use crate::convert::Converter;
use crate::decoder::AudioPacket;
use crate::{NUM_CHANNELS, SAMPLE_RATE};

#[derive(Debug, Error)]
pub enum CpalError {
    #[error("CPAL: no device available")]
    NoDeviceAvailable,
    #[error("CPAL: device \"{0}\" is not available")]
    DeviceNotAvailable(String),
    #[error("Cannot get audio devices: {0}")]
    DevicesError(#[from] cpal::DevicesError),
}

pub struct CpalSink<S: Sample> {
    stream: cpal::Stream,
    format: AudioFormat,
    sample_tx: rtrb::Producer<S>,
}

fn list_formats(device: &cpal::Device) {
    match device.default_output_config() {
        Ok(cfg) => {
            debug!("  Default config:");
            debug!("    {:?}", cfg);
        }
        Err(e) => {
            // Use loglevel debug, since even the output is only debug
            debug!("Error getting default cpal::Sink output config: {}", e);
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
            debug!("Error getting supported cpal::Sink configs: {}", e);
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

fn get_device(host: &cpal::Host, device: Option<String>) -> Result<cpal::Device, CpalError> {
    let device = match device {
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
                .ok_or(CpalError::DeviceNotAvailable(device_name))?
        }
        None => host
            .default_output_device()
            .ok_or(CpalError::NoDeviceAvailable)?,
    };

    info!(
        "Using audio device: {}",
        device.name().as_deref().unwrap_or("[unknown name]")
    );

    Ok(device)
}

fn data_callback<T: Sample>(
    mut consumer: Consumer<T>,
) -> impl FnMut(&mut [T], &cpal::OutputCallbackInfo) {
    let silence = <T as cpal::Sample>::from(&0i16);

    move |buf: &mut [T], _| {
        let mut chunk = consumer.read_chunk(consumer.slots()).unwrap();

        if chunk.len() >= buf.len() {
            buf.iter_mut()
                .zip(&mut chunk)
                .for_each(|(to, from)| *to = *from);
            chunk.commit_iterated();
        } else {
            buf.fill(silence);
        }
    }
}

pub fn open(dev: Option<String>, format: AudioFormat) -> Box<dyn Sink> {
    fn open_with_format<T: Sample + Send + 'static>(
        device: cpal::Device,
        format: AudioFormat,
    ) -> CpalSink<T> {
        let (sample_tx, sample_rx) = RingBuffer::new(4 * 4096).split();

        let stream = device
            .build_output_stream::<T, _, _>(
                &StreamConfig {
                    buffer_size: cpal::BufferSize::Default,
                    channels: NUM_CHANNELS as u16,
                    sample_rate: cpal::SampleRate(SAMPLE_RATE),
                },
                data_callback(sample_rx),
                |e| error!("Sink error: {}", e),
            )
            .expect("Could not open output stream with that format");

        CpalSink {
            stream,
            format,
            sample_tx,
        }
    }

    let host = cpal::default_host();
    info!(
        "Using CPAL sink with format {:?} and host: {}",
        format,
        host.id().name()
    );

    let device = get_device(&host, dev).expect("Could not open device");

    // Try and see if the requested output format is actually available.
    // If not, try the default output format. This provides an out-of-the-box
    // experience, particularly on certain CoreAudio devices that only support
    // F32 output while librespot defaults to S16.
    let mut format = format;
    let mut format_found = false;
    match device.supported_output_configs() {
        Ok(cfgs) => {
            for cfg in cfgs {
                format_found = match cfg.sample_format() {
                    cpal::SampleFormat::F32 => format == AudioFormat::F32,
                    cpal::SampleFormat::I16 => format == AudioFormat::S16,
                    _ => false,
                };
                if format_found {
                    break;
                }
            }
        }
        Err(e) => {
            debug!("Error getting supported cpal::Sink configs: {}", e);
        }
    }
    if !format_found {
        warn!(
            "Requested audio format {:?} not supported by device; trying default format",
            format
        );
        let default_output_config = device
            .default_output_config()
            .expect("Could not get default output config");
        let default_sample_format = default_output_config.sample_format();
        format = match default_sample_format {
            cpal::SampleFormat::F32 => AudioFormat::F32,
            cpal::SampleFormat::I16 => AudioFormat::S16,
            _ => unimplemented!(
                "Device default sample format {:?} is not implemented",
                default_sample_format
            ),
        };
        warn!("Changing to default audio format {:?}", format);
    }

    match format {
        AudioFormat::F32 => Box::new(open_with_format::<f32>(device, format)),
        AudioFormat::S16 => Box::new(open_with_format::<i16>(device, format)),
        _ => unimplemented!("CPAL currently only supports F32 and S16 formats"),
    }
}

impl<S: Sample + Default> Sink for CpalSink<S> {
    fn start(&mut self) -> io::Result<()> {
        let result = self.stream.play();
        match result {
            Ok(()) => Ok(()),
            Err(e) => {
                error!("CPAL error stream play {}", e);
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    "CPAL error: stream play failed",
                ))
            }
        }
    }

    fn stop(&mut self) -> io::Result<()> {
        // This method may fail if the device does not support suspending
        // the stream at the hardware level. That's OK so we ignore it.
        let _ = self.stream.pause();
        Ok(())
    }

    fn write(&mut self, packet: &AudioPacket, converter: &mut Converter) -> io::Result<()> {
        macro_rules! write_sink {
            ($samples: expr) => {
                let mut write_to = loop {
                    match self.sample_tx.write_chunk($samples.len()) {
                        Ok(x) => break x,
                        Err(_) => thread::sleep(time::Duration::from_millis(10)),
                    }
                };
                (&mut write_to)
                    .zip($samples.iter())
                    .for_each(|(to, from)| *to = S::from(from));
                write_to.commit_iterated();
            };
        }

        let samples = packet.samples();
        match self.format {
            AudioFormat::F32 => {
                let samples_f32: &[f32] = &converter.f64_to_f32(samples);
                write_sink!(samples_f32);
            }
            AudioFormat::S16 => {
                let samples_s16: &[i16] = &converter.f64_to_s16(samples);
                write_sink!(samples_s16);
            }
            _ => unreachable!(),
        }

        Ok(())
    }
}
