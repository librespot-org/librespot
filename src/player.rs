use eventual::{self, Async};
use portaudio;
use std::borrow::Cow;
use std::sync::{mpsc, Mutex, Arc, MutexGuard};
use std::thread;
use std::io::{Read, Seek};
use vorbis;

use metadata::{FileFormat, Track, TrackRef};
use session::{Bitrate, Session};
use audio_decrypt::AudioDecrypt;
use util::{self, SpotifyId, Subfile};
use spirc::PlayStatus;

#[cfg(not(feature = "with-tremor"))]
fn vorbis_time_seek_ms<R>(decoder: &mut vorbis::Decoder<R>, ms: i64) -> Result<(), vorbis::VorbisError> where R: Read + Seek {
    decoder.time_seek(ms as f64 / 1000f64)
}

#[cfg(not(feature = "with-tremor"))]
fn vorbis_time_tell_ms<R>(decoder: &mut vorbis::Decoder<R>) -> Result<i64, vorbis::VorbisError> where R: Read + Seek {
    decoder.time_tell().map(|t| (t / 1000f64) as i64)
}

#[cfg(feature = "with-tremor")]
fn vorbis_time_seek_ms<R>(decoder: &mut vorbis::Decoder<R>, ms: i64) -> Result<(), vorbis::VorbisError> where R: Read + Seek {
    decoder.time_seek(ms)
}

#[cfg(feature = "with-tremor")]
fn vorbis_time_tell_ms<R>(decoder: &mut vorbis::Decoder<R>) -> Result<i64, vorbis::VorbisError> where R: Read + Seek {
    decoder.time_tell()
}

pub type PlayerObserver = Box<Fn(&PlayerState) + Send>;

pub struct Player {
    state: Arc<Mutex<PlayerState>>,
    observers: Arc<Mutex<Vec<PlayerObserver>>>,

    commands: mpsc::Sender<PlayerCommand>,
}

#[derive(Clone)]
pub struct PlayerState {
    status: PlayStatus,
    position_ms: u32,
    position_measured_at: i64,
    update_time: i64,
    volume: u16,

    end_of_track: bool,
}

struct PlayerInternal {
    state: Arc<Mutex<PlayerState>>,
    observers: Arc<Mutex<Vec<PlayerObserver>>>,

    session: Session,
    commands: mpsc::Receiver<PlayerCommand>,
}

#[derive(Debug)]
enum PlayerCommand {
    Load(SpotifyId, bool, u32),
    Play,
    Pause,
    Volume(u16),
    Stop,
    Seek(u32),
}

impl Player {
    pub fn new(session: Session) -> Player {
        let (cmd_tx, cmd_rx) = mpsc::channel();

        let state = Arc::new(Mutex::new(PlayerState {
            status: PlayStatus::kPlayStatusStop,
            position_ms: 0,
            position_measured_at: 0,
            update_time: util::now_ms(),
            volume: 0xFFFF,
            end_of_track: false,
        }));

        let observers = Arc::new(Mutex::new(Vec::new()));

        let internal = PlayerInternal {
            session: session,
            commands: cmd_rx,
            state: state.clone(),
            observers: observers.clone(),
        };

        thread::spawn(move || internal.run());

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

    pub fn state(&self) -> PlayerState {
        self.state.lock().unwrap().clone()
    }

    pub fn volume(&self, vol: u16) {
        self.command(PlayerCommand::Volume(vol));
    }

    pub fn add_observer(&self, observer: PlayerObserver) {
        self.observers.lock().unwrap().push(observer);
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
                            stream.stop().unwrap();
                        }
                        state.end_of_track = false;
                        state.status = PlayStatus::kPlayStatusPause;
                        state.position_ms = position;
                        state.position_measured_at = util::now_ms();
                        true
                    });
                    drop(decoder);

                    let mut track = self.session.metadata::<Track>(track_id).await().unwrap();

                    if !track.available {
                        let alternatives = track.alternatives
                                                .iter()
                                                .map(|alt_id| {
                                                    self.session.metadata::<Track>(*alt_id)
                                                })
                                                .collect::<Vec<TrackRef>>();

                        track = eventual::sequence(alternatives.into_iter())
                                    .iter()
                                    .find(|alt| alt.available)
                                    .unwrap();
                    }

                    let format = match self.session.config().bitrate {
                        Bitrate::Bitrate96 => FileFormat::OGG_VORBIS_96,
                        Bitrate::Bitrate160 => FileFormat::OGG_VORBIS_160,
                        Bitrate::Bitrate320 => FileFormat::OGG_VORBIS_320,
                    };
                    let (file_id, _) = track.files.into_iter().find(|&(_, f)| f == format).unwrap();

                    let key = self.session.audio_key(track.id, file_id).await().unwrap();
                    decoder = Some(
                        vorbis::Decoder::new(
                        Subfile::new(
                        AudioDecrypt::new(key,
                        self.session.audio_file(file_id)), 0xa7)).unwrap());

                    vorbis_time_seek_ms(decoder.as_mut().unwrap(), position as i64).unwrap()

                    self.update(|state| {
                        state.status = if play {
                            stream.start().unwrap();
                            PlayStatus::kPlayStatusPlay
                        } else {
                            PlayStatus::kPlayStatusPause
                        };
                        state.position_ms = position;
                        state.position_measured_at = util::now_ms();

                        true
                    });
                    println!("Load Done");
                }
                Some(PlayerCommand::Seek(position)) => {
                    vorbis_time_seek_ms(decoder.as_mut().unwrap(), position as i64).unwrap()
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

                    stream.start().unwrap();
                }
                Some(PlayerCommand::Pause) => {
                    self.update(|state| {
                        state.status = PlayStatus::kPlayStatusPause;
                        state.update_time = util::now_ms();
                        state.position_ms = vorbis_time_tell_ms(decoder.as_mut().unwrap()).unwrap() as u32;
                        state.position_measured_at = util::now_ms();
                        true
                    });

                    stream.stop().unwrap();
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

                    stream.stop().unwrap();
                    decoder = None;
                }
                None => (),
            }

            if self.state.lock().unwrap().status == PlayStatus::kPlayStatusPlay {
                match decoder.as_mut().unwrap().packets().next() {
                    Some(Ok(packet)) => {
                        let buffer = apply_volume(self.state.lock().unwrap().volume,
                                                  &packet.data);
                        match stream.write(&buffer) {
                            Ok(_) => (),
                            Err(portaudio::PaError::OutputUnderflowed) => eprintln!("Underflow"),
                            Err(e) => panic!("PA Error {}", e),
                        };
                    }
                    Some(Err(vorbis::VorbisError::Hole)) => (),
                    Some(Err(e)) => panic!("Vorbis error {:?}", e),
                    None => {
                        self.update(|state| {
                            state.status = PlayStatus::kPlayStatusStop;
                            state.end_of_track = true;
                            true
                        });

                        stream.stop().unwrap();
                        decoder = None;
                    }
                }
            }
        }

        drop(stream);

        portaudio::terminate().unwrap();
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
                observer(&state);
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
