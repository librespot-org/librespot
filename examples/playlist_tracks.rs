#[macro_use]
extern crate log;
extern crate env_logger;

extern crate librespot;
extern crate tokio_core;
//extern crate tokio_fs;
extern crate tokio_io;
extern crate futures;
//extern crate futures_cpupool;

use std::env;
use tokio_core::reactor::Core;

use librespot::core::authentication::Credentials;
use librespot::core::config::SessionConfig;
use librespot::core::session::Session;
use librespot::core::spotify_id::SpotifyId;
use librespot::playback::config::PlayerConfig;
use librespot::playback::config::Bitrate;
use librespot::metadata::{FileFormat, Metadata, Track, Album, Artist, Playlist};


/*
fn make_list_playlist(core: &mut Core, session: &Session, uri: &str) -> TrackList {
    let mut tracks = Vec::new();
    let mut fnames = Vec::new();

    let plist_uri = SpotifyId::from_base62(&uri).unwrap();
    let plist = core.run(Playlist::get(&session, plist_uri)).unwrap();
    println!("album name: {}",plist.name);
    let plist_name = &plist.name;

    

    for (i, track_id) in plist.tracks.iter().enumerate() {
        let plist_track = core.run(Track::get(&session, *track_id)).unwrap();
        //println!("album track: {} - {}",i+1, alb_track.name);
        let artist = core.run(Artist::get(&session, plist_track.artists[0])).unwrap();
        println!("track artist: {}",artist.name);
        tracks.push(plist_track.id);
        let filename = format!("{} - {}.ogg",&artist.name, alb_track.name);
        fnames.push(filename);
    }
    let ntr = plist.tracks.len();

    let folder = format!("{}",plist_name);
    let mut tlist = TrackList::new(ntr, folder, tracks, fnames);
    tlist
}
*/

fn main() {
    env_logger::init();
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let session_config = SessionConfig::default();
    let mut player_config = PlayerConfig::default();
    player_config.bitrate = Bitrate::Bitrate320;

    let args: Vec<_> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: {} USERNAME PASSWORD PLAYLIST", args[0]);
    }
    let username = args[1].to_owned();
    let password = args[2].to_owned();
    let credentials = Credentials::with_password(username, password);

    let mut uri_split = args[3].split(":");
    let uri_parts: Vec<&str> = uri_split.collect();
    println!("{}, {}, {}",uri_parts[0], uri_parts[1], uri_parts[2]);
    
    let plist_uri = SpotifyId::from_base62(uri_parts[2]).unwrap();
    
    let session = core
        .run(Session::connect(session_config, credentials, None, handle))
        .unwrap();

    let plist = core.run(Playlist::get(&session, plist_uri)).unwrap();
    println!("{:?}",plist);
    for track_id in plist.tracks {
        let plist_track = core.run(Track::get(&session, track_id)).unwrap();
        println!("track: {} ", plist_track.name);
    }
}
