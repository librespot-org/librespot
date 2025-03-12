const PORT: u16 = 3745;

mod util;
use actix_web::error::ErrorBadRequest;
// Imports
use actix_web::{get, web, App, HttpServer};
use librespot::core::{
    audio_key::AudioKey, authentication::Credentials, config::SessionConfig, session::Session,
    FileId, SpotifyId,
};
use std::{
    fs,
    sync::{LazyLock, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};



#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[allow(non_snake_case)] // this is for json parsing, ignore naming conventions therefore
struct SpotRes {
    clientId: String,
    accessToken: String,
    accessTokenExpirationTimestampMs: i64,
    isAnonymous: bool,
}

fn read_config() -> (String, String, String) {
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

struct Connection {
    session: Session,
    access_token_expiration_timestamp_ms: i64,
}

static GLOBAL_CONN: LazyLock<Mutex<Connection>> = LazyLock::new(|| {
    Mutex::new(Connection {
        session: Session::new(SessionConfig::default(), None),
        access_token_expiration_timestamp_ms: 0,
    })
});

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
        let (sp_t, sp_dc, sp_key) = read_config();
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
            .json::<SpotRes>()
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

// Web server implementation starts here

#[get("/audiokey/{track_plus_file}")]
async fn audio_key(
    track_plus_file: web::Path<String>,
) -> Result<actix_web::web::Bytes, actix_web::Error> {
    // split the track plus file by an asterik (*)
    let spl: std::str::Split<'_, &str> = track_plus_file.split("*");
    let collection: Vec<&str> = spl.collect();

    let key: AudioKey = match GLOBAL_CONN
        .lock()
        .unwrap()
        .get_audio_key(collection[0], collection[1])
        .await
    {
        Ok(key) => key,
        Err(_e) => return Err(ErrorBadRequest("Bad Request")),
    };

    // pass the audio key that was retrieved as raw bytes
    Ok(web::Bytes::copy_from_slice(&key.0))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // intialize on start (not necessary considering we will be intializing
    // even when we don't have a timestamp set and you request a key)
    GLOBAL_CONN.lock().unwrap().init().await;

    // Example usage:
    // let track_id = "5B5M9o7xEcq6FdEeXrByY0";
    // let file_id = "513ec76d1265b56b3035dd21fdb4f43f93fccb5e";
    // let key: AudioKey = GLOBAL_CONN.get_audio_key(track_id, file_id).await;
    // println!("key: {:?}", key);

    println!("Starting web server on http://localhost:{}", PORT);
    HttpServer::new(|| App::new().service(audio_key))
        .bind(("127.0.0.1", PORT))?
        .run()
        .await
}
