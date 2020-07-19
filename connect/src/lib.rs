#![deny(rust_2018_idioms)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use librespot_playback as playback;
use librespot_protocol as protocol;

pub mod context;
pub mod discovery;
pub mod spirc;
