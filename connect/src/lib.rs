#![deny(rust_2018_idioms)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate librespot_playback as playback;
extern crate librespot_protocol as protocol;

pub mod context;
pub mod discovery;
pub mod spirc;
