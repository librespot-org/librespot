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
use librespot::playback::mixer::mappings::MappedCtrl;
use librespot::playback::mixer::{self, MixerConfig, MixerFn};
use librespot::playback::player::{db_to_ratio, Player};

mod player_event_handler;
use player_event_handler::{emit_sink_event, run_program_on_events};

use std::convert::TryFrom;
use std::path::Path;
use std::process::exit;
use std::str::FromStr;
use std::{env, time::Instant};
use std::{
    io::{stderr, Write},
    pin::Pin,
};

const MILLIS: f32 = 1000.0;

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

fn print_version() {
    println!(
        "librespot {semver} {sha} (Built on {build_date}, Build ID: {build_id})",
        semver = version::SEMVER,
        sha = version::SHA_SHORT,
        build_date = version::BUILD_DATE,
        build_id = version::BUILD_ID
    );
}

#[derive(Clone)]
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
    let mut opts = getopts::Options::new();
    opts.optopt(
        "c",
        "cache",
        "Path to a directory where files will be cached.",
        "CACHE",
    ).optopt(
        "",
        "system-cache",
        "Path to a directory where system files (credentials, volume) will be cached. Can be different from cache option value",
        "SYTEMCACHE",
    ).optopt(
        "",
        "cache-size-limit",
        "Limits the size of the cache for audio files.",
        "CACHE_SIZE_LIMIT"
    ).optflag("", "disable-audio-cache", "Disable caching of the audio data.")
        .optopt("n", "name", "Device name", "NAME")
        .optopt("", "device-type", "Displayed device type", "DEVICE_TYPE")
        .optopt(
            "b",
            "bitrate",
            "Bitrate (96, 160 or 320). Defaults to 160",
            "BITRATE",
        )
        .optopt(
            "",
            "onevent",
            "Run PROGRAM when playback is about to begin.",
            "PROGRAM",
        )
        .optflag("", "emit-sink-events", "Run program set by --onevent before sink is opened and after it is closed.")
        .optflag("v", "verbose", "Enable verbose output")
        .optflag("V", "version", "Display librespot version string")
        .optopt("u", "username", "Username to sign in with", "USERNAME")
        .optopt("p", "password", "Password", "PASSWORD")
        .optopt("", "proxy", "HTTP proxy to use when connecting", "PROXY")
        .optopt("", "ap-port", "Connect to AP with specified port. If no AP with that port are present fallback AP will be used. Available ports are usually 80, 443 and 4070", "AP_PORT")
        .optflag("", "disable-discovery", "Disable discovery mode")
        .optopt(
            "",
            "backend",
            "Audio backend to use. Use '?' to list options",
            "BACKEND",
        )
        .optopt(
            "",
            "device",
            "Audio device to use. Use '?' to list options if using portaudio or alsa",
            "DEVICE",
        )
        .optopt(
            "",
            "format",
            "Output format (F32, S32, S24, S24_3 or S16). Defaults to S16",
            "FORMAT",
        )
        .optopt("", "mixer", "Mixer to use (alsa or softvol)", "MIXER")
        .optopt(
            "m",
            "mixer-name",
            "Alsa mixer control, e.g. 'PCM' or 'Master'. Defaults to 'PCM'.",
            "MIXER_NAME",
        )
        .optopt(
            "",
            "mixer-card",
            "Alsa mixer card, e.g 'hw:0' or similar from `aplay -l`. Defaults to DEVICE if specified, 'default' otherwise.",
            "MIXER_CARD",
        )
        .optopt(
            "",
            "mixer-index",
            "Alsa mixer index, Index of the cards mixer. Defaults to 0",
            "MIXER_INDEX",
        )
        .optopt(
            "",
            "initial-volume",
            "Initial volume (%) once connected {0..100}. Defaults to 50 for softvol and for Alsa mixer the current volume.",
            "VOLUME",
        )
        .optopt(
            "",
            "zeroconf-port",
            "The port the internal server advertised over zeroconf uses.",
            "ZEROCONF_PORT",
        )
        .optflag(
            "",
            "enable-volume-normalisation",
            "Play all tracks at the same volume",
        )
        .optopt(
            "",
            "normalisation-method",
            "Specify the normalisation method to use - [basic, dynamic]. Default is dynamic.",
            "NORMALISATION_METHOD",
        )
        .optopt(
            "",
            "normalisation-gain-type",
            "Specify the normalisation gain type to use - [track, album]. Default is album.",
            "GAIN_TYPE",
        )
        .optopt(
            "",
            "normalisation-pregain",
            "Pregain (dB) applied by volume normalisation",
            "PREGAIN",
        )
        .optopt(
            "",
            "normalisation-threshold",
            "Threshold (dBFS) to prevent clipping. Default is -1.0.",
            "THRESHOLD",
        )
        .optopt(
            "",
            "normalisation-attack",
            "Attack time (ms) in which the dynamic limiter is reducing gain. Default is 5.",
            "ATTACK",
        )
        .optopt(
            "",
            "normalisation-release",
            "Release or decay time (ms) in which the dynamic limiter is restoring gain. Default is 100.",
            "RELEASE",
        )
        .optopt(
            "",
            "normalisation-knee",
            "Knee steepness of the dynamic limiter. Default is 1.0.",
            "KNEE",
        )
        .optopt(
            "",
            "volume-ctrl",
            "Volume control type {cubic|fixed|linear|log}. Defaults to log.",
            "VOLUME_CTRL"
        )
        .optopt(
            "",
            "volume-range",
            "Range of the volume control (dB). Defaults to 60 for softvol and for Alsa mixer what the mixer supports.",
            "RANGE",
        )
        .optflag(
            "",
            "autoplay",
            "autoplay similar songs when your music ends.",
        )
        .optflag(
            "",
            "disable-gapless",
            "disable gapless playback.",
        )
	    .optflag(
            "",
            "passthrough",
            "Pass raw stream to output, only works for \"pipe\"."
        );

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            eprintln!("error: {}\n{}", f.to_string(), usage(&args[0], &opts));
            exit(1);
        }
    };

    if matches.opt_present("version") {
        print_version();
        exit(0);
    }

    let verbose = matches.opt_present("verbose");
    setup_logging(verbose);

    info!(
        "librespot {semver} {sha} (Built on {build_date}, Build ID: {build_id})",
        semver = version::SEMVER,
        sha = version::SHA_SHORT,
        build_date = version::BUILD_DATE,
        build_id = version::BUILD_ID
    );

    let backend_name = matches.opt_str("backend");
    if backend_name == Some("?".into()) {
        list_backends();
        exit(0);
    }

    let backend = audio_backend::find(backend_name).expect("Invalid backend");

    let format = matches
        .opt_str("format")
        .as_ref()
        .map(|format| AudioFormat::try_from(format).expect("Invalid output format"))
        .unwrap_or_default();

    let device = matches.opt_str("device");
    if device == Some("?".into()) {
        backend(device, format);
        exit(0);
    }

    let mixer_name = matches.opt_str("mixer");
    let mixer = mixer::find(mixer_name.as_ref()).expect("Invalid mixer");

    let mixer_config = {
        let card = matches.opt_str("mixer-card").unwrap_or_else(|| {
            if let Some(ref device_name) = device {
                device_name.to_string()
            } else {
                String::from("default")
            }
        });
        let index = matches
            .opt_str("mixer-index")
            .map(|index| index.parse::<u32>().unwrap())
            .unwrap_or(0);
        let control = matches
            .opt_str("mixer-name")
            .unwrap_or_else(|| String::from("PCM"));
        let mut volume_range = matches
            .opt_str("volume-range")
            .map(|range| range.parse::<f32>().unwrap())
            .unwrap_or_else(|| match mixer_name.as_ref().map(AsRef::as_ref) {
                Some("alsa") => 0.0, // let Alsa query the control
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
            .opt_str("volume-ctrl")
            .as_ref()
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
            card,
            control,
            index,
            volume_ctrl,
        }
    };

    let cache = {
        let audio_dir;
        let system_dir;
        if matches.opt_present("disable-audio-cache") {
            audio_dir = None;
            system_dir = matches
                .opt_str("system-cache")
                .or_else(|| matches.opt_str("c"))
                .map(|p| p.into());
        } else {
            let cache_dir = matches.opt_str("c");
            audio_dir = cache_dir
                .as_ref()
                .map(|p| AsRef::<Path>::as_ref(p).join("files"));
            system_dir = matches
                .opt_str("system-cache")
                .or(cache_dir)
                .map(|p| p.into());
        }

        let limit = if audio_dir.is_some() {
            matches
                .opt_str("cache-size-limit")
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

        match Cache::new(system_dir, audio_dir, limit) {
            Ok(cache) => Some(cache),
            Err(e) => {
                warn!("Cannot create cache: {}", e);
                None
            }
        }
    };

    let initial_volume = matches
        .opt_str("initial-volume")
        .map(|initial_volume| {
            let volume = initial_volume.parse::<u16>().unwrap();
            if volume > 100 {
                error!("Initial volume must be in the range 0-100.");
                // the cast will saturate, not necessary to take further action
            }
            (volume as f32 / 100.0 * VolumeCtrl::MAX_VOLUME as f32) as u16
        })
        .or_else(|| match mixer_name.as_ref().map(AsRef::as_ref) {
            Some("alsa") => None,
            _ => cache.as_ref().and_then(Cache::volume),
        });

    let zeroconf_port = matches
        .opt_str("zeroconf-port")
        .map(|port| port.parse::<u16>().unwrap())
        .unwrap_or(0);

    let name = matches
        .opt_str("name")
        .unwrap_or_else(|| "Librespot".to_string());

    let credentials = {
        let cached_credentials = cache.as_ref().and_then(Cache::credentials);

        let password = |username: &String| -> Option<String> {
            write!(stderr(), "Password for {}: ", username).ok()?;
            stderr().flush().ok()?;
            rpassword::read_password().ok()
        };

        get_credentials(
            matches.opt_str("username"),
            matches.opt_str("password"),
            cached_credentials,
            password,
        )
    };

    let session_config = {
        let device_id = device_id(&name);

        SessionConfig {
            user_agent: version::VERSION_STRING.to_string(),
            device_id,
            proxy: matches.opt_str("proxy").or_else(|| std::env::var("http_proxy").ok()).map(
                |s| {
                    match Url::parse(&s) {
                        Ok(url) => {
                            if url.host().is_none() || url.port_or_known_default().is_none() {
                                panic!("Invalid proxy url, only urls on the format \"http://host:port\" are allowed");
                            }

                            if url.scheme() != "http" {
                                panic!("Only unsecure http:// proxies are supported");
                            }
                            url
                        },
                    Err(err) => panic!("Invalid proxy url: {}, only urls on the format \"http://host:port\" are allowed", err)
                    }
                },
            ),
            ap_port: matches
                .opt_str("ap-port")
                .map(|port| port.parse::<u16>().expect("Invalid port")),
        }
    };

    let player_config = {
        let bitrate = matches
            .opt_str("b")
            .as_ref()
            .map(|bitrate| Bitrate::from_str(bitrate).expect("Invalid bitrate"))
            .unwrap_or_default();
        let gapless = !matches.opt_present("disable-gapless");
        let normalisation = matches.opt_present("enable-volume-normalisation");
        let normalisation_method = matches
            .opt_str("normalisation-method")
            .as_ref()
            .map(|method| {
                NormalisationMethod::from_str(method).expect("Invalid normalisation method")
            })
            .unwrap_or_default();
        let normalisation_type = matches
            .opt_str("normalisation-gain-type")
            .as_ref()
            .map(|gain_type| {
                NormalisationType::from_str(gain_type).expect("Invalid normalisation type")
            })
            .unwrap_or_default();
        let normalisation_pregain = matches
            .opt_str("normalisation-pregain")
            .map(|pregain| pregain.parse::<f32>().expect("Invalid pregain float value"))
            .unwrap_or(PlayerConfig::default().normalisation_pregain);
        let normalisation_threshold = matches
            .opt_str("normalisation-threshold")
            .map(|threshold| {
                db_to_ratio(
                    threshold
                        .parse::<f32>()
                        .expect("Invalid threshold float value"),
                )
            })
            .unwrap_or(PlayerConfig::default().normalisation_threshold);
        let normalisation_attack = matches
            .opt_str("normalisation-attack")
            .map(|attack| attack.parse::<f32>().expect("Invalid attack float value") / MILLIS)
            .unwrap_or(PlayerConfig::default().normalisation_attack);
        let normalisation_release = matches
            .opt_str("normalisation-release")
            .map(|release| release.parse::<f32>().expect("Invalid release float value") / MILLIS)
            .unwrap_or(PlayerConfig::default().normalisation_release);
        let normalisation_knee = matches
            .opt_str("normalisation-knee")
            .map(|knee| knee.parse::<f32>().expect("Invalid knee float value"))
            .unwrap_or(PlayerConfig::default().normalisation_knee);
        let passthrough = matches.opt_present("passthrough");

        PlayerConfig {
            bitrate,
            gapless,
            normalisation,
            normalisation_type,
            normalisation_method,
            normalisation_pregain,
            normalisation_threshold,
            normalisation_attack,
            normalisation_release,
            normalisation_knee,
            passthrough,
        }
    };

    let connect_config = {
        let device_type = matches
            .opt_str("device-type")
            .as_ref()
            .map(|device_type| DeviceType::from_str(device_type).expect("Invalid device type"))
            .unwrap_or_default();
        let has_volume_ctrl = !matches!(mixer_config.volume_ctrl, VolumeCtrl::Fixed);
        let autoplay = matches.opt_present("autoplay");

        ConnectConfig {
            name,
            device_type,
            initial_volume,
            has_volume_ctrl,
            autoplay,
        }
    };

    let enable_discovery = !matches.opt_present("disable-discovery");
    let player_event_program = matches.opt_str("onevent");
    let emit_sink_events = matches.opt_present("emit-sink-events");

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
    if env::var("RUST_BACKTRACE").is_err() {
        env::set_var("RUST_BACKTRACE", "full")
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
                                            warn!("Sink event prog returned exit code {}", code);
                                        } else {
                                            warn!("Sink event prog returned failure");
                                        }
                                    }
                                    Err(e) => {
                                        warn!("Emitting sink event failed: {}", e);
                                    }
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
                    warn!("Connection failed: {}", e);
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
                                        Ok(status) if !status.success() => error!("child exited with status {:?}", status.code()),
                                        Err(e) => error!("failed to wait on child process: {}", e),
                                        _ => {}
                                    }
                                });
                            } else {
                                error!("program failed to start");
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
