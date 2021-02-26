use super::{Open, Sink};
use crate::audio::AudioPacket;
use sdl2::audio::{AudioQueue, AudioSpecDesired};
use std::{io, thread, time};

type Channel = i16;

pub struct SdlSink {
    queue: AudioQueue<Channel>,
}

impl Open for SdlSink {
    fn open(device: Option<String>) -> SdlSink {
        debug!("Using SDL sink");

        if device.is_some() {
            panic!("SDL sink does not support specifying a device name");
        }

        let ctx = sdl2::init().expect("Could not init SDL");
        let audio = ctx.audio().expect("Could not init SDL audio subsystem");

        let desired_spec = AudioSpecDesired {
            freq: Some(44_100),
            channels: Some(2),
            samples: None,
        };
        let queue = audio
            .open_queue(None, &desired_spec)
            .expect("Could not open SDL audio device");

        SdlSink { queue: queue }
    }
}

impl Sink for SdlSink {
    fn start(&mut self) -> io::Result<()> {
        self.queue.clear();
        self.queue.resume();
        Ok(())
    }

    fn stop(&mut self) -> io::Result<()> {
        self.queue.pause();
        self.queue.clear();
        Ok(())
    }

    fn write(&mut self, packet: &AudioPacket) -> io::Result<()> {
        while self.queue.size() > (2 * 2 * 44_100) {
            // sleep and wait for sdl thread to drain the queue a bit
            thread::sleep(time::Duration::from_millis(10));
        }
        self.queue.queue(packet.samples());
        Ok(())
    }
}
