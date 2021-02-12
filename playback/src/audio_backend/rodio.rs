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
    let default_fmt = match device.default_output_config() {
        Ok(fmt) => fmt,
        Err(e) => {
            warn!("Error getting default rodio::Sink config: {}", e);
            return;
        }
    };
    debug!("  Default config:");
    debug!("    {:?}", default_fmt);

    let mut output_configs = match device.supported_output_configs() {
        Ok(f) => f.peekable(),
        Err(e) => {
            warn!("Error getting supported rodio::Sink configs: {}", e);
            return;
        }
    };

    if output_configs.peek().is_some() {
        debug!("  Available configs:");
        for format in output_configs {
            debug!("    {:?}", format);
        }
    }
}

fn list_outputs_and_exit() -> ! {
    let default_device = get_default_device().unwrap();
    let default_device_name = default_device.name().expect("cannot get output name");
    println!("Default Audio Device:\n  {}", default_device_name);
    list_formats(&default_device);

    println!("Other Available Audio Devices:");
    for device in cpal::default_host()
        .output_devices()
        .expect("cannot get list of output devices")
    {
        let device_name = device.name().expect("cannot get output name");
        if device_name != default_device_name {
            println!("  {}", device_name);
            list_formats(&device);
        }
    }

    exit(0)
}

fn get_default_device() -> Result<rodio::Device, RodioError> {
    cpal::default_host()
        .default_output_device()
        .ok_or(RodioError::NoDeviceAvailable)
}

fn create_sink(device: Option<String>) -> Result<(rodio::Sink, rodio::OutputStream), RodioError> {
    let rodio_device = match device {
        Some(ask) if &ask == "?" => list_outputs_and_exit(),
        Some(device_name) => {
            cpal::default_host()
                .output_devices()?
                .find(|d| d.name().ok().map_or(false, |name| name == device_name)) // Ignore devices for which getting name fails
                .ok_or(RodioError::DeviceNotAvailable(device_name))?
        }
        None => get_default_device()?,
    };

    let name = rodio_device.name().ok();
    info!(
        "Using audio device: {}",
        name.as_deref().unwrap_or("(unknown name)")
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
