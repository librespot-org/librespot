use super::{Open, Sink, SinkAsBytes};
use crate::config::AudioFormat;
use crate::convert::Converter;
use crate::decoder::AudioPacket;
use crate::{NUM_CHANNELS, SAMPLE_RATE};
use libpulse_binding::{self as pulse, stream::Direction};
use libpulse_simple_binding::Simple;
use std::io;

const APP_NAME: &str = "librespot";
const STREAM_NAME: &str = "Spotify endpoint";

pub struct PulseAudioSink {
    s: Option<Simple>,
    device: Option<String>,
    format: AudioFormat,
    pulse_format: pulse::sample::Format,
}

impl Open for PulseAudioSink {
    fn open(device: Option<String>, format: AudioFormat) -> Self {
        let mut actual_format = format;
        if actual_format == AudioFormat::F64 {
            warn!("PulseAudio currently does not support F64 output, falling back to F32");
            actual_format = AudioFormat::F32;
        }

        // PulseAudio calls S24 and S24_3 different from the rest of the world
        let pulse_format = match actual_format {
            AudioFormat::F32 => pulse::sample::Format::FLOAT32NE,
            AudioFormat::S32 => pulse::sample::Format::S32NE,
            AudioFormat::S24 => pulse::sample::Format::S24_32NE,
            AudioFormat::S24_3 => pulse::sample::Format::S24NE,
            AudioFormat::S16 => pulse::sample::Format::S16NE,
            _ => unreachable!(),
        };

        info!("Using PulseAudio sink with format: {:?}", actual_format);

        Self {
            s: None,
            device,
            format: actual_format,
            pulse_format: pulse_format,
        }
    }
}

impl Sink for PulseAudioSink {
    fn start(&mut self) -> io::Result<()> {
        if self.s.is_some() {
            return Ok(());
        }

        let ss = pulse::sample::Spec {
            format: self.pulse_format,
            channels: NUM_CHANNELS,
            rate: SAMPLE_RATE,
        };

        if !ss.is_valid() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Invalid PulseAudio sample spec",
            ));
        }

        let device = self.device.as_deref();
        let result = Simple::new(
            None,                // Use the default server.
            APP_NAME,            // Our application's name.
            Direction::Playback, // Direction.
            device,              // Our device (sink) name.
            STREAM_NAME,         // Description of our stream.
            &ss,                 // Our sample format.
            None,                // Use default channel map.
            None,                // Use default buffering attributes.
        );
        match result {
            Ok(s) => {
                self.s = Some(s);
                Ok(())
            }
            Err(e) => Err(io::Error::new(io::ErrorKind::ConnectionRefused, e)),
        }
    }

    fn stop(&mut self) -> io::Result<()> {
        self.s = None;
        Ok(())
    }

    sink_as_bytes!();
}

impl SinkAsBytes for PulseAudioSink {
    fn write_bytes(&mut self, data: &[u8]) -> io::Result<()> {
        if let Some(s) = &self.s {
            match s.write(data) {
                Ok(_) => Ok(()),
                Err(e) => Err(io::Error::new(io::ErrorKind::BrokenPipe, e)),
            }
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotConnected,
                "Not connected to PulseAudio",
            ))
        }
    }
}

impl PulseAudioSink {
    pub const NAME: &'static str = "pulseaudio";
}
