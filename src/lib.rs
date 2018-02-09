#![crate_name = "librespot"]

#![cfg_attr(feature = "cargo-clippy", allow(unused_io_amount))]

#[macro_use] extern crate log;

extern crate base64;
extern crate crypto;
extern crate futures;
extern crate hyper;
extern crate num_bigint;
extern crate protobuf;
extern crate rand;
extern crate tokio_core;
extern crate url;

pub extern crate librespot_audio as audio;
pub extern crate librespot_core as core;
pub extern crate librespot_discovery as discovery;
pub extern crate librespot_playback as playback;
pub extern crate librespot_protocol as protocol;
pub extern crate librespot_metadata as metadata;


include!(concat!(env!("OUT_DIR"), "/lib.rs"));
