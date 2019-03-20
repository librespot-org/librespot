use super::{Open, Sink};
extern crate rodio;
extern crate cpal;
use std::{io, thread, time};
use std::process::exit;

pub struct RodioSink {
    rodio_sink: rodio::Sink,
}

fn list_formats(ref device: &rodio::Device) {
    let default_fmt = match device.default_output_format() {
        Ok(fmt) => cpal::SupportedFormat::from(fmt),
        Err(e) => {
            info!("Error getting default rodio::Sink format: {:?}", e);
            return;
        },
    };

    let mut output_formats = match device.supported_output_formats() {
        Ok(f) => f.peekable(),
        Err(e) => {
            info!("Error getting supported rodio::Sink formats: {:?}", e);
            return;
        },
    };

    if output_formats.peek().is_some() {
        debug!("  Available formats:");
        for format in output_formats {
            let s = format!("{}ch, {:?}, min {:?}, max {:?}", format.channels, format.data_type, format.min_sample_rate, format.max_sample_rate);
            if format == default_fmt {
                debug!("    (default) {}", s);
            } else {
                debug!("    {:?}", format);
            }
        }
    }
}

fn list_outputs() {
    let default_device = rodio::default_output_device().unwrap();
    println!("Default Audio Device:\n  {}", default_device.name());
    list_formats(&default_device);

    println!("Other Available Audio Devices:");
    for device in rodio::output_devices() {
        if device.name() != default_device.name() {
            println!("  {}", device.name());
            list_formats(&device);
        }
    }
}

impl Open for RodioSink {
    fn open(device: Option<String>) -> RodioSink {
        debug!("Using rodio sink");

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
        // More similar to an "unpause" than "play". Doesn't undo "stop".
        // self.rodio_sink.play();
        Ok(())
    }

    fn stop(&mut self) -> io::Result<()> {
        // This will immediately stop playback, but the sink is then unusable.
        // We just have to let the current buffer play till the end.
        // self.rodio_sink.stop();
        Ok(())
    }

    fn write(&mut self, data: &[i16]) -> io::Result<()> {
        let source = rodio::buffer::SamplesBuffer::new(2, 44100, data);
        self.rodio_sink.append(source);

        // Chunk sizes seem to be about 256 to 3000 ish items long.
        // Assuming they're on average 1628 then a half second buffer is:
        // 44100 elements --> about 27 chunks
        while self.rodio_sink.len() > 26 {
            // sleep and wait for rodio to drain a bit
            thread::sleep(time::Duration::from_millis(10));
        }
        Ok(())
    }
}
