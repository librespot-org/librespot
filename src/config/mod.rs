mod parsers;

use clap::{
    Args, CommandFactory, Parser, ValueEnum, builder::NonEmptyStringValueParser, value_parser,
};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use data_encoding::HEXLOWER;
#[cfg(discovery)]
use librespot::discovery::{Discovery, DiscoveryConfig};
use librespot::{
    connect::{ConnectConfig, Spirc},
    core::{
        Session, SessionConfig, authentication::Credentials, cache::Cache, config::DeviceType,
        version,
    },
    oauth::OAuthClientBuilder,
    playback::{
        audio_backend::AudioBackendBuilder,
        config::{AudioFormat, Bitrate, NormalisationMethod, NormalisationType, PlayerConfig},
        dither::DithererBuilder,
        mixer::{
            Mixer, MixerBuilder, MixerConfig, MixerConfigBuilder, VolumeCtrl, VolumeCtrlBuilder,
        },
        player::{Player, duration_to_coefficient},
    },
};
use log::{error, info, trace, warn};
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use std::{
    env, ffi::OsStr, fs::create_dir_all, ops::RangeInclusive, path::PathBuf, pin::Pin,
    process::exit, sync::Arc, time::Duration,
};
use tokio::sync::Semaphore;
use url::Url;

use parsers::range_parser_factory;

use crate::player_event_handler::{EventHandler, run_program_on_sink_events};

/// Spotify's Desktop app uses these. Some of these are only available when requested with Spotify's client IDs.
const OAUTH_SCOPES: &[&str] = &[
    "app-remote-control",
    "playlist-modify",
    "playlist-modify-private",
    "playlist-modify-public",
    "playlist-read",
    "playlist-read-collaborative",
    "playlist-read-private",
    "streaming",
    "ugc-image-upload",
    "user-follow-modify",
    "user-follow-read",
    "user-library-modify",
    "user-library-read",
    "user-modify",
    "user-modify-playback-state",
    "user-modify-private",
    "user-personalized",
    "user-read-birthdate",
    "user-read-currently-playing",
    "user-read-email",
    "user-read-play-history",
    "user-read-playback-position",
    "user-read-playback-state",
    "user-read-private",
    "user-read-recently-played",
    "user-top-read",
];

const VALID_INITIAL_VOLUME_RANGE: RangeInclusive<u16> = 0..=100;
const VALID_VOLUME_RANGE: RangeInclusive<f64> = 0.0..=100.0;
const VALID_NORMALISATION_PREGAIN_RANGE: RangeInclusive<f64> = -10.0..=10.0;
const VALID_NORMALISATION_THRESHOLD_RANGE: RangeInclusive<f64> = -10.0..=0.0;
const VALID_NORMALISATION_ATTACK_RANGE: RangeInclusive<u64> = 1..=500;
const VALID_NORMALISATION_RELEASE_RANGE: RangeInclusive<u64> = 1..=1000;
const VALID_NORMALISATION_KNEE_RANGE: RangeInclusive<f64> = 0.0..=10.0;

// Initialize a static semaphore with only one permit, which is used to
// prevent setting environment variables from running in parallel.
static PERMIT: Semaphore = Semaphore::const_new(1);
pub async fn set_env_var<K: AsRef<OsStr>, V: AsRef<OsStr>>(key: K, value: V) {
    let permit = PERMIT
        .acquire()
        .await
        .expect("Failed to acquire semaphore permit");

    // SAFETY: This is safe because setting the environment variable will wait if the permit is
    // already acquired by other callers.
    unsafe { env::set_var(key, value) }

    // Drop the permit manually, so the compiler doesn't optimize it away as unused variable.
    drop(permit);
}

#[cfg(feature = "pulseaudio-backend")]
async fn set_pulse_audio_env_vars(name: &str) {
    use std::borrow::Cow;

    if env::var("PULSE_PROP_application.name").is_err() {
        let pulseaudio_name: Cow<'_, str> = if name != ConnectConfig::DEFAULT_NAME {
            Cow::Owned(format!("{} - {name}", ConnectConfig::DEFAULT_NAME))
        } else {
            Cow::Borrowed(name)
        };

        set_env_var("PULSE_PROP_application.name", pulseaudio_name.as_ref()).await;
    }

    if env::var("PULSE_PROP_application.version").is_err() {
        set_env_var("PULSE_PROP_application.version", version::SEMVER).await;
    }

    if env::var("PULSE_PROP_application.icon_name").is_err() {
        set_env_var("PULSE_PROP_application.icon_name", "audio-x-generic").await;
    }

    if env::var("PULSE_PROP_application.process.binary").is_err() {
        set_env_var("PULSE_PROP_application.process.binary", "librespot").await;
    }

    if env::var("PULSE_PROP_stream.description").is_err() {
        set_env_var("PULSE_PROP_stream.description", "Spotify Connect endpoint").await;
    }

    if env::var("PULSE_PROP_media.software").is_err() {
        set_env_var("PULSE_PROP_media.software", "Spotify").await;
    }

    if env::var("PULSE_PROP_media.role").is_err() {
        set_env_var("PULSE_PROP_media.role", "music").await;
    }
}

fn about() -> String {
    let version = version::libresport_version();
    let desc = env!("CARGO_PKG_DESCRIPTION");
    let repo_home = env!("CARGO_PKG_REPOSITORY");
    format!("{version}\n\n{desc}\n\n{repo_home}")
}

#[derive(Parser, Serialize, Deserialize)]
#[command(version, author, about=about())]
pub struct Config {
    #[serde(skip_serializing, skip_deserializing)]
    #[command(flatten)]
    pub verbosity: Verbosity<InfoLevel>,

    #[cfg(discovery)]
    /// Disable zeroconf discovery mode.
    #[arg(long, short = 'O', verbatim_doc_comment)]
    disable_discovery: bool,

    #[cfg(discovery)]
    #[command(flatten)]
    discovery_config: DiscoveryConfig,

    /// Perform interactive OAuth sign in.
    #[arg(long, short = 'j', verbatim_doc_comment)]
    enable_oauth: bool,

    /// The port the oauth redirect server uses 1 - 65535.
    /// Ports bellow 1025 may require root privileges.
    #[arg(long, short='K', verbatim_doc_comment, value_parser=value_parser!(u16).range(1..), default_value_t=5588, requires("enable_oauth"))]
    oauth_port: u16,

    /// Username used to sign in with.
    #[arg(long, short = 'u', verbatim_doc_comment, value_parser=NonEmptyStringValueParser::new())]
    username: Option<String>,

    // /// Password used to sign in with.
    // #[arg(long, short = 'p', verbatim_doc_comment)]
    // password: Option<String>,
    /// Spotify access token to sign in with.
    #[arg(long, short = 'k', verbatim_doc_comment, value_parser=NonEmptyStringValueParser::new())]
    access_token: Option<String>,

    #[command(flatten)]
    connect_config: ConnectConfigClap,

    #[command(flatten)]
    session_config: SessionConfigClap,

    #[command(flatten)]
    backend_config: BackendConfig,

    /// Mixer to use.
    #[arg(long, short = 'm', verbatim_doc_comment, value_enum, default_value_t)]
    mixer: MixerBuilder,

    #[cfg(feature = "alsa-backend")]
    #[command(flatten)]
    alsa_mixer: AlsaMixerConfig,

    #[command(flatten)]
    volume_ctrl_config: VolumeCtrlConfig,

    #[command(flatten)]
    player_config: PlayerConfigClap,

    /// Run PROGRAM set by `--onevent`
    /// before the sink is opened and after it is closed.
    #[arg(long, short = 'Q', verbatim_doc_comment)]
    emit_sink_events: bool,

    /// Run PROGRAM when a playback event occurs.
    #[arg(long, short = 'o', verbatim_doc_comment)]
    onevent: Option<String>,

    #[command(flatten)]
    cache_config: CacheConfig,
}

#[derive(Args, Serialize, Deserialize)]
struct ConnectConfigClap {
    /// Device name.
    #[arg(long, short = 'n', verbatim_doc_comment, default_value=ConnectConfig::DEFAULT_NAME)]
    name: String,

    /// Displayed device type.
    #[arg(long, short = 'F', verbatim_doc_comment, value_enum, default_value_t)]
    device_type: DeviceType,

    /// Whether the device represents a group.
    #[arg(long, verbatim_doc_comment)]
    group: bool,

    /// Initial volume in % from 0 to 100. Defaults to 50% if not cached. For the alsa mixer: the current volume.
    #[arg(long, short='R', verbatim_doc_comment, value_parser=range_parser_factory(VALID_INITIAL_VOLUME_RANGE))]
    initial_volume: Option<u16>,

    /// Number of incremental steps when responding to volume control updates.
    #[arg(long, verbatim_doc_comment, value_parser=value_parser!(u16).range(1..), default_value_t=ConnectConfig::DEFAULT_VOLUME_STEPS)]
    volume_steps: u16,
}

#[derive(Args, Serialize, Deserialize)]
struct SessionConfigClap {
    /// Http proxy to use when connecting.
    #[arg(long, short = 'x', verbatim_doc_comment, value_parser=parsers::proxy_parser)]
    proxy: Option<Url>,

    /// Connect to an AP with a specified port 1 - 65535.
    /// Available ports are usually 80, 443 and 4070.
    /// Ports bellow 1025 may require root privileges.
    #[arg(long, short='a', verbatim_doc_comment, value_parser=value_parser!(u16).range(1..))]
    ap_port: Option<u16>,

    /// Path to a directory where files will be temporarily stored while downloading.
    #[arg(long, short = 't', verbatim_doc_comment)]
    temp: Option<PathBuf>,

    /// Explicitly set autoplay.
    /// Defaults to following the client setting.
    #[arg(long, short = 'A', verbatim_doc_comment, value_enum)]
    autoplay: Option<Autoplay>,
}

#[derive(ValueEnum, Clone, Copy, Serialize, Deserialize)]
enum Autoplay {
    On,
    Off,
}

#[derive(Args, Serialize, Deserialize)]
struct BackendConfig {
    /// Audio backend to use.
    #[arg(long, short = 'B', verbatim_doc_comment, value_enum, default_value_t)]
    backend: AudioBackendBuilder,

    /// Audio device to use.
    /// Use ? to list options.
    /// Defaults to the backend's default.
    #[arg(long, short = 'd', verbatim_doc_comment, value_parser=NonEmptyStringValueParser::new())]
    device: Option<String>,

    /// Output audio format.
    #[arg(long, short = 'f', verbatim_doc_comment, value_enum, default_value_t)]
    format: AudioFormat,
}

#[cfg(feature = "alsa-backend")]
#[derive(Args, Serialize, Deserialize)]
struct AlsaMixerConfig {
    /// Alsa index of the cards mixer. Defaults to 0.
    #[arg(long, short = 's', verbatim_doc_comment)]
    alsa_mixer_index: Option<u32>,

    /// Alsa mixer device, e.g hw:0 or similar from `aplay -l`. Defaults to `--device` if specified, default otherwise.
    #[arg(long, short = 'S', verbatim_doc_comment, value_parser=NonEmptyStringValueParser::new())]
    alsa_mixer_device: Option<String>,

    /// Alsa mixer control, e.g. PCM, Master or similar. Defaults to PCM.
    #[arg(long, short = 'T', verbatim_doc_comment, value_parser=NonEmptyStringValueParser::new())]
    alsa_mixer_control: Option<String>,
}

#[cfg(feature = "alsa-backend")]
impl AlsaMixerConfig {
    fn get_index(&self, device: Option<&String>) -> Option<u32> {
        self.alsa_mixer_index.or_else(|| match device {
            // Look for the dev index portion of --device.
            // Specifically <dev index> when --device is <something>:CARD=<card name>,DEV=<dev index>
            // or <something>:<card index>,<dev index>.

            // If --device does not contain a ',' it does not contain a dev index.
            // In the case that the dev index is omitted it is assumed to be 0 (mixer_default_config.index).
            // Malformed --device values will also fallback to mixer_default_config.index.
            Some(ref device_name) if device_name.contains(',') => {
                // Turn <something>:CARD=<card name>,DEV=<dev index> or <something>:<card index>,<dev index>
                // into DEV=<dev index> or <dev index>.
                let dev = &device_name[device_name.find(',').unwrap_or_default()..]
                    .trim_start_matches(',');

                // Turn DEV=<dev index> into <dev index> (noop if it's already <dev index>)
                // and then parse <dev index>.
                // Malformed --device values will fail the parse and fallback to mixer_default_config.index.
                dev[dev.find('=').unwrap_or_default()..]
                    .trim_start_matches('=')
                    .parse::<u32>()
                    .ok()
            }
            _ => None,
        })
    }

    fn get_device(&self, device: Option<&String>) -> Option<String> {
        self.alsa_mixer_device.clone().or_else(|| {
            if let Some(device_name) = device {
                // Look for the card name or card index portion of --device.
                // Specifically <card name> when --device is <something>:CARD=<card name>,DEV=<dev index>
                // or card index when --device is <something>:<card index>,<dev index>.
                // --device values like `pulse`, `default`, `jack` may be valid but there is no way to
                // infer automatically what the mixer should be so they fail auto fallback
                // so --alsa-mixer-device must be manually specified in those situations.
                let start_index = device_name.find(':').unwrap_or_default();

                let end_index = match device_name.find(',') {
                    Some(index) if index > start_index => index,
                    _ => device_name.len(),
                };

                let card = &device_name[start_index..end_index];

                if card.starts_with(':') {
                    // mixers are assumed to be hw:CARD=<card name> or hw:<card index>.
                    return Some("hw".to_owned() + card);
                }
            };
            None
        })
    }
}

#[derive(Args, Serialize, Deserialize)]
struct VolumeCtrlConfig {
    /// Volume control scale type.
    #[arg(long, short = 'E', verbatim_doc_comment, value_enum, default_value_t)]
    volume_ctrl: VolumeCtrlBuilder,

    /// Range of the volume control (dB) from 0.0 to 100.0.
    // #[cfg(not(feature = "alsa-backend"))]
    // TODO: set to 0.0 if using alsa mixer
    // TODO: warn volume range has no effect if volume_ctrl is Fixed or Linear
    #[arg(long, short='e', verbatim_doc_comment, value_parser=range_parser_factory(VALID_VOLUME_RANGE), default_value_t=VolumeCtrl::DEFAULT_DB_RANGE)]
    volume_range: f64,
    // /// Range of the volume control (dB) from 0.0 to 100.0. Default for softvol: 60.0. For the alsa mixer: what the control supports.";
    // #[cfg(feature = "alsa-backend")]
    // #[arg(long, short='e', verbatim_doc_comment, value_parser=&range_parser_factory(VALID_VOLUME_RANGE))]
    // range: f64,
}

impl VolumeCtrlConfig {
    fn build(&self) -> VolumeCtrl {
        self.volume_ctrl.build(self.volume_range)
    }
}

#[derive(Args, Serialize, Deserialize)]
struct PlayerConfigClap {
    /// Bitrate (kbps).
    #[arg(long, short = 'b', verbatim_doc_comment, value_enum, default_value_t)]
    bitrate: Bitrate,

    /// Disable gapless playback.
    #[arg(long, short = 'g', verbatim_doc_comment)]
    disable_gapless: bool,

    /// Pass a raw stream to the output. Only works with the pipe and subprocess backends.
    #[cfg(feature = "passthrough-decoder")]
    #[arg(long, short = 'P', verbatim_doc_comment)]
    passthrough: bool,

    /// Play all tracks at approximately the same apparent volume.
    #[arg(long, short = 'N', verbatim_doc_comment)]
    enable_volume_normalisation: bool,

    #[command(flatten)]
    normalization_config: NormalizationConfig,

    /// Directory to search for local file playback.
    /// Can be specified multiple times to add multiple search directories.
    #[arg(long, short = 'l', verbatim_doc_comment)]
    local_file_dir: Vec<PathBuf>,

    /// Specify the dither algorithm to use.
    #[arg(
        long,
        short='D',
        verbatim_doc_comment,
        value_enum,
        default_value_t = DithererBuilder::default(),
        default_value_ifs([("format", "S32", "none"), ("format", "F32", "none"), ("format", "F64", "none")])
    )]
    dither: DithererBuilder,
}

#[derive(Args, Serialize, Deserialize)]
struct NormalizationConfig {
    /// Specify the normalisation gain type to use.
    #[arg(
        long,
        short = 'W',
        verbatim_doc_comment,
        value_enum,
        default_value_t,
        requires("enable_volume_normalisation")
    )]
    normalisation_gain_type: NormalisationType,

    /// Specify the normalisation method to use.
    #[arg(
        long,
        short = 'X',
        verbatim_doc_comment,
        value_enum,
        default_value_t,
        requires("enable_volume_normalisation")
    )]
    normalisation_method: NormalisationMethod,

    /// Pregain (dB) applied by volume normalisation from -10.0 to 10.0.
    #[arg(long, short='Y', verbatim_doc_comment, value_parser=range_parser_factory(VALID_NORMALISATION_PREGAIN_RANGE), default_value_t=PlayerConfig::DEFAULT_PREGAIN, requires("enable_volume_normalisation"))]
    normalisation_pregain: f64,

    /// Threshold (dBFS) at which point the dynamic limiter engages to prevent clipping from 0.0 to -10.0.
    #[arg(long, short='Z', verbatim_doc_comment, value_parser=range_parser_factory(VALID_NORMALISATION_THRESHOLD_RANGE), default_value_t=PlayerConfig::DEFAULT_THRESHOLD, requires("enable_volume_normalisation"))]
    normalisation_threshold: f64,

    /// Attack time (ms) in which the dynamic limiter reduces gain from 1 to 500.
    #[arg(long, short='U', verbatim_doc_comment, value_parser=range_parser_factory(VALID_NORMALISATION_ATTACK_RANGE), default_value_t=PlayerConfig::DEFAULT_ATTACK, requires("enable_volume_normalisation"))]
    normalisation_attack: u64,

    /// Release or decay time (ms) in which the dynamic limiter restores gain from 1 to 1000.
    #[arg(long, short='y', verbatim_doc_comment, value_parser=range_parser_factory(VALID_NORMALISATION_RELEASE_RANGE), default_value_t=PlayerConfig::DEFAULT_RELEASE, requires("enable_volume_normalisation"))]
    normalisation_release: u64,

    /// Knee width (dB) of the dynamic limiter from 0.0 to 10.0.
    #[arg(long, short='w', verbatim_doc_comment, value_parser=range_parser_factory(VALID_NORMALISATION_KNEE_RANGE), default_value_t=PlayerConfig::DEFAULT_KNEE, requires("enable_volume_normalisation"))]
    normalisation_knee: f64,
}

#[derive(Args, Serialize, Deserialize)]
struct CacheConfig {
    /// Disable caching of the audio data.
    #[arg(long, short = 'G', verbatim_doc_comment)]
    disable_audio_cache: bool,

    /// Disable caching of credentials.
    #[arg(long, short = 'H', verbatim_doc_comment)]
    disable_credential_cache: bool,

    /// Path to a directory where files will be cached after downloading.
    #[arg(long, short = 'c', verbatim_doc_comment)]
    cache: Option<PathBuf>,

    /// Path to a directory where system files (credentials, volume) will be cached.
    /// May be different from the `--cache` option value.
    #[arg(long, short = 'C', verbatim_doc_comment)]
    system_cache: Option<PathBuf>,

    /// Limits the size of the cache for audio files.
    /// It's possible to use suffixes like K, M or G, e.g. 16G for example.
    #[arg(long, short='M', verbatim_doc_comment, value_parser=parsers::parse_file_size)]
    cache_size_limit: Option<u64>,
}

impl Config {
    /// Parse configuration and instantiate [Setup].
    pub async fn setup() -> Setup {
        let conf = Config::parse(); // TODO: add env var parsing, config file.

        let mut conf_cmd = Config::command();

        let mut env_logger_builder = env_logger::Builder::new();
        match env::var("RUST_LOG") {
            Ok(config) => {
                if conf.verbosity.is_present() {
                    warn!("Config verbosity flag overidden by `RUST_LOG` environment variable");
                };
                env_logger_builder.parse_filters(&config)
            }
            Err(_) => env_logger_builder.filter_level(conf.verbosity.log_level_filter()),
        }
        .init();

        info!("{}", version::libresport_version());

        let enable_oauth = conf.enable_oauth;

        let cache = {
            let audio_dir = if conf.cache_config.disable_audio_cache {
                None
            } else {
                conf.cache_config.cache.as_ref().map(|p| p.join("files"))
            };

            let volume_dir = conf.cache_config.system_cache.or(conf.cache_config.cache);

            let cred_dir = if conf.cache_config.disable_credential_cache {
                None
            } else {
                volume_dir.as_ref()
            };

            let limit = if audio_dir.is_some() {
                conf.cache_config.cache_size_limit
            } else {
                None
            };

            // if audio_dir.is_none() && limit.is_some() {
            //     warn!(
            //         "Without a `--{CACHE}` / `-{CACHE_SHORT}` path, and/or if the `--{DISABLE_AUDIO_CACHE}` / `-{DISABLE_AUDIO_CACHE_SHORT}` flag is set, `--{CACHE_SIZE_LIMIT}` / `-{CACHE_SIZE_LIMIT_SHORT}` has no effect."
            //     );
            // }

            let cache = match Cache::new(cred_dir, volume_dir.as_ref(), audio_dir.as_ref(), limit) {
                Ok(cache) => Some(cache),
                Err(e) => {
                    warn!("Cannot create cache: {e}");
                    None
                }
            };

            if enable_oauth && (cache.is_none() || cred_dir.is_none()) {
                warn!("Credential caching is unavailable, but advisable when using OAuth login.");
            }

            cache
        };

        let credentials = {
            let cached_creds = cache.as_ref().and_then(Cache::credentials).map(Arc::new);
            if let Some(access_token) = conf.access_token {
                Some(Arc::new(Credentials::with_access_token(access_token)))
            } else if let Some(username) = conf.username {
                match cached_creds {
                    Some(creds) if Some(username) == creds.username => {
                        trace!("Using cached credentials for specified username.");
                        Some(creds)
                    }
                    _ => {
                        trace!("No cached credentials for specified username.");
                        None
                    }
                }
            } else {
                if cached_creds.is_some() {
                    trace!("Using cached credentials.");
                }
                cached_creds
            }
        };

        #[cfg(discovery)]
        let discovery_config = if !conf.disable_discovery {
            Some(conf.discovery_config)
        } else {
            None
        };

        #[cfg(discovery)]
        if credentials.is_none() && discovery_config.is_none() && !enable_oauth {
            conf_cmd
                .error(
                    clap::error::ErrorKind::MissingRequiredArgument,
                    "Access token is required if discovery and oauth login are disabled.\nEither remove disable discovery or add enable oauth flags",
                )
                .exit();
        }

        #[cfg(not(discovery))]
        if credentials.is_none() && !enable_oauth {
            conf_cmd
                .error(
                    clap::error::ErrorKind::MissingRequiredArgument,
                    "Access token is required if oauth login is disabled.\nPlease use --enable-oauth flag", // TODO: flag names?
                )
                .exit();
        }

        let backend_config = conf.backend_config;
        // device help
        if matches!(backend_config.device.as_deref(), Some("?")) {
            backend_config.backend.device_options()
        }

        let player_config = {
            let bitrate = conf.player_config.bitrate;

            let gapless = !conf.player_config.disable_gapless;

            #[cfg(feature = "passthrough-decoder")]
            let passthrough = conf.passthrough;
            #[cfg(not(feature = "passthrough-decoder"))]
            let passthrough = false;

            let normalisation = conf.player_config.enable_volume_normalisation;

            let normalisation_method = conf.player_config.normalization_config.normalisation_method;
            let normalisation_type = conf
                .player_config
                .normalization_config
                .normalisation_gain_type;
            let normalisation_pregain_db = conf
                .player_config
                .normalization_config
                .normalisation_pregain;
            let normalisation_threshold_dbfs = conf
                .player_config
                .normalization_config
                .normalisation_threshold;
            let normalisation_attack_cf = duration_to_coefficient(Duration::from_millis(
                conf.player_config.normalization_config.normalisation_attack,
            ));
            let normalisation_release_cf = duration_to_coefficient(Duration::from_millis(
                conf.player_config
                    .normalization_config
                    .normalisation_release,
            ));
            let normalisation_knee_db = conf.player_config.normalization_config.normalisation_knee;

            let ditherer_builder = conf.player_config.dither;
            let format = backend_config.format;
            let ditherer_builder = if matches!(format, AudioFormat::F64 | AudioFormat::F32)
                && !matches!(ditherer_builder, DithererBuilder::None)
            {
                conf_cmd
                    .error(
                        clap::error::ErrorKind::InvalidValue,
                        format!("Dithering is not available with format: {format:?}."),
                    )
                    .exit();
            } else {
                ditherer_builder
            };

            let local_file_directories = conf.player_config.local_file_dir;

            PlayerConfig {
                bitrate,
                gapless,
                passthrough,
                normalisation,
                normalisation_type,
                normalisation_method,
                normalisation_pregain_db,
                normalisation_threshold_dbfs,
                normalisation_attack_cf,
                normalisation_release_cf,
                normalisation_knee_db,
                ditherer_builder,
                position_update_interval: None,
                local_file_directories,
            }
        };

        let mixer_config = {
            let mut mixer_config = MixerConfigBuilder::default();

            #[cfg(feature = "alsa-backend")]
            {
                // TODO: warn alsa mixer options will not have any effect if not using alsa mixer
                if matches!(conf.mixer, MixerBuilder::Alsa) {
                    let device = backend_config.device.as_ref();

                    if let Some(mixer_device) = conf.alsa_mixer.get_device(device) {
                        mixer_config.device(mixer_device);
                    }

                    if let Some(control) = conf.alsa_mixer.alsa_mixer_control {
                        mixer_config.control(control);
                    }

                    if let Some(index) = conf.alsa_mixer.get_index(device) {
                        mixer_config.index(index);
                    }
                }
            }
            mixer_config.volume_ctrl(conf.volume_ctrl_config.build());

            // Should never fail as defaults to all fields are provided.
            mixer_config.build().unwrap()
        };

        let connect_config = {
            let name = conf.connect_config.name;

            #[cfg(feature = "pulseaudio-backend")]
            set_pulse_audio_env_vars(&name).await;

            let initial_volume = if let Some(initial_volume) =
                conf.connect_config.initial_volume.map(|initial_volume| {
                    (initial_volume as f32 / 100.0 * VolumeCtrl::MAX_VOLUME as f32) as u16
                }) {
                initial_volume
            } else if !conf.mixer.is_alsa()
                && let Some(initial_volume) = cache.as_ref().and_then(Cache::volume)
            {
                initial_volume
            } else {
                ConnectConfig::DEFAULT_INITIAL_VOLUME
            };

            ConnectConfig {
                name,
                device_type: conf.connect_config.device_type,
                is_group: conf.connect_config.group,
                initial_volume,
                disable_volume: matches!(mixer_config.volume_ctrl, VolumeCtrl::Fixed),
                volume_steps: conf.connect_config.volume_steps,
                emit_set_queue_events: false,
            }
        };

        let session_config = {
            let mut session_config = SessionConfig {
                device_id: HEXLOWER.encode(&Sha1::digest(connect_config.name.as_bytes())),
                proxy: conf.session_config.proxy,
                ap_port: conf.session_config.ap_port,
                ..Default::default()
            };

            if let Some(temp_dir) = conf.session_config.temp.inspect(|tmp_dir| {
                if let Err(e) = create_dir_all(tmp_dir) {
                    conf_cmd
                        .error(
                            clap::error::ErrorKind::Io,
                            format!("Could not create or access specified tmp directory: {e}"),
                        )
                        .exit();
                }
            }) {
                session_config.tmp_dir = temp_dir
            };

            // #1046: not all connections are supplied an `autoplay` user attribute to run statelessly.
            // This knob allows for a manual override.
            if let Some(autoplay_value) = conf.session_config.autoplay {
                match autoplay_value {
                    Autoplay::On => session_config.autoplay = Some(true),
                    Autoplay::Off => session_config.autoplay = Some(false),
                };
            };
            session_config
        };

        Setup {
            backend_config,
            mixer: conf.mixer,
            mixer_config,
            enable_oauth,
            oauth_port: conf.oauth_port,
            credentials,
            #[cfg(discovery)]
            discovery_config,
            connect_config,
            session: Session::new(session_config, cache),
            player_config,
            emit_sink_events: conf.emit_sink_events,
            player_event_program: conf.onevent,
        }
    }
}

pub struct Setup {
    backend_config: BackendConfig,
    mixer: MixerBuilder,
    mixer_config: MixerConfig,
    enable_oauth: bool,
    oauth_port: u16,
    credentials: Option<Arc<Credentials>>,
    #[cfg(discovery)]
    discovery_config: Option<DiscoveryConfig>,
    connect_config: ConnectConfig,
    session: Session,
    player_config: PlayerConfig,
    emit_sink_events: bool,
    player_event_program: Option<String>,
}

impl Setup {
    pub fn session(&self) -> Session {
        self.session.clone()
    }

    #[cfg(discovery)]
    pub async fn get_discovery(&self) -> Option<Discovery> {
        if let Some(discovery_config) = &self.discovery_config {
            use log::debug;
            use sysinfo::System;
            const DISCOVERY_RETRY_TIMEOUT: Duration = Duration::from_secs(10);
            let mut sys = System::new();

            // When started at boot as a service discovery may fail due to it
            // trying to bind to interfaces before the network is actually up.
            // This could be prevented in systemd by starting the service after
            // network-online.target but it requires that a wait-online.service is
            // also enabled which is not always the case since a wait-online.service
            // can potentially hang the boot process until it times out in certain situations.
            // This allows for discovery to retry every 10 secs in the 1st min of uptime
            // before giving up thus papering over the issue and not holding up the boot process.
            loop {
                let device_id = self.session.device_id().to_string();
                let client_id = self.session.client_id();

                match librespot::discovery::Discovery::builder(
                    self.connect_config.name.clone(),
                    device_id,
                    client_id,
                    Some(discovery_config),
                )
                .device_type(self.connect_config.device_type)
                .is_group(self.connect_config.is_group)
                .build()
                {
                    Ok(d) => return Some(d),
                    Err(e) => {
                        use sysinfo::ProcessesToUpdate;

                        sys.refresh_processes(ProcessesToUpdate::All, true);

                        if System::uptime() <= 1 {
                            use log::debug;

                            debug!("Retrying to initialise discovery: {e}");
                            tokio::time::sleep(DISCOVERY_RETRY_TIMEOUT).await;
                        } else {
                            debug!("System uptime > 1 min, not retrying to initialise discovery");
                            warn!("Could not initialise discovery: {e}");
                            return None;
                        }
                    }
                }
            }
        } else {
            None
        }
    }

    pub fn get_credentials(&self, connecting: &mut bool) -> Option<Arc<Credentials>> {
        if let Some(credentials) = self.credentials.clone() {
            *connecting = true;
            Some(credentials)
        } else if self.enable_oauth {
            let client = OAuthClientBuilder::new(
                self.session.client_id().as_str(),
                &format!("http://127.0.0.1:{}/login", self.oauth_port),
                OAUTH_SCOPES.to_vec(),
            )
            .open_in_browser()
            .build()
            .unwrap_or_else(|e| {
                error!("Failed to create OAuth client: {e}");
                exit(1);
            });
            let oauth_token = client.get_access_token().unwrap_or_else(|e| {
                error!("Failed to get Spotify access token: {e}");
                exit(1);
            });
            *connecting = true;
            Some(Arc::new(Credentials::with_access_token(
                oauth_token.access_token,
            )))
        } else {
            None
        }
    }

    pub fn get_mixer(&self) -> Arc<dyn Mixer> {
        match self.mixer.build(self.mixer_config.clone()) {
            Ok(mixer) => mixer,
            Err(why) => {
                error!("{why}");
                exit(1)
            }
        }
    }

    pub fn get_player(&self, mixer: Arc<dyn Mixer>, session: Session) -> Arc<Player> {
        let player_config = self.player_config.clone();
        let soft_volume = mixer.get_soft_volume();
        let format = self.backend_config.format;
        let backend = self.backend_config.backend;
        let device = self.backend_config.device.clone();
        Player::new(player_config, session, soft_volume, move || {
            backend.build(device, format)
        })
    }

    pub fn get_player_event_handler(&self, player: Arc<Player>) -> Option<EventHandler> {
        if let Some(player_event_program) = self.player_event_program.clone() {
            let handler = Some(EventHandler::new(
                player.get_player_event_channel(),
                &player_event_program,
            ));
            if self.emit_sink_events {
                player.set_sink_event_callback(Some(Box::new(move |sink_status| {
                    run_program_on_sink_events(sink_status, &player_event_program)
                })));
            }
            handler
        } else {
            None
        }
    }

    pub async fn get_spirc(
        &self,
        session: Session,
        credentials: Credentials,
        player: Arc<Player>,
        mixer: Arc<dyn Mixer>,
    ) -> (
        Option<Spirc>,
        Option<Pin<Box<impl Future<Output = ()> + 'static>>>,
    ) {
        let connect_config = self.connect_config.clone();
        let (spirc_, spirc_task_) =
            match Spirc::new(connect_config, session, credentials, player, mixer).await {
                Ok((spirc_, spirc_task_)) => (spirc_, spirc_task_),
                Err(e) => {
                    error!("could not initialize spirc: {e}");
                    exit(1);
                }
            };
        (Some(spirc_), Some(Box::pin(spirc_task_)))
    }
}
