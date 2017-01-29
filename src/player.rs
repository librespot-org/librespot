use futures::sync::oneshot;
use futures::{future, Future};
use std::borrow::Cow;
use std::io::{Read, Seek};
use std::mem;
use std::thread;
use std;
use vorbis::{self, VorbisError};

use audio_backend::Sink;
use audio_decrypt::AudioDecrypt;
use audio_file::AudioFile;
use metadata::{FileFormat, Track};
use session::{Bitrate, Session};
use util::{self, SpotifyId, Subfile};

#[derive(Clone)]
pub struct Player {
    commands: std::sync::mpsc::Sender<PlayerCommand>,
}

struct PlayerInternal {
    session: Session,
    commands: std::sync::mpsc::Receiver<PlayerCommand>,

    state: PlayerState,
    volume: u16,
    sink: Box<Sink>,
}

//#[derive(Debug)]
enum PlayerCommand {
    Load(SpotifyId, bool, u32, oneshot::Sender<()>),
    Play,
    Pause,
    Volume(u16),
    Stop,
    Seek(u32),
}

impl Player {
    pub fn new<F>(session: Session, sink_builder: F) -> Player
        where F: FnOnce() -> Box<Sink> + Send + 'static {
        let (cmd_tx, cmd_rx) = std::sync::mpsc::channel();

        thread::spawn(move || {
            let internal = PlayerInternal {
                session: session,
                commands: cmd_rx,

                state: PlayerState::Stopped,
                volume: 0xFFFF,
                sink: sink_builder(),
            };

            internal.run();
        });

        Player {
            commands: cmd_tx,
        }
    }

    fn command(&self, cmd: PlayerCommand) {
        self.commands.send(cmd).unwrap();
    }

    pub fn load(&self, track: SpotifyId, start_playing: bool, position_ms: u32)
        -> oneshot::Receiver<()>
    {
        let (tx, rx) = oneshot::channel();
        self.command(PlayerCommand::Load(track, start_playing, position_ms, tx));

        rx
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

    pub fn volume(&self, vol: u16) {
        self.command(PlayerCommand::Volume(vol));
    }
}

type Decoder = vorbis::Decoder<Subfile<AudioDecrypt<AudioFile>>>;
enum PlayerState {
    Stopped,
    Paused {
        decoder: Decoder,
        end_of_track: oneshot::Sender<()>,
    },
    Playing {
        decoder: Decoder,
        end_of_track: oneshot::Sender<()>,
    },

    Invalid,
}

impl PlayerState {
    fn is_playing(&self) -> bool {
        use self::PlayerState::*;
        match *self {
            Stopped | Paused { .. } => false,
            Playing { .. } => true,
            Invalid => panic!("invalid state"),
        }
    }

    fn decoder(&mut self) -> Option<&mut Decoder> {
        use self::PlayerState::*;
        match *self {
            Stopped => None,
            Paused { ref mut decoder, .. } |
            Playing { ref mut decoder, .. } => Some(decoder),
            Invalid => panic!("invalid state"),
        }
    }

    fn signal_end_of_track(self) {
        use self::PlayerState::*;
        match self {
            Paused { end_of_track, .. } |
            Playing { end_of_track, .. } => {
                end_of_track.complete(())
            }

            Stopped => warn!("signal_end_of_track from stopped state"),
            Invalid => panic!("invalid state"),
        }
    }

    fn paused_to_playing(&mut self) {
        use self::PlayerState::*;
        match ::std::mem::replace(self, Invalid) {
            Paused { decoder, end_of_track } => {
                *self = Playing {
                    decoder: decoder,
                    end_of_track: end_of_track,
                };
            }
            _ => panic!("invalid state"),
        }
    }

    fn playing_to_paused(&mut self) {
        use self::PlayerState::*;
        match ::std::mem::replace(self, Invalid) {
            Playing { decoder, end_of_track } => {
                *self = Paused {
                    decoder: decoder,
                    end_of_track: end_of_track,
                };
            }
            _ => panic!("invalid state"),
        }
    }
}

impl PlayerInternal {
    fn run(mut self) {
        loop {
            let cmd = if self.state.is_playing() {
                self.commands.try_recv().ok()
            } else {
                Some(self.commands.recv().unwrap())
            };

            if let Some(cmd) = cmd {
                self.handle_command(cmd);
            }

            let packet = if let PlayerState::Playing { ref mut decoder, .. } = self.state {
                Some(decoder.packets().next())
            } else { None };

            if let Some(packet) = packet {
                self.handle_packet(packet);
            }
        }
    }

    fn handle_packet(&mut self, packet: Option<Result<vorbis::Packet, VorbisError>>) {
        match packet {
            Some(Ok(mut packet)) => {
                if self.volume < 0xFFFF {
                    for x in packet.data.iter_mut() {
                        *x = (*x as i32 * self.volume as i32 / 0xFFFF) as i16;
                    }
                }

                self.sink.write(&packet.data).unwrap();
            }

            Some(Err(vorbis::VorbisError::Hole)) => (),
            Some(Err(e)) => panic!("Vorbis error {:?}", e),
            None => {
                self.sink.stop().unwrap();
                self.run_onstop();

                let old_state = mem::replace(&mut self.state, PlayerState::Stopped);
                old_state.signal_end_of_track();
            }
        }
    }

    fn handle_command(&mut self, cmd: PlayerCommand) {
        //debug!("command={:?}", cmd);
        match cmd {
            PlayerCommand::Load(track_id, play, position, end_of_track) => {
                if self.state.is_playing() {
                    self.sink.stop().unwrap();
                }

                match self.load_track(track_id, position as i64) {
                    Some(decoder) => {
                        if play {
                            if !self.state.is_playing() {
                                self.run_onstart();
                            }
                            self.sink.start().unwrap();

                            self.state = PlayerState::Playing {
                                decoder: decoder,
                                end_of_track: end_of_track,
                            };
                        } else {
                            if self.state.is_playing() {
                                self.run_onstop();
                            }

                            self.state = PlayerState::Paused {
                                decoder: decoder,
                                end_of_track: end_of_track,
                            };
                        }
                    }

                    None => {
                        if self.state.is_playing() {
                            self.run_onstop();
                        }
                    }
                }
            }

            PlayerCommand::Seek(position) => {
                if let Some(decoder) = self.state.decoder() {
                    match vorbis_time_seek_ms(decoder, position as i64) {
                        Ok(_) => (),
                        Err(err) => error!("Vorbis error: {:?}", err),
                    }
                } else {
                    warn!("Player::seek called from invalid state");
                }
            }

            PlayerCommand::Play => {
                if let PlayerState::Paused { .. } = self.state {
                    self.state.paused_to_playing();

                    self.run_onstart();
                    self.sink.start().unwrap();
                } else {
                    warn!("Player::play called from invalid state");
                }
            }

            PlayerCommand::Pause => {
                if let PlayerState::Playing { .. } = self.state {
                    self.state.playing_to_paused();

                    self.sink.stop().unwrap();
                    self.run_onstop();
                } else {
                    warn!("Player::pause called from invalid state");
                }
            }

            PlayerCommand::Stop => {
                match self.state {
                    PlayerState::Playing { .. } => {
                        self.sink.stop().unwrap();
                        self.run_onstop();
                        self.state = PlayerState::Stopped;
                    }
                    PlayerState::Paused { .. } => {
                        self.state = PlayerState::Stopped;
                    },
                    PlayerState::Stopped => {
                        warn!("Player::stop called from invalid state");
                    }
                    PlayerState::Invalid => panic!("invalid state"),
                }
            }

            PlayerCommand::Volume(vol) => {
                self.volume = vol;
            }
        }
    }

    fn run_onstart(&self) {
        match self.session.config().onstart {
            Some(ref program) => util::run_program(program),
            None => {},
        };
    }

    fn run_onstop(&self) {
        match self.session.config().onstop {
            Some(ref program) => util::run_program(program),
            None => {},
        };
    }

    fn find_available_alternative<'a>(&self, track: &'a Track) -> Option<Cow<'a, Track>> {
        if track.available {
            Some(Cow::Borrowed(track))
        } else {
            let alternatives = track.alternatives
                .iter()
                .map(|alt_id| {
                    self.session.metadata().get::<Track>(*alt_id)
                });
            let alternatives = future::join_all(alternatives).wait().unwrap();

            alternatives.into_iter().find(|alt| alt.available).map(Cow::Owned)
        }
    }

    fn load_track(&self, track_id: SpotifyId, position: i64) -> Option<Decoder> {
        let track = self.session.metadata().get::<Track>(track_id).wait().unwrap();

        info!("Loading track \"{}\"", track.name);

        let track = match self.find_available_alternative(&track) {
            Some(track) => track,
            None => {
                warn!("Track \"{}\" is not available", track.name);
                return None;
            }
        };

        let format = match self.session.config().bitrate {
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

        let key = self.session.audio_key().request(track.id, file_id).wait().unwrap();

        let (open, _) = self.session.audio_file().open(file_id);
        let encrypted_file = open.wait().unwrap();

        let audio_file = Subfile::new(AudioDecrypt::new(key, encrypted_file), 0xa7);
        let mut decoder = vorbis::Decoder::new(audio_file).unwrap();

        match vorbis_time_seek_ms(&mut decoder, position) {
            Ok(_) => (),
            Err(err) => error!("Vorbis error: {:?}", err),
        }

        info!("Track \"{}\" loaded", track.name);

        Some(decoder)
    }
}

#[cfg(not(feature = "with-tremor"))]
fn vorbis_time_seek_ms<R>(decoder: &mut vorbis::Decoder<R>, ms: i64) -> Result<(), vorbis::VorbisError> where R: Read + Seek {
    decoder.time_seek(ms as f64 / 1000f64)
}

#[cfg(feature = "with-tremor")]
fn vorbis_time_seek_ms<R>(decoder: &mut vorbis::Decoder<R>, ms: i64) -> Result<(), vorbis::VorbisError> where R: Read + Seek {
    decoder.time_seek(ms)
}
