#![allow(clippy::unused_io_amount)]

#[macro_use]
extern crate log;

use librespot_protocol as protocol;

pub mod authentication;
pub mod cache;
pub mod config;
mod connection;
#[doc(hidden)]
pub mod diffie_hellman;
pub mod keymaster;
mod proxytunnel;
pub mod session;
pub mod spotify_id;
#[doc(hidden)]
pub mod util;
pub mod version;

pub use session::{audio_key, channel, mercury};

const AP_FALLBACK: &str = "ap.spotify.com:443";

#[cfg(feature = "apresolve")]
mod apresolve;

#[cfg(not(feature = "apresolve"))]
mod apresolve {
    pub async fn apresolve(_: Option<&url::Url>, _: Option<u16>) -> String {
        return super::AP_FALLBACK.into();
    }
}
