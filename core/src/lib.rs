#![allow(clippy::unused_io_amount)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate pin_project_lite;
extern crate aes;
extern crate base64;
extern crate byteorder;
extern crate bytes;
extern crate futures;
extern crate hmac;
extern crate httparse;
extern crate hyper;
extern crate hyper_proxy;
extern crate num_bigint;
extern crate num_integer;
extern crate num_traits;
extern crate once_cell;
extern crate pbkdf2;
extern crate protobuf;
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate sha1;
extern crate shannon;
pub extern crate tokio;
extern crate tokio_util;
extern crate url;
extern crate uuid;

extern crate librespot_protocol as protocol;

#[macro_use]
mod component;

pub mod apresolve;
pub mod audio_key;
pub mod authentication;
pub mod cache;
pub mod channel;
pub mod config;
pub mod connection;
pub mod diffie_hellman;
pub mod keymaster;
pub mod mercury;
mod proxytunnel;
pub mod session;
pub mod spotify_id;
pub mod util;
pub mod version;
pub mod volume;
