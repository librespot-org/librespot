extern crate protobuf;

include! (concat!(env!("OUT_DIR"), "/authentication.rs"));
include! (concat!(env!("OUT_DIR"), "/keyexchange.rs"));
include! (concat!(env!("OUT_DIR"), "/mercury.rs"));
include! (concat!(env!("OUT_DIR"), "/metadata.rs"));
include! (concat!(env!("OUT_DIR"), "/pubsub.rs"));
include! (concat!(env!("OUT_DIR"), "/spirc.rs"));
