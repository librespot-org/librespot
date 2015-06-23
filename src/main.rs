#![crate_name = "librespot"]

#![feature(alloc,plugin,core,collections,std_misc,zero_one)]

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

extern crate librespot_protocol;

#[macro_use] mod util;
mod audio_decrypt;
mod audio_file;
mod audio_key;
mod connection;
mod keys;
mod mercury;
mod metadata;
mod player;
mod session;
mod stream;
mod subsystem;

use std::clone::Clone;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use metadata::{MetadataCache, AlbumRef, ArtistRef, TrackRef};
use session::{Config, Session};
use util::SpotifyId;
use player::Player;

fn main() {
    let mut args = std::env::args().skip(1);
    let mut appkey_file = File::open(Path::new(&args.next().unwrap())).unwrap();
    let username = args.next().unwrap();
    let password = args.next().unwrap();
    let track_uri = args.next().unwrap();
    let track_id = SpotifyId::from_base62(track_uri.split(':').nth(2).unwrap());

    let mut appkey = Vec::new();
    appkey_file.read_to_end(&mut appkey).unwrap();

    let config = Config {
        application_key: appkey,
        user_agent: "ABC".to_string(),
        device_id: "ABC".to_string()
    };
    let session = Session::new(config);
    session.login(username, password);
    session.poll();

    let mut cache = MetadataCache::new(session.metadata.clone());
    let track : TrackRef = cache.get(track_id);

    let album : AlbumRef = {
        let handle = track.wait();
        let data = handle.unwrap();
        eprintln!("{}", data.name);
        cache.get(data.album)
    };

    let artists : Vec<ArtistRef> = {
        let handle = album.wait();
        let data = handle.unwrap();
        eprintln!("{}", data.name);
        data.artists.iter().map(|id| {
            cache.get(*id)
        }).collect()
    };

    for artist in artists {
        let handle = artist.wait();
        let data = handle.unwrap();
        eprintln!("{}", data.name);
    }
    
    Player::play(&session, track);

    loop {
        session.poll();
    }
}

