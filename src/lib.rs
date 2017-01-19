#![crate_name = "librespot"]

#![cfg_attr(not(feature = "with-syntex"), feature(plugin, custom_derive))]
#![cfg_attr(not(feature = "with-syntex"), plugin(protobuf_macros))]
#![cfg_attr(not(feature = "with-syntex"), plugin(json_macros))]
#![cfg_attr(not(feature = "with-syntex"), plugin(serde_macros))]

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use] extern crate error_chain;
#[macro_use] extern crate futures;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;

extern crate bit_set;
extern crate byteorder;
extern crate crypto;
extern crate getopts;
extern crate hyper;
extern crate linear_map;
extern crate lmdb_rs;
extern crate mdns;
extern crate num;
extern crate protobuf;
extern crate rand;
extern crate rpassword;
extern crate rustc_serialize;
extern crate serde;
extern crate serde_json;
extern crate shannon;
extern crate tempfile;
extern crate tokio_core;
extern crate tokio_proto;
extern crate url;

extern crate librespot_protocol as protocol;

#[cfg(not(feature = "with-tremor"))]
extern crate vorbis;
#[cfg(feature = "with-tremor")]
extern crate tremor as vorbis;

#[cfg(feature = "alsa-backend")]
extern crate alsa;
#[cfg(feature = "portaudio")]
extern crate portaudio;
#[cfg(feature = "libpulse-sys")]
extern crate libpulse_sys;

#[macro_use] mod component;

pub mod audio_backend;
pub mod audio_decrypt;
pub mod audio_file;
pub mod audio_key;
pub mod cache;
pub mod channel;
pub mod diffie_hellman;
pub mod mercury;
pub mod metadata;
pub mod player;
pub mod session;
pub mod util;
pub mod version;

#[cfg(feature = "with-syntex")] include!(concat!(env!("OUT_DIR"), "/lib.rs"));
#[cfg(not(feature = "with-syntex"))] include!("lib.in.rs");
