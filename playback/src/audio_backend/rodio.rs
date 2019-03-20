use super::{Open, Sink};
extern crate rodio;
use std::io;
use std::process::exit;

pub struct RodioSink {
    rodio_sink: rodio::Sink,
}

fn list_outputs() {
    println!("Default Audio Device:\n  {:?}", rodio::default_output_device().map(|e| e.name()));

    println!("Available Audio Devices:");
    for device in rodio::output_devices() {
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

impl Open for RodioSink {
    fn open(device: Option<String>) -> RodioSink {
        info!("Using rodio sink");

        let mut rodio_device = rodio::default_output_device().expect("no output device available");
        if device.is_some() {
            let device_name = device.unwrap();

            if device_name == "?".to_string() {
                list_outputs();
                exit(0)
            }
            let mut found = false;
            for d in rodio::output_devices() {
                if d.name() == device_name {
                    rodio_device = d;
                    found = true;
                    break;
                }
            }
            if !found {
                println!("No output sink matching '{}' found.", device_name);
                exit(0)
            }
        }
        let sink = rodio::Sink::new(&rodio_device);

        RodioSink {
            rodio_sink: sink,
        }
    }
}

impl Sink for RodioSink {
    fn start(&mut self) -> io::Result<()> {
        self.rodio_sink.play();
        Ok(())
    }

    fn stop(&mut self) -> io::Result<()> {
        self.rodio_sink.stop();
        Ok(())
    }

    fn write(&mut self, data: &[i16]) -> io::Result<()> {
        let source = rodio::buffer::SamplesBuffer::new(2, 44100, data);
        self.rodio_sink.append(source);
        Ok(())
    }
}
