use super::{Open, Sink, SinkAsBytes};
use crate::audio::{AudioPacket, Requantizer};
use crate::config::AudioFormat;
use std::fs::OpenOptions;
use std::io::{self, Write};

pub struct StdoutSink {
    output: Box<dyn Write>,
    format: AudioFormat,
    requantizer: Requantizer,
}

impl Open for StdoutSink {
    fn open(path: Option<String>, format: AudioFormat, requantizer: Requantizer) -> Self {
        info!("Using pipe sink with format: {:?}", format);

        let output: Box<dyn Write> = match path {
            Some(path) => Box::new(OpenOptions::new().write(true).open(path).unwrap()),
            _ => Box::new(io::stdout()),
        };

        Self {
            output,
            format,
            requantizer,
        }
    }
}

impl Sink for StdoutSink {
    start_stop_noop!();
    sink_as_bytes!();
}

impl SinkAsBytes for StdoutSink {
    fn write_bytes(&mut self, data: &[u8]) -> io::Result<()> {
        self.output.write_all(data)?;
        self.output.flush()?;
        Ok(())
    }
}
