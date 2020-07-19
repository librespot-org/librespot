#![deny(rust_2018_idioms)]

#[macro_use]
extern crate log;

use librespot_audio as audio;
use librespot_metadata as metadata;

pub mod audio_backend;
pub mod config;
pub mod mixer;
pub mod player;
