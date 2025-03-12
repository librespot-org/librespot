const PORT: u16 = 3745;

// Imports
mod connection;
mod util;
use actix_web::error::ErrorBadRequest;
use actix_web::{get, web, App, HttpServer};
use connection::Connection;
use librespot::core::{audio_key::AudioKey, config::SessionConfig, session::Session};
use std::sync::{LazyLock, Mutex};

// Initialize Connection
static GLOBAL_CONN: LazyLock<Mutex<Connection>> = LazyLock::new(|| {
    Mutex::new(Connection {
        session: Session::new(SessionConfig::default(), None),
        access_token_expiration_timestamp_ms: 0,
    })
});

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
