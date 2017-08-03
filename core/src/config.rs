use uuid::Uuid;
use std::str::FromStr;
use std::fmt;

use version;

#[derive(Clone,Debug)]
pub struct SessionConfig {
    pub user_agent: String,
    pub device_id: String,
}

impl Default for SessionConfig {
    fn default() -> SessionConfig {
        let device_id = Uuid::new_v4().hyphenated().to_string();
        SessionConfig {
            user_agent: version::version_string(),
            device_id: device_id,
        }
    }
}


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

#[derive(Clone,Debug)]
pub struct PlayerConfig {
    pub bitrate: Bitrate,
    pub onstart: Option<String>,
    pub onstop: Option<String>,
}

impl Default for PlayerConfig {
    fn default() -> PlayerConfig {
        PlayerConfig {
            bitrate: Bitrate::default(),
            onstart: None,
            onstop: None,
        }
    }
}

#[derive(Clone,Debug)]
pub struct ConnectConfig {
    pub name: String,
    pub device_type: DeviceType,
}
