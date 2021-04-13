use super::{Open, Sink};
use crate::audio::{convert, AudioPacket, Requantizer};
use crate::config::AudioFormat;
use crate::player::{NUM_CHANNELS, SAMPLE_RATE};
use portaudio_rs::device::{get_default_output_index, DeviceIndex, DeviceInfo};
use portaudio_rs::stream::*;
use std::io;
use std::process::exit;
use std::time::Duration;

pub enum PortAudioSink<'a> {
    F32(
        Option<portaudio_rs::stream::Stream<'a, f32, f32>>,
        StreamParameters<f32>,
        Requantizer,
    ),
    S32(
        Option<portaudio_rs::stream::Stream<'a, i32, i32>>,
        StreamParameters<i32>,
        Requantizer,
    ),
    S16(
        Option<portaudio_rs::stream::Stream<'a, i16, i16>>,
        StreamParameters<i16>,
        Requantizer,
    ),
}

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
    fn open(
        device: Option<String>,
        format: AudioFormat,
        requantizer: Requantizer,
    ) -> PortAudioSink<'a> {
        info!("Using PortAudio sink with format: {:?}", format);

        warn!("This backend is known to panic on several platforms.");
        warn!("Consider using some other backend, or better yet, contributing a fix.");

        portaudio_rs::initialize().unwrap();

        let device_idx = match device.as_ref().map(AsRef::as_ref) {
            Some("?") => {
                list_outputs();
                exit(0)
            }
            Some(device) => find_output(device),
            None => get_default_output_index(),
        }
        .expect("could not find device");

        let info = portaudio_rs::device::get_info(device_idx);
        let latency = match info {
            Some(info) => info.default_high_output_latency,
            None => Duration::new(0, 0),
        };

        macro_rules! open_sink {
            ($sink: expr, $type: ty) => {{
                let params = StreamParameters {
                    device: device_idx,
                    channel_count: NUM_CHANNELS as u32,
                    suggested_latency: latency,
                    data: 0.0 as $type,
                };
                $sink(None, params, requantizer)
            }};
        }
        match format {
            AudioFormat::F32 => open_sink!(Self::F32, f32),
            AudioFormat::S32 => open_sink!(Self::S32, i32),
            AudioFormat::S16 => open_sink!(Self::S16, i16),
            _ => {
                unimplemented!("PortAudio currently does not support {:?} output", format)
            }
        }
    }
}

impl<'a> Sink for PortAudioSink<'a> {
    fn start(&mut self) -> io::Result<()> {
        macro_rules! start_sink {
            (ref mut $stream: ident, ref $parameters: ident) => {{
                if $stream.is_none() {
                    *$stream = Some(
                        Stream::open(
                            None,
                            Some(*$parameters),
                            SAMPLE_RATE as f64,
                            FRAMES_PER_BUFFER_UNSPECIFIED,
                            StreamFlags::empty(),
                            None,
                        )
                        .unwrap(),
                    );
                }
                $stream.as_mut().unwrap().start().unwrap()
            }};
        }

        match self {
            Self::F32(stream, parameters, _) => start_sink!(ref mut stream, ref parameters),
            Self::S32(stream, parameters, _) => start_sink!(ref mut stream, ref parameters),
            Self::S16(stream, parameters, _) => start_sink!(ref mut stream, ref parameters),
        };

        Ok(())
    }

    fn stop(&mut self) -> io::Result<()> {
        macro_rules! stop_sink {
            (ref mut $stream: ident) => {{
                $stream.as_mut().unwrap().stop().unwrap();
                *$stream = None;
            }};
        }
        match self {
            Self::F32(stream, _, _) => stop_sink!(ref mut stream),
            Self::S32(stream, _, _) => stop_sink!(ref mut stream),
            Self::S16(stream, _, _) => stop_sink!(ref mut stream),
        };

        Ok(())
    }

    fn write(&mut self, packet: &AudioPacket) -> io::Result<()> {
        macro_rules! write_sink {
            (ref mut $stream: expr, $samples: expr) => {
                $stream.as_mut().unwrap().write($samples)
            };
        }

        let samples = packet.samples();
        let result = match self {
            Self::F32(stream, _parameters, _) => {
                write_sink!(ref mut stream, samples)
            }
            Self::S32(stream, _parameters, ref mut requantizer) => {
                let samples_s32: &[i32] = &convert::to_s32(samples, requantizer);
                write_sink!(ref mut stream, samples_s32)
            }
            Self::S16(stream, _parameters, ref mut requantizer) => {
                let samples_s16: &[i16] = &convert::to_s16(samples, requantizer);
                write_sink!(ref mut stream, samples_s16)
            }
        };
        match result {
            Ok(_) => (),
            Err(portaudio_rs::PaError::OutputUnderflowed) => error!("PortAudio write underflow"),
            Err(e) => panic!("PortAudio error {}", e),
        };

        Ok(())
    }
}

impl<'a> Drop for PortAudioSink<'a> {
    fn drop(&mut self) {
        portaudio_rs::terminate().unwrap();
    }
}
