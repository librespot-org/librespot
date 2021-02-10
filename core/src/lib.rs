#![allow(clippy::unused_io_amount)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate cfg_if;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate pin_project_lite;
#[macro_use]
extern crate error_chain;

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
pub mod diffie_hellman;
pub mod keymaster;
pub mod mercury;
mod proxytunnel;
pub mod session;
pub mod spotify_id;
pub mod util;
pub mod version;
