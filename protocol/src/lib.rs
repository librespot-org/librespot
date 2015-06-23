#![feature(plugin)]
#![plugin(mod_path)]

extern crate protobuf;

mod_path! keyexchange (concat!(env!("OUT_DIR"), "/keyexchange.rs"));
mod_path! authentication (concat!(env!("OUT_DIR"), "/authentication.rs"));
mod_path! mercury (concat!(env!("OUT_DIR"), "/mercury.rs"));
mod_path! metadata (concat!(env!("OUT_DIR"), "/metadata.rs"));

mod_path! playlist4changes (concat!(env!("OUT_DIR"), "/playlist4changes.rs"));
mod_path! playlist4content (concat!(env!("OUT_DIR"), "/playlist4content.rs"));
mod_path! playlist4issues (concat!(env!("OUT_DIR"), "/playlist4issues.rs"));
mod_path! playlist4meta (concat!(env!("OUT_DIR"), "/playlist4meta.rs"));
mod_path! playlist4ops (concat!(env!("OUT_DIR"), "/playlist4ops.rs"));
mod_path! playlist4service (concat!(env!("OUT_DIR"), "/playlist4service.rs"));

