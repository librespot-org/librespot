extern crate librespot;
extern crate tokio_core;

use std::env;
use tokio_core::reactor::Core;

use librespot::core::authentication::Credentials;
use librespot::core::config::{PlayerConfig, SessionConfig};
use librespot::core::session::Session;
use librespot::core::util::SpotifyId;

use librespot::audio_backend;
use librespot::player::Player;

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let session_config = SessionConfig::default();
    let player_config = PlayerConfig::default();

    let args : Vec<_> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: {} USERNAME PASSWORD TRACK", args[0]);
    }
    let username = args[1].to_owned();
    let password = args[2].to_owned();
    let credentials = Credentials::with_password(username, password);

    let track = SpotifyId::from_base62(&args[3]);

    let backend = audio_backend::find(None).unwrap();

    println!("Connecting ..");
    let session = core.run(Session::connect(session_config, credentials, None, handle)).unwrap();

    let player = Player::new(player_config, session.clone(), None, move || (backend)(None));

    println!("Playing...");
    core.run(player.load(track, true, 0)).unwrap();

    println!("Done");
}
