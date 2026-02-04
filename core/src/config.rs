use std::path::PathBuf;

use clap::ValueEnum;
use librespot_protocol::devices::DeviceType as ProtoDeviceType;
use serde::{Deserialize, Serialize};
use url::Url;

pub(crate) const KEYMASTER_CLIENT_ID: &str = "65b708073fc0480ea92a077233ca87bd";
pub(crate) const ANDROID_CLIENT_ID: &str = "9a8d2f0ce77a4e248bb71fefcb557637";
pub(crate) const IOS_CLIENT_ID: &str = "58bd3c95768941ea9eb4350aaa033eb3";

// Easily adjust the current platform to mock the behavior on it. If for example
// android or ios needs to be mocked, the `os_version` has to be set to a valid version.
// Otherwise, client-token or login5 requests will fail with a generic invalid-credential error.
/// See [std::env::consts::OS]
pub const OS: &str = std::env::consts::OS;

// valid versions for some os:
// 'android': 30
// 'ios': 17
/// See [sysinfo::System::os_version]
pub fn os_version() -> String {
    sysinfo::System::os_version().unwrap_or("0".into())
}

#[derive(Clone, Debug)]
pub struct SessionConfig {
    pub client_id: String,
    pub device_id: String,
    pub proxy: Option<Url>,
    pub ap_port: Option<u16>,
    pub tmp_dir: PathBuf,
    pub autoplay: Option<bool>,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self::default_for_os(OS)
    }
}

impl SessionConfig {
    pub(crate) fn default_for_os(os: &str) -> Self {
        let client_id = match os {
            "android" => ANDROID_CLIENT_ID,
            "ios" => IOS_CLIENT_ID,
            _ => KEYMASTER_CLIENT_ID,
        }
        .to_owned();
        let device_id = uuid::Uuid::new_v4().as_hyphenated().to_string();

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

#[derive(
    Clone,
    Copy,
    Debug,
    Hash,
    PartialOrd,
    Ord,
    PartialEq,
    Eq,
    Default,
    ValueEnum,
    Serialize,
    Deserialize,
)]
#[clap(rename_all = "lower")]
pub enum DeviceType {
    Unknown = 0,
    Computer = 1,
    Tablet = 2,
    Smartphone = 3,
    #[default]
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
}

impl From<DeviceType> for String {
    fn from(value: DeviceType) -> Self {
        format!("{value:?}")
    }
}

impl From<DeviceType> for ProtoDeviceType {
    fn from(value: DeviceType) -> Self {
        match value {
            DeviceType::Unknown => ProtoDeviceType::UNKNOWN,
            DeviceType::Computer => ProtoDeviceType::COMPUTER,
            DeviceType::Tablet => ProtoDeviceType::TABLET,
            DeviceType::Smartphone => ProtoDeviceType::SMARTPHONE,
            DeviceType::Speaker => ProtoDeviceType::SPEAKER,
            DeviceType::Tv => ProtoDeviceType::TV,
            DeviceType::Avr => ProtoDeviceType::AVR,
            DeviceType::Stb => ProtoDeviceType::STB,
            DeviceType::AudioDongle => ProtoDeviceType::AUDIO_DONGLE,
            DeviceType::GameConsole => ProtoDeviceType::GAME_CONSOLE,
            DeviceType::CastAudio => ProtoDeviceType::CAST_VIDEO,
            DeviceType::CastVideo => ProtoDeviceType::CAST_AUDIO,
            DeviceType::Automobile => ProtoDeviceType::AUTOMOBILE,
            DeviceType::Smartwatch => ProtoDeviceType::SMARTWATCH,
            DeviceType::Chromebook => ProtoDeviceType::CHROMEBOOK,
            DeviceType::UnknownSpotify => ProtoDeviceType::UNKNOWN_SPOTIFY,
            DeviceType::CarThing => ProtoDeviceType::CAR_THING,
            DeviceType::Observer => ProtoDeviceType::OBSERVER,
        }
    }
}
