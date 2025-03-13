// TODO: Make this PORT constant configurable via fkspot.cfg
const PORT: u16 = 3745;

// Imports
mod connection;
mod util;
use actix_web::error::ErrorBadRequest;
use actix_web::{get, web, App, HttpServer};
use connection::Connection;
use librespot::core::{audio_key::AudioKey, config::SessionConfig, session::Session};
use log::info;
use std::sync::{LazyLock, Mutex};

// Initialize Connection
// We need Mutex to ensure that only one thread can access the connection at a time
// We use LazyLock to ensure that the connection is only initialized once
// This is a global connection that will be used throughout the program
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

    // use the global connecton to get the audio key
    let key: AudioKey = match GLOBAL_CONN
        .lock()
        .expect("Couldn't lock Mutex")
        .get_audio_key(collection[0], collection[1])
        .await
    {
        Ok(key) => key,
        Err(e) => return Err(ErrorBadRequest(format!("Bad Request: {}", e.to_string()))),
    };

    // pass the audio key that was retrieved as raw bytes
    Ok(web::Bytes::copy_from_slice(&key.0))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    // intialize on start (not necessary considering we will be intializing
    // even when we don't have a timestamp set and you request a key)
    match GLOBAL_CONN
        .lock()
        .expect("Couldn't lock Mutex")
        .init()
        .await
    {
        Ok(()) => info!("Successfully initialized session"),
        Err(e) => eprintln!("Failed to initialize session: {:?}", e),
    };

    // Example usage:
    // let track_id = "5B5M9o7xEcq6FdEeXrByY0";
    // let file_id = "513ec76d1265b56b3035dd21fdb4f43f93fccb5e";
    // let key: AudioKey = GLOBAL_CONN.get_audio_key(track_id, file_id).await;
    // println!("key: {:?}", key);

    info!("Starting web server on http://localhost:{}", PORT);
    HttpServer::new(|| App::new().service(audio_key))
        .bind(("127.0.0.1", PORT))?
        .run()
        .await
}
