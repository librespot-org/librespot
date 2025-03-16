use crate::util;
use std::time::{SystemTime, UNIX_EPOCH};
use totp_rs::{Algorithm, TOTP, Secret};

const SERVER_TIME_ENDPOINT: &str = "https://open.spotify.com/server-time";
const PASSWORD: &str = "GU2TANZRGQ2TQNJTGQ4DONBZHE2TSMRSGQ4DMMZQGMZDSMZUG4";

async fn get_server_time() -> u64 {
    let res = reqwest::get(SERVER_TIME_ENDPOINT).await.unwrap();

    let res_text = res.text().await.unwrap();
    let res: util::SpotServerTs = match serde_json::from_str(&res_text) {
        Ok(parsed) => parsed,
        Err(_) => {
            panic!("Failed to parse JSON: {}", res_text);
        }
    };

    res.serverTime as u64
}

pub async fn get() -> (String, u64, String, u64) {
    let client_time: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        Secret::Encoded(PASSWORD.to_string()).to_bytes().unwrap(),
    ).unwrap();

    let totp_client: String = totp.generate_current().unwrap();
    println!("{}", totp_client);
    let server_time: u64 = get_server_time().await;
    let totp_server: String = totp.generate(server_time * 1000);

    println!(
        "{} {} {} {}",
        totp_client, client_time * 1000, totp_client, server_time
    );
    (totp_client, client_time * 1000, totp_server, server_time)
}
