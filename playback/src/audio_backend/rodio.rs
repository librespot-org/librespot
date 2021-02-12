use std::process::exit;
use std::{convert::Infallible, sync::mpsc};
use std::{io, thread, time};

use cpal::traits::{DeviceTrait, HostTrait};
use thiserror::Error;

use super::{Open, Sink};

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

    // will produce a TryRecvError on the receiver side when it is dropped.
    _close_tx: mpsc::SyncSender<Infallible>,
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

fn list_outputs() -> Result<(), cpal::DevicesError> {
    let mut default_device_name = None;

    if let Some(default_device) = get_default_device() {
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

    for device in cpal::default_host().output_devices()? {
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

fn get_default_device() -> Option<rodio::Device> {
    cpal::default_host().default_output_device()
}

fn create_sink(device: Option<String>) -> Result<(rodio::Sink, rodio::OutputStream), RodioError> {
    let rodio_device = match device {
        Some(ask) if &ask == "?" => {
            let exit_code = match list_outputs() {
                Ok(()) => 0,
                Err(e) => {
                    error!("{}", e);
                    1
                }
            };
            exit(exit_code)
        }
        Some(device_name) => {
            cpal::default_host()
                .output_devices()?
                .find(|d| d.name().ok().map_or(false, |name| name == device_name)) // Ignore devices for which getting name fails
                .ok_or(RodioError::DeviceNotAvailable(device_name))?
        }
        None => get_default_device().ok_or(RodioError::NoDeviceAvailable)?,
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

impl Open for RodioSink {
    fn open(device: Option<String>) -> RodioSink {
        debug!(
            "Using rodio sink with cpal host: {:?}",
            cpal::default_host().id().name()
        );

        let (sink_tx, sink_rx) = mpsc::sync_channel(1);
        let (close_tx, close_rx) = mpsc::sync_channel(1);

        std::thread::spawn(move || match create_sink(device) {
            Ok((sink, stream)) => {
                sink_tx.send(Ok(sink)).unwrap();

                close_rx.recv().unwrap_err(); // This will fail as soon as the sender is dropped
                debug!("drop rodio::OutputStream");
                drop(stream);
            }
            Err(e) => {
                sink_tx.send(Err(e)).unwrap();
            }
        });

        // Instead of the second `unwrap`, better error handling could be introduced
        let sink = sink_rx.recv().unwrap().unwrap();

        debug!("Rodio sink was created");
        RodioSink {
            rodio_sink: sink,
            _close_tx: close_tx,
        }
    }
}

impl Sink for RodioSink {
    fn start(&mut self) -> io::Result<()> {
        // More similar to an "unpause" than "play". Doesn't undo "stop".
        // self.rodio_sink.play();
        Ok(())
    }

    fn stop(&mut self) -> io::Result<()> {
        // This will immediately stop playback, but the sink is then unusable.
        // We just have to let the current buffer play till the end.
        // self.rodio_sink.stop();
        Ok(())
    }

    fn write(&mut self, data: &[i16]) -> io::Result<()> {
        let source = rodio::buffer::SamplesBuffer::new(2, 44100, data);
        self.rodio_sink.append(source);

        // Chunk sizes seem to be about 256 to 3000 ish items long.
        // Assuming they're on average 1628 then a half second buffer is:
        // 44100 elements --> about 27 chunks
        while self.rodio_sink.len() > 26 {
            // sleep and wait for rodio to drain a bit
            thread::sleep(time::Duration::from_millis(10));
        }
        Ok(())
    }
}
