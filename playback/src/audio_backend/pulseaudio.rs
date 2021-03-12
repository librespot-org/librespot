use super::{Open, Sink, SinkAsBytes};
use crate::audio::AudioPacket;
use crate::config::AudioFormat;
use crate::player::{NUM_CHANNELS, SAMPLE_RATE};
use libpulse_binding::{self as pulse, stream::Direction};
use libpulse_simple_binding::Simple;
use std::io;

const APP_NAME: &str = "librespot";
const STREAM_NAME: &str = "Spotify endpoint";

pub struct PulseAudioSink {
    s: Option<Simple>,
    ss: pulse::sample::Spec,
    device: Option<String>,
    format: AudioFormat,
}

impl Open for PulseAudioSink {
    fn open(device: Option<String>, format: AudioFormat) -> PulseAudioSink {
        info!("Using PulseAudio sink with format: {:?}", format);

        let pulse_format = match format {
            AudioFormat::F32 => pulse::sample::Format::F32le,
            AudioFormat::S16 => pulse::sample::Format::S16le,
        };

        let ss = pulse::sample::Spec {
            format: pulse_format,
            channels: NUM_CHANNELS,
            rate: SAMPLE_RATE,
        };
        debug_assert!(ss.is_valid());

        PulseAudioSink {
            s: None,
            ss: ss,
            device: device,
            format: format,
        }
    }
}

impl Sink for PulseAudioSink {
    fn start(&mut self) -> io::Result<()> {
        if self.s.is_some() {
            return Ok(());
        }

        let device = self.device.as_ref().map(|s| (*s).as_str());
        let result = Simple::new(
            None,                // Use the default server.
            APP_NAME,            // Our application's name.
            Direction::Playback, // Direction.
            device,              // Our device (sink) name.
            STREAM_NAME,         // Description of our stream.
            &self.ss,            // Our sample format.
            None,                // Use default channel map.
            None,                // Use default buffering attributes.
        );
        match result {
            Ok(s) => {
                self.s = Some(s);
                Ok(())
            }
            Err(e) => Err(io::Error::new(
                io::ErrorKind::ConnectionRefused,
                e.to_string().unwrap(),
            )),
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
                Err(e) => Err(io::Error::new(
                    io::ErrorKind::BrokenPipe,
                    e.to_string().unwrap(),
                )),
            }
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotConnected,
                "Not connected to PulseAudio",
            ))
        }
    }
}
