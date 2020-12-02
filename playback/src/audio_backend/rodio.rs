use super::{Open, Sink};
extern crate cpal;
extern crate rodio;
use cpal::traits::{DeviceTrait, HostTrait};
use std::process::exit;
use std::{io, thread, time};

pub struct RodioSink {
    rodio_sink: rodio::Sink,
    // We have to keep hold of this object, or the Sink can't play...
    #[allow(dead_code)]
    stream: rodio::OutputStream,
}

fn list_formats(ref device: &rodio::Device) {
    let default_fmt = match device.default_output_config() {
        Ok(fmt) => cpal::SupportedStreamConfig::from(fmt),
        Err(e) => {
            warn!("Error getting default rodio::Sink config: {}", e);
            return;
        }
    };
    debug!("  Default config:");
    debug!("    {:?}", default_fmt);

    let mut output_configs = match device.supported_output_configs() {
        Ok(f) => f.peekable(),
        Err(e) => {
            warn!("Error getting supported rodio::Sink configs: {}", e);
            return;
        }
    };

    if output_configs.peek().is_some() {
        debug!("  Available configs:");
        for format in output_configs {
            debug!("    {:?}", format);
        }
    }
}

fn list_outputs() {
    let default_device = get_default_device();
    let default_device_name = default_device.name().expect("cannot get output name");
    println!("Default Audio Device:\n  {}", default_device_name);
    list_formats(&default_device);

    println!("Other Available Audio Devices:");
    for device in cpal::default_host()
        .output_devices()
        .expect("cannot get list of output devices")
    {
        let device_name = device.name().expect("cannot get output name");
        if device_name != default_device_name {
            println!("  {}", device_name);
            list_formats(&device);
        }
    }
}

fn get_default_device() -> rodio::Device {
    cpal::default_host()
        .default_output_device()
        .expect("no default output device available")
}

fn match_device(device: Option<String>) -> rodio::Device {
    match device {
        Some(device_name) => {
            if device_name == "?".to_string() {
                list_outputs();
                exit(0)
            }
            for d in cpal::default_host()
                .output_devices()
                .expect("cannot get list of output devices")
            {
                if d.name().expect("cannot get output name") == device_name {
                    return d;
                }
            }
            println!("No output sink matching '{}' found.", device_name);
            exit(0)
        }
        None => return get_default_device(),
    }
}

impl Open for RodioSink {
    fn open(device: Option<String>) -> RodioSink {
        debug!(
            "Using rodio sink with cpal host: {:?}",
            cpal::default_host().id()
        );

        let rodio_device = match_device(device);
        debug!("Using cpal device");
        let stream = rodio::OutputStream::try_from_device(&rodio_device)
            .expect("Couldn't open output stream.");
        debug!("Using rodio stream");
        let sink = rodio::Sink::try_new(&stream.1).expect("Couldn't create output sink.");
        debug!("Using rodio sink");

        RodioSink {
            rodio_sink: sink,
            stream: stream.0,
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
