extern crate getopts;
extern crate librespot;
extern crate rpassword;

use rpassword::read_password;
use std::clone::Clone;
use std::fs::File;
use std::io::{stdout, Read, Write};
use std::path::PathBuf;
use std::thread;

use librespot::audio_sink::DefaultSink;
use librespot::authentication::Credentials;
use librespot::discovery::DiscoveryManager;
use librespot::player::Player;
use librespot::session::{Bitrate, Config, Session};
use librespot::spirc::SpircManager;
use librespot::util::version::version_string;

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
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();

    let mut opts = getopts::Options::new();
    opts.optopt("u", "username", "Username to sign in with", "USERNAME")
        .optopt("p", "password", "Password", "PASSWORD")
        .optopt("c", "cache", "Path to a directory where files will be cached.", "CACHE")
        .reqopt("n", "name", "Device name", "NAME")
        .optopt("b", "bitrate", "Bitrate (96, 160 or 320). Defaults to 160", "BITRATE");

    if APPKEY.is_none() {
        opts.reqopt("a", "appkey", "Path to a spotify appkey", "APPKEY");
    } else {
        opts.optopt("a", "appkey", "Path to a spotify appkey", "APPKEY");
    };

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            print!("Error: {}\n{}", f.to_string(), usage(&*program, &opts));
            return;
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
    let cache_location = matches.opt_str("c").map(PathBuf::from);
    let name = matches.opt_str("n").unwrap();

    let credentials = username.map(|u| {
        let password = matches.opt_str("p")
                              .or_else(|| std::env::var(PASSWORD_ENV_NAME).ok())
                              .unwrap_or_else(|| {
                                  print!("Password: ");
                                  stdout().flush().unwrap();
                                  read_password().unwrap()
                              });

        (u, password)
    });
    std::env::remove_var(PASSWORD_ENV_NAME);

    let bitrate = match matches.opt_str("b").as_ref().map(String::as_ref) {
        None => Bitrate::Bitrate160, // default value

        Some("96") => Bitrate::Bitrate96,
        Some("160") => Bitrate::Bitrate160,
        Some("320") => Bitrate::Bitrate320,
        Some(b) => panic!("Invalid bitrate {}", b),
    };

    let config = Config {
        application_key: appkey,
        user_agent: version_string(),
        device_name: name,
        cache_location: cache_location.clone(),
        bitrate: bitrate,
    };

    let session = Session::new(config);

    let credentials_path = cache_location.map(|c| c.join("credentials.json"));

    let credentials = credentials.map(|(username, password)| {
        Credentials::with_password(username, password)
    }).or_else(|| {
        credentials_path.as_ref()
                        .and_then(|p| File::open(p).ok())
                        .map(Credentials::from_reader)
    }).unwrap_or_else(|| {
        println!("No username provided and no stored credentials, starting discovery ...");

        let mut discovery = DiscoveryManager::new(session.clone());
        discovery.run()
    });

    let reusable_credentials = session.login(credentials).unwrap();
    if let Some(path) = credentials_path {
        reusable_credentials.save_to_file(path);
    }

    let player = Player::new(session.clone(), || DefaultSink::open());
    let spirc = SpircManager::new(session.clone(), player);
    thread::spawn(move || spirc.run());

    loop {
        session.poll();
    }
}
