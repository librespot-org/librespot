use std::{fmt, path::PathBuf, str::FromStr};

use url::Url;

pub(crate) const KEYMASTER_CLIENT_ID: &str = "65b708073fc0480ea92a077233ca87bd";
pub(crate) const ANDROID_CLIENT_ID: &str = "9a8d2f0ce77a4e248bb71fefcb557637";
pub(crate) const IOS_CLIENT_ID: &str = "58bd3c95768941ea9eb4350aaa033eb3";

#[derive(Clone, Debug)]
pub struct SessionConfig {
    pub client_id: String,
    pub device_id: String,
    pub proxy: Option<Url>,
    pub ap_port: Option<u16>,
    pub tmp_dir: PathBuf,
    pub autoplay: Option<bool>,
}

impl SessionConfig {
    pub(crate) fn default_for_os(os: &str) -> Self {
        let device_id = uuid::Uuid::new_v4().as_hyphenated().to_string();
        let client_id = match os {
            "android" => ANDROID_CLIENT_ID,
            "ios" => IOS_CLIENT_ID,
            _ => KEYMASTER_CLIENT_ID,
        }
        .to_owned();

        Self {
            client_id,
            device_id,
            proxy: None,
            ap_port: None,
            tmp_dir: std::env::temp_dir(),
            autoplay: None,
        }
    }
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self::default_for_os(std::env::consts::OS)
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum DeviceType {
    Unknown = 0,
    Computer = 1,
    Tablet = 2,
    Smartphone = 3,
    Speaker = 4,
    Tv = 5,
    Avr = 6,
    Stb = 7,
    AudioDongle = 8,
    GameConsole = 9,
    CastAudio = 10,
    CastVideo = 11,
    Automobile = 12,
    Smartwatch = 13,
    Chromebook = 14,
    UnknownSpotify = 100,
    CarThing = 101,
    Observer = 102,
    HomeThing = 103,
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
            "tv" => Ok(Tv),
            "avr" => Ok(Avr),
            "stb" => Ok(Stb),
            "audiodongle" => Ok(AudioDongle),
            "gameconsole" => Ok(GameConsole),
            "castaudio" => Ok(CastAudio),
            "castvideo" => Ok(CastVideo),
            "automobile" => Ok(Automobile),
            "smartwatch" => Ok(Smartwatch),
            "chromebook" => Ok(Chromebook),
            "carthing" => Ok(CarThing),
            "homething" => Ok(HomeThing),
            _ => Err(()),
        }
    }
}

impl From<&DeviceType> for &str {
    fn from(d: &DeviceType) -> &'static str {
        use self::DeviceType::*;
        match d {
            Unknown => "Unknown",
            Computer => "Computer",
            Tablet => "Tablet",
            Smartphone => "Smartphone",
            Speaker => "Speaker",
            Tv => "TV",
            Avr => "AVR",
            Stb => "STB",
            AudioDongle => "AudioDongle",
            GameConsole => "GameConsole",
            CastAudio => "CastAudio",
            CastVideo => "CastVideo",
            Automobile => "Automobile",
            Smartwatch => "Smartwatch",
            Chromebook => "Chromebook",
            UnknownSpotify => "UnknownSpotify",
            CarThing => "CarThing",
            Observer => "Observer",
            HomeThing => "HomeThing",
        }
    }
}

impl From<DeviceType> for &str {
    fn from(d: DeviceType) -> &'static str {
        (&d).into()
    }
}

impl fmt::Display for DeviceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str: &str = self.into();
        f.write_str(str)
    }
}

impl Default for DeviceType {
    fn default() -> DeviceType {
        DeviceType::Speaker
    }
}
