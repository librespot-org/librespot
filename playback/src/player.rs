use byteorder::{LittleEndian, ReadBytesExt};
use futures;
use futures::sync::oneshot;
use futures::{future, Future};
use std;
use std::borrow::Cow;
use std::io::{Error, ErrorKind, Read, Result, Seek, SeekFrom};
use std::mem;
use std::sync::mpsc::{RecvError, TryRecvError};
use std::thread;

use config::{Bitrate, PlayerConfig};
use core::session::Session;
use core::spotify_id::SpotifyId;

use audio::VorbisDecoder;
use audio::{AudioDecrypt, AudioFile};
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
    track: Option<LoadedTrack>,
    sink: Box<Sink>,
    audio_filter: Option<Box<AudioFilter + Send>>,
    event_sender: futures::sync::mpsc::UnboundedSender<PlayerEvent>,
}

enum PlayerCommand {
    Load(SpotifyId, bool, u32, oneshot::Sender<()>),
    Play,
    Pause,
    Stop,
    Seek(u32),
    /// Update volume of the player.
    Volume(Option<f32>),
}

#[derive(Debug, Clone)]
pub enum PlayerEvent {
    /// The given track has been loaded into the player.
    Started { track_id: SpotifyId },

    /// The track has been changed in the player.
    Changed {
        old_track_id: SpotifyId,
        new_track_id: SpotifyId,
    },

    /// The given track has been unloaded from the player.
    Stopped { track_id: SpotifyId },
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
                track: None,
                audio_filter: audio_filter,
                event_sender: event_sender,
            };

            if let Err(e) = internal.run() {
                panic!("player thread errored: {}", e);
            }
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

    /// Update the volume of the player.
    pub fn volume(&self, volume: Option<f32>) {
        self.command(PlayerCommand::Volume(volume))
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

#[derive(Debug, Clone, Copy)]
enum PlayerState {
    /// Stop the sink and close the current track.
    Stopped,
    /// Pause playback.
    Paused,
    /// Start playing.
    Playing,
}

struct LoadedTrack {
    track_id: SpotifyId,
    decoder: Decoder,
    /// oneshot that must called when a track is unloaded.
    end_of_track: Option<oneshot::Sender<()>>,
    normalisation_factor: f32,
}

impl Drop for LoadedTrack {
    fn drop(&mut self) {
        if let Some(end_of_track) = mem::replace(&mut self.end_of_track, None) {
            let _ = end_of_track.send(());
        }
    }
}

impl PlayerState {
    fn is_playing(&self) -> bool {
        match *self {
            PlayerState::Playing => true,
            PlayerState::Stopped | PlayerState::Paused => false,
        }
    }
}

impl PlayerInternal {
    fn run(mut self) -> Result<()> {
        loop {
            let cmd = if self.state.is_playing() {
                match self.commands.try_recv() {
                    Ok(cmd) => Some(cmd),
                    Err(TryRecvError::Empty) => None,
                    Err(TryRecvError::Disconnected) => return Ok(()),
                }
            } else {
                match self.commands.recv() {
                    Ok(cmd) => Some(cmd),
                    Err(RecvError) => return Ok(()),
                }
            };

            if let Some(cmd) = cmd {
                self.handle_command(cmd)?;
            }

            if self.handle_packet()? {
                if let Some(track) = self.track.take() {
                    self.sink.stop()?;
                    self.send_event(PlayerEvent::Stopped {
                        track_id: track.track_id,
                    });
                }

                self.state = PlayerState::Stopped;
            }

            if self.session.is_invalid() {
                return Err(Error::new(ErrorKind::Other, "session not valid"));
            }
        }
    }

    /// Handle processing of a single packet.
    ///
    /// Returns `true` when the track has ended.
    fn handle_packet(&mut self) -> Result<bool> {
        if !self.state.is_playing() {
            return Ok(false);
        }

        let track = match self.track.as_mut() {
            Some(track) => track,
            None => return Ok(false),
        };

        let mut normalisation_factor = track.normalisation_factor;

        let mut packet = match track
            .decoder
            .next_packet()
            .map_err(|e| Error::new(ErrorKind::Other, e))?
        {
            Some(packet) => packet,
            // end of track
            None => return Ok(true),
        };

        if let Some(volume) = self.config.volume {
            normalisation_factor *= volume;
        }

        if packet.data().len() > 0 {
            if let Some(ref editor) = self.audio_filter {
                editor.modify_stream(&mut packet.data_mut())
            };

            let adjusted = self.config.normalisation || self.config.volume.is_some();

            if adjusted && normalisation_factor != 1.0 {
                for x in packet.data_mut() {
                    *x = (*x as f32 * normalisation_factor) as i16;
                }
            }

            // on errors, stop the sink and end the current track.
            // this gives the user a chance to e.g. skip the current song.
            if let Err(e) = self.sink.write(&packet.data()) {
                error!("sink error: {}", e);
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn handle_command(&mut self, cmd: PlayerCommand) -> Result<()> {
        debug!("command={:?}", cmd);

        match cmd {
            PlayerCommand::Load(track_id, play, position, end_of_track) => {
                let (decoder, normalisation_factor) = match self.load_track(track_id, position as i64) {
                    Some(result) => result,
                    None => return Err(Error::new(ErrorKind::Other, "failed to load track")),
                };

                // NB: we always swap out the loaded track.
                let mut track = mem::replace(
                    &mut self.track,
                    Some(LoadedTrack {
                        track_id,
                        decoder,
                        end_of_track: Some(end_of_track),
                        normalisation_factor,
                    }),
                );

                if let Some(old_track_id) = track.map(|t| t.track_id) {
                    self.send_event(PlayerEvent::Changed {
                        old_track_id,
                        new_track_id: track_id,
                    })
                } else {
                    self.send_event(PlayerEvent::Started { track_id })
                }

                match (play, self.state) {
                    (true, PlayerState::Paused) => {
                        self.sink.start()?;
                        self.state = PlayerState::Playing;
                    }
                    (false, PlayerState::Paused) => {
                        // ignore: wants to pause but already paused.
                    }
                    (true, PlayerState::Playing) => {
                        // ignore: wants to play but already playing.
                    }
                    (false, PlayerState::Playing) => {
                        self.sink.stop()?;
                        self.state = PlayerState::Paused;
                    }
                    (true, PlayerState::Stopped) => {
                        self.sink.start()?;
                        self.state = PlayerState::Playing;
                    }
                    (false, PlayerState::Stopped) => {
                        self.state = PlayerState::Paused;
                    }
                }
            }

            PlayerCommand::Seek(position) => {
                if let Some(track) = self.track.as_mut() {
                    if let Err(e) = track.decoder.seek(position as i64) {
                        error!("seek error: {}", e);
                    }
                }
            }

            PlayerCommand::Play => {
                let track_id = match self.track.as_ref() {
                    Some(track) => track.track_id,
                    None => {
                        warn!("no track loaded");
                        return Ok(());
                    }
                };

                match self.state {
                    PlayerState::Playing => {
                        warn!("already playing");
                    }
                    PlayerState::Stopped => {
                        self.sink.start()?;
                        self.state = PlayerState::Playing;
                        self.send_event(PlayerEvent::Started { track_id });
                    }
                    PlayerState::Paused => {
                        self.sink.start()?;
                        self.state = PlayerState::Playing;
                    }
                }
            }

            PlayerCommand::Pause => match self.state {
                PlayerState::Playing => {
                    self.sink.stop()?;
                    self.state = PlayerState::Paused;
                }
                PlayerState::Stopped => {
                    warn!("player is stopped");
                }
                PlayerState::Paused => {
                    // ignore
                }
            },

            // unload current track.
            PlayerCommand::Stop => {
                let mut track = match self.track.take() {
                    Some(track) => track,
                    None => {
                        warn!("no track loaded");
                        return Ok(());
                    }
                };

                match self.state {
                    PlayerState::Playing | PlayerState::Paused => {
                        self.sink.stop()?;
                        self.send_event(PlayerEvent::Stopped {
                            track_id: track.track_id,
                        });
                        self.state = PlayerState::Stopped;
                    }
                    PlayerState::Stopped => {
                        // ignore
                    }
                }
            }

            PlayerCommand::Volume(volume) => {
                self.config.volume = volume;
            }
        }

        Ok(())
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
            .request(track.id, file_id);
        let encrypted_file = AudioFile::open(&self.session, file_id);


        let encrypted_file = encrypted_file.wait().unwrap();
        let key = key.wait().unwrap();
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

        if position != 0 {
            match decoder.seek(position) {
                Ok(_) => (),
                Err(err) => error!("Vorbis error: {:?}", err),
            }
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
            PlayerCommand::Volume(volume) => f.debug_tuple("Volume").field(&volume).finish(),
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
