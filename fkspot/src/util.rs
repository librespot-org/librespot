use std::fs;

// Spotify's access token response format
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
            // sp_t's length is 5
            sp_t = line[5..].trim_matches('"').to_string();
        } else if line.starts_with("sp_dc=") {
            // sp_dc's length is 6
            sp_dc = line[6..].trim_matches('"').to_string();
        } else if line.starts_with("sp_key=") {
            // sp_key's length is 7
            sp_key = line[7..].trim_matches('"').to_string();
        }
    }

    (sp_t, sp_dc, sp_key)
}
