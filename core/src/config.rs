use std::fmt;
use std::str::FromStr;
use url::Url;
use uuid::Uuid;

use crate::{version, volume::Volume};

#[derive(Clone, Debug)]
pub struct SessionConfig {
    pub user_agent: String,
    pub device_id: String,
    pub proxy: Option<Url>,
    pub ap_port: Option<u16>,
}

impl Default for SessionConfig {
    fn default() -> SessionConfig {
        let device_id = Uuid::new_v4().to_hyphenated().to_string();
        SessionConfig {
            user_agent: version::version_string(),
            device_id: device_id,
            proxy: None,
            ap_port: None,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum DeviceType {
    Unknown = 0,
    Computer = 1,
    Tablet = 2,
    Smartphone = 3,
    Speaker = 4,
    TV = 5,
    AVR = 6,
    STB = 7,
    AudioDongle = 8,
}

impl FromStr for DeviceType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::DeviceType::*;
        match s.to_lowercase().as_ref() {
            "computer" => Ok(Computer),
            "tablet" => Ok(Tablet),
            "smartphone" => Ok(Smartphone),
            "speaker" => Ok(Speaker),
            "tv" => Ok(TV),
            "avr" => Ok(AVR),
            "stb" => Ok(STB),
            "audiodongle" => Ok(AudioDongle),
            _ => Err(()),
        }
    }
}

impl fmt::Display for DeviceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::DeviceType::*;
        match *self {
            Unknown => f.write_str("Unknown"),
            Computer => f.write_str("Computer"),
            Tablet => f.write_str("Tablet"),
            Smartphone => f.write_str("Smartphone"),
            Speaker => f.write_str("Speaker"),
            TV => f.write_str("TV"),
            AVR => f.write_str("AVR"),
            STB => f.write_str("STB"),
            AudioDongle => f.write_str("AudioDongle"),
        }
    }
}

impl Default for DeviceType {
    fn default() -> DeviceType {
        DeviceType::Speaker
    }
}

#[derive(Clone, Debug)]
pub struct ConnectConfig {
    pub name: String,
    pub device_type: DeviceType,
    pub volume: Volume,
    pub volume_ctrl: VolumeCtrl,
    pub autoplay: bool,
}

#[derive(Copy, Clone, Debug)]
pub enum VolumeCtrl {
    Linear,
    Log,
    Fixed,
}

impl FromStr for VolumeCtrl {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::VolumeCtrl::*;
        match s.to_lowercase().as_ref() {
            "linear" => Ok(Linear),
            "log" => Ok(Log),
            "fixed" => Ok(Fixed),
            _ => Err(()),
        }
    }
}

impl Default for VolumeCtrl {
    fn default() -> VolumeCtrl {
        VolumeCtrl::Log
    }
}
