use super::{Open, Sink};
use crate::audio::AudioPacket;
use crate::config::AudioFormat;
use crate::player::{NUM_CHANNELS, SAMPLE_RATE};
use sdl2::audio::{AudioQueue, AudioSpecDesired};
use std::{io, mem, thread, time};

pub enum SdlSink {
    F32(AudioQueue<f32>),
    S32(AudioQueue<i32>),
    S16(AudioQueue<i16>),
}

impl Open for SdlSink {
    fn open(device: Option<String>, format: AudioFormat) -> SdlSink {
        info!("Using SDL sink with format: {:?}", format);

        if device.is_some() {
            warn!("SDL sink does not support specifying a device name");
        }

        let ctx = sdl2::init().expect("could not initialize SDL");
        let audio = ctx
            .audio()
            .expect("could not initialize SDL audio subsystem");

        let desired_spec = AudioSpecDesired {
            freq: Some(SAMPLE_RATE as i32),
            channels: Some(NUM_CHANNELS),
            samples: None,
        };

        macro_rules! open_sink {
            ($sink: expr, $type: ty) => {{
                let queue: AudioQueue<$type> = audio
                    .open_queue(None, &desired_spec)
                    .expect("could not open SDL audio device");
                $sink(queue)
            }};
        }
        match format {
            AudioFormat::F32 => open_sink!(Self::F32, f32),
            AudioFormat::S32 => open_sink!(Self::S32, i32),
            AudioFormat::S16 => open_sink!(Self::S16, i16),
            _ => {
                unimplemented!("SDL currently does not support {:?} output", format)
            }
        }
    }
}

impl Sink for SdlSink {
    fn start(&mut self) -> io::Result<()> {
        macro_rules! start_sink {
            ($queue: expr) => {{
                $queue.clear();
                $queue.resume();
            }};
        }
        match self {
            Self::F32(queue) => start_sink!(queue),
            Self::S32(queue) => start_sink!(queue),
            Self::S16(queue) => start_sink!(queue),
        };
        Ok(())
    }

    fn stop(&mut self) -> io::Result<()> {
        macro_rules! stop_sink {
            ($queue: expr) => {{
                $queue.pause();
                $queue.clear();
            }};
        }
        match self {
            Self::F32(queue) => stop_sink!(queue),
            Self::S32(queue) => stop_sink!(queue),
            Self::S16(queue) => stop_sink!(queue),
        };
        Ok(())
    }

    fn write(&mut self, packet: &AudioPacket) -> io::Result<()> {
        macro_rules! drain_sink {
            ($queue: expr, $size: expr) => {{
                // sleep and wait for sdl thread to drain the queue a bit
                while $queue.size() > (NUM_CHANNELS as u32 * $size as u32 * SAMPLE_RATE) {
                    thread::sleep(time::Duration::from_millis(10));
                }
            }};
        }
        match self {
            Self::F32(queue) => {
                drain_sink!(queue, mem::size_of::<f32>());
                queue.queue(packet.samples())
            }
            Self::S32(queue) => {
                drain_sink!(queue, mem::size_of::<i32>());
                let samples_s32: Vec<i32> = AudioPacket::f32_to_s32(packet.samples());
                queue.queue(&samples_s32)
            }
            Self::S16(queue) => {
                drain_sink!(queue, mem::size_of::<i16>());
                let samples_s16: Vec<i16> = AudioPacket::f32_to_s16(packet.samples());
                queue.queue(&samples_s16)
            }
        };
        Ok(())
    }
}
