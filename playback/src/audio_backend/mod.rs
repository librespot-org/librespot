use std::process::exit;

use crate::config::AudioFormat;
use crate::convert::Converter;
use crate::decoder::AudioPacket;
use clap::ValueEnum;
use enum_assoc::Assoc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SinkError {
    #[error("Audio Sink Error Not Connected: {0}")]
    NotConnected(String),
    #[error("Audio Sink Error Connection Refused: {0}")]
    ConnectionRefused(String),
    #[error("Audio Sink Error On Write: {0}")]
    OnWrite(String),
    #[error("Audio Sink Error Invalid Parameters: {0}")]
    InvalidParams(String),
    #[error("Audio Sink Error Changing State: {0}")]
    StateChange(String),
}

pub type SinkResult<T> = Result<T, SinkError>;

pub trait Open {
    fn device_options() -> ! {
        println!("No device options available!");
        exit(0)
    }
    fn open(_: Option<String>, format: AudioFormat) -> Box<Self>;
}

pub trait Sink {
    fn start(&mut self) -> SinkResult<()> {
        Ok(())
    }
    fn stop(&mut self) -> SinkResult<()> {
        Ok(())
    }
    fn write(&mut self, packet: AudioPacket, converter: &mut Converter) -> SinkResult<()>;
}

pub trait SinkAsBytes {
    fn write_bytes(&mut self, data: &[u8]) -> SinkResult<()>;
}

// reuse code for various backends
macro_rules! sink_as_bytes {
    () => {
        #[inline]
        fn write(&mut self, packet: AudioPacket, converter: &mut Converter) -> SinkResult<()> {
            use crate::convert::i24;
            use zerocopy::IntoBytes;
            match packet {
                AudioPacket::Samples(samples) => match self.format {
                    AudioFormat::F64 => self.write_bytes(samples.as_bytes()),
                    AudioFormat::F32 => {
                        let samples_f32: &[f32] = &converter.f64_to_f32(&samples);
                        self.write_bytes(samples_f32.as_bytes())
                    }
                    AudioFormat::S32 => {
                        let samples_s32: &[i32] = &converter.f64_to_s32(&samples);
                        self.write_bytes(samples_s32.as_bytes())
                    }
                    AudioFormat::S24 => {
                        let samples_s24: &[i32] = &converter.f64_to_s24(&samples);
                        self.write_bytes(samples_s24.as_bytes())
                    }
                    AudioFormat::S24_3 => {
                        let samples_s24_3: &[i24] = &converter.f64_to_s24_3(&samples);
                        self.write_bytes(samples_s24_3.as_bytes())
                    }
                    AudioFormat::S16 => {
                        let samples_s16: &[i16] = &converter.f64_to_s16(&samples);
                        self.write_bytes(samples_s16.as_bytes())
                    }
                },
                AudioPacket::Raw(samples) => self.write_bytes(&samples),
            }
        }
    };
}

#[cfg(feature = "alsa-backend")]
mod alsa;

#[cfg(feature = "portaudio-backend")]
mod portaudio;

#[cfg(feature = "pulseaudio-backend")]
mod pulseaudio;

#[cfg(feature = "jackaudio-backend")]
mod jackaudio;

#[cfg(feature = "gstreamer-backend")]
mod gstreamer;

#[cfg(any(feature = "rodio-backend", feature = "rodiojack-backend"))]
mod rodio;

#[cfg(feature = "sdl-backend")]
mod sdl;

mod pipe;
use self::pipe::StdoutSink;

mod subprocess;
use self::subprocess::SubprocessSink;

#[derive(Default, Clone, Copy, Debug, ValueEnum, Assoc, Serialize, Deserialize)]
#[func(pub fn build(&self, device: Option<String>, format: AudioFormat) -> Box<dyn Sink>)]
#[func(pub fn device_options(&self) -> !)]
pub enum AudioBackendBuilder {
    #[cfg(feature = "rodio-backend")]
    #[default]
    #[assoc(build = rodio::open_rodio(device, format))]
    #[assoc(device_options = rodio::rodio_device_options())]
    Rodio,
    #[cfg(feature = "alsa-backend")]
    #[cfg_attr(not(feature = "rodio-backend"), default)]
    #[assoc(build = alsa::AlsaSink::open(device, format))]
    #[assoc(device_options = alsa::AlsaSink::device_options())]
    Alsa,
    #[cfg(feature = "portaudio-backend")]
    #[cfg_attr(not(any(feature = "rodio-backend", feature = "alsa-backend")), default)]
    #[assoc(build = portaudio::PortAudioSink::<'_>::open(device, format))]
    #[assoc(device_options = portaudio::PortAudioSink::<'_>::device_options())]
    Portaudio,
    #[cfg(feature = "pulseaudio-backend")]
    #[cfg_attr(
        not(any(
            feature = "rodio-backend",
            feature = "alsa-backend",
            feature = "portaudio-backend"
        )),
        default
    )]
    #[assoc(build = pulseaudio::PulseAudioSink::open(device, format))]
    #[assoc(device_options = pulseaudio::PulseAudioSink::device_options())]
    Pulseaudio,
    #[cfg(feature = "jackaudio-backend")]
    #[cfg_attr(
        not(any(
            feature = "rodio-backend",
            feature = "alsa-backend",
            feature = "portaudio-backend",
            feature = "pulseaudio-backend"
        )),
        default
    )]
    #[assoc(build = jackaudio::JackSink::open(device, format))]
    #[assoc(device_options = jackaudio::JackSink::device_options())]
    Jackaudio,
    #[cfg(feature = "gstreamer-backend")]
    #[cfg_attr(
        not(any(
            feature = "rodio-backend",
            feature = "alsa-backend",
            feature = "portaudio-backend",
            feature = "pulseaudio-backend",
            feature = "jackaudio-backend"
        )),
        default
    )]
    #[assoc(build = gstreamer::GstreamerSink::open(device, format))]
    #[assoc(device_options = gstreamer::GstreamerSink::device_options())]
    Gstreamer,
    #[cfg(feature = "rodiojack-backend")]
    #[cfg_attr(
        not(any(
            feature = "rodio-backend",
            feature = "alsa-backend",
            feature = "portaudio-backend",
            feature = "pulseaudio-backend",
            feature = "jackaudio-backend",
            feature = "gstreamer-backend"
        )),
        default
    )]
    #[assoc(build = rodio::open_rodiojack(device, format))]
    #[assoc(device_options = rodio::rodiojack_device_options())]
    Rodiojack,
    #[cfg(feature = "sdl-backend")]
    #[cfg_attr(
        not(any(
            feature = "rodio-backend",
            feature = "alsa-backend",
            feature = "portaudio-backend",
            feature = "pulseaudio-backend",
            feature = "jackaudio-backend",
            feature = "gstreamer-backend",
            feature = "rodiojack-backend"
        )),
        default
    )]
    #[assoc(build = sdl::SdlSink::open(device, format))]
    #[assoc(device_options = sdl::SdlSink::device_options())]
    Sdl,
    #[cfg_attr(
        not(any(
            feature = "rodio-backend",
            feature = "alsa-backend",
            feature = "portaudio-backend",
            feature = "pulseaudio-backend",
            feature = "jackaudio-backend",
            feature = "gstreamer-backend",
            feature = "rodiojack-backend",
            feature = "sdl-backend"
        )),
        default
    )]
    #[assoc(build = StdoutSink::open(device, format))]
    #[assoc(device_options = StdoutSink::device_options())]
    Pipe,
    #[assoc(build = SubprocessSink::open(device, format))]
    #[assoc(device_options = SubprocessSink::device_options())]
    Subprocess,
}
