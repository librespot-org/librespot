use futures::sync::mpsc::UnboundedReceiver;
use futures::{Async, Future, Poll, Stream};
use log::{error, info, trace, warn};
use sha1::{Digest, Sha1};
use std::env;
use std::io::{stderr, Write};
use std::mem;
use std::path::Path;
use std::process::exit;
use std::str::FromStr;
use std::time::Instant;
use tokio_core::reactor::{Core, Handle};
use tokio_io::IoStream;
use url::Url;

use librespot::core::authentication::{get_credentials, Credentials};
use librespot::core::cache::Cache;
use librespot::core::config::{ConnectConfig, DeviceType, SessionConfig, VolumeCtrl};
use librespot::core::session::{AuthenticationError, Session};
use librespot::core::version;

use librespot::connect::discovery::{discovery, DiscoveryStream};
use librespot::connect::spirc::{Spirc, SpircTask};
use librespot::playback::audio_backend::{self, Sink, BACKENDS};
use librespot::playback::config::{Bitrate, NormalisationType, PlayerConfig};
use librespot::playback::mixer::{self, Mixer, MixerConfig};
use librespot::playback::player::{Player, PlayerEvent};

mod player_event_handler;
use crate::player_event_handler::{emit_sink_event, run_program_on_events};

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

#[derive(Clone)]
struct Setup {
    backend: fn(Option<String>) -> Box<dyn Sink>,
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
        version::SHA_SHORT,
        version::COMMIT_DATE,
        version::BUILD_DATE,
        version::BUILD_ID
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
            .unwrap_or(String::from("default")),
        mixer: matches.opt_str("mixer-name").unwrap_or(String::from("PCM")),
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
                .or_else(|| cache_dir)
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

        let password = |username: &String| -> String {
            write!(stderr(), "Password for {}: ", username).unwrap();
            stderr().flush().unwrap();
            rpassword::read_password().unwrap()
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

struct Main {
    cache: Option<Cache>,
    player_config: PlayerConfig,
    session_config: SessionConfig,
    connect_config: ConnectConfig,
    backend: fn(Option<String>) -> Box<dyn Sink>,
    device: Option<String>,
    mixer: fn(Option<MixerConfig>) -> Box<dyn Mixer>,
    mixer_config: MixerConfig,
    handle: Handle,

    discovery: Option<DiscoveryStream>,
    signal: IoStream<()>,

    spirc: Option<Spirc>,
    spirc_task: Option<SpircTask>,
    connect: Box<dyn Future<Item = Session, Error = AuthenticationError>>,

    shutdown: bool,
    last_credentials: Option<Credentials>,
    auto_connect_times: Vec<Instant>,

    player_event_channel: Option<UnboundedReceiver<PlayerEvent>>,
    player_event_program: Option<String>,
    emit_sink_events: bool,
}

impl Main {
    fn new(handle: Handle, setup: Setup) -> Main {
        let mut task = Main {
            handle: handle.clone(),
            cache: setup.cache,
            session_config: setup.session_config,
            player_config: setup.player_config,
            connect_config: setup.connect_config,
            backend: setup.backend,
            device: setup.device,
            mixer: setup.mixer,
            mixer_config: setup.mixer_config,

            connect: Box::new(futures::future::empty()),
            discovery: None,
            spirc: None,
            spirc_task: None,
            shutdown: false,
            last_credentials: None,
            auto_connect_times: Vec::new(),
            signal: Box::new(tokio_signal::ctrl_c().flatten_stream()),

            player_event_channel: None,
            player_event_program: setup.player_event_program,
            emit_sink_events: setup.emit_sink_events,
        };

        if setup.enable_discovery {
            let config = task.connect_config.clone();
            let device_id = task.session_config.device_id.clone();

            task.discovery =
                Some(discovery(&handle, config, device_id, setup.zeroconf_port).unwrap());
        }

        if let Some(credentials) = setup.credentials {
            task.credentials(credentials);
        }

        task
    }

    fn credentials(&mut self, credentials: Credentials) {
        self.last_credentials = Some(credentials.clone());
        let config = self.session_config.clone();
        let handle = self.handle.clone();

        let connection = Session::connect(config, credentials, self.cache.clone(), handle);

        self.connect = connection;
        self.spirc = None;
        let task = mem::replace(&mut self.spirc_task, None);
        if let Some(task) = task {
            self.handle.spawn(task);
        }
    }
}

impl Future for Main {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<(), ()> {
        loop {
            let mut progress = false;

            if let Some(Async::Ready(Some(creds))) =
                self.discovery.as_mut().map(|d| d.poll().unwrap())
            {
                if let Some(ref spirc) = self.spirc {
                    spirc.shutdown();
                }
                self.auto_connect_times.clear();
                self.credentials(creds);

                progress = true;
            }

            match self.connect.poll() {
                Ok(Async::Ready(session)) => {
                    self.connect = Box::new(futures::future::empty());
                    let mixer_config = self.mixer_config.clone();
                    let mixer = (self.mixer)(Some(mixer_config));
                    let player_config = self.player_config.clone();
                    let connect_config = self.connect_config.clone();

                    let audio_filter = mixer.get_audio_filter();
                    let backend = self.backend;
                    let device = self.device.clone();
                    let (player, event_channel) =
                        Player::new(player_config, session.clone(), audio_filter, move || {
                            (backend)(device)
                        });

                    if self.emit_sink_events {
                        if let Some(player_event_program) = &self.player_event_program {
                            let player_event_program = player_event_program.clone();
                            player.set_sink_event_callback(Some(Box::new(move |sink_status| {
                                emit_sink_event(sink_status, &player_event_program)
                            })));
                        }
                    }

                    let (spirc, spirc_task) = Spirc::new(connect_config, session, player, mixer);
                    self.spirc = Some(spirc);
                    self.spirc_task = Some(spirc_task);
                    self.player_event_channel = Some(event_channel);

                    progress = true;
                }
                Ok(Async::NotReady) => (),
                Err(error) => {
                    error!("Could not connect to server: {}", error);
                    self.connect = Box::new(futures::future::empty());
                }
            }

            if let Async::Ready(Some(())) = self.signal.poll().unwrap() {
                trace!("Ctrl-C received");
                if !self.shutdown {
                    if let Some(ref spirc) = self.spirc {
                        spirc.shutdown();
                    } else {
                        return Ok(Async::Ready(()));
                    }
                    self.shutdown = true;
                } else {
                    return Ok(Async::Ready(()));
                }

                progress = true;
            }

            let mut drop_spirc_and_try_to_reconnect = false;
            if let Some(ref mut spirc_task) = self.spirc_task {
                if let Async::Ready(()) = spirc_task.poll().unwrap() {
                    if self.shutdown {
                        return Ok(Async::Ready(()));
                    } else {
                        warn!("Spirc shut down unexpectedly");
                        drop_spirc_and_try_to_reconnect = true;
                    }
                    progress = true;
                }
            }
            if drop_spirc_and_try_to_reconnect {
                self.spirc_task = None;
                while (!self.auto_connect_times.is_empty())
                    && ((Instant::now() - self.auto_connect_times[0]).as_secs() > 600)
                {
                    let _ = self.auto_connect_times.remove(0);
                }

                if let Some(credentials) = self.last_credentials.clone() {
                    if self.auto_connect_times.len() >= 5 {
                        warn!("Spirc shut down too often. Not reconnecting automatically.");
                    } else {
                        self.auto_connect_times.push(Instant::now());
                        self.credentials(credentials);
                    }
                }
            }

            if let Some(ref mut player_event_channel) = self.player_event_channel {
                if let Async::Ready(Some(event)) = player_event_channel.poll().unwrap() {
                    progress = true;
                    if let Some(ref program) = self.player_event_program {
                        if let Some(child) = run_program_on_events(event, program) {
                            let child = child
                                .expect("program failed to start")
                                .map(|status| {
                                    if !status.success() {
                                        error!("child exited with status {:?}", status.code());
                                    }
                                })
                                .map_err(|e| error!("failed to wait on child process: {}", e));

                            self.handle.spawn(child);
                        }
                    }
                }
            }

            if !progress {
                return Ok(Async::NotReady);
            }
        }
    }
}

fn main() {
    if env::var("RUST_BACKTRACE").is_err() {
        env::set_var("RUST_BACKTRACE", "full")
    }
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let args: Vec<String> = std::env::args().collect();

    core.run(Main::new(handle, setup(&args))).unwrap()
}
