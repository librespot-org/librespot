use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::config::VolumeCtrl;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Volume(pub u16);

impl Volume {
    fn calc_logarithmic(self) -> Self {
        // Volume conversion taken from https://www.dr-lex.be/info-stuff/volumecontrols.html#ideal2
        // Convert the given volume [0..0xffff] to a dB gain
        // We assume a dB range of 60dB.
        // Use the equation: a * exp(b * x)
        // in which a = IDEAL_FACTOR, b = 1/1000
        const IDEAL_FACTOR: f64 = 6.908;
        let volume = self.0;
        let normalized_volume = volume as f64 / std::u16::MAX as f64; // To get a value between 0 and 1

        let mut val = std::u16::MAX;
        // Prevent val > std::u16::MAX due to rounding errors
        if normalized_volume < 0.999 {
            let new_volume = (normalized_volume * IDEAL_FACTOR).exp() / 1000.0;
            val = (new_volume * std::u16::MAX as f64) as u16;
        }

        debug!("input volume:{} to mixer: {}", volume, val);

        // return the scale factor (0..0xffff) (equivalent to a voltage multiplier).
        Volume(val)
    }

    pub fn to_mixer_volume(self, volume_ctrl: VolumeCtrl) -> Volume {
        match volume_ctrl {
            VolumeCtrl::Linear | VolumeCtrl::Fixed => self,
            VolumeCtrl::Log => self.calc_logarithmic(),
        }
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

impl Default for Volume {
    fn default() -> Self {
        Volume(0x8000)
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
