use std::str::FromStr;

#[derive(Clone, Copy, Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum Bitrate {
    Bitrate96,
    Bitrate160,
    Bitrate320,
}

impl FromStr for Bitrate {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "96" => Ok(Bitrate::Bitrate96),
            "160" => Ok(Bitrate::Bitrate160),
            "320" => Ok(Bitrate::Bitrate320),
            _ => Err(()),
        }
    }
}

impl Default for Bitrate {
    fn default() -> Bitrate {
        Bitrate::Bitrate160
    }
}

#[derive(Clone, Debug)]
pub struct PlayerConfig {
    pub bitrate: Bitrate,
    pub normalisation: bool,
    pub normalisation_pregain: f32,
    pub gapless: bool,
    pub passthrough: bool,
}

impl Default for PlayerConfig {
    fn default() -> PlayerConfig {
        PlayerConfig {
            bitrate: Bitrate::default(),
            normalisation: false,
            normalisation_pregain: 0.0,
            gapless: true,
            passthrough: false,
        }
    }
}
