#[macro_use] extern crate log;
extern crate ctrlc;
extern crate env_logger;
extern crate futures;
extern crate getopts;
extern crate librespot;
extern crate tokio_core;

use env_logger::LogBuilder;
use futures::Future;
use std::cell::{RefCell, Cell};
use std::env;
use std::io::{stderr, Write};
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;
use tokio_core::reactor::Core;

use librespot::spirc::Spirc;
use librespot::authentication::{get_credentials, Credentials};
use librespot::audio_backend::{self, Sink, BACKENDS};
use librespot::cache::Cache;
use librespot::player::Player;
use librespot::session::{Bitrate, Config, Session};
use librespot::mixer::{self, Mixer};

use librespot::version;

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

struct Setup {
    backend: fn(Option<String>) -> Box<Sink>,
    mixer: Box<Mixer + Send>,
    cache: Option<Cache>,
    config: Config,
    credentials: Credentials,
    device: Option<String>,
}

fn setup(args: &[String]) -> Setup {
    let mut opts = getopts::Options::new();
    opts.optopt("c", "cache", "Path to a directory where files will be cached.", "CACHE")
        .reqopt("n", "name", "Device name", "NAME")
        .optopt("b", "bitrate", "Bitrate (96, 160 or 320). Defaults to 160", "BITRATE")
        .optopt("", "onstart", "Run PROGRAM when playback is about to begin.", "PROGRAM")
        .optopt("", "onstop", "Run PROGRAM when playback has ended.", "PROGRAM")
        .optflag("v", "verbose", "Enable verbose output")
        .optopt("u", "username", "Username to sign in with", "USERNAME")
        .optopt("p", "password", "Password", "PASSWORD")
        .optopt("", "backend", "Audio backend to use. Use '?' to list options", "BACKEND")
        .optopt("", "device", "Audio device to use. Use '?' to list options", "DEVICE")
        .optopt("", "mixer", "Mixer to use", "MIXER");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            writeln!(stderr(), "error: {}\n{}", f.to_string(), usage(&args[0], &opts)).unwrap();
            exit(1);
        }
    };

    let verbose = matches.opt_present("verbose");
    setup_logging(verbose);

    info!("librespot {} ({}). Built on {}.",
             version::short_sha(),
             version::commit_date(),
             version::short_now());

    let backend_name = matches.opt_str("backend");
    if backend_name == Some("?".into()) {
        list_backends();
        exit(0);
    }

    let backend = audio_backend::find(backend_name.as_ref())
        .expect("Invalid backend");

    let mixer_name = matches.opt_str("mixer");
    let mixer = mixer::find(mixer_name.as_ref())
        .expect("Invalid mixer");

    let bitrate = matches.opt_str("b").as_ref()
        .map(|bitrate| Bitrate::from_str(bitrate).expect("Invalid bitrate"))
        .unwrap_or(Bitrate::Bitrate160);

    let name = matches.opt_str("name").unwrap();
    let device_id = librespot::session::device_id(&name);

    let cache = matches.opt_str("c").map(|cache_location| {
        Cache::new(PathBuf::from(cache_location))
    });

    let cached_credentials = cache.as_ref().and_then(Cache::credentials);

    let credentials = get_credentials(&name, &device_id,
                                      matches.opt_str("username"),
                                      matches.opt_str("password"),
                                      cached_credentials);

    let config = Config {
        user_agent: version::version_string(),
        name: name,
        device_id: device_id,
        bitrate: bitrate,
        onstart: matches.opt_str("onstart"),
        onstop: matches.opt_str("onstop"),
    };

    let device = matches.opt_str("device");

    Setup {
        backend: backend,
        mixer: mixer,
        cache: cache,
        config: config,
        credentials: credentials,
        device: device,
    }
}

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let args: Vec<String> = std::env::args().collect();

    let Setup { backend, mixer, cache, config, credentials, device }
        = setup(&args);

    let connection = Session::connect(config, credentials, cache, handle);

    let task = connection.and_then(move |session| {
        let audio_filter = mixer.get_audio_filter();
        let player = Player::new(session.clone(), audio_filter, move || {
            (backend)(device)
        });

        let (spirc, task) = Spirc::new(session.clone(), player, mixer);
        let spirc = RefCell::new(spirc);

        let shutting_down = Cell::new(false);
        ctrlc::set_handler(move || {
            if shutting_down.get() {
                warn!("Forced shutdown");
                exit(1);
            } else {
                info!("Shutting down");
                spirc.borrow_mut().shutdown();
                shutting_down.set(true);
            }
        });

        task.map_err(|()| panic!("spirc error"))
    });

    core.run(task).unwrap()
}
