use std::io;
use std::cmp::{min, max};

pub struct ZeroFile {
    position: u64,
    size: u64
}

impl ZeroFile {
    pub fn new(size: u64) -> ZeroFile {
        ZeroFile {
            position: 0,
            size: size
        }
    }
}

impl io::Seek for ZeroFile {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        let newpos = match pos {
            io::SeekFrom::Start(offset) => offset as i64,
            io::SeekFrom::End(offset) => self.size as i64 + offset,
            io::SeekFrom::Current(offset) => self.position as i64 + offset,
        };

        self.position = max(min(newpos, self.size as i64), 0) as u64;

        Ok(self.position)
    }
}

impl io::Read for ZeroFile {
    // TODO optimize with memset or similar
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        let len = min(output.len(), (self.size - self.position) as usize);
        for b in output {
            *b = 0;
        }

        self.position += len as u64;
        Ok(len)
    }
}

