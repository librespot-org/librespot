use futures::{future, Future};
use futures::sync::mpsc;
use std;
use std::borrow::Cow;
use std::io::{Read, Seek};
use std::sync::{Mutex, Arc, MutexGuard};
use std::thread;
use vorbis;

use audio_file::AudioFile;
use audio_decrypt::AudioDecrypt;
use audio_backend::Sink;
use metadata::{FileFormat, Track};
use session::{Bitrate, Session};
use util::{self, SpotifyId, Subfile};
pub use spirc::PlayStatus;

#[cfg(not(feature = "with-tremor"))]
fn vorbis_time_seek_ms<R>(decoder: &mut vorbis::Decoder<R>, ms: i64) -> Result<(), vorbis::VorbisError> where R: Read + Seek {
    decoder.time_seek(ms as f64 / 1000f64)
}

#[cfg(not(feature = "with-tremor"))]
fn vorbis_time_tell_ms<R>(decoder: &mut vorbis::Decoder<R>) -> Result<i64, vorbis::VorbisError> where R: Read + Seek {
    decoder.time_tell().map(|t| (t * 1000f64) as i64)
}

#[cfg(feature = "with-tremor")]
fn vorbis_time_seek_ms<R>(decoder: &mut vorbis::Decoder<R>, ms: i64) -> Result<(), vorbis::VorbisError> where R: Read + Seek {
    decoder.time_seek(ms)
}

#[cfg(feature = "with-tremor")]
fn vorbis_time_tell_ms<R>(decoder: &mut vorbis::Decoder<R>) -> Result<i64, vorbis::VorbisError> where R: Read + Seek {
    decoder.time_tell()
}

#[derive(Clone)]
pub struct Player {
    state: Arc<Mutex<PlayerState>>,
    observers: Arc<Mutex<Vec<mpsc::UnboundedSender<PlayerState>>>>,

    commands: std::sync::mpsc::Sender<PlayerCommand>,
}

#[derive(Clone)]
pub struct PlayerState {
    pub status: PlayStatus,
    pub position_ms: u32,
    pub position_measured_at: i64,
    pub update_time: i64,
    pub volume: u16,
    pub track: Option<SpotifyId>,

    pub end_of_track: bool,
}

struct PlayerInternal {
    state: Arc<Mutex<PlayerState>>,
    observers: Arc<Mutex<Vec<mpsc::UnboundedSender<PlayerState>>>>,

    session: Session,
    commands: std::sync::mpsc::Receiver<PlayerCommand>,
}

#[derive(Debug)]
enum PlayerCommand {
    Load(SpotifyId, bool, u32),
    Play,
    Pause,
    Volume(u16),
    Stop,
    Seek(u32),
    SeekAt(u32, i64),
}

impl Player {
    pub fn new<F>(session: Session, sink_builder: F) -> Player
        where F: FnOnce() -> Box<Sink> + Send + 'static {
        let (cmd_tx, cmd_rx) = std::sync::mpsc::channel();

        let state = Arc::new(Mutex::new(PlayerState {
            status: PlayStatus::kPlayStatusStop,
            position_ms: 0,
            position_measured_at: 0,
            update_time: util::now_ms(),
            volume: 0xFFFF,
            track: None,
            end_of_track: false,
        }));

        let observers = Arc::new(Mutex::new(Vec::new()));

        let internal = PlayerInternal {
            session: session,
            commands: cmd_rx,
            state: state.clone(),
            observers: observers.clone(),
        };

        thread::spawn(move || internal.run(sink_builder()));

        Player {
            commands: cmd_tx,
            state: state,
            observers: observers,
        }
    }

    fn command(&self, cmd: PlayerCommand) {
        self.commands.send(cmd).unwrap();
    }

    pub fn load(&self, track: SpotifyId, start_playing: bool, position_ms: u32) {
        self.command(PlayerCommand::Load(track, start_playing, position_ms));
    }

    pub fn play(&self) {
        self.command(PlayerCommand::Play)
    }

    pub fn pause(&self) {
        self.command(PlayerCommand::Pause)
    }

    pub fn stop(&self) {
        self.command(PlayerCommand::Stop)
    }

    pub fn seek(&self, position_ms: u32) {
        self.command(PlayerCommand::Seek(position_ms));
    }

    pub fn seek_at(&self, position_ms: u32, measured_at: i64) {
        self.command(PlayerCommand::SeekAt(position_ms, measured_at));
    }

    pub fn state(&self) -> PlayerState {
        self.state.lock().unwrap().clone()
    }

    pub fn volume(&self, vol: u16) {
        self.command(PlayerCommand::Volume(vol));
    }

    pub fn observe(&self) -> mpsc::UnboundedReceiver<PlayerState> {
        let (tx, rx) = mpsc::unbounded();
        self.observers.lock().unwrap().push(tx);

        rx
    }
}

fn apply_volume(volume: u16, data: &[i16]) -> Cow<[i16]> {
    // Fast path when volume is 100%
    if volume == 0xFFFF {
        Cow::Borrowed(data)
    } else {
        Cow::Owned(data.iter()
                       .map(|&x| {
                           (x as i32
                            * volume as i32
                            / 0xFFFF) as i16
                       })
                       .collect())
    }
}

fn find_available_alternative<'a>(session: &Session, track: &'a Track) -> Option<Cow<'a, Track>> {
    if track.available {
        Some(Cow::Borrowed(track))
    } else {
        let alternatives = track.alternatives
            .iter()
            .map(|alt_id| {
                session.metadata().get::<Track>(*alt_id)
            });
        let alternatives = future::join_all(alternatives).wait().unwrap();

        alternatives.into_iter().find(|alt| alt.available).map(Cow::Owned)
    }
}

fn load_track(session: &Session, track_id: SpotifyId)
    -> Option<vorbis::Decoder<Subfile<AudioDecrypt<AudioFile>>>>
{
    let track = session.metadata().get::<Track>(track_id).wait().unwrap();

    info!("Loading track \"{}\"", track.name);

    let track = match find_available_alternative(session, &track) {
        Some(track) => track,
        None => {
            warn!("Track \"{}\" is not available", track.name);
            return None;
        }
    };

    let format = match session.config().bitrate {
        Bitrate::Bitrate96 => FileFormat::OGG_VORBIS_96,
        Bitrate::Bitrate160 => FileFormat::OGG_VORBIS_160,
        Bitrate::Bitrate320 => FileFormat::OGG_VORBIS_320,
    };



    let file_id = match track.files.get(&format) {
        Some(&file_id) => file_id,
        None => {
            warn!("Track \"{}\" is not available in format {:?}", track.name, format);
            return None;
        }
    };

    let key = session.audio_key().request(track.id, file_id).wait().unwrap();

    let (open, _) = session.audio_file().open(file_id);
    let encrypted_file = open.wait().unwrap();

    let audio_file = Subfile::new(AudioDecrypt::new(key, encrypted_file), 0xa7);
    let decoder = vorbis::Decoder::new(audio_file).unwrap();

    Some(decoder)
}

fn run_onstart(session: &Session) {
    match session.config().onstart {
        Some(ref program) => util::run_program(program),
        None => {},
    };
}

fn run_onstop(session: &Session) {
    match session.config().onstop {
        Some(ref program) => util::run_program(program),
        None => {},
    };
}

impl PlayerInternal {
    fn run(self, mut sink: Box<Sink>) {
        let mut decoder = None;

        loop {
            let playing = self.state.lock().unwrap().status == PlayStatus::kPlayStatusPlay;
            let cmd = if playing {
                self.commands.try_recv().ok()
            } else {
                Some(self.commands.recv().unwrap())
            };

            match cmd {
                Some(PlayerCommand::Load(track_id, play, position)) => {
                    self.update(|state| {
                        if state.status == PlayStatus::kPlayStatusPlay {
                            sink.stop().unwrap();
                            run_onstop(&self.session);
                        }
                        state.end_of_track = false;
                        state.status = PlayStatus::kPlayStatusPause;
                        state.position_ms = position;
                        state.position_measured_at = util::now_ms();
                        state.track = Some(track_id);
                        true
                    });
                    drop(decoder);

                    decoder = match load_track(&self.session, track_id) {
                        Some(mut decoder) => {
                            match vorbis_time_seek_ms(&mut decoder, position as i64) {
                                Ok(_) => (),
                                Err(err) => error!("Vorbis error: {:?}", err),
                            }

                            self.update(|state| {
                                state.status = if play {
                                    run_onstart(&self.session);
                                    sink.start().unwrap();
                                    PlayStatus::kPlayStatusPlay
                                } else {
                                    PlayStatus::kPlayStatusPause
                                };
                                state.position_ms = position;
                                state.position_measured_at = util::now_ms();

                                true
                            });

                            info!("Load Done");
                            Some(decoder)
                        }

                        None => {
                            self.update(|state| {
                                state.status = PlayStatus::kPlayStatusStop;
                                state.end_of_track = true;
                                true
                            });

                            None
                        }
                    }


                }
                Some(PlayerCommand::Seek(position)) => {
                    match vorbis_time_seek_ms(decoder.as_mut().unwrap(), position as i64) {
                        Ok(_) => (),
                        Err(err) => error!("Vorbis error: {:?}", err),
                    }
                    self.update(|state| {
                        state.position_ms = vorbis_time_tell_ms(decoder.as_mut().unwrap()).unwrap() as u32;
                        state.position_measured_at = util::now_ms();

                        true
                    });
                }
                Some(PlayerCommand::SeekAt(position, measured_at)) => {
                    let position = (util::now_ms() - measured_at + position as i64) as u32;

                    match vorbis_time_seek_ms(decoder.as_mut().unwrap(), position as i64) {
                        Ok(_) => (),
                        Err(err) => error!("Vorbis error: {:?}", err),
                    }
                    self.update(|state| {
                        state.position_ms = vorbis_time_tell_ms(decoder.as_mut().unwrap()).unwrap() as u32;
                        state.position_measured_at = util::now_ms();

                        true
                    });
                }
                Some(PlayerCommand::Play) => {
                    self.update(|state| {
                        state.status = PlayStatus::kPlayStatusPlay;
                        state.position_ms = vorbis_time_tell_ms(decoder.as_mut().unwrap()).unwrap() as u32;
                        state.position_measured_at = util::now_ms();
                        true
                    });

                    run_onstart(&self.session);
                    sink.start().unwrap();
                }
                Some(PlayerCommand::Pause) => {
                    self.update(|state| {
                        state.status = PlayStatus::kPlayStatusPause;
                        state.update_time = util::now_ms();
                        state.position_ms = decoder.as_mut().map(|d| vorbis_time_tell_ms(d).unwrap()).unwrap_or(0) as u32;
                        state.position_measured_at = util::now_ms();
                        true
                    });

                    sink.stop().unwrap();
                    run_onstop(&self.session);
                }
                Some(PlayerCommand::Volume(vol)) => {
                    self.update(|state| {
                        state.volume = vol;
                        true
                    });
                }
                Some(PlayerCommand::Stop) => {
                    self.update(|state| {
                        if state.status == PlayStatus::kPlayStatusPlay {
                            state.status = PlayStatus::kPlayStatusPause;
                        }
                        state.position_ms = 0;
                        state.position_measured_at = util::now_ms();
                        true
                    });

                    sink.stop().unwrap();
                    run_onstop(&self.session);
                    decoder = None;
                }
                None => (),
            }

            if self.state.lock().unwrap().status == PlayStatus::kPlayStatusPlay {
                let packet = decoder.as_mut().unwrap().packets().next();

                match packet {
                    Some(Ok(packet)) => {
                        let buffer = apply_volume(self.state.lock().unwrap().volume,
                                                  &packet.data);
                        sink.write(&buffer).unwrap();

                        self.update(|state| {
                            state.position_ms = vorbis_time_tell_ms(decoder.as_mut().unwrap()).unwrap() as u32;
                            state.position_measured_at = util::now_ms();

                            false
                        });
                    }
                    Some(Err(vorbis::VorbisError::Hole)) => (),
                    Some(Err(e)) => panic!("Vorbis error {:?}", e),
                    None => {
                        self.update(|state| {
                            state.status = PlayStatus::kPlayStatusStop;
                            state.end_of_track = true;
                            true
                        });

                        sink.stop().unwrap();
                        run_onstop(&self.session);
                        decoder = None;
                    }
                }
            }
        }
    }

    fn update<F>(&self, f: F)
        where F: FnOnce(&mut MutexGuard<PlayerState>) -> bool
    {
        let mut guard = self.state.lock().unwrap();
        let update = f(&mut guard);

        let observers = self.observers.lock().unwrap();
        if update {
            guard.update_time = util::now_ms();
            let state = guard.clone();
            drop(guard);

            for observer in observers.iter() {
                observer.send(state.clone()).unwrap();
            }
        }
    }
}

impl PlayerState {
    pub fn status(&self) -> PlayStatus {
        self.status
    }

    pub fn position(&self) -> (u32, i64) {
        (self.position_ms, self.position_measured_at)
    }

    pub fn volume(&self) -> u16 {
        self.volume
    }

    pub fn update_time(&self) -> i64 {
        self.update_time
    }

    pub fn end_of_track(&self) -> bool {
        self.end_of_track
    }
}
