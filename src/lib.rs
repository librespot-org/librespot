#![crate_name = "librespot"]

#![cfg_attr(not(feature = "with-syntex"), feature(plugin))]
#![cfg_attr(not(feature = "with-syntex"), plugin(protobuf_macros))]
#![cfg_attr(not(feature = "with-syntex"), plugin(json_macros))]

#[macro_use]
extern crate lazy_static;

extern crate bit_set;
extern crate byteorder;
extern crate crypto;
extern crate eventual;
extern crate hyper;
extern crate num;
extern crate portaudio;
extern crate protobuf;
extern crate shannon;
extern crate rand;
extern crate rustc_serialize;
extern crate time;
extern crate tiny_http;
extern crate tempfile;
extern crate url;

#[cfg(not(feature = "with-tremor"))]
extern crate vorbis;
#[cfg(feature = "with-tremor")]
extern crate tremor as vorbis;

#[cfg(feature = "dns-sd")]
extern crate dns_sd;

extern crate librespot_protocol as protocol;

#[cfg(feature = "with-syntex")]
include!(concat!(env!("OUT_DIR"), "/lib.rs"));

#[cfg(not(feature = "with-syntex"))]
include!("lib.in.rs");
