use std::env;
use tokio_core::reactor::Core;

use librespot::core::authentication::Credentials;
use librespot::core::config::SessionConfig;
use librespot::core::session::Session;
use librespot::core::spotify_id::SpotifyId;
use librespot::playback::config::PlayerConfig;

use futures::stream::Stream;
use futures::{Async, Future, Poll};
use librespot::playback::audio_backend;
use librespot::playback::player::{Player, PlayerEvent, PlayerEventChannel};

pub struct SingleTrackPlayer {
    play_request_id: u64,
    event_channel: PlayerEventChannel,
}

impl SingleTrackPlayer {
    pub fn new(ref mut player: Player, track_id: SpotifyId) -> SingleTrackPlayer {
        let event_channel = player.get_player_event_channel();
        let play_request_id = player.load(track_id, true, 0);
        SingleTrackPlayer {
            play_request_id,
            event_channel,
        }
    }
}

impl Future for SingleTrackPlayer {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<(), ()> {
        loop {
            match self.event_channel.poll().unwrap() {
                Async::NotReady => return Ok(Async::NotReady),
                Async::Ready(None) => return Ok(Async::Ready(())),
                Async::Ready(Some(event)) => match event {
                    PlayerEvent::EndOfTrack {
                        play_request_id, ..
                    }
                    | PlayerEvent::Stopped {
                        play_request_id, ..
                    } => {
                        if play_request_id == self.play_request_id {
                            return Ok(Async::Ready(()));
                        }
                    }
                    _ => (),
                },
            }
        }
    }
}

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let session_config = SessionConfig::default();
    let player_config = PlayerConfig::default();

    let args: Vec<_> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: {} USERNAME PASSWORD TRACK", args[0]);
    }
    let username = args[1].to_owned();
    let password = args[2].to_owned();
    let credentials = Credentials::with_password(username, password);

    let track = SpotifyId::from_base62(&args[3]).unwrap();

    let backend = audio_backend::find(None).unwrap();

    println!("Connecting ..");
    let session = core
        .run(Session::connect(session_config, credentials, None, handle))
        .unwrap();

    let (player, _) = Player::new(player_config, session.clone(), None, move || {
        (backend)(None)
    });

    println!("Playing...");
    core.run(SingleTrackPlayer::new(player, track)).unwrap();

    println!("Done");
}
