use std::env;
use std::process;

use librespot::core::authentication::Credentials;
use librespot::core::config::SessionConfig;
use librespot::core::session::Session;
use librespot::core::spotify_id::SpotifyId;
use librespot::metadata::{Metadata, Playlist, Track};

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
        process::exit(1);
    });

    let (session, _) = Session::connect(session_config, credentials, None, false)
        .await
        .unwrap();

    let plist = Playlist::get(&session, plist_uri).await.unwrap();
    println!("{:?}", plist);
    for track_id in plist.tracks {
        let plist_track = Track::get(&session, track_id).await.unwrap();
        println!("track: {} ", plist_track.name);
    }
}
