#![cfg_attr(feature = "cargo-clippy", allow(unused_io_amount))]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate futures;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

extern crate aes;
extern crate base64;
extern crate byteorder;
extern crate bytes;
extern crate hmac;
extern crate httparse;
extern crate hyper;
extern crate hyper_proxy;
extern crate num_bigint;
extern crate num_integer;
extern crate num_traits;
extern crate pbkdf2;
extern crate protobuf;
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate sha1;
extern crate shannon;
extern crate tokio_codec;
extern crate tokio_core;
extern crate tokio_io;
extern crate url;
extern crate uuid;

extern crate librespot_protocol as protocol;

#[macro_use]
mod component;
mod apresolve;
pub mod audio_key;
pub mod authentication;
pub mod cache;
pub mod channel;
pub mod config;
mod connection;
pub mod diffie_hellman;
pub mod keymaster;
pub mod mercury;
mod proxytunnel;
pub mod session;
pub mod spotify_id;
pub mod util;
pub mod version;
