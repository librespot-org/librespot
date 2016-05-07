#![crate_name = "librespot"]

#![cfg_attr(not(feature = "with-syntex"), feature(plugin, custom_derive))]
#![cfg_attr(not(feature = "with-syntex"), plugin(protobuf_macros))]
#![cfg_attr(not(feature = "with-syntex"), plugin(json_macros))]
#![cfg_attr(not(feature = "with-syntex"), plugin(serde_macros))]

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use]
extern crate lazy_static;

extern crate bit_set;
extern crate byteorder;
extern crate crypto;
extern crate eventual;
extern crate getopts;
extern crate hyper;
extern crate linear_map;
extern crate lmdb_rs;
extern crate mdns;
extern crate num;
extern crate protobuf;
extern crate shannon;
extern crate rand;
extern crate rpassword;
extern crate rustc_serialize;
extern crate serde;
extern crate serde_json;
extern crate time;
extern crate tempfile;
extern crate url;

#[macro_use]
extern crate log;

#[cfg(not(feature = "with-tremor"))]
extern crate vorbis;
#[cfg(feature = "with-tremor")]
extern crate tremor as vorbis;

#[cfg(feature = "openssl")]
extern crate openssl;

#[cfg(feature = "portaudio")]
extern crate portaudio;

#[cfg(feature = "libpulse-sys")]
extern crate libpulse_sys;

extern crate librespot_protocol as protocol;

// include!/include_bytes! don't play nice with syntex, so place these here
pub mod version {
    include!(concat!(env!("OUT_DIR"), "/version.rs"));

    pub fn version_string() -> String {
        format!("librespot-{}", short_sha())
    }
}

#[cfg(feature = "static-appkey")]
static APPKEY: Option<&'static [u8]> = Some(include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/spotify_appkey.key")));
#[cfg(not(feature = "static-appkey"))]
static APPKEY: Option<&'static [u8]> = None;

#[cfg(feature = "with-syntex")]
include!(concat!(env!("OUT_DIR"), "/lib.rs"));

#[cfg(not(feature = "with-syntex"))]
include!("lib.in.rs");
