extern crate getopts;
extern crate librespot;
extern crate env_logger;
#[macro_use]
extern crate log;

use std::process::exit;
use std::thread;
use std::env;

use librespot::spirc::SpircManager;
use librespot::main_helper;

fn usage(program: &str, opts: &getopts::Options) -> String {
    let brief = format!("Usage: {} [options]", program);
    format!("{}", opts.usage(&brief))
}

fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info,librespot=trace")
    }
    env_logger::init().unwrap();

    let mut opts = getopts::Options::new();
    main_helper::add_session_arguments(&mut opts);
    main_helper::add_authentication_arguments(&mut opts);
    main_helper::add_player_arguments(&mut opts);
    main_helper::add_program_arguments(&mut opts);

    let args: Vec<String> = std::env::args().collect();

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            error!("Error: {}\n{}", f.to_string(), usage(&args[0], &opts));
            exit(1)
        }
    };

    let session = main_helper::create_session(&matches);
    let credentials = main_helper::get_credentials(&session, &matches);
    session.login(credentials).unwrap();

    let player = main_helper::create_player(&session, &matches);

    let spirc = SpircManager::new(session.clone(), player);
    thread::spawn(move || spirc.run());

    loop {
        session.poll();
    }
}
