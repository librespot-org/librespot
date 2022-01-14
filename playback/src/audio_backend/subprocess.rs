use super::{Open, Sink, SinkAsBytes, SinkError, SinkResult};
use crate::config::AudioFormat;
use crate::convert::Converter;
use crate::decoder::AudioPacket;
use shell_words::split;

use std::io::Write;
use std::process::{exit, Child, Command, Stdio};

pub struct SubprocessSink {
    shell_command: String,
    child: Option<Child>,
    format: AudioFormat,
}

impl Open for SubprocessSink {
    fn open(shell_command: Option<String>, format: AudioFormat) -> Self {
        let shell_command = match shell_command.as_deref() {
            Some("?") => {
                info!("Usage: --backend subprocess --device {{shell_command}}");
                exit(0);
            }
            Some(cmd) => cmd.to_owned(),
            None => {
                error!("subprocess sink requires specifying a shell command");
                exit(1);
            }
        };

        info!("Using subprocess sink with format: {:?}", format);

        Self {
            shell_command,
            child: None,
            format,
        }
    }
}

impl Sink for SubprocessSink {
    fn start(&mut self) -> SinkResult<()> {
        let args = split(&self.shell_command).unwrap();
        let child = Command::new(&args[0])
            .args(&args[1..])
            .stdin(Stdio::piped())
            .spawn()
            .map_err(|e| SinkError::ConnectionRefused(e.to_string()))?;
        self.child = Some(child);
        Ok(())
    }

    fn stop(&mut self) -> SinkResult<()> {
        if let Some(child) = &mut self.child.take() {
            child
                .kill()
                .map_err(|e| SinkError::OnWrite(e.to_string()))?;
            child
                .wait()
                .map_err(|e| SinkError::OnWrite(e.to_string()))?;
        }
        Ok(())
    }

    sink_as_bytes!();
}

impl SinkAsBytes for SubprocessSink {
    fn write_bytes(&mut self, data: &[u8]) -> SinkResult<()> {
        if let Some(child) = &mut self.child {
            let child_stdin = child
                .stdin
                .as_mut()
                .ok_or_else(|| SinkError::NotConnected("Child is None".to_string()))?;
            child_stdin
                .write_all(data)
                .map_err(|e| SinkError::OnWrite(e.to_string()))?;
            child_stdin
                .flush()
                .map_err(|e| SinkError::OnWrite(e.to_string()))?;
        }
        Ok(())
    }
}

impl SubprocessSink {
    pub const NAME: &'static str = "subprocess";
}
