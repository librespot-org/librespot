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
pub enum NormalisationType {
    Album,
    Track,
}

impl FromStr for NormalisationType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "album" => Ok(NormalisationType::Album),
            "track" => Ok(NormalisationType::Track),
            _ => Err(()),
        }
    }
}

impl Default for NormalisationType {
    fn default() -> NormalisationType {
        NormalisationType::Album
    }
}

#[derive(Clone, Debug)]
pub struct PlayerConfig {
    pub bitrate: Bitrate,
    pub normalisation: bool,
    pub normalisation_type: NormalisationType,
    pub normalisation_pregain: f32,
    pub gapless: bool,
}

impl Default for PlayerConfig {
    fn default() -> PlayerConfig {
        PlayerConfig {
            bitrate: Bitrate::default(),
            normalisation: false,
            normalisation_type: NormalisationType::default(),
            normalisation_pregain: 0.0,
            gapless: true,
        }
    }
}
