#![crate_name = "librespot"]
#![cfg_attr(feature = "cargo-clippy", allow(unused_io_amount))]

pub extern crate librespot_audio as audio;
// pub extern crate librespot_connect as connect;
pub extern crate librespot_core as core;
pub extern crate librespot_metadata as metadata;
pub extern crate librespot_playback as playback;
pub extern crate librespot_protocol as protocol;
