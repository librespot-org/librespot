#![crate_name = "librespot"]

#![feature(plugin,zero_one,iter_arith,slice_position_elem,slice_bytes,bitset,mpsc_select,arc_weak,append)]
#![allow(unused_imports,dead_code)]

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
use std::sync::mpsc;

use metadata::{MetadataCache, AlbumRef, ArtistRef, TrackRef};
use session::{Config, Session};
use util::SpotifyId;
use player::Player;
use mercury::{MercuryRequest, MercuryMethod};
use librespot_protocol as protocol;

fn main() {
    let mut args = std::env::args().skip(1);
    let mut appkey_file = File::open(Path::new(&args.next().unwrap())).unwrap();
    let username = args.next().unwrap();
    let password = args.next().unwrap();

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

    let (tx, rx) = mpsc::channel();

    session.mercury.send(MercuryRequest{
        method: MercuryMethod::SUB,
        uri: "hm://remote/user/lietar/v23".to_string(),
        content_type: None,
        callback: Some(tx)
    }).unwrap();

    for pkt in rx.iter() {
        let frame : protocol::spirc::Frame =
            protobuf::parse_from_bytes(pkt.payload.front().unwrap()).unwrap();

        if frame.get_device_state().get_is_active() &&
            frame.has_state() {
            let index = frame.get_state().get_playing_track_index();
            let ref track = frame.get_state().get_track()[index as usize];
            println!("{}", frame.get_device_state().get_name());
            print_track(&mut cache, SpotifyId::from_raw(track.get_gid()));
            println!("");
        }
    }

    loop {
        session.poll();
    }
}

fn print_track(cache: &mut MetadataCache, track_id: SpotifyId) {
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
}

