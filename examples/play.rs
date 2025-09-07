use std::{env, process::exit};

use librespot::{
    core::{
        SpotifyUri, authentication::Credentials, config::SessionConfig, session::Session,
        spotify_id::SpotifyId,
    },
    playback::{
        audio_backend,
        config::{AudioFormat, PlayerConfig},
        mixer::NoOpVolume,
        player::Player,
    },
};

#[tokio::main]
async fn main() {
    let session_config = SessionConfig::default();
    let player_config = PlayerConfig::default();
    let audio_format = AudioFormat::default();

    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} ACCESS_TOKEN TRACK", args[0]);
        return;
    }
    let credentials = Credentials::with_access_token(&args[1]);

    let track = SpotifyUri::Track {
        id: SpotifyId::from_base62(&args[2]).unwrap(),
    };

    let backend = audio_backend::find(None).unwrap();

    println!("Connecting...");
    let session = Session::new(session_config, None);
    if let Err(e) = session.connect(credentials, false).await {
        println!("Error connecting: {e}");
        exit(1);
    }

    let player = Player::new(player_config, session, Box::new(NoOpVolume), move || {
        backend(None, audio_format)
    });

    player.load(track, true, 0);

    println!("Playing...");

    player.await_end_of_track().await;

    println!("Done");
}
