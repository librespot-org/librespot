#[macro_use]
extern crate log;

extern crate byteorder;
extern crate futures;
extern crate shell_words;

#[cfg(feature = "alsa-backend")]
extern crate alsa;

#[cfg(feature = "portaudio-backend")]
extern crate portaudio_rs;

#[cfg(feature = "pulseaudio-backend")]
extern crate libpulse_binding;
#[cfg(feature = "pulseaudio-backend")]
extern crate libpulse_simple_binding;

#[cfg(feature = "jackaudio-backend")]
extern crate jack;

#[cfg(feature = "gstreamer-backend")]
extern crate glib;
#[cfg(feature = "gstreamer-backend")]
extern crate gstreamer as gst;
#[cfg(feature = "gstreamer-backend")]
extern crate gstreamer_app as gst_app;
#[cfg(feature = "gstreamer-backend")]
extern crate zerocopy;

#[cfg(feature = "sdl-backend")]
extern crate sdl2;

extern crate librespot_audio as audio;
extern crate librespot_core;
extern crate librespot_metadata as metadata;

pub mod audio_backend;
pub mod config;
pub mod mixer;
pub mod player;
