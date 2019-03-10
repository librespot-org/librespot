use super::{Open, Sink};
use portaudio_rs;
use portaudio_rs::device::{get_default_output_index, DeviceIndex, DeviceInfo};
use portaudio_rs::stream::*;
use portaudio_rs::PaError;
use std::io;
use std::process::exit;
use std::time::Duration;

/// Helper function to convert a portaudio error into an io::Error.
fn into_io_error(e: PaError) -> io::Error {
    io::Error::new(io::ErrorKind::Other, format!("portaudio error: {}", e))
}

pub struct PortAudioSink<'a>(
    Option<portaudio_rs::stream::Stream<'a, i16, i16>>,
    StreamParameters<i16>,
);

fn output_devices() -> Box<Iterator<Item = (DeviceIndex, DeviceInfo)>> {
    let count = portaudio_rs::device::get_count().unwrap();
    let devices = (0..count)
        .filter_map(|idx| portaudio_rs::device::get_info(idx).map(|info| (idx, info)))
        .filter(|&(_, ref info)| info.max_output_channels > 0);

    Box::new(devices)
}

fn list_outputs() {
    let default = get_default_output_index();

    for (idx, info) in output_devices() {
        if Some(idx) == default {
            println!("- {} (default)", info.name);
        } else {
            println!("- {}", info.name)
        }
    }
}

fn find_output(device: &str) -> Option<DeviceIndex> {
    output_devices()
        .find(|&(_, ref info)| info.name == device)
        .map(|(idx, _)| idx)
}

impl<'a> Open for PortAudioSink<'a> {
    fn open(device: Option<String>) -> PortAudioSink<'a> {
        debug!("Using PortAudio sink");

        portaudio_rs::initialize().unwrap();

        let device_idx = match device.as_ref().map(AsRef::as_ref) {
            Some("?") => {
                list_outputs();
                exit(0)
            }
            Some(device) => find_output(device),
            None => get_default_output_index(),
        }
        .expect("Could not find device");

        let info = portaudio_rs::device::get_info(device_idx);
        let latency = match info {
            Some(info) => info.default_high_output_latency,
            None => Duration::new(0, 0),
        };

        let params = StreamParameters {
            device: device_idx,
            channel_count: 2,
            suggested_latency: latency,
            data: 0i16,
        };

        PortAudioSink(None, params)
    }
}

impl<'a> Sink for PortAudioSink<'a> {
    fn start(&mut self) -> io::Result<()> {
        let stream = match self.0.take() {
            Some(stream) => stream,
            None => {
                let stream = Stream::open(
                    None,
                    Some(self.1),
                    44100.0,
                    FRAMES_PER_BUFFER_UNSPECIFIED,
                    StreamFlags::empty(),
                    None,
                )
                .map_err(into_io_error)?;

                stream.start().map_err(into_io_error)?;
                stream
            }
        };

        self.0 = Some(stream);
        Ok(())
    }

    fn stop(&mut self) -> io::Result<()> {
        if let Some(stream) = self.0.take() {
            stream.stop().map_err(into_io_error)?;
        }

        Ok(())
    }

    fn write(&mut self, data: &[i16]) -> io::Result<()> {
        match self.0.as_mut() {
            Some(stream) => stream.write(data).map_err(into_io_error),
            None => Err(io::Error::new(io::ErrorKind::Other, "stream closed")),
        }
    }
}

impl<'a> Drop for PortAudioSink<'a> {
    fn drop(&mut self) {
        portaudio_rs::terminate().unwrap();
    }
}
