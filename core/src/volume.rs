use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Volume(pub u16);

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
