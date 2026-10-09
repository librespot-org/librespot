use std::{env, process::exit};

use librespot::{
    core::{
        authentication::Credentials, config::SessionConfig, session::Session,
        spotify_uri::SpotifyUri,
    },
    metadata::{Metadata, Playlist},
    playback::{
        audio_backend,
        config::{AudioFormat, PlayerConfig},
        mixer::NoOpVolume,
        player::{Player, PlayerEvent},
    },
};

#[tokio::main]
async fn main() {
    env_logger::init();
    let session_config = SessionConfig::default();
    let player_config = PlayerConfig::default();
    let audio_format = AudioFormat::default();

    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} ACCESS_TOKEN PLAYLIST", args[0]);
        return;
    }
    let credentials = Credentials::with_access_token(&args[1]);

    let plist_uri = SpotifyUri::from_uri(&args[2]).unwrap_or_else(|_| {
        eprintln!(
            "PLAYLIST should be a playlist URI such as: \
                \"spotify:playlist:37i9dQZF1DXec50AjHrNTq\""
        );
        exit(1);
    });

    let backend = audio_backend::find(None).unwrap();

    println!("Connecting...");
    let session = Session::new(session_config, None);
    if let Err(e) = session.connect(credentials, false).await {
        println!("Error connecting: {e}");
        exit(1);
    }

    let plist = Playlist::get(&session, &plist_uri).await.unwrap();
    let tracks: Vec<SpotifyUri> = plist.tracks().cloned().collect();
    if tracks.is_empty() {
        eprintln!("Playlist \"{}\" has no tracks", plist.name());
        exit(1);
    }
    println!(
        "Playing playlist \"{}\" ({} tracks)",
        plist.name(),
        tracks.len()
    );

    let player = Player::new(player_config, session, Box::new(NoOpVolume), move || {
        backend(None, audio_format)
    });

    let mut current = 0usize;
    player.load(tracks[current].clone(), true, 0);

    let mut events = player.get_player_event_channel();
    while let Some(event) = events.recv().await {
        match event {
            PlayerEvent::TimeToPreloadNextTrack { .. } => {
                if let Some(next) = tracks.get(current + 1) {
                    player.preload(next.clone());
                }
            }
            PlayerEvent::EndOfTrack { .. } => {
                if let Some(next) = tracks.get(current + 1) {
                    current += 1;
                    player.load(next.clone(), true, 0);
                } else {
                    println!("Done");
                    break;
                }
            }
            PlayerEvent::Unavailable { track_id, .. } => {
                eprintln!("Track unavailable: {track_id}");
                if let Some(idx) = tracks.iter().position(|t| *t == track_id) {
                    if idx >= current {
                        current = idx + 1;
                    }
                } else {
                    current += 1;
                }
                if let Some(next) = tracks.get(current) {
                    player.load(next.clone(), true, 0);
                } else {
                    println!("Done");
                    break;
                }
            }
            PlayerEvent::TrackChanged { audio_item } => {
                println!("Now playing: {}", audio_item.name);
            }
            _ => (),
        }
    }
}
