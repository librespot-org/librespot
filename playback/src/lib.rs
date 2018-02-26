#[macro_use]
extern crate log;

extern crate byteorder;
extern crate futures;

#[cfg(feature = "alsa-backend")]
extern crate alsa;

#[cfg(feature = "portaudio-rs")]
extern crate portaudio_rs;

#[cfg(feature = "libpulse-sys")]
extern crate libpulse_sys;

#[cfg(feature = "jackaudio-backend")]
extern crate jack;

#[cfg(feature = "libc")]
extern crate libc;

extern crate librespot_audio as audio;
extern crate librespot_core as core;
extern crate librespot_metadata as metadata;

pub mod audio_backend;
pub mod config;
pub mod mixer;
pub mod player;
