use super::{Open, Sink, SinkAsBytes, SinkError};
use crate::config::AudioFormat;
use crate::convert::Converter;
use crate::decoder::AudioPacket;
use crate::{NUM_CHANNELS, SAMPLE_RATE};
use libpulse_binding::{self as pulse, error::PAErr, stream::Direction};
use libpulse_simple_binding::Simple;
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
    NotConnected,
    #[error("Error writing from PulseAudioSink to PulseAudio server, {0}")]
    OnWrite(PAErr),
}

impl From<PulseError> for SinkError {
    fn from(e: PulseError) -> SinkError {
        use PulseError::*;
        let es = e.to_string();
        match e {
            DrainFailure(_) | OnWrite(_) => SinkError::OnWrite(es),
            ConnectionRefused(_) => SinkError::ConnectionRefused(es),
            NotConnected => SinkError::NotConnected(es),
            InvalidSampleSpec => SinkError::InvalidParams(es),
        }
    }
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
    fn start(&mut self) -> Result<(), SinkError> {
        if self.s.is_none() {
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
                return Err(SinkError::from(PulseError::InvalidSampleSpec));
            }

            let s = Simple::new(
                None,                   // Use the default server.
                APP_NAME,               // Our application's name.
                Direction::Playback,    // Direction.
                self.device.as_deref(), // Our device (sink) name.
                STREAM_NAME,            // Description of our stream.
                &ss,                    // Our sample format.
                None,                   // Use default channel map.
                None,                   // Use default buffering attributes.
            )
            .map_err(PulseError::ConnectionRefused)?;

            self.s = Some(s);
        }

        Ok(())
    }

    fn stop(&mut self) -> Result<(), SinkError> {
        let s = self.s.as_mut().ok_or(PulseError::NotConnected)?;

        s.drain().map_err(PulseError::DrainFailure)?;

        self.s = None;
        Ok(())
    }

    sink_as_bytes!();
}

impl SinkAsBytes for PulseAudioSink {
    fn write_bytes(&mut self, data: &[u8]) -> Result<(), SinkError> {
        let s = self.s.as_mut().ok_or(PulseError::NotConnected)?;

        s.write(data).map_err(PulseError::OnWrite)?;

        Ok(())
    }
}

impl PulseAudioSink {
    pub const NAME: &'static str = "pulseaudio";
}
