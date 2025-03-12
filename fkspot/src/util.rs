use std::fs;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[allow(non_snake_case)] // this is for json parsing, ignore naming conventions therefore
pub struct SpotTokenRes {
    pub clientId: String,
    pub accessToken: String,
    pub accessTokenExpirationTimestampMs: i64,
    pub isAnonymous: bool,
}
pub fn read_config() -> (String, String, String) {
    let config_content = fs::read_to_string("fkspot.cfg").expect("Unable to read config");
    let mut sp_t = String::new();
    let mut sp_dc = String::new();
    let mut sp_key = String::new();

    for line in config_content.lines() {
        if line.starts_with("sp_t=") {
            sp_t = line[5..].trim_matches('"').to_string();
        } else if line.starts_with("sp_dc=") {
            sp_dc = line[6..].to_string();
        } else if line.starts_with("sp_key=") {
            sp_key = line[7..].to_string();
        }
    }

    (sp_t, sp_dc, sp_key)
}
