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
use librespot_connect::spirc::PlayingTrack;
use librespot_connect::{
    spirc::{Spirc, SpircLoadCommand},
    state::ConnectStateConfig,
};
use librespot_metadata::{Album, Metadata};
use librespot_playback::mixer::{softmixer::SoftMixer, Mixer, MixerConfig};
use std::env;
use std::sync::Arc;
use tokio::join;

#[tokio::main]
async fn main() {
    let session_config = SessionConfig::default();
    let player_config = PlayerConfig::default();
    let audio_format = AudioFormat::default();
    let connect_config = ConnectStateConfig::default();

    let mut args: Vec<_> = env::args().collect();
    let context_uri = if args.len() == 3 {
        args.pop().unwrap()
    } else if args.len() == 2 {
        String::from("spotify:album:79dL7FLiJFOO0EoehUHQBv")
    } else {
        eprintln!("Usage: {} ACCESS_TOKEN (ALBUM URI)", args[0]);
        return;
    };

    let credentials = Credentials::with_access_token(&args[1]);
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
        Arc::new(SoftMixer::open(MixerConfig::default())),
    )
    .await
    .unwrap();

    join!(spirc_task, async {
        let album = Album::get(&session, &SpotifyId::from_uri(&context_uri).unwrap())
            .await
            .unwrap();

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
                seek_to: 0,
                shuffle: false,
                repeat: false,
                repeat_track: false,
                // the index specifies which track in the context starts playing, in this case the first in the album
                playing_track: PlayingTrack::Index(0).into(),
            })
            .unwrap();
    });
}
