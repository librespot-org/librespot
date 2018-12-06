use byteorder::{LittleEndian, ReadBytesExt};
use futures;
use futures::sync::oneshot;
use futures::{future, Future};
use std;
use std::borrow::Cow;
use std::io::{Read, Result, Seek, SeekFrom};
use std::mem;
use std::sync::mpsc::{RecvError, RecvTimeoutError, TryRecvError};
use std::thread;
use std::time::Duration;

use config::{Bitrate, PlayerConfig};
use core::session::Session;
use core::spotify_id::SpotifyId;

use audio::{AudioDecrypt, AudioFile};
use audio::{VorbisDecoder, VorbisPacket};
use audio_backend::Sink;
use metadata::{FileFormat, Metadata, Track};
use mixer::AudioFilter;

pub struct Player {
    commands: Option<std::sync::mpsc::Sender<PlayerCommand>>,
    thread_handle: Option<thread::JoinHandle<()>>,
}

struct PlayerInternal {
    session: Session,
    config: PlayerConfig,
    commands: std::sync::mpsc::Receiver<PlayerCommand>,

    state: PlayerState,
    sink: Box<Sink>,
    sink_running: bool,
    audio_filter: Option<Box<AudioFilter + Send>>,
    event_sender: futures::sync::mpsc::UnboundedSender<PlayerEvent>,
}

enum PlayerCommand {
    Load(SpotifyId, bool, u32, oneshot::Sender<()>),
    Play,
    Pause,
    Stop,
    Seek(u32),
}

#[derive(Debug, Clone)]
pub enum PlayerEvent {
    Started {
        track_id: SpotifyId,
    },

    Changed {
        old_track_id: SpotifyId,
        new_track_id: SpotifyId,
    },

    Stopped {
        track_id: SpotifyId,
    },
}

type PlayerEventChannel = futures::sync::mpsc::UnboundedReceiver<PlayerEvent>;

#[derive(Clone, Copy, Debug)]
struct NormalisationData {
    track_gain_db: f32,
    track_peak: f32,
    album_gain_db: f32,
    album_peak: f32,
}

impl NormalisationData {
    fn parse_from_file<T: Read + Seek>(mut file: T) -> Result<NormalisationData> {
        const SPOTIFY_NORMALIZATION_HEADER_START_OFFSET: u64 = 144;
        file.seek(SeekFrom::Start(SPOTIFY_NORMALIZATION_HEADER_START_OFFSET))
            .unwrap();

        let track_gain_db = file.read_f32::<LittleEndian>().unwrap();
        let track_peak = file.read_f32::<LittleEndian>().unwrap();
        let album_gain_db = file.read_f32::<LittleEndian>().unwrap();
        let album_peak = file.read_f32::<LittleEndian>().unwrap();

        let r = NormalisationData {
            track_gain_db: track_gain_db,
            track_peak: track_peak,
            album_gain_db: album_gain_db,
            album_peak: album_peak,
        };

        Ok(r)
    }

    fn get_factor(config: &PlayerConfig, data: NormalisationData) -> f32 {
        let mut normalisation_factor =
            f32::powf(10.0, (data.track_gain_db + config.normalisation_pregain) / 20.0);

        if normalisation_factor * data.track_peak > 1.0 {
            warn!("Reducing normalisation factor to prevent clipping. Please add negative pregain to avoid.");
            normalisation_factor = 1.0 / data.track_peak;
        }

        debug!("Normalisation Data: {:?}", data);
        debug!("Applied normalisation factor: {}", normalisation_factor);

        normalisation_factor
    }
}

impl Player {
    pub fn new<F>(
        config: PlayerConfig,
        session: Session,
        audio_filter: Option<Box<AudioFilter + Send>>,
        sink_builder: F,
    ) -> (Player, PlayerEventChannel)
    where
        F: FnOnce() -> Box<Sink> + Send + 'static,
    {
        let (cmd_tx, cmd_rx) = std::sync::mpsc::channel();
        let (event_sender, event_receiver) = futures::sync::mpsc::unbounded();

        let handle = thread::spawn(move || {
            debug!("new Player[{}]", session.session_id());

            let internal = PlayerInternal {
                session: session,
                config: config,
                commands: cmd_rx,

                state: PlayerState::Stopped,
                sink: sink_builder(),
                sink_running: false,
                audio_filter: audio_filter,
                event_sender: event_sender,
            };

            internal.run();
        });

        (
            Player {
                commands: Some(cmd_tx),
                thread_handle: Some(handle),
            },
            event_receiver,
        )
    }

    fn command(&self, cmd: PlayerCommand) {
        self.commands.as_ref().unwrap().send(cmd).unwrap();
    }

    pub fn load(
        &self,
        track: SpotifyId,
        start_playing: bool,
        position_ms: u32,
    ) -> oneshot::Receiver<()> {
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
}

impl Drop for Player {
    fn drop(&mut self) {
        debug!("Shutting down player thread ...");
        self.commands = None;
        if let Some(handle) = self.thread_handle.take() {
            match handle.join() {
                Ok(_) => (),
                Err(_) => error!("Player thread panicked!"),
            }
        }
    }
}

type Decoder = VorbisDecoder<Subfile<AudioDecrypt<AudioFile>>>;
enum PlayerState {
    Stopped,
    Paused {
        track_id: SpotifyId,
        decoder: Decoder,
        end_of_track: oneshot::Sender<()>,
        normalisation_factor: f32,
    },
    Playing {
        track_id: SpotifyId,
        decoder: Decoder,
        end_of_track: oneshot::Sender<()>,
        normalisation_factor: f32,
    },
    EndOfTrack {
        track_id: SpotifyId,
    },
    Invalid,
}

impl PlayerState {
    fn is_playing(&self) -> bool {
        use self::PlayerState::*;
        match *self {
            Stopped | EndOfTrack { .. } | Paused { .. } => false,
            Playing { .. } => true,
            Invalid => panic!("invalid state"),
        }
    }

    fn decoder(&mut self) -> Option<&mut Decoder> {
        use self::PlayerState::*;
        match *self {
            Stopped | EndOfTrack { .. } => None,
            Paused { ref mut decoder, .. } | Playing { ref mut decoder, .. } => Some(decoder),
            Invalid => panic!("invalid state"),
        }
    }

    fn playing_to_end_of_track(&mut self) {
        use self::PlayerState::*;
        match mem::replace(self, Invalid) {
            Playing {
                track_id,
                end_of_track,
                ..
            } => {
                let _ = end_of_track.send(());
                *self = EndOfTrack { track_id };
            }
            _ => panic!("Called playing_to_end_of_track in non-playing state."),
        }
    }

    fn paused_to_playing(&mut self) {
        use self::PlayerState::*;
        match ::std::mem::replace(self, Invalid) {
            Paused {
                track_id,
                decoder,
                end_of_track,
                normalisation_factor,
            } => {
                *self = Playing {
                    track_id: track_id,
                    decoder: decoder,
                    end_of_track: end_of_track,
                    normalisation_factor: normalisation_factor,
                };
            }
            _ => panic!("invalid state"),
        }
    }

    fn playing_to_paused(&mut self) {
        use self::PlayerState::*;
        match ::std::mem::replace(self, Invalid) {
            Playing {
                track_id,
                decoder,
                end_of_track,
                normalisation_factor,
            } => {
                *self = Paused {
                    track_id: track_id,
                    decoder: decoder,
                    end_of_track: end_of_track,
                    normalisation_factor: normalisation_factor,
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
                if self.sink_running {
                    match self.commands.try_recv() {
                        Ok(cmd) => Some(cmd),
                        Err(TryRecvError::Empty) => None,
                        Err(TryRecvError::Disconnected) => return,
                    }
                } else {
                    match self.commands.recv_timeout(Duration::from_secs(5)) {
                        Ok(cmd) => Some(cmd),
                        Err(RecvTimeoutError::Timeout) => None,
                        Err(RecvTimeoutError::Disconnected) => return,
                    }
                }
            } else {
                match self.commands.recv() {
                    Ok(cmd) => Some(cmd),
                    Err(RecvError) => return,
                }
            };

            if let Some(cmd) = cmd {
                self.handle_command(cmd);
            }

            if self.state.is_playing() && !self.sink_running {
                self.start_sink();
            }

            if self.sink_running {
                let mut current_normalisation_factor: f32 = 1.0;

                let packet = if let PlayerState::Playing {
                    ref mut decoder,
                    normalisation_factor,
                    ..
                } = self.state
                {
                    current_normalisation_factor = normalisation_factor;
                    Some(decoder.next_packet().expect("Vorbis error"))
                } else {
                    None
                };

                if let Some(packet) = packet {
                    self.handle_packet(packet, current_normalisation_factor);
                }
            }

            if self.session.is_invalid() {
                return;
            }
        }
    }

    fn start_sink(&mut self) {
        match self.sink.start() {
            Ok(()) => self.sink_running = true,
            Err(err) => error!("Could not start audio: {}", err),
        }
    }

    fn stop_sink_if_running(&mut self) {
        if self.sink_running {
            self.stop_sink();
        }
    }

    fn stop_sink(&mut self) {
        self.sink.stop().unwrap();
        self.sink_running = false;
    }

    fn handle_packet(&mut self, packet: Option<VorbisPacket>, normalisation_factor: f32) {
        match packet {
            Some(mut packet) => {
                if packet.data().len() > 0 {
                    if let Some(ref editor) = self.audio_filter {
                        editor.modify_stream(&mut packet.data_mut())
                    };

                    if self.config.normalisation && normalisation_factor != 1.0 {
                        for x in packet.data_mut().iter_mut() {
                            *x = (*x as f32 * normalisation_factor) as i16;
                        }
                    }

                    if let Err(err) = self.sink.write(&packet.data()) {
                        error!("Could not write audio: {}", err);
                        self.stop_sink();
                    }
                }
            }

            None => {
                self.stop_sink();
                self.state.playing_to_end_of_track();
            }
        }
    }

    fn handle_command(&mut self, cmd: PlayerCommand) {
        debug!("command={:?}", cmd);
        match cmd {
            PlayerCommand::Load(track_id, play, position, end_of_track) => {
                if self.state.is_playing() {
                    self.stop_sink_if_running();
                }

                match self.load_track(track_id, position as i64) {
                    Some((decoder, normalisation_factor)) => {
                        if play {
                            match self.state {
                                PlayerState::Playing {
                                    track_id: old_track_id,
                                    ..
                                }
                                | PlayerState::EndOfTrack {
                                    track_id: old_track_id,
                                    ..
                                } => self.send_event(PlayerEvent::Changed {
                                    old_track_id: old_track_id,
                                    new_track_id: track_id,
                                }),
                                _ => self.send_event(PlayerEvent::Started { track_id }),
                            }

                            self.start_sink();

                            self.state = PlayerState::Playing {
                                track_id: track_id,
                                decoder: decoder,
                                end_of_track: end_of_track,
                                normalisation_factor: normalisation_factor,
                            };
                        } else {
                            self.state = PlayerState::Paused {
                                track_id: track_id,
                                decoder: decoder,
                                end_of_track: end_of_track,
                                normalisation_factor: normalisation_factor,
                            };
                            match self.state {
                                PlayerState::Playing {
                                    track_id: old_track_id,
                                    ..
                                }
                                | PlayerState::EndOfTrack {
                                    track_id: old_track_id,
                                    ..
                                } => self.send_event(PlayerEvent::Changed {
                                    old_track_id: old_track_id,
                                    new_track_id: track_id,
                                }),
                                _ => (),
                            }
                            self.send_event(PlayerEvent::Stopped { track_id });
                        }
                    }

                    None => {
                        let _ = end_of_track.send(());
                    }
                }
            }

            PlayerCommand::Seek(position) => {
                if let Some(decoder) = self.state.decoder() {
                    match decoder.seek(position as i64) {
                        Ok(_) => (),
                        Err(err) => error!("Vorbis error: {:?}", err),
                    }
                } else {
                    warn!("Player::seek called from invalid state");
                }
            }

            PlayerCommand::Play => {
                if let PlayerState::Paused { track_id, .. } = self.state {
                    self.state.paused_to_playing();

                    self.send_event(PlayerEvent::Started { track_id });
                    self.start_sink();
                } else {
                    warn!("Player::play called from invalid state");
                }
            }

            PlayerCommand::Pause => {
                if let PlayerState::Playing { track_id, .. } = self.state {
                    self.state.playing_to_paused();

                    self.stop_sink_if_running();
                    self.send_event(PlayerEvent::Stopped { track_id });
                } else {
                    warn!("Player::pause called from invalid state");
                }
            }

            PlayerCommand::Stop => match self.state {
                PlayerState::Playing { track_id, .. }
                | PlayerState::Paused { track_id, .. }
                | PlayerState::EndOfTrack { track_id } => {
                    self.stop_sink_if_running();
                    self.send_event(PlayerEvent::Stopped { track_id });
                    self.state = PlayerState::Stopped;
                }
                PlayerState::Stopped => {
                    warn!("Player::stop called from invalid state");
                }
                PlayerState::Invalid => panic!("invalid state"),
            },
        }
    }

    fn send_event(&mut self, event: PlayerEvent) {
        let _ = self.event_sender.unbounded_send(event.clone());
    }

    fn find_available_alternative<'a>(&self, track: &'a Track) -> Option<Cow<'a, Track>> {
        if track.available {
            Some(Cow::Borrowed(track))
        } else {
            let alternatives = track
                .alternatives
                .iter()
                .map(|alt_id| Track::get(&self.session, *alt_id));
            let alternatives = future::join_all(alternatives).wait().unwrap();

            alternatives.into_iter().find(|alt| alt.available).map(Cow::Owned)
        }
    }

    fn load_track(&self, track_id: SpotifyId, position: i64) -> Option<(Decoder, f32)> {
        let track = Track::get(&self.session, track_id).wait().unwrap();

        info!(
            "Loading track \"{}\" with Spotify URI \"spotify:track:{}\"",
            track.name,
            track_id.to_base62()
        );

        let track = match self.find_available_alternative(&track) {
            Some(track) => track,
            None => {
                warn!("Track \"{}\" is not available", track.name);
                return None;
            }
        };

        let format = match self.config.bitrate {
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

        let key = self
            .session
            .audio_key()
            .request(track.id, file_id)
            .wait()
            .unwrap();

        let encrypted_file = AudioFile::open(&self.session, file_id).wait().unwrap();

        let mut decrypted_file = AudioDecrypt::new(key, encrypted_file);

        let normalisation_factor = match NormalisationData::parse_from_file(&mut decrypted_file) {
            Ok(normalisation_data) => NormalisationData::get_factor(&self.config, normalisation_data),
            Err(_) => {
                warn!("Unable to extract normalisation data, using default value.");
                1.0 as f32
            }
        };

        let audio_file = Subfile::new(decrypted_file, 0xa7);

        let mut decoder = VorbisDecoder::new(audio_file).unwrap();

        match decoder.seek(position) {
            Ok(_) => (),
            Err(err) => error!("Vorbis error: {:?}", err),
        }

        info!("Track \"{}\" loaded", track.name);

        Some((decoder, normalisation_factor))
    }
}

impl Drop for PlayerInternal {
    fn drop(&mut self) {
        debug!("drop Player[{}]", self.session.session_id());
    }
}

impl ::std::fmt::Debug for PlayerCommand {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            PlayerCommand::Load(track, play, position, _) => f
                .debug_tuple("Load")
                .field(&track)
                .field(&play)
                .field(&position)
                .finish(),
            PlayerCommand::Play => f.debug_tuple("Play").finish(),
            PlayerCommand::Pause => f.debug_tuple("Pause").finish(),
            PlayerCommand::Stop => f.debug_tuple("Stop").finish(),
            PlayerCommand::Seek(position) => f.debug_tuple("Seek").field(&position).finish(),
        }
    }
}

struct Subfile<T: Read + Seek> {
    stream: T,
    offset: u64,
}

impl<T: Read + Seek> Subfile<T> {
    pub fn new(mut stream: T, offset: u64) -> Subfile<T> {
        stream.seek(SeekFrom::Start(offset)).unwrap();
        Subfile {
            stream: stream,
            offset: offset,
        }
    }
}

impl<T: Read + Seek> Read for Subfile<T> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.stream.read(buf)
    }
}

impl<T: Read + Seek> Seek for Subfile<T> {
    fn seek(&mut self, mut pos: SeekFrom) -> Result<u64> {
        pos = match pos {
            SeekFrom::Start(offset) => SeekFrom::Start(offset + self.offset),
            x => x,
        };

        let newpos = try!(self.stream.seek(pos));
        if newpos > self.offset {
            Ok(newpos - self.offset)
        } else {
            Ok(0)
        }
    }
}
