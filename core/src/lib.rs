#![allow(clippy::unused_io_amount)]

#[macro_use]
extern crate log;

use librespot_protocol as protocol;

#[macro_use]
mod component;

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

const AP_FALLBACK: &str = "ap.spotify.com:443";

#[cfg(feature = "apresolve")]
mod apresolve;

#[cfg(not(feature = "apresolve"))]
mod apresolve {
    pub async fn apresolve(_: Option<&url::Url>, _: Option<u16>) -> String {
        return super::AP_FALLBACK.into();
    }
}
