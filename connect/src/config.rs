use crate::core::config::DeviceType;

#[derive(Clone, Debug)]
pub struct ConnectConfig {
    pub name: String,
    pub device_type: DeviceType,
    pub initial_volume: Option<u16>,
    pub has_volume_ctrl: bool,
}

impl Default for ConnectConfig {
    fn default() -> ConnectConfig {
        ConnectConfig {
            name: "Librespot".to_string(),
            device_type: DeviceType::default(),
            initial_volume: Some(50),
            has_volume_ctrl: true,
        }
    }
}
