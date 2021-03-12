use super::{Open, Sink, SinkAsBytes};
use crate::audio::AudioPacket;
use crate::config::AudioFormat;
use shell_words::split;
use std::io::{self, Write};
use std::process::{Child, Command, Stdio};

pub struct SubprocessSink {
    shell_command: String,
    child: Option<Child>,
    format: AudioFormat,
}

impl Open for SubprocessSink {
    fn open(shell_command: Option<String>, format: AudioFormat) -> SubprocessSink {
        info!("Using subprocess sink with format: {:?}", format);

        if let Some(shell_command) = shell_command {
            SubprocessSink {
                shell_command: shell_command,
                child: None,
                format: format,
            }
        } else {
            panic!("subprocess sink requires specifying a shell command");
        }
    }
}

impl Sink for SubprocessSink {
    fn start(&mut self) -> io::Result<()> {
        let args = split(&self.shell_command).unwrap();
        self.child = Some(
            Command::new(&args[0])
                .args(&args[1..])
                .stdin(Stdio::piped())
                .spawn()?,
        );
        Ok(())
    }

    fn stop(&mut self) -> io::Result<()> {
        if let Some(child) = &mut self.child.take() {
            child.kill()?;
            child.wait()?;
        }
        Ok(())
    }

    sink_as_bytes!();
}

impl SinkAsBytes for SubprocessSink {
    fn write_bytes(&mut self, data: &[u8]) -> io::Result<()> {
        if let Some(child) = &mut self.child {
            let child_stdin = child.stdin.as_mut().unwrap();
            child_stdin.write_all(data)?;
            child_stdin.flush()?;
        }
        Ok(())
    }
}
