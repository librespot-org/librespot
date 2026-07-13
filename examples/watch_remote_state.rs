//! Demonstrates observing Spotify Connect playback/device state from `Spirc`,
//! independent of whether this device is the one actively playing. This is the
//! pattern an MPRIS bridge (e.g. spotifyd) would use: watch channels give you
//! the current snapshot, broadcast channels tell you when something changed
//! and why.

use librespot::{
    connect::{ConnectConfig, Spirc},
    core::{authentication::Credentials, cache::Cache, config::SessionConfig, session::Session},
    playback::mixer::MixerConfig,
    playback::{
        audio_backend,
        config::{AudioFormat, PlayerConfig},
        mixer,
        player::Player,
    },
};

use log::LevelFilter;

const CACHE: &str = ".cache";
const CACHE_FILES: &str = ".cache/files";

#[tokio::main]
async fn main() -> Result<(), librespot::core::Error> {
    env_logger::builder()
        .filter_module("librespot", LevelFilter::Info)
        .init();

    let session_config = SessionConfig::default();
    let player_config = PlayerConfig::default();
    let audio_format = AudioFormat::default();
    let connect_config = ConnectConfig::default();
    let mixer_config = MixerConfig::default();

    let sink_builder = audio_backend::find(None).unwrap();
    let mixer_builder = mixer::find(None).unwrap();

    let cache = Cache::new(Some(CACHE), Some(CACHE), Some(CACHE_FILES), None)?;
    let credentials = cache
        .credentials()
        .ok_or(librespot::core::Error::unavailable(
            "credentials not cached",
        ))
        .or_else(|_| {
            librespot_oauth::OAuthClientBuilder::new(
                &session_config.client_id,
                "http://127.0.0.1:8898/login",
                vec!["streaming"],
            )
            .open_in_browser()
            .build()?
            .get_access_token()
            .map(|t| Credentials::with_access_token(t.access_token))
        })?;

    let session = Session::new(session_config, Some(cache));
    let mixer = mixer_builder(mixer_config)?;

    let player = Player::new(
        player_config,
        session.clone(),
        mixer.get_soft_volume(),
        move || sink_builder(None, audio_format),
    );

    let (spirc, spirc_task) =
        Spirc::new(connect_config, session.clone(), credentials, player, mixer).await?;

    // watch channels: current state snapshots
    let mut cluster_state = spirc.watch_cluster_state();
    let mut player_state = spirc.watch_player_state();
    let mut queue_list = spirc.watch_queue_list();

    // broadcast channels: lightweight "something changed, here's why" notifications
    let mut cluster_updates = spirc.get_cluster_update_channel();
    let mut player_updates = spirc.get_player_update_channel();
    let mut queue_updates = spirc.get_queue_update_channel();

    tokio::spawn(async move {
        loop {
            tokio::select! {
                Ok(()) = cluster_state.changed() => {
                    let state = cluster_state.borrow_and_update();
                    println!(
                        "cluster snapshot: {} device(s), active={:?}",
                        state.devices.len(),
                        state.active_device_id
                    );
                }
                Ok(event) = cluster_updates.recv() => {
                    println!("cluster update: {event:?}");
                }
                Ok(()) = player_state.changed() => {
                    let state = player_state.borrow_and_update();
                    if let Some(state) = state.as_ref() {
                        let track = state.track.as_ref().map(|t| t.uri.as_str()).unwrap_or("<none>");
                        println!("player snapshot: track={track} playing={}", state.is_playing);
                    }
                }
                Ok(event) = player_updates.recv() => {
                    println!("player update: {event:?}");
                }
                Ok(()) = queue_list.changed() => {
                    let queue = queue_list.borrow_and_update();
                    println!(
                        "queue snapshot: {} prev, {} next",
                        queue.prev_tracks.len(),
                        queue.next_tracks.len()
                    );
                }
                Ok(event) = queue_updates.recv() => {
                    println!("queue update: {event:?}");
                }
                else => break,
            }
        }
    });

    // this device stays passive; it only observes remote Connect state
    spirc_task.await;

    Ok(())
}
