use std::fs;

use librespot_core::FileId;

// Spotify's access token response format
#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[allow(non_snake_case)] // this is for json parsing, ignore naming conventions therefore
pub struct SpotTokenRes {
    pub clientId: String,
    pub accessToken: String,
    pub accessTokenExpirationTimestampMs: i64,
    pub isAnonymous: bool,
    pub totpValidity: i64,
    pub _notes: String,
}

pub fn file_id_from_string(file_id: &str) -> Result<FileId, Box<dyn std::error::Error>> {
    // Create an array of 20 bytes initialized to 0
    let mut bytes = [0u8; 20];

    // Loop through each pair of characters in the input string
    for i in 0..20 {
        // Extract a substring of 2 characters
        let byte_str = &file_id[i * 2..i * 2 + 2];
        // Convert the substring from a hexadecimal string to a byte and store it in the array
        bytes[i] = match u8::from_str_radix(byte_str, 16) {
            Ok(byte) => byte,
            Err(_) => {
                return Err("Invalid file ID".into());
            }
        };
    }

    // Create a FileId from the byte array and return it
    Ok(FileId::from(&bytes[..]))
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
