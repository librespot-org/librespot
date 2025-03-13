use crate::util::{self, file_id_from_string};
use librespot::core::{
    audio_key::AudioKey, authentication::Credentials, session::Session, SessionConfig, SpotifyId,
};
use log::{debug, info};
use std::time::{SystemTime, UNIX_EPOCH};

const ACCESS_TOKEN_ENDPOINT: &str =
    "https://open.spotify.com/get_access_token?reason=transport&productType=web_player";

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
        let client: reqwest::Client = reqwest::Client::new();

        let res: util::SpotTokenRes = client
            .get(ACCESS_TOKEN_ENDPOINT)
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
                Err(e) => return Err(Box::new(e)),
            };

        let aud_key: AudioKey = self
            .session
            .audio_key()
            .request(spot_id, file_id_from_string(file_id))
            .await?;
        debug!("{:?}", spot_id);
        debug!("{:?}", file_id_from_string(file_id));
        debug!("{:?}", aud_key);

        Ok(aud_key)
    }
}
