#![crate_name = "librespot"]
#![cfg_attr(feature = "cargo-clippy", allow(unused_io_amount))]

#[cfg(feature = "audio")]
pub extern crate librespot_audio as audio;
#[cfg(feature = "connect")]
pub extern crate librespot_connect as connect;
#[cfg(feature = "core")]
pub extern crate librespot_core as core;
#[cfg(feature = "metadata")]
pub extern crate librespot_metadata as metadata;
#[cfg(feature = "playback")]
pub extern crate librespot_playback as playback;
#[cfg(feature = "protocol")]
pub extern crate librespot_protocol as protocol;
