use super::{Open, Sink};
extern crate rodio;
extern crate cpal;
use std::{io, thread, time};
use std::process::exit;

pub struct RodioSink {
    rodio_sink: Option<rodio::Sink>,
    device_name: Option<String>,
}

fn list_formats(ref device: &rodio::Device) {
    let default_fmt = match device.default_output_format() {
        Ok(fmt) => cpal::SupportedFormat::from(fmt),
        Err(e) => {
            warn!("Error getting default rodio::Sink format: {:?}", e);
            return;
        },
    };

    let mut output_formats = match device.supported_output_formats() {
        Ok(f) => f.peekable(),
        Err(e) => {
            warn!("Error getting supported rodio::Sink formats: {:?}", e);
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

fn match_output(device_name: Option<String>) -> cpal::Device {
    match device_name {
        Some(dn) => {
            let mut rodio_device = None;
            for device in cpal::output_devices() {
                if device.name() == dn {
                    rodio_device = Some(device);
                    break;
                }
            }
            match rodio_device {
                Some(cd) => cd,
                None => {
                    println!("No output sink matching '{}' found.", dn);
                    exit(0)
                }
            }
        },
        None => rodio::default_output_device().expect("no output device available")
    }
}

impl Open for RodioSink {
    fn open(device_name: Option<String>) -> RodioSink {
        debug!("Using rodio sink");

        if device_name == Some("?".to_string()) {
            list_outputs();
            exit(0)
        }

        RodioSink {
            device_name,
            rodio_sink: None,
        }
    }
}

impl Sink for RodioSink {
    fn start(&mut self) -> io::Result<()> {
        let rodio_device = match_output(self.device_name.clone());
        let sink = rodio::Sink::new(&rodio_device);
        self.rodio_sink = Some(sink);
        Ok(())
    }

    fn stop(&mut self) -> io::Result<()> {
        let sink = self.rodio_sink.as_mut().expect("stop called before start");
        sink.stop();
        Ok(())
    }

    fn write(&mut self, data: &[i16]) -> io::Result<()> {
        let sink = self.rodio_sink.as_mut().expect("write called before start");

        let source = rodio::buffer::SamplesBuffer::new(2, 44100, data);
        sink.append(source);

        // Chunk sizes seem to be about 256 to 3000 ish items long.
        // Assuming they're on average 1628 then a half second buffer is:
        // 44100 elements --> about 27 chunks
        while sink.len() > 26 {
            // sleep and wait for rodio to drain a bit
            thread::sleep(time::Duration::from_millis(10));
        }
        Ok(())
    }
}
