use super::{Open, Sink};
use std::io;
use std::process::exit;
use std::time::Duration;
use portaudio;
use portaudio::device::{DeviceIndex, DeviceInfo, get_default_output_index};

pub struct PortAudioSink<'a>(portaudio::stream::Stream<'a, i16, i16>);

fn output_devices() -> Box<Iterator<Item=(DeviceIndex, DeviceInfo)>> {
    let count = portaudio::device::get_count().unwrap();
    let devices = (0..count)
        .filter_map(|idx| {
            portaudio::device::get_info(idx).map(|info| (idx, info))
        }).filter(|&(_, ref info)| {
            info.max_output_channels > 0
        });

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

impl <'a> Open for PortAudioSink<'a> {
    fn open(device: Option<&str>) -> PortAudioSink<'a> {
        use portaudio::stream::*;

        debug!("Using PortAudio sink");

        portaudio::initialize().unwrap();

        let device_idx = match device {
            Some("?") => {
                list_outputs();
                exit(0)
            }
            Some(device) => find_output(device),
            None => get_default_output_index(),
        }.expect("Could not find device");

        let info = portaudio::device::get_info(device_idx);
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

        let stream = Stream::open(
            None, Some(params),
            44100.0,
            FRAMES_PER_BUFFER_UNSPECIFIED,
            StreamFlags::empty(),
            None
        ).unwrap();

        PortAudioSink(stream)
    }
}

impl <'a> Sink for PortAudioSink<'a> {
    fn start(&self) -> io::Result<()> {
        self.0.start().unwrap();
        Ok(())
    }
    fn stop(&self) -> io::Result<()> {
        self.0.stop().unwrap();
        Ok(())
    }
    fn write(&self, data: &[i16]) -> io::Result<()> {
        match self.0.write(&data) {
            Ok(_) => (),
            Err(portaudio::PaError::OutputUnderflowed) =>
                error!("PortAudio write underflow"),
            Err(e) => panic!("PA Error {}", e),
        };

        Ok(())
    }
}

impl <'a> Drop for PortAudioSink<'a> {
    fn drop(&mut self) {
        portaudio::terminate().unwrap();
    }
}
