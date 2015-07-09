#![feature(scoped)]
#![allow(deprecated)]

extern crate librespot;

use std::clone::Clone;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::thread;
use std::path::PathBuf;

use librespot::session::{Config, Session};
use librespot::util::version::version_string;
use librespot::player::Player;
use librespot::spirc::SpircManager;

fn main() {
    let mut args = std::env::args().skip(1);
    let mut appkey_file = File::open(Path::new(&args.next().unwrap())).unwrap();
    let username = args.next().unwrap();
    let password = args.next().unwrap();
    let cache_location = args.next().unwrap();
    let name = args.next().unwrap();

    let mut appkey = Vec::new();
    appkey_file.read_to_end(&mut appkey).unwrap();

    let config = Config {
        application_key: appkey,
        user_agent: version_string(),
        device_id: name.clone(),
        cache_location: PathBuf::from(cache_location)
    };
    let session = Session::new(config);
    session.login(username.clone(), password);
    session.poll();

    let poll_thread = thread::scoped(|| {
        loop {
            session.poll();
        }
    });

    let player = Player::new(&session);

    let mut spirc_manager = SpircManager::new(&session, player, username, name);
    spirc_manager.run();

    poll_thread.join();
}

