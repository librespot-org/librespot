use std::path::PathBuf;

use url::Url;

#[derive(Clone, Debug)]
pub struct SessionConfig {
    pub device_id: String,
    pub proxy: Option<Url>,
    pub ap_port: Option<u16>,
    pub tmp_dir: PathBuf,
}

impl Default for SessionConfig {
    fn default() -> SessionConfig {
        let device_id = uuid::Uuid::new_v4().to_hyphenated().to_string();
        SessionConfig {
            device_id,
            proxy: None,
            ap_port: None,
            tmp_dir: std::env::temp_dir(),
        }
    }
}
