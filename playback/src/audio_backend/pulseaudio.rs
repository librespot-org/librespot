use super::{Open, Sink, SinkAsBytes};
use crate::config::AudioFormat;
use crate::convert::Converter;
use crate::decoder::AudioPacket;
use crate::{NUM_CHANNELS, SAMPLE_RATE};
use libpulse_binding::{self as pulse, error::PAErr, stream::Direction};
use libpulse_simple_binding::Simple;
use std::io;
use thiserror::Error;

const APP_NAME: &str = "librespot";
const STREAM_NAME: &str = "Spotify endpoint";

#[derive(Debug, Error)]
enum PulseError {
    #[error("Error starting PulseAudioSink, invalid PulseAudio sample spec")]
    InvalidSampleSpec,
    #[error("Error starting PulseAudioSink, could not connect to PulseAudio server, {0}")]
    ConnectionRefused(PAErr),
    #[error("Error stopping PulseAudioSink, failed to drain PulseAudio server buffer, {0}")]
    DrainFailure(PAErr),
    #[error("Error in PulseAudioSink, Not connected to PulseAudio server")]
    ServerNone,
    #[error("Error writing from PulseAudioSink to PulseAudio server, {0}")]
    OnWrite(PAErr),
}

pub struct PulseAudioSink {
    s: Option<Simple>,
    device: Option<String>,
    format: AudioFormat,
}

impl Open for PulseAudioSink {
    fn open(device: Option<String>, format: AudioFormat) -> Self {
        let mut actual_format = format;

        if actual_format == AudioFormat::F64 {
            warn!("PulseAudio currently does not support F64 output");
            actual_format = AudioFormat::F32;
        }

        info!("Using PulseAudioSink with format: {:?}", actual_format);

        Self {
            s: None,
            device,
            format: actual_format,
        }
    }
}

impl Sink for PulseAudioSink {
    fn start(&mut self) -> io::Result<()> {
        if self.s.is_some() {
            return Ok(());
        }

        // PulseAudio calls S24 and S24_3 different from the rest of the world
        let pulse_format = match self.format {
            AudioFormat::F32 => pulse::sample::Format::FLOAT32NE,
            AudioFormat::S32 => pulse::sample::Format::S32NE,
            AudioFormat::S24 => pulse::sample::Format::S24_32NE,
            AudioFormat::S24_3 => pulse::sample::Format::S24NE,
            AudioFormat::S16 => pulse::sample::Format::S16NE,
            _ => unreachable!(),
        };

        let ss = pulse::sample::Spec {
            format: pulse_format,
            channels: NUM_CHANNELS,
            rate: SAMPLE_RATE,
        };

        if !ss.is_valid() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                PulseError::InvalidSampleSpec,
            ));
        }

        let result = Simple::new(
            None,                   // Use the default server.
            APP_NAME,               // Our application's name.
            Direction::Playback,    // Direction.
            self.device.as_deref(), // Our device (sink) name.
            STREAM_NAME,            // Description of our stream.
            &ss,                    // Our sample format.
            None,                   // Use default channel map.
            None,                   // Use default buffering attributes.
        );

        match result {
            Ok(s) => {
                self.s = Some(s);
            }
            Err(e) => {
                return Err(io::Error::new(
                    io::ErrorKind::ConnectionRefused,
                    PulseError::ConnectionRefused(e),
                ));
            }
        }

        Ok(())
    }

    fn stop(&mut self) -> io::Result<()> {
        match &self.s {
            Some(s) => {
                if let Err(e) = s.drain() {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        PulseError::DrainFailure(e),
                    ));
                }
            }
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::NotConnected,
                    PulseError::ServerNone,
                ));
            }
        }

        self.s = None;
        Ok(())
    }

    sink_as_bytes!();
}

impl SinkAsBytes for PulseAudioSink {
    fn write_bytes(&mut self, data: &[u8]) -> io::Result<()> {
        match &self.s {
            Some(s) => {
                if let Err(e) = s.write(data) {
                    return Err(io::Error::new(
                        io::ErrorKind::BrokenPipe,
                        PulseError::OnWrite(e),
                    ));
                }
            }
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::NotConnected,
                    PulseError::ServerNone,
                ));
            }
        }

        Ok(())
    }
}

impl PulseAudioSink {
    pub const NAME: &'static str = "pulseaudio";
}
