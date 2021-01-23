use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Volume(pub u16);

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
        writer.write_all(self.0.to_string().as_bytes()).unwrap();
    }

    pub(crate) fn save_to_file<P: AsRef<Path>>(&self, path: P) {
        let mut file = File::create(path).unwrap();
        self.save_to_writer(&mut file)
    }
}

impl Display for Volume {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

impl FromStr for Volume {
    type Err = <u16 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.trim().parse().map(Volume)
    }
}

#[cfg(test)]
mod test {
    use super::Volume;

    #[test]
    fn parse_volume() {
        assert_eq!("66".parse::<Volume>(), Ok(Volume(66)));
        assert_eq!("  2392 \n".parse::<Volume>(), Ok(Volume(2392)));
        assert!("-32".parse::<Volume>().is_err());
    }

    #[test]
    fn volume_display() {
        assert_eq!(Volume(2392).to_string(), "2392");
    }
}
