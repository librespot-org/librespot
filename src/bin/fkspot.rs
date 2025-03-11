// TODO
// Each time a request is made, compare accessTokenExpirationTimestampMs
// with current time so we know if token is expired or not
// if expired, rerun session init

use actix_web::{get, web, App, HttpServer, Responder};
use librespot::core::{authentication::Credentials, config::SessionConfig, session::Session};
use librespot_core::{audio_key::AudioKey, FileId, SpotifyId};
use std::fs;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
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
}

use std::sync::LazyLock;

static GLOBAL_CONN: LazyLock<Connection> = LazyLock::new(|| Connection {
    session: Session::new(SessionConfig::default(), None),
});

impl Connection {
    pub async fn init(&self) {
        let access_token = self.get_token().await;
        println!("access_token: {}", access_token);
        let credentials = Credentials::with_access_token(access_token);
        println!("Connecting with token..");
        self.session
            .connect(credentials, false)
            .await
            .expect("failed to connect with credentials");
    }

    async fn get_token(&self) -> String {
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
            .await;

        // TODO: PANIC IF res.isAnonymous is true
        res.unwrap().accessToken
    }

    pub async fn get_audio_key(&self, track_id: &str, file_id: &str) -> AudioKey {
        let spot_id: SpotifyId =
            SpotifyId::from_uri(format!("spotify:track:{}", track_id).as_str())
                .expect("failed to create spotify id object");

        self.session
            .audio_key()
            .request(spot_id, FileId::from_raw(file_id.as_bytes()))
            .await
            .expect("failed to request audio key")
    }
}

#[get("/audiokey/{track}")]
async fn audio_key(track: web::Path<String>) -> impl Responder {
    let spl = track.split("*");
    let collection: Vec<&str> = spl.collect();
    let key: AudioKey = GLOBAL_CONN
        .get_audio_key(collection[0], collection[1])
        .await;
    web::Bytes::copy_from_slice(&key.0)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    GLOBAL_CONN.init().await;

    // Example usage:
    // let track_id = "5B5M9o7xEcq6FdEeXrByY0";
    // let file_id = "513ec76d1265b56b3035dd21fdb4f43f93fccb5e";
    // let key: AudioKey = GLOBAL_CONN.get_audio_key(track_id, file_id).await;
    // println!("key: {:?}", key);

    HttpServer::new(|| App::new().service(audio_key))
        .bind(("127.0.0.1", 3745))?
        .run()
        .await
}
