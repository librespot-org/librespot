#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate base64;
extern crate futures;
extern crate hyper;
extern crate num_bigint;
extern crate protobuf;
extern crate rand;
extern crate tokio_core;
extern crate url;

extern crate aes_ctr;
extern crate block_modes;
extern crate hmac;
extern crate sha1;

#[cfg(feature = "with-dns-sd")]
extern crate dns_sd;

#[cfg(not(feature = "with-dns-sd"))]
extern crate libmdns;

extern crate librespot_core;
extern crate librespot_playback as playback;
extern crate librespot_protocol as protocol;

pub mod context;
pub mod discovery;
pub mod spirc;
