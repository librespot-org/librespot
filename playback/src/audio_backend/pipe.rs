use super::{Open, Sink};
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::mem;
use std::slice;

pub struct StdoutSink(Box<Write>);

impl Open for StdoutSink {
    fn open(path: Option<String>) -> StdoutSink {
        if let Some(path) = path {
            let file = OpenOptions::new().write(true).open(path).unwrap();
            StdoutSink(Box::new(file))
        } else {
            StdoutSink(Box::new(io::stdout()))
        }
    }
}

impl Sink for StdoutSink {
    fn start(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn stop(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn write(&mut self, data: &[i16]) -> io::Result<()> {
        let data: &[u8] = unsafe {
            slice::from_raw_parts(data.as_ptr() as *const u8, data.len() * mem::size_of::<i16>())
        };

        self.0.write_all(data)?;
        self.0.flush()?;

        Ok(())
    }
}
