#[macro_use] extern crate log;
extern crate getopts;
extern crate librespot;
extern crate ctrlc;
extern crate env_logger;

use env_logger::LogBuilder;
use std::io::{stderr, Write};
use std::process::exit;
use std::thread;
use std::env;
use std::path::PathBuf;
use std::str::FromStr;

use librespot::spirc::SpircManager;
use librespot::authentication::get_credentials;
use librespot::audio_backend::{self, BACKENDS};
use librespot::cache::{Cache, DefaultCache, NoCache};
use librespot::player::Player;
use librespot::session::{Bitrate, Config, Session};
use librespot::mixer::{self, Mixer};

use librespot::version;

fn usage(program: &str, opts: &getopts::Options) -> String {
    let brief = format!("Usage: {} [options]", program);
    format!("{}", opts.usage(&brief))
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

fn setup(args: &[String]) -> (Session, Player, Box<Mixer + Send>) {
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

    let bitrate = matches.opt_str("b").as_ref()
        .map(|bitrate| Bitrate::from_str(bitrate).expect("Invalid bitrate"))
        .unwrap_or(Bitrate::Bitrate160);

    let config = Config {
        user_agent: version::version_string(),
        device_name: matches.opt_str("name").unwrap(),
        bitrate: bitrate,
        onstart: matches.opt_str("onstart"),
        onstop: matches.opt_str("onstop"),
    };

    let cache = matches.opt_str("c").map(|cache_location| {
        Box::new(DefaultCache::new(PathBuf::from(cache_location)).unwrap()) 
            as Box<Cache + Send + Sync>
    }).unwrap_or_else(|| Box::new(NoCache));

    let session = Session::new(config, cache);

    let credentials = get_credentials(&session, matches.opt_str("username"),
    matches.opt_str("password"));
    session.login(credentials).unwrap();
 
    let mixer_name = matches.opt_str("mixer");
    let mixer = mixer::find(mixer_name.as_ref()).expect("Invalid mixer");
    let audio_filter = mixer.get_audio_filter();

    let device_name = matches.opt_str("device");
    let player = Player::new(session.clone(), audio_filter, move || {
        (backend)(device_name.as_ref().map(AsRef::as_ref))
    });

    (session, player, mixer)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (session, player, mixer) = setup(&args);

    let spirc = SpircManager::new(session.clone(), player, mixer);
    let spirc_signal = spirc.clone();
    thread::spawn(move || spirc.run());

    ctrlc::set_handler(move || {
        spirc_signal.send_goodbye();
        exit(0);
    });

    loop {
        session.poll();
    }
}
