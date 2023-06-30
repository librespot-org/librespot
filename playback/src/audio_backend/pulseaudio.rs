use super::{Open, Sink, SinkAsBytes, SinkError, SinkResult};

use crate::{
    config::AudioFormat, convert::Converter, decoder::AudioPacket, CommonSampleRates, NUM_CHANNELS,
    SAMPLE_RATE as DECODER_SAMPLE_RATE,
};

use libpulse_binding::{self as pulse, error::PAErr, stream::Direction};
use libpulse_simple_binding::Simple;
use std::env;
use thiserror::Error;

#[derive(Debug, Error)]
enum PulseError {
    #[error("<PulseAudioSink> Unsupported Pulseaudio Sample Spec, Format {pulse_format:?} ({format:?}), Channels {channels}, Rate {rate}")]
    InvalidSampleSpec {
        pulse_format: pulse::sample::Format,
        format: AudioFormat,
        channels: u8,
        rate: u32,
    },

    #[error("<PulseAudioSink> {0}")]
    ConnectionRefused(PAErr),

    #[error("<PulseAudioSink> Failed to Drain Pulseaudio Buffer, {0}")]
    DrainFailure(PAErr),

    #[error("<PulseAudioSink> {0}")]
    OnWrite(PAErr),
}

impl From<PulseError> for SinkError {
    fn from(e: PulseError) -> SinkError {
        use PulseError::*;
        let es = e.to_string();
        match e {
            DrainFailure(_) | OnWrite(_) => SinkError::OnWrite(es),
            ConnectionRefused(_) => SinkError::ConnectionRefused(es),
            InvalidSampleSpec { .. } => SinkError::InvalidParams(es),
        }
    }
}

impl From<AudioFormat> for pulse::sample::Format {
    fn from(f: AudioFormat) -> pulse::sample::Format {
        use AudioFormat::*;
        match f {
            F64 | F32 => pulse::sample::Format::FLOAT32NE,
            S32 => pulse::sample::Format::S32NE,
            S24 => pulse::sample::Format::S24_32NE,
            S24_3 => pulse::sample::Format::S24NE,
            S16 => pulse::sample::Format::S16NE,
        }
    }
}

pub struct PulseAudioSink {
    sink: Option<Simple>,
    device: Option<String>,
    app_name: String,
    stream_desc: String,
    format: AudioFormat,
    sample_rate: u32,

    sample_spec: pulse::sample::Spec,
}

impl Open for PulseAudioSink {
    fn open(device: Option<String>, format: AudioFormat, sample_rate: u32) -> Self {
        let app_name = env::var("PULSE_PROP_application.name").unwrap_or_default();
        let stream_desc = env::var("PULSE_PROP_stream.description").unwrap_or_default();

        let format = if format == AudioFormat::F64 {
            warn!("PulseAudio currently does not support F64 output");
            AudioFormat::F32
        } else {
            format
        };

        info!(
            "Using PulseAudioSink with format: {format:?}, sample rate: {}",
            CommonSampleRates::try_from(sample_rate)
                .unwrap_or_default()
                .to_string()
        );

        let sample_spec = pulse::sample::Spec {
            format: format.into(),
            channels: NUM_CHANNELS,
            rate: sample_rate,
        };

        Self {
            sink: None,
            device,
            app_name,
            stream_desc,
            format,
            sample_rate,
            sample_spec,
        }
    }
}

impl Sink for PulseAudioSink {
    fn start(&mut self) -> SinkResult<()> {
        if self.sink.is_none() {
            if !self.sample_spec.is_valid() {
                let pulse_error = PulseError::InvalidSampleSpec {
                    pulse_format: self.sample_spec.format,
                    format: self.format,
                    channels: NUM_CHANNELS,
                    rate: self.sample_rate,
                };

                return Err(pulse_error.into());
            }

            let sink = Simple::new(
                None,                   // Use the default server.
                &self.app_name,         // Our application's name.
                Direction::Playback,    // Direction.
                self.device.as_deref(), // Our device (sink) name.
                &self.stream_desc,      // Description of our stream.
                &self.sample_spec,      // Our sample format.
                None,                   // Use default channel map.
                None,                   // Use default buffering attributes.
            )
            .map_err(PulseError::ConnectionRefused)?;

            self.sink = Some(sink);
        }

        Ok(())
    }

    fn stop(&mut self) -> SinkResult<()> {
        if let Some(sink) = self.sink.take() {
            sink.drain().map_err(PulseError::DrainFailure)?;
        }

        Ok(())
    }

    fn get_latency_pcm(&mut self) -> u64 {
        self.sink
            .as_mut()
            .and_then(|sink| {
                sink.get_latency()
                    .ok()
                    .map(|micro_sec| (micro_sec.as_secs_f64() * DECODER_SAMPLE_RATE as f64) as u64)
            })
            .unwrap_or(0)
    }

    sink_as_bytes!();
}

impl SinkAsBytes for PulseAudioSink {
    fn write_bytes(&mut self, data: &[u8]) -> SinkResult<()> {
        if let Some(sink) = self.sink.as_mut() {
            sink.write(data).map_err(PulseError::OnWrite)?;
        }

        Ok(())
    }
}

impl PulseAudioSink {
    pub const NAME: &'static str = "pulseaudio";
}
