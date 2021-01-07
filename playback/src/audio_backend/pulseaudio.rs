use super::{Open, Sink};
use crate::audio::AudioPacket;
use libpulse_binding::{self as pulse, stream::Direction};
use libpulse_simple_binding::Simple;
use std::io;

const APP_NAME: &str = "librespot";
const STREAM_NAME: &str = "Spotify endpoint";

pub struct PulseAudioSink {
    s: Option<Simple>,
    ss: pulse::sample::Spec,
    device: Option<String>,
}

impl Open for PulseAudioSink {
    fn open(device: Option<String>) -> PulseAudioSink {
        debug!("Using PulseAudio sink");

        let ss = pulse::sample::Spec {
            format: pulse::sample::Format::S16le,
            channels: 2, // stereo
            rate: 44100,
        };
        debug_assert!(ss.is_valid());

        PulseAudioSink {
            s: None,
            ss: ss,
            device: device,
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

    fn write(&mut self, packet: &AudioPacket) -> io::Result<()> {
        if let Some(s) = &self.s {
            // SAFETY: An i16 consists of two bytes, so that the given slice can be interpreted
            // as a byte array of double length. Each byte pointer is validly aligned, and so
            // is the newly created slice.
            let d: &[u8] = unsafe {
                std::slice::from_raw_parts(
                    packet.samples().as_ptr() as *const u8,
                    packet.samples().len() * 2,
                )
            };

            match s.write(d) {
                Ok(_) => Ok(()),
                Err(e) => Err(io::Error::new(
                    io::ErrorKind::BrokenPipe,
                    e.to_string().unwrap(),
                )),
            }
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotConnected,
                "Not connected to pulseaudio",
            ))
        }
    }
}
