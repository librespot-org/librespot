// This file is parsed by build.rs
// Each included module will be compiled from the matching .proto definition.

mod impl_trait;

mod conversion;

include!(concat!(env!("OUT_DIR"), "/mod.rs"));
