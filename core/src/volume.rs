use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

#[derive(Clone, Copy, Debug)]
pub struct Volume {
    pub volume: u16,
}

impl Volume {
    // read volume from file
    fn from_reader<R: Read>(mut reader: R) -> u16 {
        let mut contents = String::new();
        reader.read_to_string(&mut contents).unwrap();
        contents.trim().parse::<u16>().unwrap()
    }

    pub(crate) fn from_file<P: AsRef<Path>>(path: P) -> Option<u16> {
        File::open(path).ok().map(Volume::from_reader)
    }

    // write volume to file
    fn save_to_writer<W: Write>(&self, writer: &mut W) {
        writer.write_all(self.volume.to_string().as_bytes()).unwrap();
    }

    pub(crate) fn save_to_file<P: AsRef<Path>>(&self, path: P) {
        let mut file = File::create(path).unwrap();
        self.save_to_writer(&mut file)
    }
}
