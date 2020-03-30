use env_logger;
use std::env;
use tokio::runtime::Runtime;

use librespot_core::{apresolve::apresolve_or_fallback, connection};

// TODO: Rewrite this into an actual test instead of this wonder
fn main() {
    env_logger::init();
    let mut rt = Runtime::new().unwrap();

    let args: Vec<_> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: {} USERNAME PASSWORD PLAYLIST", args[0]);
    }
    // let username = args[1].to_owned();
    // let password = args[2].to_owned();

    let ap = rt.block_on(apresolve_or_fallback(&None, &Some(80)));

    println!("AP: {:?}", ap);
    let connection = rt.block_on(connection::connect(&None));
}
