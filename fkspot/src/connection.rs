use crate::{
    totp,
    util::{self, file_id_from_string},
};
use librespot::core::{
    audio_key::AudioKey, authentication::Credentials, session::Session, FileId, SessionConfig,
    SpotifyId,
};
use log::{debug, info};
use std::time::{SystemTime, UNIX_EPOCH};

const ACCESS_TOKEN_ENDPOINT: &str =
    "https://open.spotify.com/get_access_token?reason=init&productType=web_player";

pub struct Connection {
    pub session: Session,
    pub access_token_expiration_timestamp_ms: i64,
}

impl Connection {
    pub async fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let access_token = self.get_token().await;
        debug!("access_token: {}", access_token);

        let credentials = Credentials::with_access_token(access_token);
        match self.session.connect(credentials, false).await {
            Ok(()) => Ok(()),
            Err(e) => return Err(Box::new(e)),
        }
    }

    async fn get_token(&mut self) -> String {
        let (sp_t, sp_dc, sp_key) = util::read_config();
        let (totp_client, client_time, totp_server, server_time) = totp::get().await;

        let client: reqwest::Client = reqwest::Client::new();
        let final_endpoint: String = format!("{}&totp={totp_client}&totpServer={totp_server}&totpVer=5&cTime={client_time}&sTime={server_time}", ACCESS_TOKEN_ENDPOINT);
        println!("{}", final_endpoint);
        let res: reqwest::Response = client
            .get(final_endpoint)
            .header(
                "Cookie",
                format!("sp_t={}; sp_dc={}; sp_key={}; sp_gaid=0088fcc4d9614c1285a279219845c6cf638b31d8b6f99ab0c8a8a7", sp_t, sp_dc, sp_key),
            )
            .send()
            .await
            .unwrap();

        let res_text = res.text().await.unwrap();
        let res: util::SpotTokenRes = match serde_json::from_str(&res_text) {
            Ok(parsed) => parsed,
            Err(_) => {
                panic!("Failed to parse JSON: {}", res_text);
            }
        };

        if res.isAnonymous {
            panic!("Invalid credentials")
        }

        self.access_token_expiration_timestamp_ms = res.accessTokenExpirationTimestampMs;

        println!("{:?}", res);
        res.accessToken
    }

    pub async fn get_audio_key(
        &mut self,
        track_id: &str,
        file_id: &str,
    ) -> Result<AudioKey, Box<dyn std::error::Error>> {
        // compare current time to expiration time
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as i64;

        if current_time >= self.access_token_expiration_timestamp_ms {
            info!("Session expired, renewing..");
            self.session.shutdown();
            self.session = Session::new(SessionConfig::default(), None);
            self.init().await.expect("Failed to renew session");
        }

        debug!(
            "current time: {}, expiration time: {}",
            current_time, self.access_token_expiration_timestamp_ms
        );

        let spot_id: SpotifyId =
            match SpotifyId::from_uri(format!("spotify:track:{}", track_id).as_str()) {
                Ok(id) => id,
                Err(_) => return Err("Invalid Track ID".into()),
            };

        let file_id: FileId = match file_id_from_string(file_id) {
            Ok(id) => id,
            Err(_) => return Err("Invalid file ID".into()),
        };

        let aud_key: AudioKey = self.session.audio_key().request(spot_id, file_id).await?;
        debug!("{:?}", spot_id);
        debug!("{:?}", file_id);
        debug!("{:?}", aud_key);

        Ok(aud_key)
    }
}
