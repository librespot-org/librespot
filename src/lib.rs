#![crate_name = "librespot"]

#![feature(plugin,scoped,zero_one,iter_arith,slice_position_elem,slice_bytes,bitset,arc_weak,append,future)]
#![allow(deprecated)]
//#![allow(unused_imports,dead_code)]

#![plugin(protobuf_macros)]
#[macro_use] extern crate lazy_static;


extern crate byteorder;
extern crate crypto;
extern crate gmp;
extern crate num;
extern crate portaudio;
extern crate protobuf;
extern crate shannon;
extern crate rand;
extern crate readall;
extern crate vorbis;
extern crate time;
extern crate tempfile;

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
pub mod subsystem;
