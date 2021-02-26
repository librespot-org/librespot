use super::{Open, Sink};
use crate::audio::AudioPacket;
use shell_words::split;
use std::io::{self, Write};
use std::mem;
use std::process::{Child, Command, Stdio};
use std::slice;

pub struct SubprocessSink {
    shell_command: String,
    child: Option<Child>,
}

impl Open for SubprocessSink {
    fn open(shell_command: Option<String>) -> SubprocessSink {
        if let Some(shell_command) = shell_command {
            SubprocessSink {
                shell_command: shell_command,
                child: None,
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

    fn write(&mut self, packet: &AudioPacket) -> io::Result<()> {
        let data: &[u8] = unsafe {
            slice::from_raw_parts(
                packet.samples().as_ptr() as *const u8,
                packet.samples().len() * mem::size_of::<i16>(),
            )
        };
        if let Some(child) = &mut self.child {
            let child_stdin = child.stdin.as_mut().unwrap();
            child_stdin.write_all(data)?;
        }
        Ok(())
    }
}
