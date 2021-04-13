use super::{Open, Sink};
use crate::audio::{convert, AudioPacket, Requantizer};
use crate::config::AudioFormat;
use crate::player::{NUM_CHANNELS, SAMPLE_RATE};
use sdl2::audio::{AudioQueue, AudioSpecDesired};
use std::{io, thread, time};

pub enum SdlSink {
    F32(AudioQueue<f32>, Requantizer),
    S32(AudioQueue<i32>, Requantizer),
    S16(AudioQueue<i16>, Requantizer),
}

impl Open for SdlSink {
    fn open(device: Option<String>, format: AudioFormat, requantizer: Requantizer) -> Self {
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
                $sink(queue, requantizer)
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
            Self::F32(queue, _) => start_sink!(queue),
            Self::S32(queue, _) => start_sink!(queue),
            Self::S16(queue, _) => start_sink!(queue),
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
            Self::F32(queue, _) => stop_sink!(queue),
            Self::S32(queue, _) => stop_sink!(queue),
            Self::S16(queue, _) => stop_sink!(queue),
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

        let samples = packet.samples();
        match self {
            Self::F32(queue, _) => {
                drain_sink!(queue, AudioFormat::F32.size());
                queue.queue(samples)
            }
            Self::S32(queue, ref mut requantizer) => {
                let samples_s32: &[i32] = &convert::to_s32(samples, requantizer);
                drain_sink!(queue, AudioFormat::S32.size());
                queue.queue(samples_s32)
            }
            Self::S16(queue, ref mut requantizer) => {
                let samples_s16: &[i16] = &convert::to_s16(samples, requantizer);
                drain_sink!(queue, AudioFormat::S16.size());
                queue.queue(samples_s16)
            }
        };
        Ok(())
    }
}
