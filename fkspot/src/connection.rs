use crate::util;
use std::time::{SystemTime, UNIX_EPOCH};
use librespot::core::{
    audio_key::AudioKey, authentication::Credentials, session::Session, FileId, SpotifyId,
};

pub struct Connection {
    pub session: Session,
    pub access_token_expiration_timestamp_ms: i64,
}

impl Connection {
    pub async fn init(&mut self) {
        let access_token = self.get_token().await;
        println!("access_token: {}", access_token);
        let credentials = Credentials::with_access_token(access_token);
        println!("Connecting with token..");
        self.session
            .connect(credentials, false)
            .await
            .expect("failed to connect with credentials");
    }

    async fn get_token(&mut self) -> String {
        let (sp_t, sp_dc, sp_key) = util::read_config();
        let client = reqwest::Client::new();
        let res = client
            .get(
                "https://open.spotify.com/get_access_token?reason=transport&productType=web_player",
            )
            .header(
                "Cookie",
                format!("sp_t={}; sp_dc={}; sp_key={}", sp_t, sp_dc, sp_key),
            )
            .send()
            .await
            .unwrap()
            .json::<util::SpotTokenRes>()
            .await
            .unwrap();

        if res.isAnonymous {
            panic!("Invalid credentials")
        }

        self.access_token_expiration_timestamp_ms = res.accessTokenExpirationTimestampMs;

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
            self.session.shutdown();
            self.init().await;
        }

        println!(
            "{}, {}",
            current_time, self.access_token_expiration_timestamp_ms
        );
        let spot_id: SpotifyId =
            match SpotifyId::from_uri(format!("spotify:track:{}", track_id).as_str()) {
                Ok(id) => id,
                Err(e) => return Err(Box::new(e)),
            };

        let aud_key: AudioKey = self
            .session
            .audio_key()
            .request(spot_id, FileId::from_raw(file_id.as_bytes()))
            .await?;

        Ok(aud_key)
    }
}
