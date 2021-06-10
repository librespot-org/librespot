#![allow(clippy::unused_io_amount)]

#[macro_use]
extern crate log;

use librespot_protocol as protocol;

#[macro_use]
mod component;

mod apresolve;
pub mod audio_key;
pub mod authentication;
pub mod cache;
pub mod channel;
pub mod config;
mod connection;
#[allow(dead_code)]
mod dealer;
#[doc(hidden)]
pub mod diffie_hellman;
pub mod keymaster;
pub mod mercury;
mod proxytunnel;
pub mod session;
mod socket;
pub mod spotify_id;
#[doc(hidden)]
pub mod util;
pub mod version;
