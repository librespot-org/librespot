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

#[cfg(all(
    feature = "rodiojack-backend",
    any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd")
))]
pub struct JackRodioSink {
    jackrodio_sink: rodio::Sink,
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

fn list_outputs(ref host: &cpal::Host) {
    let default_device = get_default_device(host);
    let default_device_name = default_device.name().expect("cannot get output name");
    println!("Default Audio Device:\n  {}", default_device_name);
    list_formats(&default_device);

    println!("Other Available Audio Devices:");

    let found_devices = host.output_devices().expect(&format!(
        "Cannot get list of output devices of Host: {:?}",
        host.id()
    ));
    for device in found_devices {
        let device_name = device.name().expect("cannot get output name");
        if device_name != default_device_name {
            println!("  {}", device_name);
            list_formats(&device);
        }
    }
}

fn get_default_device(ref host: &cpal::Host) -> rodio::Device {
    host.default_output_device()
        .expect("no default output device available")
}

fn match_device(ref host: &cpal::Host, device: Option<String>) -> rodio::Device {
    match device {
        Some(device_name) => {
            if device_name == "?".to_string() {
                list_outputs(host);
                exit(0)
            }

            let found_devices = host.output_devices().expect(&format!(
                "Cannot get list of output devices of Host: {:?}",
                host.id()
            ));
            for d in found_devices {
                if d.name().expect("cannot get output name") == device_name {
                    return d;
                }
            }
            println!("No output sink matching '{}' found.", device_name);
            exit(0)
        }
        None => return get_default_device(host),
    }
}

impl Open for RodioSink {
    fn open(device: Option<String>) -> RodioSink {
        let host = cpal::default_host();
        debug!("Using rodio sink with cpal host: {:?}", host.id());

        let rodio_device = match_device(&host, device);
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

#[cfg(all(
    feature = "rodiojack-backend",
    any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd")
))]
impl Open for JackRodioSink {
    fn open(device: Option<String>) -> JackRodioSink {
        let host = cpal::host_from_id(
            cpal::available_hosts()
                .into_iter()
                .find(|id| *id == cpal::HostId::Jack)
                .expect("Jack Host not found"),
        )
        .expect("Jack Host not found");
        debug!("Using jack rodio sink with cpal Jack host");

        let rodio_device = match_device(&host, device);
        debug!("Using cpal device");
        let stream = rodio::OutputStream::try_from_device(&rodio_device)
            .expect("Couldn't open output stream.");
        debug!("Using jack rodio stream");
        let sink = rodio::Sink::try_new(&stream.1).expect("Couldn't create output sink.");
        debug!("Using jack rodio sink");

        JackRodioSink {
            jackrodio_sink: sink,
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

#[cfg(all(
    feature = "rodiojack-backend",
    any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd")
))]
impl Sink for JackRodioSink {
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
        self.jackrodio_sink.append(source);

        // Chunk sizes seem to be about 256 to 3000 ish items long.
        // Assuming they're on average 1628 then a half second buffer is:
        // 44100 elements --> about 27 chunks
        while self.jackrodio_sink.len() > 26 {
            // sleep and wait for rodio to drain a bit
            thread::sleep(time::Duration::from_millis(10));
        }
        Ok(())
    }
}
