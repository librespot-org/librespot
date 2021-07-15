use super::{Open, Sink, SinkAsBytes, SinkError, SinkResult};
use crate::config::AudioFormat;
use crate::convert::Converter;
use crate::decoder::AudioPacket;
use std::fs::OpenOptions;
use std::io::{self, Write};

pub struct StdoutSink {
    output: Option<Box<dyn Write>>,
    path: Option<String>,
    format: AudioFormat,
}

impl Open for StdoutSink {
    fn open(path: Option<String>, format: AudioFormat) -> Self {
        info!("Using pipe sink with format: {:?}", format);
        Self {
            output: None,
            path,
            format,
        }
    }
}

impl Sink for StdoutSink {
    fn start(&mut self) -> SinkResult<()> {
        if self.output.is_none() {
            let output: Box<dyn Write> = match self.path.as_deref() {
                Some(path) => {
                    let open_op = OpenOptions::new()
                        .write(true)
                        .open(path)
                        .map_err(|e| SinkError::ConnectionRefused(e.to_string()))?;
                    Box::new(open_op)
                }
                None => Box::new(io::stdout()),
            };

            self.output = Some(output);
        }

        Ok(())
    }

    sink_as_bytes!();
}

impl SinkAsBytes for StdoutSink {
    fn write_bytes(&mut self, data: &[u8]) -> SinkResult<()> {
        match self.output.as_deref_mut() {
            Some(output) => {
                output
                    .write_all(data)
                    .map_err(|e| SinkError::OnWrite(e.to_string()))?;
                output
                    .flush()
                    .map_err(|e| SinkError::OnWrite(e.to_string()))?;
            }
            None => {
                return Err(SinkError::NotConnected("Output is None".to_string()));
            }
        }

        Ok(())
    }
}

impl StdoutSink {
    pub const NAME: &'static str = "pipe";
}
