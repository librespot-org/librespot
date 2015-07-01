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
extern crate time;

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
use protobuf::core::Message;

use metadata::{MetadataCache, AlbumRef, ArtistRef, TrackRef};
use session::{Config, Session};
use util::SpotifyId;
use util::version::version_string;
use player::Player;
use mercury::{MercuryRequest, MercuryMethod};
use librespot_protocol as protocol;

fn main() {
    let mut args = std::env::args().skip(1);
    let mut appkey_file = File::open(Path::new(&args.next().unwrap())).unwrap();
    let username = args.next().unwrap();
    let password = args.next().unwrap();
    let name = args.next().unwrap();

    let mut appkey = Vec::new();
    appkey_file.read_to_end(&mut appkey).unwrap();

    let config = Config {
        application_key: appkey,
        user_agent: version_string(),
        device_id: name.to_string()
    };
    let session = Session::new(config);
    session.login(username.clone(), password);
    session.poll();

    let ident = session.config.device_id.clone();
    SpircManager{
        session: session,
        username: username.clone(),
        name: name.clone(),
        ident: ident,
        device_type: 5,

        state_update_id: 0,
        seq_nr: 0,

        volume: 0x8000,
        can_play: true,
        is_active: false,
        became_active_at: 0,
    }.run();

    /*
    loop {
        session.poll();
    }
    */
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

struct SpircManager {
    session: Session,
    username: String,
    state_update_id: i64,
    seq_nr: u32,

    name: String,
    ident: String,
    device_type: u8,

    volume: u16,
    can_play: bool,
    is_active: bool,
    became_active_at: i64,
}

impl SpircManager {
    fn run(&mut self) {
        let (tx, rx) = mpsc::channel();

        self.session.mercury.send(MercuryRequest{
            method: MercuryMethod::SUB,
            uri: format!("hm://remote/user/{}/v23", self.username).to_string(),
            content_type: None,
            callback: Some(tx),
            payload: Vec::new()
        }).unwrap();

        self.notify(None);

        for pkt in rx.iter() {
            let frame : protocol::spirc::Frame =
                protobuf::parse_from_bytes(pkt.payload.front().unwrap()).unwrap();

            println!("{:?} {} {} {}",
                     frame.get_typ(),
                     frame.get_device_state().get_name(),
                     frame.get_ident(),
                     frame.get_device_state().get_became_active_at());

            if frame.get_ident() == self.ident ||
                (frame.get_recipient().len() > 0 &&
                 !frame.get_recipient().contains(&self.ident)) {
                    continue;
                }

            self.handle(frame);
        }
    }

    fn handle(&mut self, frame: protocol::spirc::Frame) {
        match frame.get_typ() {
            protocol::spirc::MessageType::kMessageTypeHello => {
                self.notify(Some(frame.get_ident()));
            }
            protocol::spirc::MessageType::kMessageTypeLoad => {
                self.is_active = true;
                self.became_active_at = {
                    let ts = time::now_utc().to_timespec();
                    ts.sec * 1000 + ts.nsec as i64 / 1000000
                };
                println!("{:?} {}", frame, self.became_active_at);
                self.notify(None)
            }
            _ => ()
        }
    }

    fn notify(&mut self, recipient: Option<&str>) {
        let device_state = self.device_state();
        self.session.mercury.send(MercuryRequest{
            method: MercuryMethod::SEND,
            uri: format!("hm://remote/user/{}", self.username).to_string(),
            content_type: None,
            callback: None,
            payload: vec![
                protobuf_init!(protocol::spirc::Frame::new(), {
                    version: 1,
                    ident: self.ident.clone(),
                    protocol_version: "2.0.0".to_string(),
                    seq_nr: { self.seq_nr += 1; self.seq_nr  },
                    typ: protocol::spirc::MessageType::kMessageTypeNotify,
                    device_state: device_state,
                    recipient: protobuf::RepeatedField::from_vec(
                        recipient.map(|r| vec![r.to_string()] ).unwrap_or(vec![])
                    )
                }).write_to_bytes().unwrap()
            ]
        }).unwrap();
    }

    fn device_state(&mut self) -> protocol::spirc::DeviceState {
        protobuf_init!(protocol::spirc::DeviceState::new(), {
            sw_version: version_string(),
            is_active: self.is_active,
            can_play: self.can_play,
            volume: self.volume as u32,
            name: self.name.clone(),
            error_code: 0,
            became_active_at: if self.is_active { self.became_active_at } else { 0 },
            capabilities => [
                @{
                    typ: protocol::spirc::CapabilityType::kCanBePlayer,
                    intValue => [0]
                },
                @{
                    typ: protocol::spirc::CapabilityType::kDeviceType,
                    intValue => [ self.device_type as i64 ]
                },
                @{
                    typ: protocol::spirc::CapabilityType::kGaiaEqConnectId,
                    intValue => [1]
                },
                @{
                    typ: protocol::spirc::CapabilityType::kSupportsLogout,
                    intValue => [0]
                },
                @{
                    typ: protocol::spirc::CapabilityType::kIsObservable,
                    intValue => [1]
                },
                @{
                    typ: protocol::spirc::CapabilityType::kVolumeSteps,
                    intValue => [10]
                },
                @{
                    typ: protocol::spirc::CapabilityType::kSupportedContexts,
                    stringValue => [
                        "album".to_string(),
                        "playlist".to_string(),
                        "search".to_string(),
                        "inbox".to_string(),
                        "toplist".to_string(),
                        "starred".to_string(),
                        "publishedstarred".to_string(),
                        "track".to_string(),
                    ]
                },
                @{
                    typ: protocol::spirc::CapabilityType::kSupportedTypes,
                    stringValue => [
                        "audio/local".to_string(),
                        "audio/track".to_string(),
                        "local".to_string(),
                        "track".to_string(),
                    ]
                }
            ],
        })
    }
}

