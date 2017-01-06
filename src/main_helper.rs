use env_logger::LogBuilder;
use getopts;
use rpassword;
use std::env;
use std::io::{stderr, Write};
use std::path::PathBuf;
use std::process::exit;

use audio_backend::{BACKENDS, Sink};
use authentication::{Credentials, discovery_login};
use cache::{Cache, DefaultCache, NoCache};
use player::Player;
use session::{Bitrate, Config, Session};
use version;

pub fn find_backend(name: Option<&str>) -> &'static (Fn(Option<&str>) -> Box<Sink> + Send + Sync) {
    match name {
        Some("?") => {
            println!("Available Backends : ");
            for (&(name, _), idx) in BACKENDS.iter().zip(0..) {
                if idx == 0 {
                    println!("- {} (default)", name);
                } else {
                    println!("- {}", name);
                }
            }

            exit(0);
        },
        Some(name) => {
            BACKENDS.iter().find(|backend| name == backend.0).expect("Unknown backend").1
        },
        None => {
            BACKENDS.first().expect("No backends were enabled at build time").1
        }
    }
}

pub fn add_session_arguments(opts: &mut getopts::Options) {
    opts.optopt("c", "cache", "Path to a directory where files will be cached.", "CACHE")
        .reqopt("n", "name", "Device name", "NAME")
        .optopt("b", "bitrate", "Bitrate (96, 160 or 320). Defaults to 160", "BITRATE")
        .optopt("", "onstart", "Run PROGRAM when playback is about to begin.", "PROGRAM")
        .optopt("", "onstop", "Run PROGRAM when playback has ended.", "PROGRAM")
        .optflag("v", "verbose", "Enable verbose output");
}

pub fn add_authentication_arguments(opts: &mut getopts::Options) {
    opts.optopt("u", "username", "Username to sign in with", "USERNAME")
        .optopt("p", "password", "Password", "PASSWORD");
}

pub fn add_player_arguments(opts: &mut getopts::Options) {
    opts.optopt("", "backend", "Audio backend to use. Use '?' to list options", "BACKEND")
        .optopt("", "device", "Audio device to use. Use '?' to list options", "DEVICE");
}

pub fn session_from_matches(matches: &getopts::Matches) -> Session {
    create_session(matches.opt_str("n").unwrap(),
                   matches.opt_str("b").as_ref().map(String::as_ref),
                   matches.opt_str("c").as_ref().map(String::as_ref),
                   matches.opt_str("onstart"),
                   matches.opt_str("onstop"))
}

pub fn create_session(name: String,
                      bitrate: Option<&str>,
                      cache: Option<&str>,
                      onstart: Option<String>,
                      onstop: Option<String>)
                      -> Session {
    info!("librespot {} ({}). Built on {}.",
             version::short_sha(),
             version::commit_date(),
             version::short_now());

    let bitrate = match bitrate {
        None => Bitrate::Bitrate160, // default value

        Some("96") => Bitrate::Bitrate96,
        Some("160") => Bitrate::Bitrate160,
        Some("320") => Bitrate::Bitrate320,
        Some(b) => {
            error!("Invalid bitrate {}", b);
            exit(1)
        }
    };

    let cache = cache.map(|cache_location| {
        Box::new(DefaultCache::new(PathBuf::from(cache_location)).unwrap()) as Box<Cache + Send + Sync>
    }).unwrap_or_else(|| Box::new(NoCache) as Box<Cache + Send + Sync>);

    let config = Config {
        user_agent: version::version_string(),
        device_name: name,
        bitrate: bitrate,
        onstart: onstart,
        onstop: onstop,
    };

    Session::new(config, cache)
}

pub fn credentials_from_matches(session: &Session,
                                matches: &getopts::Matches) -> Credentials {
    get_credentials(session, matches.opt_str("username"), matches.opt_str("password"))
}

pub fn get_credentials(session: &Session, username: Option<String>,
                       password: Option<String>) -> Credentials {
    let credentials = session.cache().get_credentials();

    match (username, password, credentials) {
        (Some(username), Some(password), _)
            => Credentials::with_password(username, password),

        (Some(ref username), _, Some(ref credentials)) if *username == credentials.username
            => credentials.clone(),

        (Some(username), None, _) => {
            write!(stderr(), "Password for {}: ", username).unwrap();
            stderr().flush().unwrap();
            let password = rpassword::read_password().unwrap();
            Credentials::with_password(username.clone(), password)
        }

        (None, _, Some(credentials))
            => credentials,

        (None, _, None) => {
            info!("No username provided and no stored credentials, starting discovery ...");
            discovery_login(&session.config().device_name, session.device_id()).unwrap()
        }
    }
}

pub fn player_from_matches(session: &Session, matches: &getopts::Matches) -> Player {
    create_player(session, matches.opt_str("backend").as_ref().map(String::as_ref)
                  , matches.opt_str("device"))
}

pub fn create_player(session: &Session, backend_name: Option<&str>
                     , device_name: Option<String>) -> Player {
    let make_backend = find_backend(backend_name);

    Player::new(session.clone(), move || {
        make_backend(device_name.as_ref().map(AsRef::as_ref))
    })
}

pub fn setup_logging(matches: &getopts::Matches) {
    let verbose = matches.opt_present("verbose");
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
