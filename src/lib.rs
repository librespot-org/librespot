#![crate_name = "librespot"]

#![feature(plugin,zero_one,iter_arith)]

#![plugin(protobuf_macros)]
#![plugin(json_macros)]
#[macro_use]
extern crate lazy_static;

extern crate bit_set;
extern crate byteorder;
extern crate crypto;
extern crate eventual;
extern crate num;
extern crate portaudio;
extern crate protobuf;
extern crate shannon;
extern crate rand;
extern crate rustc_serialize;
extern crate time;
extern crate tiny_http;
extern crate tempfile;
extern crate url;
extern crate vorbis;

#[cfg(feature = "dns-sd")]
extern crate dns_sd;

extern crate librespot_protocol;

#[macro_use]pub mod util;
mod audio_decrypt;
mod audio_file;
mod audio_key;
mod authentication;
mod connection;
mod diffie_hellman;
pub mod discovery;
pub mod mercury;
pub mod metadata;
pub mod player;
pub mod session;
pub mod spirc;
mod stream;
mod zeroconf;
