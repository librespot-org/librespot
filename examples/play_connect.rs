use librespot::{
    core::{
        authentication::Credentials, config::SessionConfig, session::Session, spotify_id::SpotifyId,
    },
    playback::{
        audio_backend,
        config::{AudioFormat, PlayerConfig},
        mixer::NoOpVolume,
        player::Player,
    },
};
use librespot_connect::{
    config::ConnectConfig,
    spirc::{Spirc, SpircLoadCommand},
};
use librespot_metadata::{Album, Metadata};
use librespot_playback::mixer::{softmixer::SoftMixer, Mixer, MixerConfig};
use librespot_protocol::spirc::TrackRef;
use std::env;
use tokio::join;

#[tokio::main]
async fn main() {
    let session_config = SessionConfig::default();
    let player_config = PlayerConfig::default();
    let audio_format = AudioFormat::default();
    let connect_config = ConnectConfig::default();

    let mut args: Vec<_> = env::args().collect();
    let context_uri = if args.len() == 4 {
        args.pop().unwrap()
    } else if args.len() == 3 {
        String::from("spotify:album:79dL7FLiJFOO0EoehUHQBv")
    } else {
        eprintln!("Usage: {} USERNAME PASSWORD (ALBUM URI)", args[0]);
        return;
    };

    let credentials = Credentials::with_password(&args[1], &args[2]);
    let backend = audio_backend::find(None).unwrap();

    println!("Connecting...");
    let session = Session::new(session_config, None);

    let player = Player::new(
        player_config,
        session.clone(),
        Box::new(NoOpVolume),
        move || backend(None, audio_format),
    );

    let (spirc, spirc_task) = Spirc::new(
        connect_config,
        session.clone(),
        credentials,
        player,
        Box::new(SoftMixer::open(MixerConfig::default())),
    )
    .await
    .unwrap();

    join!(spirc_task, async {
        let album = Album::get(&session, &SpotifyId::from_uri(&context_uri).unwrap())
            .await
            .unwrap();
        let tracks = album
            .tracks()
            .map(|track_id| {
                let mut track = TrackRef::new();
                track.set_gid(Vec::from(track_id.to_raw()));
                track
            })
            .collect();

        println!(
            "Playing album: {} by {}",
            &album.name,
            album
                .artists
                .first()
                .map_or("unknown", |artist| &artist.name)
        );

        spirc.activate().unwrap();
        spirc
            .load(SpircLoadCommand {
                context_uri,
                start_playing: true,
                shuffle: false,
                repeat: false,
                playing_track_index: 0, // the index specifies which track in the context starts playing, in this case the first in the album
                tracks,
            })
            .unwrap();
    });
}
