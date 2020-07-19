#![allow(rust_2018_idioms)] // because of generated code

// This file is parsed by build.rs
// Each included module will be compiled from the matching .proto definition.
pub mod authentication;
pub mod keyexchange;
pub mod mercury;
pub mod metadata;
pub mod playlist4changes;
mod playlist4content;
mod playlist4issues;
mod playlist4meta;
mod playlist4ops;
pub mod pubsub;
pub mod spirc;
