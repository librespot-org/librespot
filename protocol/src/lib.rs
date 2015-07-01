#![feature(plugin)]
#![plugin(mod_path)]

extern crate protobuf;

mod_path! keyexchange (concat!(env!("OUT_DIR"), "/keyexchange.rs"));
mod_path! authentication (concat!(env!("OUT_DIR"), "/authentication.rs"));
mod_path! mercury (concat!(env!("OUT_DIR"), "/mercury.rs"));
mod_path! metadata (concat!(env!("OUT_DIR"), "/metadata.rs"));
mod_path! spirc (concat!(env!("OUT_DIR"), "/spirc.rs"));

