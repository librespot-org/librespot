use super::{Open, Sink};
use crate::audio::AudioPacket;
use portaudio_rs;
use portaudio_rs::device::{get_default_output_index, DeviceIndex, DeviceInfo};
use portaudio_rs::stream::*;
use std::io;
use std::process::exit;
use std::time::Duration;

pub struct PortAudioSink<'a>(
    Option<portaudio_rs::stream::Stream<'a, i16, i16>>,
    StreamParameters<i16>,
);

fn output_devices() -> Box<dyn Iterator<Item = (DeviceIndex, DeviceInfo)>> {
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
        if self.0.is_none() {
            self.0 = Some(
                Stream::open(
                    None,
                    Some(self.1),
                    44100.0,
                    FRAMES_PER_BUFFER_UNSPECIFIED,
                    StreamFlags::empty(),
                    None,
                )
                .unwrap(),
            );
        }

        self.0.as_mut().unwrap().start().unwrap();
        Ok(())
    }
    fn stop(&mut self) -> io::Result<()> {
        self.0.as_mut().unwrap().stop().unwrap();
        self.0 = None;
        Ok(())
    }
    fn write(&mut self, packet: &AudioPacket) -> io::Result<()> {
        match self.0.as_mut().unwrap().write(packet.samples()) {
            Ok(_) => (),
            Err(portaudio_rs::PaError::OutputUnderflowed) => error!("PortAudio write underflow"),
            Err(e) => panic!("PA Error {}", e),
        };

        Ok(())
    }
}

impl<'a> Drop for PortAudioSink<'a> {
    fn drop(&mut self) {
        portaudio_rs::terminate().unwrap();
    }
}
