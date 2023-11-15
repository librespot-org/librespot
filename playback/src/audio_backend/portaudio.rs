use super::{Open, Sink, SinkError, SinkResult};

use crate::{
    config::AudioFormat, convert::Converter, decoder::AudioPacket, CommonSampleRates, NUM_CHANNELS,
};

use portaudio_rs::{
    device::{get_default_output_index, DeviceIndex, DeviceInfo},
    stream::*,
};

use std::{process::exit, time::Duration};

pub enum PortAudioSink<'a> {
    F32(
        Option<portaudio_rs::stream::Stream<'a, f32, f32>>,
        StreamParameters<f32>,
        f64,
    ),
    S32(
        Option<portaudio_rs::stream::Stream<'a, i32, i32>>,
        StreamParameters<i32>,
        f64,
    ),
    S16(
        Option<portaudio_rs::stream::Stream<'a, i16, i16>>,
        StreamParameters<i16>,
        f64,
    ),
}

fn output_devices() -> Box<dyn Iterator<Item = (DeviceIndex, DeviceInfo)>> {
    let count = portaudio_rs::device::get_count().unwrap();
    let devices = (0..count)
        .filter_map(|idx| portaudio_rs::device::get_info(idx).map(|info| (idx, info)))
        .filter(|(_, info)| info.max_output_channels > 0);

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
        .find(|(_, info)| info.name == device)
        .map(|(idx, _)| idx)
}

impl<'a> Open for PortAudioSink<'a> {
    fn open(device: Option<String>, format: AudioFormat, sample_rate: u32) -> PortAudioSink<'a> {
        info!(
            "Using PortAudioSink with format: {format:?}, sample rate: {}",
            CommonSampleRates::try_from(sample_rate)
                .unwrap_or_default()
                .to_string()
        );

        portaudio_rs::initialize().unwrap();

        let device_idx = match device.as_deref() {
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
            ($sink: expr, $type: ty, $sample_rate: ident) => {{
                let params = StreamParameters {
                    device: device_idx,
                    channel_count: NUM_CHANNELS as u32,
                    suggested_latency: latency,
                    data: 0.0 as $type,
                };
                $sink(None, params, $sample_rate)
            }};
        }

        let sample_rate = sample_rate as f64;

        match format {
            AudioFormat::F32 => open_sink!(Self::F32, f32, sample_rate),
            AudioFormat::S32 => open_sink!(Self::S32, i32, sample_rate),
            AudioFormat::S16 => open_sink!(Self::S16, i16, sample_rate),
            _ => {
                unimplemented!("PortAudio currently does not support {format:?} output")
            }
        }
    }
}

impl<'a> Sink for PortAudioSink<'a> {
    fn start(&mut self) -> SinkResult<()> {
        macro_rules! start_sink {
            (ref mut $stream: ident, ref $parameters: ident, ref $sample_rate: ident ) => {{
                if $stream.is_none() {
                    *$stream = Some(
                        Stream::open(
                            None,
                            Some(*$parameters),
                            *$sample_rate,
                            FRAMES_PER_BUFFER_UNSPECIFIED,
                            StreamFlags::DITHER_OFF, // no need to dither twice; use librespot dithering instead
                            None,
                        )
                        .unwrap(),
                    );
                }
                $stream.as_mut().unwrap().start().unwrap()
            }};
        }

        match self {
            Self::F32(stream, parameters, sample_rate) => {
                start_sink!(ref mut stream, ref parameters, ref sample_rate)
            }
            Self::S32(stream, parameters, sample_rate) => {
                start_sink!(ref mut stream, ref parameters, ref sample_rate)
            }
            Self::S16(stream, parameters, sample_rate) => {
                start_sink!(ref mut stream, ref parameters, ref sample_rate)
            }
        };

        Ok(())
    }

    fn stop(&mut self) -> SinkResult<()> {
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

    fn write(&mut self, packet: AudioPacket, converter: &mut Converter) -> SinkResult<()> {
        macro_rules! write_sink {
            (ref mut $stream: expr, $samples: expr) => {
                $stream.as_mut().unwrap().write($samples)
            };
        }

        let samples = packet
            .samples()
            .map_err(|e| SinkError::OnWrite(e.to_string()))?;

        let result = match self {
            Self::F32(stream, _parameters, _sample_rate) => {
                let samples_f32: &[f32] = &converter.f64_to_f32(samples);
                write_sink!(ref mut stream, samples_f32)
            }
            Self::S32(stream, _parameters, _sample_rate) => {
                let samples_s32: &[i32] = &converter.f64_to_s32(samples);
                write_sink!(ref mut stream, samples_s32)
            }
            Self::S16(stream, _parameters, _sample_rate) => {
                let samples_s16: &[i16] = &converter.f64_to_s16(samples);
                write_sink!(ref mut stream, samples_s16)
            }
        };
        match result {
            Ok(_) => (),
            Err(portaudio_rs::PaError::OutputUnderflowed) => error!("PortAudio write underflow"),
            Err(e) => panic!("PortAudio error {e}"),
        };

        Ok(())
    }
}

impl<'a> Drop for PortAudioSink<'a> {
    fn drop(&mut self) {
        portaudio_rs::terminate().unwrap();
    }
}

impl<'a> PortAudioSink<'a> {
    pub const NAME: &'static str = "portaudio";
}
