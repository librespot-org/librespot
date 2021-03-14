use std::convert::TryFrom;
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

#[derive(Clone, Copy, Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum AudioFormat {
    F32,
    S32,
    S16,
}

impl TryFrom<&String> for AudioFormat {
    type Error = ();
    fn try_from(s: &String) -> Result<Self, Self::Error> {
        match s.to_uppercase().as_str() {
            "F32" => Ok(AudioFormat::F32),
            "S32" => Ok(AudioFormat::S32),
            "S16" => Ok(AudioFormat::S16),
            _ => unimplemented!(),
        }
    }
}

impl Default for AudioFormat {
    fn default() -> AudioFormat {
        AudioFormat::F32
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

#[derive(Clone, Debug, PartialEq)]
pub enum NormalisationMethod {
    Basic,
    Dynamic,
}

impl FromStr for NormalisationMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "basic" => Ok(NormalisationMethod::Basic),
            "dynamic" => Ok(NormalisationMethod::Dynamic),
            _ => Err(()),
        }
    }
}

impl Default for NormalisationMethod {
    fn default() -> NormalisationMethod {
        NormalisationMethod::Dynamic
    }
}

#[derive(Clone, Debug)]
pub struct PlayerConfig {
    pub bitrate: Bitrate,
    pub normalisation: bool,
    pub normalisation_type: NormalisationType,
    pub normalisation_method: NormalisationMethod,
    pub normalisation_pregain: f32,
    pub normalisation_threshold: f32,
    pub normalisation_attack: f32,
    pub normalisation_release: f32,
    pub normalisation_knee: f32,
    pub gapless: bool,
    pub passthrough: bool,
}

impl Default for PlayerConfig {
    fn default() -> PlayerConfig {
        PlayerConfig {
            bitrate: Bitrate::default(),
            normalisation: false,
            normalisation_type: NormalisationType::default(),
            normalisation_method: NormalisationMethod::default(),
            normalisation_pregain: 0.0,
            normalisation_threshold: -1.0,
            normalisation_attack: 0.005,
            normalisation_release: 0.1,
            normalisation_knee: 1.0,
            gapless: true,
            passthrough: false,
        }
    }
}
