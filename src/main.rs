#![feature(plugin,scoped)]
#![allow(deprecated)]
//#![allow(unused_imports,dead_code)]

#![plugin(protobuf_macros)]
#[macro_use] extern crate lazy_static;


extern crate byteorder;
extern crate crypto;
extern crate gmp;
extern crate num;
extern crate portaudio;
extern crate protobuf;
extern crate shannon;
extern crate rand;
extern crate readall;
extern crate vorbis;

#[macro_use] extern crate librespot;

use std::clone::Clone;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::thread;
use std::path::PathBuf;

use librespot::metadata::{AlbumRef, ArtistRef, TrackRef};
use librespot::session::{Config, Session};
use librespot::util::SpotifyId;
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

    let mut spirc_manager = SpircManager::new(&session, &player, username, name);
    spirc_manager.run();

    poll_thread.join();
}

fn print_track(session: &Session, track_id: SpotifyId) {
    let track : TrackRef = session.metadata(track_id);

    let album : AlbumRef = {
        let handle = track.wait();
        let data = handle.unwrap();
        eprintln!("{}", data.name);
        session.metadata(data.album)
    };

    let artists : Vec<ArtistRef> = {
        let handle = album.wait();
        let data = handle.unwrap();
        eprintln!("{}", data.name);
        data.artists.iter().map(|id| {
            session.metadata(*id)
        }).collect()
    };

    for artist in artists {
        let handle = artist.wait();
        let data = handle.unwrap();
        eprintln!("{}", data.name);
    }
}
