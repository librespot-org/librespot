use super::{Open, Sink};
use std::io::{self, Write};
use std::slice;
use std::mem;


pub struct StdoutSink;//Option<PCM>, String);

impl Open for StdoutSink {
    fn open(_: Option<&str>) -> StdoutSink {
        StdoutSink
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
// http://stackoverflow.com/questions/30838358/writing-vecu16-to-a-file
        let slice_u8: &[u8] = unsafe { slice::from_raw_parts(data.as_ptr() as *const u8, data.len() * mem::size_of::<i16>()) };
		try!(io::stdout().write_all(slice_u8));
		try!(io::stdout().flush());

        Ok(())
    }
}

