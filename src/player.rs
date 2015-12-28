use eventual::{self, Async};
use portaudio;
use std::sync::{mpsc, Mutex, Arc, Condvar, MutexGuard};
use std::thread;
use vorbis;

use metadata::{Track, TrackRef};
use session::Session;
use audio_decrypt::AudioDecrypt;
use util::{self, SpotifyId, Subfile};
use spirc::{SpircState, SpircDelegate, PlayStatus};

pub struct Player {
    state: Arc<(Mutex<PlayerState>, Condvar)>,

    commands: mpsc::Sender<PlayerCommand>,
}

pub struct PlayerState {
    status: PlayStatus,
    position_ms: u32,
    position_measured_at: i64,
    update_time: i64,

    end_of_track: bool
}

struct PlayerInternal {
    state: Arc<(Mutex<PlayerState>, Condvar)>,
    session: Session,
    commands: mpsc::Receiver<PlayerCommand>,
}

enum PlayerCommand {
    Load(SpotifyId, bool, u32),
    Play,
    Pause,
    Stop,
    Seek(u32)
}

impl Player {
    pub fn new(session: &Session) -> Player {
        let (cmd_tx, cmd_rx) = mpsc::channel();

        let state = Arc::new((Mutex::new(PlayerState {
            status: PlayStatus::kPlayStatusStop,
            position_ms: 0,
            position_measured_at: 0,
            update_time: util::now_ms(),
            end_of_track: false,
        }), Condvar::new()));

        let internal = PlayerInternal {
            session: session.clone(),
            commands: cmd_rx,
            state: state.clone()
        };

        thread::spawn(move || {
            internal.run()
        });

        Player {
            commands: cmd_tx,
            state: state,
        }
    }

    fn command(&self, cmd: PlayerCommand) {
        self.commands.send(cmd).unwrap();
    }
}

impl PlayerInternal {
    fn run(self) {
        portaudio::initialize().unwrap();

        let stream = portaudio::stream::Stream::<i16, i16>::open_default(
                0, 2, 44100.0,
                portaudio::stream::FRAMES_PER_BUFFER_UNSPECIFIED,
                None
        ).unwrap();

        let mut decoder = None;

        loop {
            let cmd = if self.state.0.lock().unwrap().status == PlayStatus::kPlayStatusPlay {
                self.commands.try_recv().ok()
            } else {
                Some(self.commands.recv().unwrap())
            };

            match cmd {
                Some(PlayerCommand::Load(track_id, play, position)) => {
                    self.update(|state| {
                        if state.status == PlayStatus::kPlayStatusPlay {
                            stream.stop().unwrap();
                        }
                        state.end_of_track = false;
                        state.status = PlayStatus::kPlayStatusLoading;
                        state.position_ms = position;
                        state.position_measured_at = util::now_ms();
                        return true;
                    });
                    drop(decoder);

                    let mut track = self.session.metadata::<Track>(track_id).await().unwrap();

                    if !track.available {
                        let alternatives = track.alternatives.iter()
                            .map(|alt_id| self.session.metadata::<Track>(*alt_id))
                            .collect::<Vec<TrackRef>>();

                        track = eventual::sequence(alternatives.into_iter()).iter().find(|alt| alt.available).unwrap();
                    }

                    let file_id = track.files[0];

                    let key = self.session.audio_key(track.id, file_id).await().unwrap();
                    decoder = Some(
                        vorbis::Decoder::new(
                        Subfile::new(
                        AudioDecrypt::new(key,
                        self.session.audio_file(file_id)), 0xa7)).unwrap());
                    decoder.as_mut().unwrap().time_seek(position as f64 / 1000f64).unwrap();

                    self.update(|state| {
                        state.status = if play {
                            stream.start().unwrap();
                            PlayStatus::kPlayStatusPlay
                        } else {
                            PlayStatus::kPlayStatusPause
                        };
                        state.position_ms = position;
                        state.position_measured_at = util::now_ms();

                        return true;
                    });
                    println!("Load Done");
                }
                Some(PlayerCommand::Seek(ms)) => {
                    decoder.as_mut().unwrap().time_seek(ms as f64 / 1000f64).unwrap();
                    self.update(|state| {
                        state.position_ms = (decoder.as_mut().unwrap().time_tell().unwrap() * 1000f64) as u32;
                        state.position_measured_at = util::now_ms();
                        return true;
                    });
                },
                Some(PlayerCommand::Play) => {
                    self.update(|state| {
                        state.status = PlayStatus::kPlayStatusPlay;
                        return true;
                    });

                    stream.start().unwrap();
                },
                Some(PlayerCommand::Pause) => {
                    self.update(|state| {
                        state.status = PlayStatus::kPlayStatusPause;
                        state.update_time = util::now_ms();
                        return true;
                    });

                    stream.stop().unwrap();
                },
                Some(PlayerCommand::Stop) => {
                    self.update(|state| {
                        if state.status == PlayStatus::kPlayStatusPlay {
                            state.status = PlayStatus::kPlayStatusPause;
                        }
                        return true;
                    });

                    stream.stop().unwrap();
                    decoder = None;
                },
                None => (),
            }

            if self.state.0.lock().unwrap().status == PlayStatus::kPlayStatusPlay {
                match decoder.as_mut().unwrap().packets().next() {
                    Some(Ok(packet)) => {
                        match stream.write(&packet.data) {
                            Ok(_) => (),
                            Err(portaudio::PaError::OutputUnderflowed)
                                => eprintln!("Underflow"),
                            Err(e) => panic!("PA Error {}", e)
                        };
                    },
                    Some(Err(vorbis::VorbisError::Hole)) => (),
                    Some(Err(e)) => panic!("Vorbis error {:?}", e),
                    None => {
                        self.update(|state| {
                            state.status = PlayStatus::kPlayStatusStop;
                            state.end_of_track = true;
                            return true;
                        });

                        stream.stop().unwrap();
                        decoder = None;
                    }
                }

                self.update(|state| {
                    let now = util::now_ms();

                    if now - state.position_measured_at > 5000 {
                        state.position_ms = (decoder.as_mut().unwrap().time_tell().unwrap() * 1000f64) as u32;
                        state.position_measured_at = now;
                        return true;
                    } else {
                        return false;
                    }
                });
            }
        }

        drop(stream);

        portaudio::terminate().unwrap();
    }

    fn update<F>(&self, f: F)
        where F: FnOnce(&mut MutexGuard<PlayerState>) -> bool {
        let mut guard = self.state.0.lock().unwrap();
        let update = f(&mut guard);
        if update {
            guard.update_time = util::now_ms();
            self.state.1.notify_all();
        }
    }
}

impl SpircDelegate for Player {
    type State = PlayerState;

    fn load(&self, track: SpotifyId,
            start_playing: bool, position_ms: u32) {
        self.command(PlayerCommand::Load(track, start_playing, position_ms));
    }

    fn play(&self) {
        self.command(PlayerCommand::Play)
    }

    fn pause(&self) {
        self.command(PlayerCommand::Pause)
    }

    fn stop(&self) {
        self.command(PlayerCommand::Stop)
    }

    fn seek(&self, position_ms: u32) {
        self.command(PlayerCommand::Seek(position_ms));
    }

    fn state(&self) -> MutexGuard<Self::State> {
        self.state.0.lock().unwrap()
    }

    fn updates(&self) -> mpsc::Receiver<i64> {
        let state = self.state.clone();
        let (update_tx, update_rx) = mpsc::channel();

        thread::spawn(move || {
            let mut guard = state.0.lock().unwrap();
            let mut last_update;
            loop {
                last_update = guard.update_time;
                update_tx.send(guard.update_time).unwrap();

                while last_update >= guard.update_time {
                    guard = state.1.wait(guard).unwrap();
                }
            }
        });

        return update_rx;
    }
}

impl SpircState for PlayerState {
    fn status(&self) -> PlayStatus {
        return self.status;
    }

    fn position(&self) -> (u32, i64) {
        return (self.position_ms, self.position_measured_at);
    }

    fn update_time(&self) -> i64 {
        return self.update_time;
    }

    fn end_of_track(&self) -> bool {
        return self.end_of_track;
    }
}

