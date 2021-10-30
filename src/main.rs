use futures_util::{future, FutureExt, StreamExt};
use librespot_playback::player::PlayerEvent;
use log::{error, info, warn};
use sha1::{Digest, Sha1};
use thiserror::Error;
use tokio::sync::mpsc::UnboundedReceiver;
use url::Url;

use librespot::connect::spirc::Spirc;
use librespot::core::authentication::Credentials;
use librespot::core::cache::Cache;
use librespot::core::config::{ConnectConfig, DeviceType, SessionConfig};
use librespot::core::session::Session;
use librespot::core::version;
use librespot::playback::audio_backend::{self, SinkBuilder, BACKENDS};
use librespot::playback::config::{
    AudioFormat, Bitrate, NormalisationMethod, NormalisationType, PlayerConfig, VolumeCtrl,
};
use librespot::playback::dither;
#[cfg(feature = "alsa-backend")]
use librespot::playback::mixer::alsamixer::AlsaMixer;
use librespot::playback::mixer::mappings::MappedCtrl;
use librespot::playback::mixer::{self, MixerConfig, MixerFn};
use librespot::playback::player::{db_to_ratio, Player};

mod player_event_handler;
use player_event_handler::{emit_sink_event, run_program_on_events};

use std::env;
use std::io::{stderr, Write};
use std::path::Path;
use std::pin::Pin;
use std::process::exit;
use std::str::FromStr;
use std::time::Duration;
use std::time::Instant;

fn device_id(name: &str) -> String {
    hex::encode(Sha1::digest(name.as_bytes()))
}

fn usage(program: &str, opts: &getopts::Options) -> String {
    let brief = format!("Usage: {} [options]", program);
    opts.usage(&brief)
}

fn setup_logging(verbose: bool) {
    let mut builder = env_logger::Builder::new();
    match env::var("RUST_LOG") {
        Ok(config) => {
            builder.parse_filters(&config);
            builder.init();

            if verbose {
                warn!("`--verbose` flag overidden by `RUST_LOG` environment variable");
            }
        }
        Err(_) => {
            if verbose {
                builder.parse_filters("libmdns=info,librespot=trace");
            } else {
                builder.parse_filters("libmdns=info,librespot=info");
            }
            builder.init();
        }
    }
}

fn list_backends() {
    println!("Available backends : ");
    for (&(name, _), idx) in BACKENDS.iter().zip(0..) {
        if idx == 0 {
            println!("- {} (default)", name);
        } else {
            println!("- {}", name);
        }
    }
}

pub fn get_credentials<F: FnOnce(&String) -> Option<String>>(
    username: Option<String>,
    password: Option<String>,
    cached_credentials: Option<Credentials>,
    prompt: F,
) -> Option<Credentials> {
    if let Some(username) = username {
        if let Some(password) = password {
            return Some(Credentials::with_password(username, password));
        }

        match cached_credentials {
            Some(credentials) if username == credentials.username => Some(credentials),
            _ => {
                let password = prompt(&username)?;
                Some(Credentials::with_password(username, password))
            }
        }
    } else {
        cached_credentials
    }
}

#[derive(Debug, Error)]
pub enum ParseFileSizeError {
    #[error("empty argument")]
    EmptyInput,
    #[error("invalid suffix")]
    InvalidSuffix,
    #[error("invalid number: {0}")]
    InvalidNumber(#[from] std::num::ParseFloatError),
    #[error("non-finite number specified")]
    NotFinite(f64),
}

pub fn parse_file_size(input: &str) -> Result<u64, ParseFileSizeError> {
    use ParseFileSizeError::*;

    let mut iter = input.chars();
    let mut suffix = iter.next_back().ok_or(EmptyInput)?;
    let mut suffix_len = 0;

    let iec = matches!(suffix, 'i' | 'I');

    if iec {
        suffix_len += 1;
        suffix = iter.next_back().ok_or(InvalidSuffix)?;
    }

    let base: u64 = if iec { 1024 } else { 1000 };

    suffix_len += 1;
    let exponent = match suffix.to_ascii_uppercase() {
        '0'..='9' if !iec => {
            suffix_len -= 1;
            0
        }
        'K' => 1,
        'M' => 2,
        'G' => 3,
        'T' => 4,
        'P' => 5,
        'E' => 6,
        'Z' => 7,
        'Y' => 8,
        _ => return Err(InvalidSuffix),
    };

    let num = {
        let mut iter = input.chars();

        for _ in (&mut iter).rev().take(suffix_len) {}

        iter.as_str().parse::<f64>()?
    };

    if !num.is_finite() {
        return Err(NotFinite(num));
    }

    Ok((num * base.pow(exponent) as f64) as u64)
}

fn get_version_string() -> String {
    #[cfg(debug_assertions)]
    const BUILD_PROFILE: &str = "debug";
    #[cfg(not(debug_assertions))]
    const BUILD_PROFILE: &str = "release";

    format!(
        "librespot {semver} {sha} (Built on {build_date}, Build ID: {build_id}, Profile: {build_profile})",
        semver = version::SEMVER,
        sha = version::SHA_SHORT,
        build_date = version::BUILD_DATE,
        build_id = version::BUILD_ID,
        build_profile = BUILD_PROFILE
    )
}

struct Setup {
    format: AudioFormat,
    backend: SinkBuilder,
    device: Option<String>,
    mixer: MixerFn,
    cache: Option<Cache>,
    player_config: PlayerConfig,
    session_config: SessionConfig,
    connect_config: ConnectConfig,
    mixer_config: MixerConfig,
    credentials: Option<Credentials>,
    enable_discovery: bool,
    zeroconf_port: u16,
    player_event_program: Option<String>,
    emit_sink_events: bool,
}

fn get_setup(args: &[String]) -> Setup {
    const AP_PORT: &str = "ap-port";
    const AUTOPLAY: &str = "autoplay";
    const BACKEND: &str = "backend";
    const BITRATE: &str = "b";
    const CACHE: &str = "c";
    const CACHE_SIZE_LIMIT: &str = "cache-size-limit";
    const DEVICE: &str = "device";
    const DEVICE_TYPE: &str = "device-type";
    const DISABLE_AUDIO_CACHE: &str = "disable-audio-cache";
    const DISABLE_CREDENTIAL_CACHE: &str = "disable-credential-cache";
    const DISABLE_DISCOVERY: &str = "disable-discovery";
    const DISABLE_GAPLESS: &str = "disable-gapless";
    const DITHER: &str = "dither";
    const EMIT_SINK_EVENTS: &str = "emit-sink-events";
    const ENABLE_VOLUME_NORMALISATION: &str = "enable-volume-normalisation";
    const FORMAT: &str = "format";
    const HELP: &str = "h";
    const INITIAL_VOLUME: &str = "initial-volume";
    const MIXER_TYPE: &str = "mixer";
    const ALSA_MIXER_DEVICE: &str = "alsa-mixer-device";
    const ALSA_MIXER_INDEX: &str = "alsa-mixer-index";
    const ALSA_MIXER_CONTROL: &str = "alsa-mixer-control";
    const NAME: &str = "name";
    const NORMALISATION_ATTACK: &str = "normalisation-attack";
    const NORMALISATION_GAIN_TYPE: &str = "normalisation-gain-type";
    const NORMALISATION_KNEE: &str = "normalisation-knee";
    const NORMALISATION_METHOD: &str = "normalisation-method";
    const NORMALISATION_PREGAIN: &str = "normalisation-pregain";
    const NORMALISATION_RELEASE: &str = "normalisation-release";
    const NORMALISATION_THRESHOLD: &str = "normalisation-threshold";
    const ONEVENT: &str = "onevent";
    const PASSTHROUGH: &str = "passthrough";
    const PASSWORD: &str = "password";
    const PROXY: &str = "proxy";
    const SYSTEM_CACHE: &str = "system-cache";
    const USERNAME: &str = "username";
    const VERBOSE: &str = "verbose";
    const VERSION: &str = "version";
    const VOLUME_CTRL: &str = "volume-ctrl";
    const VOLUME_RANGE: &str = "volume-range";
    const ZEROCONF_PORT: &str = "zeroconf-port";

    let mut opts = getopts::Options::new();
    opts.optflag(
        HELP,
        "help",
        "Print this help menu.",
    ).optopt(
        CACHE,
        "cache",
        "Path to a directory where files will be cached.",
        "PATH",
    ).optopt(
        "",
        SYSTEM_CACHE,
        "Path to a directory where system files (credentials, volume) will be cached. May be different from the cache option value.",
        "PATH",
    ).optopt(
        "",
        CACHE_SIZE_LIMIT,
        "Limits the size of the cache for audio files.",
        "SIZE"
    ).optflag("", DISABLE_AUDIO_CACHE, "Disable caching of the audio data.")
    .optflag("", DISABLE_CREDENTIAL_CACHE, "Disable caching of credentials.")
    .optopt("n", NAME, "Device name.", "NAME")
    .optopt("", DEVICE_TYPE, "Displayed device type. Defaults to 'Speaker'.", "TYPE")
    .optopt(
        BITRATE,
        "bitrate",
        "Bitrate (kbps) {96|160|320}. Defaults to 160.",
        "BITRATE",
    )
    .optopt(
        "",
        ONEVENT,
        "Run PROGRAM when a playback event occurs.",
        "PROGRAM",
    )
    .optflag("", EMIT_SINK_EVENTS, "Run PROGRAM set by --onevent before sink is opened and after it is closed.")
    .optflag("v", VERBOSE, "Enable verbose output.")
    .optflag("V", VERSION, "Display librespot version string.")
    .optopt("u", USERNAME, "Username used to sign in with.", "USERNAME")
    .optopt("p", PASSWORD, "Password used to sign in with.", "PASSWORD")
    .optopt("", PROXY, "HTTP proxy to use when connecting.", "URL")
    .optopt("", AP_PORT, "Connect to an AP with a specified port. If no AP with that port is present a fallback AP will be used. Available ports are usually 80, 443 and 4070.", "PORT")
    .optflag("", DISABLE_DISCOVERY, "Disable zeroconf discovery mode.")
    .optopt(
        "",
        BACKEND,
        "Audio backend to use. Use '?' to list options.",
        "NAME",
    )
    .optopt(
        "",
        DEVICE,
        "Audio device to use. Use '?' to list options if using alsa, portaudio or rodio. Defaults to the backend's default.",
        "NAME",
    )
    .optopt(
        "",
        FORMAT,
        "Output format {F64|F32|S32|S24|S24_3|S16}. Defaults to S16.",
        "FORMAT",
    )
    .optopt(
        "",
        DITHER,
        "Specify the dither algorithm to use {none|gpdf|tpdf|tpdf_hp}. Defaults to 'tpdf' for formats S16, S24, S24_3 and 'none' for other formats.",
        "DITHER",
    )
    .optopt("m", MIXER_TYPE, "Mixer to use {alsa|softvol}. Defaults to softvol", "MIXER")
    .optopt(
        "",
        "mixer-name", // deprecated
        "",
        "",
    )
    .optopt(
        "",
        ALSA_MIXER_CONTROL,
        "Alsa mixer control, e.g. 'PCM', 'Master' or similar. Defaults to 'PCM'.",
        "NAME",
    )
    .optopt(
        "",
        "mixer-card", // deprecated
        "",
        "",
    )
    .optopt(
        "",
        ALSA_MIXER_DEVICE,
        "Alsa mixer device, e.g 'hw:0' or similar from `aplay -l`. Defaults to `--device` if specified, 'default' otherwise.",
        "DEVICE",
    )
    .optopt(
        "",
        "mixer-index", // deprecated
        "",
        "",
    )
    .optopt(
        "",
        ALSA_MIXER_INDEX,
        "Alsa index of the cards mixer. Defaults to 0.",
        "NUMBER",
    )
    .optopt(
        "",
        INITIAL_VOLUME,
        "Initial volume in % from 0-100. Default for softvol: '50'. For the Alsa mixer: the current volume.",
        "VOLUME",
    )
    .optopt(
        "",
        ZEROCONF_PORT,
        "The port the internal server advertises over zeroconf.",
        "PORT",
    )
    .optflag(
        "",
        ENABLE_VOLUME_NORMALISATION,
        "Play all tracks at approximately the same apparent volume.",
    )
    .optopt(
        "",
        NORMALISATION_METHOD,
        "Specify the normalisation method to use {basic|dynamic}. Defaults to dynamic.",
        "METHOD",
    )
    .optopt(
        "",
        NORMALISATION_GAIN_TYPE,
        "Specify the normalisation gain type to use {track|album|auto}. Defaults to auto.",
        "TYPE",
    )
    .optopt(
        "",
        NORMALISATION_PREGAIN,
        "Pregain (dB) applied by volume normalisation. Defaults to 0.",
        "PREGAIN",
    )
    .optopt(
        "",
        NORMALISATION_THRESHOLD,
        "Threshold (dBFS) at which the dynamic limiter engages to prevent clipping. Defaults to -2.0.",
        "THRESHOLD",
    )
    .optopt(
        "",
        NORMALISATION_ATTACK,
        "Attack time (ms) in which the dynamic limiter reduces gain. Defaults to 5.",
        "TIME",
    )
    .optopt(
        "",
        NORMALISATION_RELEASE,
        "Release or decay time (ms) in which the dynamic limiter restores gain. Defaults to 100.",
        "TIME",
    )
    .optopt(
        "",
        NORMALISATION_KNEE,
        "Knee steepness of the dynamic limiter. Defaults to 1.0.",
        "KNEE",
    )
    .optopt(
        "",
        VOLUME_CTRL,
        "Volume control scale type {cubic|fixed|linear|log}. Defaults to log.",
        "VOLUME_CTRL"
    )
    .optopt(
        "",
        VOLUME_RANGE,
        "Range of the volume control (dB). Default for softvol: 60. For the Alsa mixer: what the control supports.",
        "RANGE",
    )
    .optflag(
        "",
        AUTOPLAY,
        "Automatically play similar songs when your music ends.",
    )
    .optflag(
        "",
        DISABLE_GAPLESS,
        "Disable gapless playback.",
    )
    .optflag(
        "",
        PASSTHROUGH,
        "Pass a raw stream to the output. Only works with the pipe and subprocess backends.",
    );

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            eprintln!(
                "Error parsing command line options: {}\n{}",
                f,
                usage(&args[0], &opts)
            );
            exit(1);
        }
    };

    if matches.opt_present(HELP) {
        println!("{}", usage(&args[0], &opts));
        exit(0);
    }

    if matches.opt_present(VERSION) {
        println!("{}", get_version_string());
        exit(0);
    }

    let verbose = matches.opt_present(VERBOSE);
    setup_logging(verbose);

    info!("{}", get_version_string());

    let backend_name = matches.opt_str(BACKEND);
    if backend_name == Some("?".into()) {
        list_backends();
        exit(0);
    }

    let backend = audio_backend::find(backend_name).expect("Invalid backend");

    let format = matches
        .opt_str(FORMAT)
        .as_deref()
        .map(|format| AudioFormat::from_str(format).expect("Invalid output format"))
        .unwrap_or_default();

    let device = matches.opt_str(DEVICE);
    if device == Some("?".into()) {
        backend(device, format);
        exit(0);
    }

    let mixer_type = matches.opt_str(MIXER_TYPE);
    let mixer = mixer::find(mixer_type.as_deref()).expect("Invalid mixer");

    let mixer_config = {
        let mixer_device = match matches.opt_str("mixer-card") {
            Some(card) => {
                warn!("--mixer-card is deprecated and will be removed in a future release.");
                warn!("Please use --alsa-mixer-device instead.");
                card
            }
            None => matches.opt_str(ALSA_MIXER_DEVICE).unwrap_or_else(|| {
                if let Some(ref device_name) = device {
                    device_name.to_string()
                } else {
                    MixerConfig::default().device
                }
            }),
        };

        let index = match matches.opt_str("mixer-index") {
            Some(index) => {
                warn!("--mixer-index is deprecated and will be removed in a future release.");
                warn!("Please use --alsa-mixer-index instead.");
                index
                    .parse::<u32>()
                    .expect("Mixer index is not a valid number")
            }
            None => matches
                .opt_str(ALSA_MIXER_INDEX)
                .map(|index| {
                    index
                        .parse::<u32>()
                        .expect("Alsa mixer index is not a valid number")
                })
                .unwrap_or(0),
        };

        let control = match matches.opt_str("mixer-name") {
            Some(name) => {
                warn!("--mixer-name is deprecated and will be removed in a future release.");
                warn!("Please use --alsa-mixer-control instead.");
                name
            }
            None => matches
                .opt_str(ALSA_MIXER_CONTROL)
                .unwrap_or_else(|| MixerConfig::default().control),
        };

        let mut volume_range = matches
            .opt_str(VOLUME_RANGE)
            .map(|range| range.parse::<f64>().unwrap())
            .unwrap_or_else(|| match mixer_type.as_deref() {
                #[cfg(feature = "alsa-backend")]
                Some(AlsaMixer::NAME) => 0.0, // let Alsa query the control
                _ => VolumeCtrl::DEFAULT_DB_RANGE,
            });
        if volume_range < 0.0 {
            // User might have specified range as minimum dB volume.
            volume_range = -volume_range;
            warn!(
                "Please enter positive volume ranges only, assuming {:.2} dB",
                volume_range
            );
        }
        let volume_ctrl = matches
            .opt_str(VOLUME_CTRL)
            .as_deref()
            .map(|volume_ctrl| {
                VolumeCtrl::from_str_with_range(volume_ctrl, volume_range)
                    .expect("Invalid volume control type")
            })
            .unwrap_or_else(|| {
                let mut volume_ctrl = VolumeCtrl::default();
                volume_ctrl.set_db_range(volume_range);
                volume_ctrl
            });

        MixerConfig {
            device: mixer_device,
            control,
            index,
            volume_ctrl,
        }
    };

    let cache = {
        let volume_dir = matches
            .opt_str(SYSTEM_CACHE)
            .or_else(|| matches.opt_str(CACHE))
            .map(|p| p.into());

        let cred_dir = if matches.opt_present(DISABLE_CREDENTIAL_CACHE) {
            None
        } else {
            volume_dir.clone()
        };

        let audio_dir = if matches.opt_present(DISABLE_AUDIO_CACHE) {
            None
        } else {
            matches
                .opt_str(CACHE)
                .as_ref()
                .map(|p| AsRef::<Path>::as_ref(p).join("files"))
        };

        let limit = if audio_dir.is_some() {
            matches
                .opt_str(CACHE_SIZE_LIMIT)
                .as_deref()
                .map(parse_file_size)
                .map(|e| {
                    e.unwrap_or_else(|e| {
                        eprintln!("Invalid argument passed as cache size limit: {}", e);
                        exit(1);
                    })
                })
        } else {
            None
        };

        match Cache::new(cred_dir, volume_dir, audio_dir, limit) {
            Ok(cache) => Some(cache),
            Err(e) => {
                warn!("Cannot create cache: {}", e);
                None
            }
        }
    };

    let initial_volume = matches
        .opt_str(INITIAL_VOLUME)
        .map(|initial_volume| {
            let volume = initial_volume.parse::<u16>().unwrap();
            if volume > 100 {
                error!("Initial volume must be in the range 0-100.");
                // the cast will saturate, not necessary to take further action
            }
            (volume as f32 / 100.0 * VolumeCtrl::MAX_VOLUME as f32) as u16
        })
        .or_else(|| match mixer_type.as_deref() {
            #[cfg(feature = "alsa-backend")]
            Some(AlsaMixer::NAME) => None,
            _ => cache.as_ref().and_then(Cache::volume),
        });

    let zeroconf_port = matches
        .opt_str(ZEROCONF_PORT)
        .map(|port| port.parse::<u16>().unwrap())
        .unwrap_or(0);

    let name = matches
        .opt_str(NAME)
        .unwrap_or_else(|| "Librespot".to_string());

    let credentials = {
        let cached_credentials = cache.as_ref().and_then(Cache::credentials);

        let password = |username: &String| -> Option<String> {
            write!(stderr(), "Password for {}: ", username).ok()?;
            stderr().flush().ok()?;
            rpassword::read_password().ok()
        };

        get_credentials(
            matches.opt_str(USERNAME),
            matches.opt_str(PASSWORD),
            cached_credentials,
            password,
        )
    };

    if credentials.is_none() && matches.opt_present(DISABLE_DISCOVERY) {
        error!("Credentials are required if discovery is disabled.");
        exit(1);
    }

    let session_config = {
        let device_id = device_id(&name);

        SessionConfig {
            user_agent: version::VERSION_STRING.to_string(),
            device_id,
            proxy: matches.opt_str(PROXY).or_else(|| std::env::var("http_proxy").ok()).map(
                |s| {
                    match Url::parse(&s) {
                        Ok(url) => {
                            if url.host().is_none() || url.port_or_known_default().is_none() {
                                panic!("Invalid proxy url, only URLs on the format \"http://host:port\" are allowed");
                            }

                            if url.scheme() != "http" {
                                panic!("Only unsecure http:// proxies are supported");
                            }
                            url
                        },
                        Err(err) => panic!("Invalid proxy URL: {}, only URLs in the format \"http://host:port\" are allowed", err)
                    }
                },
            ),
            ap_port: matches
                .opt_str(AP_PORT)
                .map(|port| port.parse::<u16>().expect("Invalid port")),
        }
    };

    let player_config = {
        let bitrate = matches
            .opt_str(BITRATE)
            .as_deref()
            .map(|bitrate| Bitrate::from_str(bitrate).expect("Invalid bitrate"))
            .unwrap_or_default();

        let gapless = !matches.opt_present(DISABLE_GAPLESS);

        let normalisation = matches.opt_present(ENABLE_VOLUME_NORMALISATION);
        let normalisation_method = matches
            .opt_str(NORMALISATION_METHOD)
            .as_deref()
            .map(|method| {
                NormalisationMethod::from_str(method).expect("Invalid normalisation method")
            })
            .unwrap_or_default();
        let normalisation_type = matches
            .opt_str(NORMALISATION_GAIN_TYPE)
            .as_deref()
            .map(|gain_type| {
                NormalisationType::from_str(gain_type).expect("Invalid normalisation type")
            })
            .unwrap_or_default();
        let normalisation_pregain = matches
            .opt_str(NORMALISATION_PREGAIN)
            .map(|pregain| pregain.parse::<f64>().expect("Invalid pregain float value"))
            .unwrap_or(PlayerConfig::default().normalisation_pregain);
        let normalisation_threshold = matches
            .opt_str(NORMALISATION_THRESHOLD)
            .map(|threshold| {
                db_to_ratio(
                    threshold
                        .parse::<f64>()
                        .expect("Invalid threshold float value"),
                )
            })
            .unwrap_or(PlayerConfig::default().normalisation_threshold);
        let normalisation_attack = matches
            .opt_str(NORMALISATION_ATTACK)
            .map(|attack| {
                Duration::from_millis(attack.parse::<u64>().expect("Invalid attack value"))
            })
            .unwrap_or(PlayerConfig::default().normalisation_attack);
        let normalisation_release = matches
            .opt_str(NORMALISATION_RELEASE)
            .map(|release| {
                Duration::from_millis(release.parse::<u64>().expect("Invalid release value"))
            })
            .unwrap_or(PlayerConfig::default().normalisation_release);
        let normalisation_knee = matches
            .opt_str(NORMALISATION_KNEE)
            .map(|knee| knee.parse::<f64>().expect("Invalid knee float value"))
            .unwrap_or(PlayerConfig::default().normalisation_knee);

        let ditherer_name = matches.opt_str(DITHER);
        let ditherer = match ditherer_name.as_deref() {
            // explicitly disabled on command line
            Some("none") => None,
            // explicitly set on command line
            Some(_) => {
                if format == AudioFormat::F64 || format == AudioFormat::F32 {
                    unimplemented!("Dithering is not available on format {:?}", format);
                }
                Some(dither::find_ditherer(ditherer_name).expect("Invalid ditherer"))
            }
            // nothing set on command line => use default
            None => match format {
                AudioFormat::S16 | AudioFormat::S24 | AudioFormat::S24_3 => {
                    PlayerConfig::default().ditherer
                }
                _ => None,
            },
        };

        let passthrough = matches.opt_present(PASSTHROUGH);

        PlayerConfig {
            bitrate,
            gapless,
            passthrough,
            normalisation,
            normalisation_type,
            normalisation_method,
            normalisation_pregain,
            normalisation_threshold,
            normalisation_attack,
            normalisation_release,
            normalisation_knee,
            ditherer,
        }
    };

    let connect_config = {
        let device_type = matches
            .opt_str(DEVICE_TYPE)
            .as_deref()
            .map(|device_type| DeviceType::from_str(device_type).expect("Invalid device type"))
            .unwrap_or_default();
        let has_volume_ctrl = !matches!(mixer_config.volume_ctrl, VolumeCtrl::Fixed);
        let autoplay = matches.opt_present(AUTOPLAY);

        ConnectConfig {
            name,
            device_type,
            initial_volume,
            has_volume_ctrl,
            autoplay,
        }
    };

    let enable_discovery = !matches.opt_present(DISABLE_DISCOVERY);
    let player_event_program = matches.opt_str(ONEVENT);
    let emit_sink_events = matches.opt_present(EMIT_SINK_EVENTS);

    Setup {
        format,
        backend,
        device,
        mixer,
        cache,
        player_config,
        session_config,
        connect_config,
        mixer_config,
        credentials,
        enable_discovery,
        zeroconf_port,
        player_event_program,
        emit_sink_events,
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    const RUST_BACKTRACE: &str = "RUST_BACKTRACE";
    if env::var(RUST_BACKTRACE).is_err() {
        env::set_var(RUST_BACKTRACE, "full")
    }

    let args: Vec<String> = std::env::args().collect();
    let setup = get_setup(&args);

    let mut last_credentials = None;
    let mut spirc: Option<Spirc> = None;
    let mut spirc_task: Option<Pin<_>> = None;
    let mut player_event_channel: Option<UnboundedReceiver<PlayerEvent>> = None;
    let mut auto_connect_times: Vec<Instant> = vec![];
    let mut discovery = None;
    let mut connecting: Pin<Box<dyn future::FusedFuture<Output = _>>> = Box::pin(future::pending());

    if setup.enable_discovery {
        let device_id = setup.session_config.device_id.clone();

        discovery = Some(
            librespot::discovery::Discovery::builder(device_id)
                .name(setup.connect_config.name.clone())
                .device_type(setup.connect_config.device_type)
                .port(setup.zeroconf_port)
                .launch()
                .unwrap(),
        );
    }

    if let Some(credentials) = setup.credentials {
        last_credentials = Some(credentials.clone());
        connecting = Box::pin(
            Session::connect(
                setup.session_config.clone(),
                credentials,
                setup.cache.clone(),
            )
            .fuse(),
        );
    }

    loop {
        tokio::select! {
            credentials = async { discovery.as_mut().unwrap().next().await }, if discovery.is_some() => {
                match credentials {
                    Some(credentials) => {
                        last_credentials = Some(credentials.clone());
                        auto_connect_times.clear();

                        if let Some(spirc) = spirc.take() {
                            spirc.shutdown();
                        }
                        if let Some(spirc_task) = spirc_task.take() {
                            // Continue shutdown in its own task
                            tokio::spawn(spirc_task);
                        }

                        connecting = Box::pin(Session::connect(
                            setup.session_config.clone(),
                            credentials,
                            setup.cache.clone(),
                        ).fuse());
                    },
                    None => {
                        warn!("Discovery stopped!");
                        discovery = None;
                    }
                }
            },
            session = &mut connecting, if !connecting.is_terminated() => match session {
                Ok(session) => {
                    let mixer_config = setup.mixer_config.clone();
                    let mixer = (setup.mixer)(mixer_config);
                    let player_config = setup.player_config.clone();
                    let connect_config = setup.connect_config.clone();

                    let audio_filter = mixer.get_audio_filter();
                    let format = setup.format;
                    let backend = setup.backend;
                    let device = setup.device.clone();
                    let (player, event_channel) =
                        Player::new(player_config, session.clone(), audio_filter, move || {
                            (backend)(device, format)
                        });

                    if setup.emit_sink_events {
                        if let Some(player_event_program) = setup.player_event_program.clone() {
                            player.set_sink_event_callback(Some(Box::new(move |sink_status| {
                                match emit_sink_event(sink_status, &player_event_program) {
                                    Ok(e) if e.success() => (),
                                    Ok(e) => {
                                        if let Some(code) = e.code() {
                                            warn!("Sink event program returned exit code {}", code);
                                        } else {
                                            warn!("Sink event program returned failure");
                                        }
                                    },
                                    Err(e) => {
                                        warn!("Emitting sink event failed: {}", e);
                                    },
                                }
                            })));
                        }
                    };

                    let (spirc_, spirc_task_) = Spirc::new(connect_config, session, player, mixer);

                    spirc = Some(spirc_);
                    spirc_task = Some(Box::pin(spirc_task_));
                    player_event_channel = Some(event_channel);
                },
                Err(e) => {
                    error!("Connection failed: {}", e);
                    exit(1);
                }
            },
            _ = async { spirc_task.as_mut().unwrap().await }, if spirc_task.is_some() => {
                spirc_task = None;

                warn!("Spirc shut down unexpectedly");
                while !auto_connect_times.is_empty()
                    && ((Instant::now() - auto_connect_times[0]).as_secs() > 600)
                {
                    let _ = auto_connect_times.remove(0);
                }

                if let Some(credentials) = last_credentials.clone() {
                    if auto_connect_times.len() >= 5 {
                        warn!("Spirc shut down too often. Not reconnecting automatically.");
                    } else {
                        auto_connect_times.push(Instant::now());

                        connecting = Box::pin(Session::connect(
                            setup.session_config.clone(),
                            credentials,
                            setup.cache.clone(),
                        ).fuse());
                    }
                }
            },
            event = async { player_event_channel.as_mut().unwrap().recv().await }, if player_event_channel.is_some() => match event {
                Some(event) => {
                    if let Some(program) = &setup.player_event_program {
                        if let Some(child) = run_program_on_events(event, program) {
                            if child.is_ok() {

                                let mut child = child.unwrap();

                                tokio::spawn(async move {
                                    match child.wait().await  {
                                        Ok(e) if e.success() => (),
                                        Ok(e) => {
                                            if let Some(code) = e.code() {
                                                warn!("On event program returned exit code {}", code);
                                            } else {
                                                warn!("On event program returned failure");
                                            }
                                        },
                                        Err(e) => {
                                            warn!("On event program failed: {}", e);
                                        },
                                    }
                                });
                            } else {
                                warn!("On event program failed to start");
                            }
                        }
                    }
                },
                None => {
                    player_event_channel = None;
                }
            },
            _ = tokio::signal::ctrl_c() => {
                break;
            }
        }
    }

    info!("Gracefully shutting down");

    // Shutdown spirc if necessary
    if let Some(spirc) = spirc {
        spirc.shutdown();

        if let Some(mut spirc_task) = spirc_task {
            tokio::select! {
                _ = tokio::signal::ctrl_c() => (),
                _ = spirc_task.as_mut() => ()
            }
        }
    }
}
