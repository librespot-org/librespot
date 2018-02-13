#[macro_use] extern crate log;
extern crate env_logger;
extern crate futures;
extern crate getopts;
extern crate librespot;
extern crate sentry_rs;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_signal;

use env_logger::LogBuilder;
use futures::{Future, Async, Poll, Stream};
use std::env;
use std::io::{self, stderr, Write};
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;
use std::thread;
use sentry_rs::models::SentryCredentials;
use sentry_rs::Sentry;
use tokio_core::reactor::{Handle, Core};
use tokio_io::IoStream;
use std::mem;

use librespot::core::authentication::{get_credentials, Credentials};
use librespot::core::cache::Cache;
use librespot::core::config::{DeviceType, SessionConfig, ConnectConfig};
use librespot::core::session::Session;
use librespot::core::version;

use librespot::playback::audio_backend::{self, Sink, BACKENDS};
use librespot::playback::config::{Bitrate, PlayerConfig};
use librespot::connect::discovery::{discovery, DiscoveryStream};
use librespot::playback::mixer::{self, Mixer};
use librespot::playback::player::Player;
use librespot::connect::spirc::{Spirc, SpircTask};

fn usage(program: &str, opts: &getopts::Options) -> String {
    let brief = format!("Usage: {} [options]", program);
    opts.usage(&brief)
}

fn setup_logging(verbose: bool) {
    let mut builder = LogBuilder::new();
    match env::var("RUST_LOG") {
        Ok(config) => {
            builder.parse(&config);
            builder.init().unwrap();

            if verbose {
                warn!("`--verbose` flag overidden by `RUST_LOG` environment variable");
            }
        }
        Err(_) => {
            if verbose {
                builder.parse("mdns=info,librespot=trace");
            } else {
                builder.parse("mdns=info,librespot=info");
            }
            builder.init().unwrap();
        }
    }
}

fn setup_error_reporting() {
    thread::spawn(move || {
        let credentials = SentryCredentials {
            key: env::var("SENTRY_KEY").unwrap_or("576fd4f54c2346e9aa9b3fd733956d89".to_owned()),
            secret: env::var("SENTRY_SECRET").unwrap_or("d34412cb53e14625908209b1ce7d00c3".to_owned()),
            host: Some(env::var("SENTRY_HOST").unwrap_or("sentry.io".to_owned())),
            project_id: env::var("SENTRY_PROJECT_ID").unwrap_or("279607".to_owned()),
        };
        const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
        let sentry = Sentry::new(
            format!("librespot_{}_{}", version::short_sha(), version::build_id()),
            format!("{}", VERSION.unwrap_or("0.1.0")),
            format!("Debug"),
            credentials
        );
        sentry.register_panic_handler();
    });
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
    backend: fn(Option<String>) -> Box<Sink>,
    device: Option<String>,

    mixer: fn() -> Box<Mixer>,

    cache: Option<Cache>,
    player_config: PlayerConfig,
    session_config: SessionConfig,
    connect_config: ConnectConfig,
    credentials: Option<Credentials>,
    enable_discovery: bool,
    zeroconf_port: u16,
}

fn setup(args: &[String]) -> Setup {
    let mut opts = getopts::Options::new();
    opts.optopt("c", "cache", "Path to a directory where files will be cached.", "CACHE")
        .optflag("", "disable-audio-cache", "Disable caching of the audio data.")
        .reqopt("n", "name", "Device name", "NAME")
        .optopt("", "device-type", "Displayed device type", "DEVICE_TYPE")
        .optopt("b", "bitrate", "Bitrate (96, 160 or 320). Defaults to 160", "BITRATE")
        .optopt("", "onstart", "Run PROGRAM when playback is about to begin.", "PROGRAM")
        .optopt("", "onstop", "Run PROGRAM when playback has ended.", "PROGRAM")
        .optflag("v", "verbose", "Enable verbose output")
        .optopt("u", "username", "Username to sign in with", "USERNAME")
        .optopt("p", "password", "Password", "PASSWORD")
        .optflag("", "disable-discovery", "Disable discovery mode")
        .optopt("", "backend", "Audio backend to use. Use '?' to list options", "BACKEND")
        .optopt("", "device", "Audio device to use. Use '?' to list options if using portaudio", "DEVICE")
        .optopt("", "mixer", "Mixer to use", "MIXER")
        .optopt("", "initial-volume", "Initial volume in %, once connected (must be from 0 to 100)", "VOLUME")
        .optopt("", "zeroconf-port", "The port the internal server advertised over zeroconf uses.", "ZEROCONF_PORT");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            writeln!(stderr(), "error: {}\n{}", f.to_string(), usage(&args[0], &opts)).unwrap();
            exit(1);
        }
    };

    let verbose = matches.opt_present("verbose");
    setup_logging(verbose);

    info!("librespot {} ({}). Built on {}. Build ID: {}",
             version::short_sha(),
             version::commit_date(),
             version::short_now(),
             version::build_id());

    let backend_name = matches.opt_str("backend");
    if backend_name == Some("?".into()) {
        list_backends();
        exit(0);
    }

    let backend = audio_backend::find(backend_name)
        .expect("Invalid backend");

    let device = matches.opt_str("device");

    let mixer_name = matches.opt_str("mixer");
    let mixer = mixer::find(mixer_name.as_ref())
        .expect("Invalid mixer");

    let initial_volume = matches
        .opt_str("initial-volume")
        .map(|volume| {
            let volume = volume.parse::<i32>().unwrap();
            if volume < 0 || volume > 100 {
                panic!("Initial volume must be in the range 0-100");
            }
            volume * 0xFFFF / 100
        })
        .unwrap_or(0x8000);

    let zeroconf_port =
    matches.opt_str("zeroconf-port")
           .map(|port| port.parse::<u16>().unwrap())
           .unwrap_or(0);

    let name = matches.opt_str("name").unwrap();
    let use_audio_cache = !matches.opt_present("disable-audio-cache");

    let cache = matches.opt_str("c").map(|cache_location| {
        Cache::new(PathBuf::from(cache_location), use_audio_cache)
    });

    let credentials = {
        let cached_credentials = cache.as_ref().and_then(Cache::credentials);

        get_credentials(
            matches.opt_str("username"),
            matches.opt_str("password"),
            cached_credentials
        )
    };

    let session_config = {
        let device_id = librespot::core::session::device_id(&name);

        SessionConfig {
            user_agent: version::version_string(),
            device_id: device_id,
        }
    };

    let player_config = {
        let bitrate = matches.opt_str("b").as_ref()
            .map(|bitrate| Bitrate::from_str(bitrate).expect("Invalid bitrate"))
            .unwrap_or(Bitrate::default());

        PlayerConfig {
            bitrate: bitrate,
            onstart: matches.opt_str("onstart"),
            onstop: matches.opt_str("onstop"),
        }
    };

    let connect_config = {
        let device_type = matches.opt_str("device-type").as_ref()
            .map(|device_type| DeviceType::from_str(device_type).expect("Invalid device type"))
            .unwrap_or(DeviceType::default());

        ConnectConfig {
            name: name,
            device_type: device_type,
            volume: initial_volume,
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
    }
}

struct Main {
    cache: Option<Cache>,
    player_config: PlayerConfig,
    session_config: SessionConfig,
    connect_config: ConnectConfig,
    backend: fn(Option<String>) -> Box<Sink>,
    device: Option<String>,
    mixer: fn() -> Box<Mixer>,
    handle: Handle,

    discovery: Option<DiscoveryStream>,
    signal: IoStream<()>,

    spirc: Option<Spirc>,
    spirc_task: Option<SpircTask>,
    connect: Box<Future<Item=Session, Error=io::Error>>,

    shutdown: bool,
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

            connect: Box::new(futures::future::empty()),
            discovery: None,
            spirc: None,
            spirc_task: None,
            shutdown: false,
            signal: Box::new(tokio_signal::ctrl_c(&handle).flatten_stream()),
        };

        if setup.enable_discovery {
            let config = task.connect_config.clone();
            let device_id = task.session_config.device_id.clone();

            task.discovery = Some(discovery(&handle, config, device_id, setup.zeroconf_port).unwrap());
        }

        if let Some(credentials) = setup.credentials {
            task.credentials(credentials);
        }

        task
    }

    fn credentials(&mut self, credentials: Credentials) {
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

            if let Some(Async::Ready(Some(creds))) = self.discovery.as_mut().map(|d| d.poll().unwrap()) {
                if let Some(ref spirc) = self.spirc {
                    spirc.shutdown();
                }
                self.credentials(creds);

                progress = true;
            }

            if let Async::Ready(session) = self.connect.poll().unwrap() {
                self.connect = Box::new(futures::future::empty());
                let device = self.device.clone();
                let mixer = (self.mixer)();
                let player_config = self.player_config.clone();
                let connect_config = self.connect_config.clone();

                let audio_filter = mixer.get_audio_filter();
                let backend = self.backend;
                let player = Player::new(player_config, session.clone(), audio_filter, move || {
                    (backend)(device)
                });

                let (spirc, spirc_task) = Spirc::new(connect_config, session, player, mixer);
                self.spirc = Some(spirc);
                self.spirc_task = Some(spirc_task);

                progress = true;
            }

            if let Async::Ready(Some(())) = self.signal.poll().unwrap() {
                if !self.shutdown {
                    if let Some(ref spirc) = self.spirc {
                        spirc.shutdown();
                    }
                    self.shutdown = true;
                } else {
                    return Ok(Async::Ready(()));
                }

                progress = true;
            }

            if let Some(ref mut spirc_task) = self.spirc_task {
                if let Async::Ready(()) = spirc_task.poll().unwrap() {
                    if self.shutdown {
                        return Ok(Async::Ready(()));
                    } else {
                        panic!("Spirc shut down unexpectedly");
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
    setup_error_reporting();
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let args: Vec<String> = std::env::args().collect();

    core.run(Main::new(handle, setup(&args))).unwrap()
}
