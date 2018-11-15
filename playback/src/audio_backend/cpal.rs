use super::{Open, Sink};
extern crate cpal;
use std::io;
use std::thread;
use std::sync::mpsc::{sync_channel, SyncSender};
use std::process::exit;

pub struct CpalSink {
    // event_loop: cpal::EventLoop,
    send: SyncSender<i16>,
}

fn list_outputs() {
    println!("Default Audio Device:\n  {:?}", cpal::default_output_device().map(|e| e.name()));

    println!("Available Audio Devices:");
    for device in cpal::output_devices() {
        println!("- {}", device.name());
        // Output formats
        if let Ok(fmt) = device.default_output_format() {
            println!("  Default format:\n    {:?}", fmt);
        }
        let mut output_formats = match device.supported_output_formats() {
            Ok(f) => f.peekable(),
            Err(e) => {
                println!("Error: {:?}", e);
                continue;
            },
        };
        if output_formats.peek().is_some() {
            println!("  All formats:");
            for format in output_formats {
                println!("    {:?}", format);
            }
        }
    }
}

impl Open for CpalSink {
    fn open(device: Option<String>) -> CpalSink {
        info!("Using cpal sink");

        // buffer for samples from librespot (~10ms)
        let (tx, rx) = sync_channel::<i16>(2 * 1024 * 4);
        let event_loop = cpal::EventLoop::new();

        if device.is_some() {
            if device == Some("?".to_string()) {
                list_outputs();
                exit(0)
            }
            // N.B. This is perfectly possible to support.
            // TODO: First need to enable listing of devices.
                // Remember to filter to those which support Stereo 16bit 44100Hz
            // TODO: Choose cpal sink by name.
            panic!("cpal sink does not support specifying a device name");
        }
        let cpal_device = cpal::default_output_device().expect("no output device available");
        // TODO: Support more formats? Surely cpal will handle that.
        let format = cpal::Format{channels: 2, sample_rate: cpal::SampleRate(44100), data_type: cpal::SampleFormat::I16};

        let stream_id = event_loop.build_output_stream(&cpal_device, &format).expect("could not build output stream");
        event_loop.play_stream(stream_id);

        thread::spawn(move |/*event_loop, rx*/| {
            event_loop.run(move |_stream_id, stream_data| {
                match stream_data {
                    cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::I16(mut buffer) } => {
                        for (sample, recv) in buffer.iter_mut().zip(rx.try_iter()) {
                            *sample = recv;
                        }
                    },
                    _ => (),
                }
            });
        });

        CpalSink {
            send: tx,
            // event_loop: event_loop,
        }
    }
}

impl Sink for CpalSink {
    fn start(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn stop(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn write(&mut self, data: &[i16]) -> io::Result<()> {
        for s in data.iter() {
            let res = self.send.send(*s);
            if res.is_err() {
                error!("jackaudio: cannot write to channel");
            }
        }
        Ok(())
    }
}
