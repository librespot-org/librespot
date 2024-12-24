// This file is parsed by build.rs
// Each included module will be compiled from the matching .proto definition.

mod conversion;

include!(concat!(env!("OUT_DIR"), "/mod.rs"));
