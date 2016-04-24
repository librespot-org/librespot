use getopts;
use rpassword;
use std::fs::File;
use std::io::{stdout, Read, Write};
use std::path::PathBuf;
use std::path::Path;
use std::process::exit;

use audio_backend::{BACKENDS, Sink};
use authentication::{Credentials, facebook_login, discovery_login};
use cache::{Cache, DefaultCache, NoCache};
use player::Player;
use session::{Bitrate, Config, Session};
use version;
use APPKEY;

pub fn find_backend(name: Option<&str>) -> &'static (Fn() -> Box<Sink> + Send + Sync) {
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

pub fn load_appkey<P: AsRef<Path>>(path: Option<P>) -> Vec<u8> {
    path.map(|path| {
        let mut file = File::open(path).expect("Could not open app key.");

        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();

        data
    }).or_else(|| APPKEY.map(ToOwned::to_owned)).unwrap()
}

pub fn add_session_arguments(opts: &mut getopts::Options) {
    opts.optopt("c", "cache", "Path to a directory where files will be cached.", "CACHE")
        .reqopt("n", "name", "Device name", "NAME")
        .optopt("b", "bitrate", "Bitrate (96, 160 or 320). Defaults to 160", "BITRATE");

    if APPKEY.is_none() {
        opts.reqopt("a", "appkey", "Path to a spotify appkey", "APPKEY");
    } else {
        opts.optopt("a", "appkey", "Path to a spotify appkey", "APPKEY");
    };
}

pub fn add_authentication_arguments(opts: &mut getopts::Options) {
    opts.optopt("u", "username", "Username to sign in with", "USERNAME")
        .optopt("p", "password", "Password", "PASSWORD");

    if cfg!(feature = "facebook") {
        opts.optflag("", "facebook", "Login with a Facebook account");
    }
}

pub fn add_player_arguments(opts: &mut getopts::Options) {
    opts.optopt("", "backend", "Audio backend to use. Use '?' to list options", "BACKEND");
}

pub fn create_session(matches: &getopts::Matches) -> Session {
    info!("librespot {} ({}). Built on {}.",
             version::short_sha(),
             version::commit_date(),
             version::short_now());

    let appkey = load_appkey(matches.opt_str("a"));
    let name = matches.opt_str("n").unwrap();
    let bitrate = match matches.opt_str("b").as_ref().map(String::as_ref) {
        None => Bitrate::Bitrate160, // default value

        Some("96") => Bitrate::Bitrate96,
        Some("160") => Bitrate::Bitrate160,
        Some("320") => Bitrate::Bitrate320,
        Some(b) => {
            error!("Invalid bitrate {}", b);
            exit(1)
        }
    };

    let cache = matches.opt_str("c").map(|cache_location| {
        Box::new(DefaultCache::new(PathBuf::from(cache_location)).unwrap()) as Box<Cache + Send + Sync>
    }).unwrap_or_else(|| Box::new(NoCache) as Box<Cache + Send + Sync>);

    let config = Config {
        application_key: appkey,
        user_agent: version::version_string(),
        device_name: name,
        bitrate: bitrate,
    };

    Session::new(config, cache)
}

pub fn get_credentials(session: &Session, matches: &getopts::Matches) -> Credentials {
    let credentials = session.cache().get_credentials();

    match (matches.opt_str("username"),
           matches.opt_str("password"),
           credentials) {

        (Some(username), Some(password), _)
            => Credentials::with_password(username, password),

        (Some(ref username), _, Some(ref credentials)) if *username == credentials.username
            => credentials.clone(),

        (Some(username), None, _) => {
            print!("Password for {}: ", username);
            stdout().flush().unwrap();
            let password = rpassword::read_password().unwrap();
            Credentials::with_password(username.clone(), password)
        }

        (None, _, _) if cfg!(feature = "facebook") && matches.opt_present("facebook")
            => facebook_login().unwrap(),

        (None, _, Some(credentials))
            => credentials,

        (None, _, None) if cfg!(feature = "discovery") => {
            info!("No username provided and no stored credentials, starting discovery ...");
            discovery_login(&session.config().device_name, session.device_id()).unwrap()
        }

        (None, _, None) => {
            error!("No credentials provided");
            exit(1)
        }
    }
}

pub fn create_player(session: &Session, matches: &getopts::Matches) -> Player {
    let make_backend = find_backend(matches.opt_str("backend").as_ref().map(AsRef::as_ref));

    Player::new(session.clone(), move || make_backend())
}
