#![crate_name = "librespot"]

#![cfg_attr(not(feature = "with-syntex"), feature(plugin))]
#![cfg_attr(not(feature = "with-syntex"), plugin(protobuf_macros))]
#![cfg_attr(not(feature = "with-syntex"), plugin(json_macros))]

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use]
extern crate lazy_static;

extern crate bit_set;
extern crate byteorder;
extern crate crypto;
extern crate eventual;
extern crate hyper;
extern crate lmdb_rs;
extern crate num;
extern crate protobuf;
extern crate shannon;
extern crate rand;
extern crate rustc_serialize;
extern crate time;
extern crate tempfile;
extern crate url;

#[macro_use]
extern crate log;

#[cfg(not(feature = "with-tremor"))]
extern crate vorbis;
#[cfg(feature = "with-tremor")]
extern crate tremor as vorbis;

#[cfg(feature = "dns-sd")]
extern crate dns_sd;

#[cfg(feature = "openssl")]
extern crate openssl;

#[cfg(feature = "portaudio")]
extern crate portaudio;

#[cfg(feature = "libpulse-sys")]
extern crate libpulse_sys;

extern crate librespot_protocol as protocol;

// This doesn't play nice with syntex, so place it here
pub mod version {
    include!(concat!(env!("OUT_DIR"), "/version.rs"));

    pub fn version_string() -> String {
        format!("librespot-{}", short_sha())
    }
}

#[cfg(feature = "with-syntex")]
include!(concat!(env!("OUT_DIR"), "/lib.rs"));

#[cfg(not(feature = "with-syntex"))]
include!("lib.in.rs");
