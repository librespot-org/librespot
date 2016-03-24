extern crate getopts;
extern crate librespot;
extern crate rpassword;
extern crate env_logger;
#[macro_use]
extern crate log;

use rpassword::read_password;
use std::clone::Clone;
use std::fs::File;
use std::io::{stdout, Read, Write};
use std::path::PathBuf;
use std::thread;
use std::env;

use librespot::audio_backend::BACKENDS;
use librespot::authentication::{Credentials, facebook_login, discovery_login};
use librespot::cache::{Cache, DefaultCache, NoCache};
use librespot::player::Player;
use librespot::session::{Bitrate, Config, Session};
use librespot::spirc::SpircManager;
use librespot::version;

static PASSWORD_ENV_NAME: &'static str = "SPOTIFY_PASSWORD";

fn usage(program: &str, opts: &getopts::Options) -> String {
    let brief = format!("Usage: {} [options]", program);
    format!("{}", opts.usage(&brief))
}

#[cfg(feature = "static-appkey")]
static APPKEY: Option<&'static [u8]> = Some(include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/spotify_appkey.key")));
#[cfg(not(feature = "static-appkey"))]
static APPKEY: Option<&'static [u8]> = None;

fn main() {
    let rust_log = "RUST_LOG";
    if let Err(_) = env::var(rust_log) {
        env::set_var(rust_log, "debug")
    }

    env_logger::init().unwrap();

    info!("librespot {} ({}). Built on {}.",
             version::short_sha(),
             version::commit_date(),
             version::short_now());

    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();

    let mut opts = getopts::Options::new();
    opts.optopt("u", "username", "Username to sign in with", "USERNAME")
        .optopt("p", "password", "Password", "PASSWORD")
        .optopt("c", "cache", "Path to a directory where files will be cached.", "CACHE")
        .reqopt("n", "name", "Device name", "NAME")
        .optopt("b", "bitrate", "Bitrate (96, 160 or 320). Defaults to 160", "BITRATE")
        .optopt("", "backend", "Audio backend to use. Use '?' to list options", "BACKEND");

    if APPKEY.is_none() {
        opts.reqopt("a", "appkey", "Path to a spotify appkey", "APPKEY");
    } else {
        opts.optopt("a", "appkey", "Path to a spotify appkey", "APPKEY");
    };

    if cfg!(feature = "facebook") {
        opts.optflag("", "facebook", "Login with a Facebook account");
    }

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            print!("Error: {}\n{}", f.to_string(), usage(&*program, &opts));
            return;
        }
    };

    let make_backend = match matches.opt_str("backend").as_ref().map(AsRef::as_ref) {
        Some("?") => {
            println!("Available Backends : ");
            for (&(name, _), idx) in BACKENDS.iter().zip(0..) {
                if idx == 0 {
                    println!("- {} (default)", name);
                } else {
                    println!("- {}", name);
                }
            }

            return;
        },
        Some(name) => {
            BACKENDS.iter().find(|backend| name == backend.0).expect("Unknown backend").1
        },
        None => {
            BACKENDS.first().expect("No backends were enabled at build time").1
        }
    };

    let appkey = matches.opt_str("a").map(|appkey_path| {
        let mut file = File::open(appkey_path)
                            .expect("Could not open app key.");

        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();

        data
    }).or_else(|| APPKEY.map(ToOwned::to_owned)).unwrap();

    let username = matches.opt_str("u");
    let name = matches.opt_str("n").unwrap();

    let cache = matches.opt_str("c").map(|cache_location| {
        Box::new(DefaultCache::new(PathBuf::from(cache_location)).unwrap()) as Box<Cache + Send + Sync>
    }).unwrap_or_else(|| Box::new(NoCache) as Box<Cache + Send + Sync>);

    let bitrate = match matches.opt_str("b").as_ref().map(String::as_ref) {
        None => Bitrate::Bitrate160, // default value

        Some("96") => Bitrate::Bitrate96,
        Some("160") => Bitrate::Bitrate160,
        Some("320") => Bitrate::Bitrate320,
        Some(b) => panic!("Invalid bitrate {}", b),
    };

    let config = Config {
        application_key: appkey,
        user_agent: version::version_string(),
        device_name: name,
        bitrate: bitrate,
    };

    let stored_credentials = cache.get_credentials();

    let session = Session::new(config, cache);

    let credentials = username.map(|username| {
        let password = matches.opt_str("p")
                              .or_else(|| std::env::var(PASSWORD_ENV_NAME).ok())
                              .unwrap_or_else(|| {
                                  print!("Password: ");
                                  stdout().flush().unwrap();
                                  read_password().unwrap()
                              });

        Credentials::with_password(username, password)
    }).or_else(|| {
        if cfg!(feature = "facebook") && matches.opt_present("facebook") {
            Some(facebook_login().unwrap())
        } else {
            None
        }
    }).or(stored_credentials)
      .or_else(|| {
        if cfg!(feature = "discovery") {
            info!("No username provided and no stored credentials, starting discovery ...");
            Some(discovery_login(&session.config().device_name,
                                 session.device_id()).unwrap())
        } else {
            None
        }
    }).expect("No username provided and no stored credentials.");

    std::env::remove_var(PASSWORD_ENV_NAME);

    let reusable_credentials = session.login(credentials).unwrap();
    session.cache().put_credentials(&reusable_credentials);

    let player = Player::new(session.clone(), move || make_backend());

    let spirc = SpircManager::new(session.clone(), player);
    thread::spawn(move || spirc.run());

    loop {
        session.poll();
    }
}
