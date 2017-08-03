#![crate_name = "librespot"]

#![cfg_attr(feature = "cargo-clippy", allow(unused_io_amount))]

// TODO: many items from tokio-core::io have been deprecated in favour of tokio-io
#![allow(deprecated)]

#[macro_use] extern crate futures;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;

extern crate base64;
extern crate bit_set;
extern crate byteorder;
extern crate crypto;
extern crate getopts;
extern crate hyper;
extern crate linear_map;
extern crate mdns;
extern crate num_bigint;
extern crate num_integer;
extern crate num_traits;
extern crate protobuf;
extern crate rand;
extern crate rpassword;
extern crate serde;
extern crate shannon;
extern crate tempfile;
extern crate tokio_core;
extern crate tokio_proto;
extern crate url;
extern crate uuid;

pub extern crate librespot_core as core;
pub extern crate librespot_protocol as protocol;
pub extern crate librespot_metadata as metadata;

#[cfg(not(feature = "with-tremor"))]
extern crate vorbis;
#[cfg(feature = "with-tremor")]
extern crate tremor as vorbis;

#[cfg(feature = "alsa-backend")]
extern crate alsa;

#[cfg(feature = "portaudio-rs")]
extern crate portaudio_rs;

#[cfg(feature = "libpulse-sys")]
extern crate libpulse_sys;

pub mod audio_backend;
pub mod audio_decrypt;
pub mod audio_file;
pub mod discovery;
pub mod keymaster;
pub mod mixer;
pub mod player;

include!(concat!(env!("OUT_DIR"), "/lib.rs"));
