use std::{env, process::exit};

use librespot::{
    core::{
        authentication::Credentials, config::SessionConfig, session::Session, spotify_id::SpotifyId,
    },
    metadata::{Metadata, Playlist, Track},
};

#[tokio::main]
async fn main() {
    env_logger::init();
    let session_config = SessionConfig::default();

    let args: Vec<_> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} USERNAME PASSWORD PLAYLIST", args[0]);
        return;
    }
    let credentials = Credentials::with_password(&args[1], &args[2]);

    let plist_uri = SpotifyId::from_uri(&args[3]).unwrap_or_else(|_| {
        eprintln!(
            "PLAYLIST should be a playlist URI such as: \
                \"spotify:playlist:37i9dQZF1DXec50AjHrNTq\""
        );
        exit(1);
    });

    let session = Session::new(session_config, None);
    if let Err(e) = session.connect(credentials).await {
        println!("Error connecting: {}", e);
        exit(1);
    }

    let plist = Playlist::get(&session, plist_uri).await.unwrap();
    println!("{:?}", plist);
    for track_id in plist.tracks() {
        let plist_track = Track::get(&session, track_id).await.unwrap();
        println!("track: {} ", plist_track.name);
    }
}
