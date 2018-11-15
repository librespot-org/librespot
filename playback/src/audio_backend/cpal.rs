use super::{Open, Sink};
extern crate cpal;
use std::io;
use std::thread;
use std::collections::VecDeque;

pub struct CpalSink {
    event_loop: cpal::EventLoop,
    buffer: mut VecDeque<i16>,
    stream_id: Option<cpal::StreamId>,
}

impl Open for CpalSink {
    fn open(device: Option<String>) -> CpalSink {
        info!("Using cpal sink");

        if device.is_some() {
            // N.B. This is perfectly possible to support.
            // TODO: First need to enable listing of devices.
                // Remember to filter to those which support Stereo 16bit 44100Hz
            // TODO: Choose cpal sink by name.
            panic!("cpal sink does not support specifying a device name");
        }

        let event_loop = cpal::EventLoop::new();

        CpalSink {
            // Allow an (arbitrary) 2 second buffer before resizing.
            buffer: VecDeque::with_capacity(44100 * 2 * 2),
            event_loop: event_loop,
        }
    }
}

impl Sink for CpalSink {
    fn start(&mut self) -> io::Result<()> {

        if self.stream_id.is_none() {

            let device = cpal::default_output_device().expect("no output device available");
            // TODO: Support more formats.
            let format = cpal::Format(2, 44100, cpal::SampleFormat::I16);

            self.stream_id = self.event_loop.build_output_stream(&device, &format)?;

            self.event_loop.play_stream(self.stream_id.clone());
        }

        if self.thread.is_none() {
            let event_loop = self.event_loop;
            let source  = self.buffer;
            thread::spawn(move |event_loop, source| {
                event_loop.run(move |_stream_id, mut stream_data| {
                    match data {
                        cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::I16(mut buffer) } => {
                            let sl = source.len();
                            if (sl > buffer.len()) {
                                sl = buffer.len();
                            }
                            // let u: Vec<_> = source.drain(..sl).collect();
                            // buffer[..s1].copy_from_slice(u[..s1]);

                            for (sample, data) in buffer.iter_mut().zip(source.drain(..sl)) {
                                *sample = data;
                            }
                        },
                        _ => (),
                    }
                });
            })
        }

        Ok(())
    }

    fn stop(&mut self) -> io::Result<()> {
        if !self.stream_id.is_none() {
            self.event_loop.destroy_stream(self.stream_id);
            self.stream_id = None;
            self.buffer.clear();
        }
        Ok(())
    }

    fn write(&mut self, data: &[i16]) -> io::Result<()> {
        // self.0.as_mut().unwrap().write_interleaved(&data).unwrap();
        // self.buffer.reserve(data.len()); // Unneccessary?
        // self.buffer.extend_from_slice(data);
        self.buffer.extend(data);
        Ok(())
    }
}
