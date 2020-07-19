#![deny(rust_2018_idioms)]

#[macro_use]
extern crate log;

extern crate librespot_audio as audio;
extern crate librespot_metadata as metadata;

pub mod audio_backend;
pub mod config;
pub mod mixer;
pub mod player;
