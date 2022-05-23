use std::fmt;
use std::str::FromStr;
use url::Url;

#[derive(Clone, Debug)]
pub struct SessionConfig {
    pub user_agent: String,
    pub device_id: String,
    pub proxy: Option<Url>,
    pub ap_port: Option<u16>,
}

impl Default for SessionConfig {
    fn default() -> SessionConfig {
        let device_id = uuid::Uuid::new_v4().as_hyphenated().to_string();
        SessionConfig {
            user_agent: crate::version::VERSION_STRING.to_string(),
            device_id,
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str: &str = self.into();
        f.write_str(str)
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
    pub initial_volume: Option<u16>,
    pub has_volume_ctrl: bool,
    pub autoplay: bool,
}

impl Default for ConnectConfig {
    fn default() -> ConnectConfig {
        ConnectConfig {
            name: "Librespot".to_string(),
            device_type: DeviceType::default(),
            initial_volume: Some(50),
            has_volume_ctrl: true,
            autoplay: false,
        }
    }
}
