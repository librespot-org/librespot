use librespot::{
    connect::{ConnectConfig, LoadRequest, LoadRequestOptions, Spirc},
    core::{
        authentication::Credentials, cache::Cache, config::SessionConfig, session::Session, Error,
    },
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
async fn main() -> Result<(), Error> {
    env_logger::builder()
        .filter_module("librespot", LevelFilter::Debug)
        .init();

    let session_config = SessionConfig::default();
    let player_config = PlayerConfig::default();
    let audio_format = AudioFormat::default();
    let connect_config = ConnectConfig::default();
    let mixer_config = MixerConfig::default();
    let request_options = LoadRequestOptions::default();

    let sink_builder = audio_backend::find(None).unwrap();
    let mixer_builder = mixer::find(None).unwrap();

    let cache = Cache::new(Some(CACHE), Some(CACHE), Some(CACHE_FILES), None)?;
    let credentials = cache
        .credentials()
        .ok_or(Error::unavailable("credentials not cached"))
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
    let mixer = mixer_builder(mixer_config);

    let player = Player::new(
        player_config,
        session.clone(),
        mixer.get_soft_volume(),
        move || sink_builder(None, audio_format),
    );

    let (spirc, spirc_task) =
        Spirc::new(connect_config, session.clone(), credentials, player, mixer).await?;

    // these calls can be seen as "queued"
    spirc.activate()?;
    spirc.load(LoadRequest::from_context_uri(
        format!("spotify:user:{}:collection", session.username()),
        request_options,
    ))?;
    spirc.play()?;

    // starting the connect device and processing the previously "queued" calls
    spirc_task.await;

    Ok(())
}
