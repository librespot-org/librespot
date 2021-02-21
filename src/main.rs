use futures::{future::FusedFuture, FutureExt, StreamExt};
use librespot_playback::player::PlayerEvent;
use log::{error, info, warn};
use sha1::{Digest, Sha1};
use std::path::Path;
use std::process::exit;
use std::str::FromStr;
use std::{env, time::Instant};
use std::{
    io::{stderr, Write},
    pin::Pin,
};
use tokio::sync::mpsc::UnboundedReceiver;
use url::Url;

use librespot::core::authentication::Credentials;
use librespot::core::cache::Cache;
use librespot::core::config::{ConnectConfig, DeviceType, SessionConfig, VolumeCtrl};
use librespot::core::session::Session;
use librespot::core::version;

use librespot::connect::spirc::Spirc;
use librespot::playback::audio_backend::{self, Sink, BACKENDS};
use librespot::playback::config::{Bitrate, NormalisationType, PlayerConfig};
use librespot::playback::mixer::{self, Mixer, MixerConfig};
use librespot::playback::player::Player;

mod player_event_handler;

use player_event_handler::{emit_sink_event, run_program_on_events};

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
    println!("Available Backends : ");
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

#[derive(Clone)]
struct Setup {
    backend: fn(Option<String>) -> Box<dyn Sink + Send + 'static>,
    device: Option<String>,

    mixer: fn(Option<MixerConfig>) -> Box<dyn Mixer>,

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

fn setup(args: &[String]) -> Setup {
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
    ).optflag("", "disable-audio-cache", "Disable caching of the audio data.")
        .reqopt("n", "name", "Device name", "NAME")
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
        .optopt("", "mixer", "Mixer to use (alsa or softvol)", "MIXER")
        .optopt(
            "m",
            "mixer-name",
            "Alsa mixer name, e.g \"PCM\" or \"Master\". Defaults to 'PCM'",
            "MIXER_NAME",
        )
        .optopt(
            "",
            "mixer-card",
            "Alsa mixer card, e.g \"hw:0\" or similar from `aplay -l`. Defaults to 'default' ",
            "MIXER_CARD",
        )
        .optopt(
            "",
            "mixer-index",
            "Alsa mixer index, Index of the cards mixer. Defaults to 0",
            "MIXER_INDEX",
        )
        .optflag(
            "",
            "mixer-linear-volume",
            "Disable alsa's mapped volume scale (cubic). Default false",
        )
        .optopt(
            "",
            "initial-volume",
            "Initial volume in %, once connected (must be from 0 to 100)",
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
            "volume-ctrl",
            "Volume control type - [linear, log, fixed]. Default is logarithmic",
            "VOLUME_CTRL"
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
            writeln!(
                stderr(),
                "error: {}\n{}",
                f.to_string(),
                usage(&args[0], &opts)
            )
            .unwrap();
            exit(1);
        }
    };

    let verbose = matches.opt_present("verbose");
    setup_logging(verbose);

    info!(
        "librespot {} ({}). Built on {}. Build ID: {}",
        version::short_sha(),
        version::commit_date(),
        version::short_now(),
        version::build_id()
    );

    let backend_name = matches.opt_str("backend");
    if backend_name == Some("?".into()) {
        list_backends();
        exit(0);
    }

    let backend = audio_backend::find(backend_name).expect("Invalid backend");

    let device = matches.opt_str("device");
    if device == Some("?".into()) {
        backend(device);
        exit(0);
    }

    let mixer_name = matches.opt_str("mixer");
    let mixer = mixer::find(mixer_name.as_ref()).expect("Invalid mixer");

    let mixer_config = MixerConfig {
        card: matches
            .opt_str("mixer-card")
            .unwrap_or_else(|| String::from("default")),
        mixer: matches
            .opt_str("mixer-name")
            .unwrap_or_else(|| String::from("PCM")),
        index: matches
            .opt_str("mixer-index")
            .map(|index| index.parse::<u32>().unwrap())
            .unwrap_or(0),
        mapped_volume: !matches.opt_present("mixer-linear-volume"),
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

        match Cache::new(system_dir, audio_dir) {
            Ok(cache) => Some(cache),
            Err(e) => {
                warn!("Cannot create cache: {}", e);
                None
            }
        }
    };

    let initial_volume = matches
        .opt_str("initial-volume")
        .map(|volume| {
            let volume = volume.parse::<u16>().unwrap();
            if volume > 100 {
                panic!("Initial volume must be in the range 0-100");
            }
            (volume as i32 * 0xFFFF / 100) as u16
        })
        .or_else(|| cache.as_ref().and_then(Cache::volume))
        .unwrap_or(0x8000);

    let zeroconf_port = matches
        .opt_str("zeroconf-port")
        .map(|port| port.parse::<u16>().unwrap())
        .unwrap_or(0);

    let name = matches.opt_str("name").unwrap();

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
            user_agent: version::version_string(),
            device_id: device_id,
            proxy: matches.opt_str("proxy").or(std::env::var("http_proxy").ok()).map(
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

    let passthrough = matches.opt_present("passthrough");

    let player_config = {
        let bitrate = matches
            .opt_str("b")
            .as_ref()
            .map(|bitrate| Bitrate::from_str(bitrate).expect("Invalid bitrate"))
            .unwrap_or(Bitrate::default());
        let gain_type = matches
            .opt_str("normalisation-gain-type")
            .as_ref()
            .map(|gain_type| {
                NormalisationType::from_str(gain_type).expect("Invalid normalisation type")
            })
            .unwrap_or(NormalisationType::default());
        PlayerConfig {
            bitrate: bitrate,
            gapless: !matches.opt_present("disable-gapless"),
            normalisation: matches.opt_present("enable-volume-normalisation"),
            normalisation_type: gain_type,
            normalisation_pregain: matches
                .opt_str("normalisation-pregain")
                .map(|pregain| pregain.parse::<f32>().expect("Invalid pregain float value"))
                .unwrap_or(PlayerConfig::default().normalisation_pregain),
            passthrough,
        }
    };

    let connect_config = {
        let device_type = matches
            .opt_str("device-type")
            .as_ref()
            .map(|device_type| DeviceType::from_str(device_type).expect("Invalid device type"))
            .unwrap_or(DeviceType::default());

        let volume_ctrl = matches
            .opt_str("volume-ctrl")
            .as_ref()
            .map(|volume_ctrl| VolumeCtrl::from_str(volume_ctrl).expect("Invalid volume ctrl type"))
            .unwrap_or(VolumeCtrl::default());

        ConnectConfig {
            name: name,
            device_type: device_type,
            volume: initial_volume,
            volume_ctrl: volume_ctrl,
            autoplay: matches.opt_present("autoplay"),
        }
    };

    let enable_discovery = !matches.opt_present("disable-discovery");

    Setup {
        backend: backend,
        cache: cache,
        session_config: session_config,
        player_config: player_config,
        connect_config: connect_config,
        credentials: credentials,
        device: device,
        enable_discovery: enable_discovery,
        zeroconf_port: zeroconf_port,
        mixer: mixer,
        mixer_config: mixer_config,
        player_event_program: matches.opt_str("onevent"),
        emit_sink_events: matches.opt_present("emit-sink-events"),
    }
}

#[tokio::main]
async fn main() {
    if env::var("RUST_BACKTRACE").is_err() {
        env::set_var("RUST_BACKTRACE", "full")
    }

    let args: Vec<String> = std::env::args().collect();
    let setupp = setup(&args);

    let mut last_credentials = None;
    let mut spirc: Option<Spirc> = None;
    let mut spirc_task: Option<Pin<_>> = None;
    let mut player_event_channel: Option<UnboundedReceiver<PlayerEvent>> = None;
    let mut auto_connect_times: Vec<Instant> = vec![];
    let mut discovery = None;
    let mut connecting: Pin<Box<dyn FusedFuture<Output = _>>> =
        Box::pin(futures::future::pending());

    if setupp.enable_discovery {
        let config = setupp.connect_config.clone();
        let device_id = setupp.session_config.device_id.clone();

        discovery = Some(
            librespot_connect::discovery::discovery(config, device_id, setupp.zeroconf_port)
                .unwrap(),
        );
    }

    if let Some(credentials) = setupp.credentials {
        last_credentials = Some(credentials.clone());
        connecting = Box::pin(
            Session::connect(
                setupp.session_config.clone(),
                credentials,
                setupp.cache.clone(),
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
                            setupp.session_config.clone(),
                            credentials,
                            setupp.cache.clone(),
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
                    let mixer_config = setupp.mixer_config.clone();
                    let mixer = (setupp.mixer)(Some(mixer_config));
                    let player_config = setupp.player_config.clone();
                    let connect_config = setupp.connect_config.clone();

                    let audio_filter = mixer.get_audio_filter();
                    let backend = setupp.backend;
                    let device = setupp.device.clone();
                    let (player, event_channel) =
                        Player::new(player_config, session.clone(), audio_filter, move || {
                            (backend)(device)
                        });

                    if setupp.emit_sink_events {
                        if let Some(player_event_program) = setupp.player_event_program.clone() {
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
                            setupp.session_config.clone(),
                            credentials,
                            setupp.cache.clone(),
                        ).fuse());
                    }
                }
            },
            event = async { player_event_channel.as_mut().unwrap().recv().await }, if player_event_channel.is_some() => match event {
                Some(event) => {
                    if let Some(program) = &setupp.player_event_program {
                        if let Some(child) = run_program_on_events(event, program) {
                            let mut child = child.expect("program failed to start");

                            tokio::spawn(async move {
                                match child.wait().await  {
                                    Ok(status) if !status.success() => error!("child exited with status {:?}", status.code()),
                                    Err(e) => error!("failed to wait on child process: {}", e),
                                    _ => {}
                                }
                            });
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
