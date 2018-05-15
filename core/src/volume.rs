use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

#[derive(Clone, Copy, Debug)]
pub struct Volume {
    pub volume: i32,
}

impl Volume {
    // read volume from file, enforce upper/lower bounds
    // convert volume from 0..100 to 0..0xFFFF
    fn from_reader<R: Read>(mut reader: R) -> Volume {
        let mut contents = String::new();
        reader.read_to_string(&mut contents).unwrap();
        let volume = contents.trim().parse::<i32>().unwrap();
        if volume > 100 {
            volume = 100;
        }
        Volume {
            volume: volume * 0xFFFF / 100,
        }
    }

    pub(crate) fn from_file<P: AsRef<Path>>(path: P) -> Option<Volume> {
        File::open(path).ok().map(Volume::from_reader)
    }

    // convert volume from 0..0xFFFF to 0..100
    // write to plaintext file
    fn save_to_writer<W: Write>(&self, writer: &mut W) {
        let volume = self.volume * 100 / 0xFFFF;
        writer.write_all(volume.to_string().as_bytes()).unwrap();
    }

    pub(crate) fn save_to_file<P: AsRef<Path>>(&self, path: P) {
        let mut file = File::create(path).unwrap();
        self.save_to_writer(&mut file)
    }
}
