#![crate_name = "librespot"]

#![feature(plugin,read_exact,zero_one,iter_arith,slice_bytes,mpsc_select)]

#![plugin(protobuf_macros)]
#[macro_use] extern crate lazy_static;

extern crate bit_set;
extern crate byteorder;
extern crate crypto;
extern crate eventual;
extern crate num;
extern crate portaudio;
extern crate protobuf;
extern crate shannon;
extern crate rand;
extern crate time;
extern crate tempfile;
extern crate vorbis;

extern crate librespot_protocol;

#[macro_use] pub mod util;
pub mod audio_decrypt;
pub mod audio_file;
pub mod audio_key;
pub mod connection;
pub mod keys;
pub mod mercury;
pub mod metadata;
pub mod player;
pub mod session;
pub mod spirc;
pub mod stream;

