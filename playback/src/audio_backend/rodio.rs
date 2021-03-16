use super::{Open, Sink};
extern crate cpal;
extern crate rodio;
use crate::audio::AudioPacket;
use crate::config::AudioFormat;
use cpal::traits::{DeviceTrait, HostTrait};
use std::process::exit;
use std::{io, thread, time};

// most code is shared between RodioSink and JackRodioSink
macro_rules! rodio_sink {
    ($name: ident) => {
        pub struct $name {
            rodio_sink: rodio::Sink,
            // We have to keep hold of this object, or the Sink can't play...
            #[allow(dead_code)]
            stream: rodio::OutputStream,
            format: AudioFormat,
        }

        impl Sink for $name {
            start_stop_noop!();

            fn write(&mut self, packet: &AudioPacket) -> io::Result<()> {
                let samples = packet.samples();
                match self.format {
                    AudioFormat::F32 => {
                        let source = rodio::buffer::SamplesBuffer::new(2, 44100, samples);
                        self.rodio_sink.append(source)
                    },
                    AudioFormat::S16 => {
                        let samples_s16: Vec<i16> = AudioPacket::f32_to_s16(samples);
                        let source = rodio::buffer::SamplesBuffer::new(2, 44100, samples_s16);
                        self.rodio_sink.append(source)
                    },
                    _ => unimplemented!(),
                };

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

        impl $name {
            fn open_sink(host: &cpal::Host, device: Option<String>, format: AudioFormat) -> $name {
                match format  {
                    AudioFormat::F32 => {
                        #[cfg(target_os = "linux")]
                        {
                            warn!("Rodio output to Alsa is known to cause garbled sound on output formats other than 16-bit signed integer.");
                            warn!("Consider using `--backend alsa` OR `--format {:?}`", AudioFormat::S16);
                        }
                    },
                    AudioFormat::S16 => {},
                    _ => unimplemented!("Rodio currently only supports F32 and S16 formats"),
                }

                let rodio_device = match_device(&host, device);
                debug!("Using cpal device");
                let stream = rodio::OutputStream::try_from_device(&rodio_device)
                    .expect("couldn't open output stream.");
                debug!("Using Rodio stream");
                let sink = rodio::Sink::try_new(&stream.1).expect("couldn't create output sink.");
                debug!("Using Rodio sink");

                Self {
                    rodio_sink: sink,
                    stream: stream.0,
                    format: format,
                }
            }
        }
    };
}
rodio_sink!(RodioSink);

#[cfg(all(
    feature = "rodiojack-backend",
    any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd")
))]
rodio_sink!(JackRodioSink);

fn list_formats(ref device: &rodio::Device) {
    let default_fmt = match device.default_output_config() {
        Ok(fmt) => cpal::SupportedStreamConfig::from(fmt),
        Err(e) => {
            warn!("Error getting default Rodio output config: {}", e);
            return;
        }
    };
    debug!("  Default config:");
    debug!("    {:?}", default_fmt);

    let mut output_configs = match device.supported_output_configs() {
        Ok(f) => f.peekable(),
        Err(e) => {
            warn!("Error getting supported Rodio output configs: {}", e);
            return;
        }
    };

    if output_configs.peek().is_some() {
        debug!("  Available output configs:");
        for format in output_configs {
            debug!("    {:?}", format);
        }
    }
}

fn list_outputs(ref host: &cpal::Host) {
    let default_device = get_default_device(host);
    let default_device_name = default_device.name().expect("cannot get output name");
    println!("Default audio device:\n  {}", default_device_name);
    list_formats(&default_device);

    println!("Other available audio devices:");

    let found_devices = host.output_devices().expect(&format!(
        "Cannot get list of output devices of host: {:?}",
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
                "cannot get list of output devices of host: {:?}",
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
    fn open(device: Option<String>, format: AudioFormat) -> RodioSink {
        let host = cpal::default_host();
        info!(
            "Using Rodio sink with format {:?} and cpal host: {:?}",
            format,
            host.id()
        );
        Self::open_sink(&host, device, format)
    }
}

#[cfg(all(
    feature = "rodiojack-backend",
    any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd")
))]
impl Open for JackRodioSink {
    fn open(device: Option<String>, format: AudioFormat) -> JackRodioSink {
        let host = cpal::host_from_id(
            cpal::available_hosts()
                .into_iter()
                .find(|id| *id == cpal::HostId::Jack)
                .expect("JACK host not found"),
        )
        .expect("JACK host not found");
        info!(
            "Using JACK Rodio sink with format {:?} and cpal JACK host",
            format
        );
        Self::open_sink(&host, device, format)
    }
}
