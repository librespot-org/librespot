#[macro_use]
extern crate log;

use librespot_core as core;
use librespot_playback as playback;
use librespot_protocol as protocol;

pub mod context;
#[deprecated(
    since = "0.2.1",
    note = "Please use the crate `librespot_discovery` instead."
)]
pub mod discovery;
pub mod spirc;
