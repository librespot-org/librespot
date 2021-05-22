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

fn ap_fallback() -> (String, u16) {
    (String::from("ap.spotify.com"), 443)
}

#[cfg(feature = "apresolve")]
mod apresolve;

#[cfg(not(feature = "apresolve"))]
mod apresolve {
    pub async fn apresolve(_: Option<&url::Url>, _: Option<u16>) -> (String, u16) {
        super::ap_fallback()
    }
}
