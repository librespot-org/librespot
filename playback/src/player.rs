use std::{
    collections::HashMap,
    fmt,
    future::Future,
    io::{self, Read, Seek, SeekFrom},
    mem,
    pin::Pin,
    process::exit,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    task::{Context, Poll},
    thread,
    time::{Duration, Instant},
};

use futures_util::{
    future, future::FusedFuture, stream::futures_unordered::FuturesUnordered, StreamExt,
    TryFutureExt,
};
use parking_lot::Mutex;
use symphonia::core::io::MediaSource;
use tokio::sync::{mpsc, oneshot};

use crate::{
    audio::{
        AudioDecrypt, AudioFile, StreamLoaderController, READ_AHEAD_BEFORE_PLAYBACK,
        READ_AHEAD_DURING_PLAYBACK,
    },
    audio_backend::Sink,
    config::{Bitrate, PlayerConfig},
    core::{util::SeqGenerator, Error, Session, SpotifyId},
    decoder::{AudioDecoder, AudioPacket, AudioPacketPosition, SymphoniaDecoder},
    metadata::audio::{AudioFileFormat, AudioFiles, AudioItem},
    mixer::VolumeGetter,
    sample_pipeline::SamplePipeline,
};

#[cfg(feature = "passthrough-decoder")]
use crate::decoder::PassthroughDecoder;

const PRELOAD_NEXT_TRACK_BEFORE_END_DURATION_MS: u32 = 30000;

// Spotify inserts a custom Ogg packet at the start with custom metadata values, that you would
// otherwise expect in Vorbis comments. This packet isn't well-formed and players may balk at it.
const SPOTIFY_OGG_HEADER_END: u64 = 0xa7;

pub type PlayerResult = Result<(), Error>;

pub struct Player {
    commands: Option<mpsc::UnboundedSender<PlayerCommand>>,
    thread_handle: Option<thread::JoinHandle<()>>,
    play_request_id_generator: SeqGenerator<u64>,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum SinkStatus {
    Running,
    Closed,
    TemporarilyClosed,
}

pub type SinkEventCallback = Box<dyn Fn(SinkStatus) + Send>;

struct PlayerInternal {
    session: Session,
    config: PlayerConfig,
    commands: mpsc::UnboundedReceiver<PlayerCommand>,
    load_handles: Arc<Mutex<HashMap<thread::ThreadId, thread::JoinHandle<()>>>>,

    state: PlayerState,
    preload: PlayerPreload,
    sink_status: SinkStatus,
    sink_event_callback: Option<SinkEventCallback>,
    sample_pipeline: SamplePipeline,
    event_senders: Vec<mpsc::UnboundedSender<PlayerEvent>>,

    auto_normalise_as_album: bool,

    player_id: usize,
}

pub static PLAYER_COUNTER: AtomicUsize = AtomicUsize::new(0);

enum PlayerCommand {
    Load {
        track_id: SpotifyId,
        play_request_id: u64,
        play: bool,
        position_ms: u32,
    },
    Preload {
        track_id: SpotifyId,
    },
    Play,
    Pause,
    Stop,
    Seek(u32),
    AddEventSender(mpsc::UnboundedSender<PlayerEvent>),
    SetSinkEventCallback(Option<SinkEventCallback>),
    EmitVolumeChangedEvent(u16),
    SetAutoNormaliseAsAlbum(bool),
    EmitSessionDisconnectedEvent {
        connection_id: String,
        user_name: String,
    },
    EmitSessionConnectedEvent {
        connection_id: String,
        user_name: String,
    },
    EmitSessionClientChangedEvent {
        client_id: String,
        client_name: String,
        client_brand_name: String,
        client_model_name: String,
    },
    EmitFilterExplicitContentChangedEvent(bool),
    EmitShuffleChangedEvent(bool),
    EmitRepeatChangedEvent(bool),
    EmitAutoPlayChangedEvent(bool),
}

#[derive(Debug, Clone)]
pub enum PlayerEvent {
    // Fired when the player is stopped (e.g. by issuing a "stop" command to the player).
    Stopped {
        play_request_id: u64,
        track_id: SpotifyId,
    },
    // The player is delayed by loading a track.
    Loading {
        play_request_id: u64,
        track_id: SpotifyId,
        position_ms: u32,
    },
    // The player is preloading a track.
    Preloading {
        track_id: SpotifyId,
    },
    // The player is playing a track.
    // This event is issued at the start of playback of whenever the position must be communicated
    // because it is out of sync. This includes:
    // start of a track
    // un-pausing
    // after a seek
    // after a buffer-underrun
    Playing {
        play_request_id: u64,
        track_id: SpotifyId,
        position_ms: u32,
    },
    // The player entered a paused state.
    Paused {
        play_request_id: u64,
        track_id: SpotifyId,
        position_ms: u32,
    },
    // The player thinks it's a good idea to issue a preload command for the next track now.
    // This event is intended for use within spirc.
    TimeToPreloadNextTrack {
        play_request_id: u64,
        track_id: SpotifyId,
    },
    // The player reached the end of a track.
    // This event is intended for use within spirc. Spirc will respond by issuing another command.
    EndOfTrack {
        play_request_id: u64,
        track_id: SpotifyId,
    },
    // The player was unable to load the requested track.
    Unavailable {
        play_request_id: u64,
        track_id: SpotifyId,
    },
    // The mixer volume was set to a new level.
    VolumeChanged {
        volume: u16,
    },
    PositionCorrection {
        play_request_id: u64,
        track_id: SpotifyId,
        position_ms: u32,
    },
    Seeked {
        play_request_id: u64,
        track_id: SpotifyId,
        position_ms: u32,
    },
    TrackChanged {
        audio_item: Box<AudioItem>,
    },
    SessionConnected {
        connection_id: String,
        user_name: String,
    },
    SessionDisconnected {
        connection_id: String,
        user_name: String,
    },
    SessionClientChanged {
        client_id: String,
        client_name: String,
        client_brand_name: String,
        client_model_name: String,
    },
    ShuffleChanged {
        shuffle: bool,
    },
    RepeatChanged {
        repeat: bool,
    },
    AutoPlayChanged {
        auto_play: bool,
    },
    FilterExplicitContentChanged {
        filter: bool,
    },
}

impl PlayerEvent {
    pub fn get_play_request_id(&self) -> Option<u64> {
        use PlayerEvent::*;
        match self {
            Loading {
                play_request_id, ..
            }
            | Unavailable {
                play_request_id, ..
            }
            | Playing {
                play_request_id, ..
            }
            | TimeToPreloadNextTrack {
                play_request_id, ..
            }
            | EndOfTrack {
                play_request_id, ..
            }
            | Paused {
                play_request_id, ..
            }
            | Stopped {
                play_request_id, ..
            }
            | PositionCorrection {
                play_request_id, ..
            }
            | Seeked {
                play_request_id, ..
            } => Some(*play_request_id),
            _ => None,
        }
    }
}

pub type PlayerEventChannel = mpsc::UnboundedReceiver<PlayerEvent>;

#[derive(Clone, Copy, Debug)]
pub struct NormalisationData {
    // Spotify provides these as `f32`, but audio metadata can contain up to `f64`.
    // Also, this negates the need for casting during sample processing.
    pub track_gain_db: f64,
    pub track_peak: f64,
    pub album_gain_db: f64,
    pub album_peak: f64,
}

impl Default for NormalisationData {
    fn default() -> Self {
        Self {
            track_gain_db: 0.0,
            track_peak: 1.0,
            album_gain_db: 0.0,
            album_peak: 1.0,
        }
    }
}

impl NormalisationData {
    fn parse_from_ogg<T: Read + Seek>(mut file: T) -> io::Result<NormalisationData> {
        const SPOTIFY_NORMALIZATION_HEADER_START_OFFSET: u64 = 144;
        const NORMALISATION_DATA_SIZE: usize = 16;

        let newpos = file.seek(SeekFrom::Start(SPOTIFY_NORMALIZATION_HEADER_START_OFFSET))?;
        if newpos != SPOTIFY_NORMALIZATION_HEADER_START_OFFSET {
            error!(
                "NormalisationData::parse_from_file seeking to {} but position is now {}",
                SPOTIFY_NORMALIZATION_HEADER_START_OFFSET, newpos
            );

            error!("Falling back to default (non-track and non-album) normalisation data.");

            return Ok(NormalisationData::default());
        }

        let mut buf = [0u8; NORMALISATION_DATA_SIZE];

        file.read_exact(&mut buf)?;

        let track_gain_db = f32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]) as f64;
        let track_peak = f32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]) as f64;
        let album_gain_db = f32::from_le_bytes([buf[8], buf[9], buf[10], buf[11]]) as f64;
        let album_peak = f32::from_le_bytes([buf[12], buf[13], buf[14], buf[15]]) as f64;

        Ok(Self {
            track_gain_db,
            track_peak,
            album_gain_db,
            album_peak,
        })
    }
}

impl Player {
    pub fn new<F>(
        config: PlayerConfig,
        session: Session,
        volume_getter: Box<dyn VolumeGetter>,
        sink_builder: F,
    ) -> Self
    where
        F: FnOnce() -> Box<dyn Sink> + Send + 'static,
    {
        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();

        let player_id = PLAYER_COUNTER.fetch_add(1, Ordering::AcqRel);

        let thread_name = format!("player:{}", player_id);

        let builder = thread::Builder::new().name(thread_name.clone());

        let handle = match builder.spawn(move || {
            let sample_pipeline = SamplePipeline::new(&config, sink_builder(), volume_getter);

            let internal = PlayerInternal {
                session,
                config,
                commands: cmd_rx,
                load_handles: Arc::new(Mutex::new(HashMap::new())),

                state: PlayerState::Stopped,
                preload: PlayerPreload::None,
                sink_status: SinkStatus::Closed,
                sink_event_callback: None,
                sample_pipeline,
                event_senders: vec![],

                auto_normalise_as_album: false,

                player_id,
            };

            // While PlayerInternal is written as a future, it still contains blocking code.
            // It must be run by using block_on() in a dedicated thread.
            let runtime = match tokio::runtime::Runtime::new() {
                Ok(runtime) => runtime,
                Err(e) => {
                    match thread::current().name() {
                        Some(name) => debug!("Error creating <PlayerInternal> [{name}] thread, Failed to create Tokio runtime: {e}"),
                        None => debug!("Error creating <PlayerInternal> thread, Failed to create Tokio runtime: {e}"),
                    }

                    exit(1);
                }
            };

            runtime.block_on(internal);

            match thread::current().name() {
                Some(name) => debug!("<PlayerInternal> [{name}] thread finished"),
                None => debug!("<PlayerInternal> thread finished"),
            }
        }) {
            Ok(handle) => {
                debug!("Created <PlayerInternal> [{thread_name}] thread");
                handle
            }
            Err(e) => {
                error!("Error creating <PlayerInternal> [{thread_name}] thread: {e}");
                exit(1);
            }
        };

        Self {
            commands: Some(cmd_tx),
            thread_handle: Some(handle),
            play_request_id_generator: SeqGenerator::new(0),
        }
    }

    fn command(&self, cmd: PlayerCommand) {
        if let Some(commands) = self.commands.as_ref() {
            if let Err(e) = commands.send(cmd) {
                error!("Player Commands Error: {}", e);
            }
        }
    }

    pub fn load(&mut self, track_id: SpotifyId, start_playing: bool, position_ms: u32) -> u64 {
        let play_request_id = self.play_request_id_generator.get();
        self.command(PlayerCommand::Load {
            track_id,
            play_request_id,
            play: start_playing,
            position_ms,
        });

        play_request_id
    }

    pub fn preload(&self, track_id: SpotifyId) {
        self.command(PlayerCommand::Preload { track_id });
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

    pub fn get_player_event_channel(&self) -> PlayerEventChannel {
        let (event_sender, event_receiver) = mpsc::unbounded_channel();
        self.command(PlayerCommand::AddEventSender(event_sender));
        event_receiver
    }

    pub async fn await_end_of_track(&self) {
        let mut channel = self.get_player_event_channel();
        while let Some(event) = channel.recv().await {
            if matches!(
                event,
                PlayerEvent::EndOfTrack { .. } | PlayerEvent::Stopped { .. }
            ) {
                return;
            }
        }
    }

    pub fn set_sink_event_callback(&self, callback: Option<SinkEventCallback>) {
        self.command(PlayerCommand::SetSinkEventCallback(callback));
    }

    pub fn emit_volume_changed_event(&self, volume: u16) {
        self.command(PlayerCommand::EmitVolumeChangedEvent(volume));
    }

    pub fn set_auto_normalise_as_album(&self, setting: bool) {
        self.command(PlayerCommand::SetAutoNormaliseAsAlbum(setting));
    }

    pub fn emit_filter_explicit_content_changed_event(&self, filter: bool) {
        self.command(PlayerCommand::EmitFilterExplicitContentChangedEvent(filter));
    }

    pub fn emit_session_connected_event(&self, connection_id: String, user_name: String) {
        self.command(PlayerCommand::EmitSessionConnectedEvent {
            connection_id,
            user_name,
        });
    }

    pub fn emit_session_disconnected_event(&self, connection_id: String, user_name: String) {
        self.command(PlayerCommand::EmitSessionDisconnectedEvent {
            connection_id,
            user_name,
        });
    }

    pub fn emit_session_client_changed_event(
        &self,
        client_id: String,
        client_name: String,
        client_brand_name: String,
        client_model_name: String,
    ) {
        self.command(PlayerCommand::EmitSessionClientChangedEvent {
            client_id,
            client_name,
            client_brand_name,
            client_model_name,
        });
    }

    pub fn emit_shuffle_changed_event(&self, shuffle: bool) {
        self.command(PlayerCommand::EmitShuffleChangedEvent(shuffle));
    }

    pub fn emit_repeat_changed_event(&self, repeat: bool) {
        self.command(PlayerCommand::EmitRepeatChangedEvent(repeat));
    }

    pub fn emit_auto_play_changed_event(&self, auto_play: bool) {
        self.command(PlayerCommand::EmitAutoPlayChangedEvent(auto_play));
    }
}

impl Drop for Player {
    fn drop(&mut self) {
        debug!("Shutting down <Player> thread ...");
        self.commands = None;
        if let Some(handle) = self.thread_handle.take() {
            if let Err(e) = handle.join() {
                error!("<Player> thread Error: {:?}", e);
            }
        }
    }
}

struct PlayerLoadedTrackData {
    decoder: Decoder,
    normalisation_data: NormalisationData,
    stream_loader_controller: StreamLoaderController,
    audio_item: AudioItem,
    bytes_per_second: usize,
    duration_ms: u32,
    stream_position_ms: u32,
    is_explicit: bool,
}

enum PlayerPreload {
    None,
    Loading {
        track_id: SpotifyId,
        loader: Pin<Box<dyn FusedFuture<Output = Result<PlayerLoadedTrackData, ()>> + Send>>,
    },
    Ready {
        track_id: SpotifyId,
        loaded_track: Box<PlayerLoadedTrackData>,
    },
}

type Decoder = Box<dyn AudioDecoder + Send>;

enum PlayerState {
    Stopped,
    Loading {
        track_id: SpotifyId,
        play_request_id: u64,
        start_playback: bool,
        loader: Pin<Box<dyn FusedFuture<Output = Result<PlayerLoadedTrackData, ()>> + Send>>,
    },
    Paused {
        track_id: SpotifyId,
        play_request_id: u64,
        decoder: Decoder,
        audio_item: AudioItem,
        normalisation_data: NormalisationData,
        stream_loader_controller: StreamLoaderController,
        bytes_per_second: usize,
        duration_ms: u32,
        stream_position_ms: u32,
        suggested_to_preload_next_track: bool,
        is_explicit: bool,
    },
    Playing {
        track_id: SpotifyId,
        play_request_id: u64,
        decoder: Decoder,
        normalisation_data: NormalisationData,
        audio_item: AudioItem,
        stream_loader_controller: StreamLoaderController,
        bytes_per_second: usize,
        duration_ms: u32,
        stream_position_ms: u32,
        reported_nominal_start_time: Option<Instant>,
        suggested_to_preload_next_track: bool,
        is_explicit: bool,
    },
    EndOfTrack {
        track_id: SpotifyId,
        play_request_id: u64,
        loaded_track: PlayerLoadedTrackData,
    },
    Invalid,
}

impl PlayerState {
    fn is_playing(&self) -> bool {
        use self::PlayerState::*;
        match *self {
            Stopped | EndOfTrack { .. } | Paused { .. } | Loading { .. } => false,
            Playing { .. } => true,
            Invalid => {
                error!("PlayerState::is_playing in invalid state");
                exit(1);
            }
        }
    }

    #[allow(dead_code)]
    fn is_stopped(&self) -> bool {
        use self::PlayerState::*;
        matches!(self, Stopped)
    }

    #[allow(dead_code)]
    fn is_loading(&self) -> bool {
        use self::PlayerState::*;
        matches!(self, Loading { .. })
    }

    fn decoder(&mut self) -> Option<&mut Decoder> {
        use self::PlayerState::*;
        match *self {
            Stopped | EndOfTrack { .. } | Loading { .. } => None,
            Paused {
                ref mut decoder, ..
            }
            | Playing {
                ref mut decoder, ..
            } => Some(decoder),
            Invalid => {
                error!("PlayerState::decoder in invalid state");
                exit(1);
            }
        }
    }

    fn playing_to_end_of_track(&mut self) {
        use self::PlayerState::*;
        let new_state = mem::replace(self, Invalid);
        match new_state {
            Playing {
                track_id,
                play_request_id,
                decoder,
                duration_ms,
                bytes_per_second,
                normalisation_data,
                stream_loader_controller,
                stream_position_ms,
                is_explicit,
                audio_item,
                ..
            } => {
                *self = EndOfTrack {
                    track_id,
                    play_request_id,
                    loaded_track: PlayerLoadedTrackData {
                        decoder,
                        normalisation_data,
                        stream_loader_controller,
                        audio_item,
                        bytes_per_second,
                        duration_ms,
                        stream_position_ms,
                        is_explicit,
                    },
                };
            }
            _ => {
                error!(
                    "Called playing_to_end_of_track in non-playing state: {:?}",
                    new_state
                );
                exit(1);
            }
        }
    }

    fn paused_to_playing(&mut self) {
        use self::PlayerState::*;
        let new_state = mem::replace(self, Invalid);
        match new_state {
            Paused {
                track_id,
                play_request_id,
                decoder,
                audio_item,
                normalisation_data,
                stream_loader_controller,
                duration_ms,
                bytes_per_second,
                stream_position_ms,
                suggested_to_preload_next_track,
                is_explicit,
            } => {
                *self = Playing {
                    track_id,
                    play_request_id,
                    decoder,
                    audio_item,
                    normalisation_data,
                    stream_loader_controller,
                    duration_ms,
                    bytes_per_second,
                    stream_position_ms,
                    reported_nominal_start_time: Instant::now()
                        .checked_sub(Duration::from_millis(stream_position_ms as u64)),
                    suggested_to_preload_next_track,
                    is_explicit,
                };
            }
            _ => {
                error!(
                    "PlayerState::paused_to_playing in invalid state: {:?}",
                    new_state
                );
                exit(1);
            }
        }
    }

    fn playing_to_paused(&mut self) {
        use self::PlayerState::*;
        let new_state = mem::replace(self, Invalid);
        match new_state {
            Playing {
                track_id,
                play_request_id,
                decoder,
                audio_item,
                normalisation_data,
                stream_loader_controller,
                duration_ms,
                bytes_per_second,
                stream_position_ms,
                suggested_to_preload_next_track,
                is_explicit,
                ..
            } => {
                *self = Paused {
                    track_id,
                    play_request_id,
                    decoder,
                    audio_item,
                    normalisation_data,
                    stream_loader_controller,
                    duration_ms,
                    bytes_per_second,
                    stream_position_ms,
                    suggested_to_preload_next_track,
                    is_explicit,
                };
            }
            _ => {
                error!(
                    "PlayerState::playing_to_paused in invalid state: {:?}",
                    new_state
                );
                exit(1);
            }
        }
    }
}

struct PlayerTrackLoader {
    session: Session,
    config: PlayerConfig,
}

impl PlayerTrackLoader {
    async fn find_available_alternative(&self, audio_item: AudioItem) -> Option<AudioItem> {
        if let Err(e) = audio_item.availability {
            error!("Track is unavailable: {}", e);
            None
        } else if !audio_item.files.is_empty() {
            Some(audio_item)
        } else if let Some(alternatives) = &audio_item.alternatives {
            let alternatives: FuturesUnordered<_> = alternatives
                .iter()
                .map(|alt_id| AudioItem::get_file(&self.session, *alt_id))
                .collect();

            alternatives
                .filter_map(|x| future::ready(x.ok()))
                .filter(|x| future::ready(x.availability.is_ok()))
                .next()
                .await
        } else {
            error!("Track should be available, but no alternatives found.");
            None
        }
    }

    fn stream_data_rate(&self, format: AudioFileFormat) -> usize {
        let kbps = match format {
            AudioFileFormat::OGG_VORBIS_96 => 12,
            AudioFileFormat::OGG_VORBIS_160 => 20,
            AudioFileFormat::OGG_VORBIS_320 => 40,
            AudioFileFormat::MP3_256 => 32,
            AudioFileFormat::MP3_320 => 40,
            AudioFileFormat::MP3_160 => 20,
            AudioFileFormat::MP3_96 => 12,
            AudioFileFormat::MP3_160_ENC => 20,
            AudioFileFormat::AAC_24 => 3,
            AudioFileFormat::AAC_48 => 6,
            AudioFileFormat::FLAC_FLAC => 112, // assume 900 kbit/s on average
        };
        kbps * 1024
    }

    async fn load_track(
        &self,
        spotify_id: SpotifyId,
        position_ms: u32,
    ) -> Option<PlayerLoadedTrackData> {
        let audio_item = match AudioItem::get_file(&self.session, spotify_id).await {
            Ok(audio) => match self.find_available_alternative(audio).await {
                Some(audio) => audio,
                None => {
                    warn!(
                        "<{}> is not available",
                        spotify_id.to_uri().unwrap_or_default()
                    );
                    return None;
                }
            },
            Err(e) => {
                error!("Unable to load audio item: {:?}", e);
                return None;
            }
        };

        info!(
            "Loading <{}> with Spotify URI <{}>",
            audio_item.name, audio_item.uri
        );

        // (Most) podcasts seem to support only 96 kbps Ogg Vorbis, so fall back to it
        let formats = match self.config.bitrate {
            Bitrate::Bitrate96 => [
                AudioFileFormat::OGG_VORBIS_96,
                AudioFileFormat::MP3_96,
                AudioFileFormat::OGG_VORBIS_160,
                AudioFileFormat::MP3_160,
                AudioFileFormat::MP3_256,
                AudioFileFormat::OGG_VORBIS_320,
                AudioFileFormat::MP3_320,
            ],
            Bitrate::Bitrate160 => [
                AudioFileFormat::OGG_VORBIS_160,
                AudioFileFormat::MP3_160,
                AudioFileFormat::OGG_VORBIS_96,
                AudioFileFormat::MP3_96,
                AudioFileFormat::MP3_256,
                AudioFileFormat::OGG_VORBIS_320,
                AudioFileFormat::MP3_320,
            ],
            Bitrate::Bitrate320 => [
                AudioFileFormat::OGG_VORBIS_320,
                AudioFileFormat::MP3_320,
                AudioFileFormat::MP3_256,
                AudioFileFormat::OGG_VORBIS_160,
                AudioFileFormat::MP3_160,
                AudioFileFormat::OGG_VORBIS_96,
                AudioFileFormat::MP3_96,
            ],
        };

        let (format, file_id) =
            match formats
                .iter()
                .find_map(|format| match audio_item.files.get(format) {
                    Some(&file_id) => Some((*format, file_id)),
                    _ => None,
                }) {
                Some(t) => t,
                None => {
                    warn!(
                        "<{}> is not available in any supported format",
                        audio_item.name
                    );
                    return None;
                }
            };

        let bytes_per_second = self.stream_data_rate(format);

        // This is only a loop to be able to reload the file if an error occurred
        // while opening a cached file.
        loop {
            let encrypted_file = AudioFile::open(&self.session, file_id, bytes_per_second);

            let encrypted_file = match encrypted_file.await {
                Ok(encrypted_file) => encrypted_file,
                Err(e) => {
                    error!("Unable to load encrypted file: {:?}", e);
                    return None;
                }
            };

            let is_cached = encrypted_file.is_cached();

            let stream_loader_controller = encrypted_file.get_stream_loader_controller().ok()?;

            // Not all audio files are encrypted. If we can't get a key, try loading the track
            // without decryption. If the file was encrypted after all, the decoder will fail
            // parsing and bail out, so we should be safe from outputting ear-piercing noise.
            let key = match self.session.audio_key().request(spotify_id, file_id).await {
                Ok(key) => Some(key),
                Err(e) => {
                    warn!("Unable to load key, continuing without decryption: {}", e);
                    None
                }
            };
            let mut decrypted_file = AudioDecrypt::new(key, encrypted_file);

            let is_ogg_vorbis = AudioFiles::is_ogg_vorbis(format);
            let (offset, mut normalisation_data) = if is_ogg_vorbis {
                // Spotify stores normalisation data in a custom Ogg packet instead of Vorbis comments.
                let normalisation_data =
                    NormalisationData::parse_from_ogg(&mut decrypted_file).ok();
                (SPOTIFY_OGG_HEADER_END, normalisation_data)
            } else {
                (0, None)
            };

            let audio_file = match Subfile::new(
                decrypted_file,
                offset,
                stream_loader_controller.len() as u64,
            ) {
                Ok(audio_file) => audio_file,
                Err(e) => {
                    error!("PlayerTrackLoader::load_track error opening subfile: {}", e);
                    return None;
                }
            };

            let mut symphonia_decoder = |audio_file, format| {
                SymphoniaDecoder::new(audio_file, format).map(|mut decoder| {
                    // For formats other that Vorbis, we'll try getting normalisation data from
                    // ReplayGain metadata fields, if present.
                    if normalisation_data.is_none() {
                        normalisation_data = decoder.normalisation_data();
                    }
                    Box::new(decoder) as Decoder
                })
            };

            #[cfg(feature = "passthrough-decoder")]
            let decoder_type = if self.config.passthrough {
                PassthroughDecoder::new(audio_file, format).map(|x| Box::new(x) as Decoder)
            } else {
                symphonia_decoder(audio_file, format)
            };

            #[cfg(not(feature = "passthrough-decoder"))]
            let decoder_type = symphonia_decoder(audio_file, format);

            let normalisation_data = normalisation_data.unwrap_or_else(|| {
                warn!("Unable to get normalisation data, continuing with defaults.");
                NormalisationData::default()
            });

            let mut decoder = match decoder_type {
                Ok(decoder) => decoder,
                Err(e) if is_cached => {
                    warn!(
                        "Unable to read cached audio file: {}. Trying to download it.",
                        e
                    );

                    match self.session.cache() {
                        Some(cache) => {
                            if cache.remove_file(file_id).is_err() {
                                error!("Error removing file from cache");
                                return None;
                            }
                        }
                        None => {
                            error!("If the audio file is cached, a cache should exist");
                            return None;
                        }
                    }

                    // Just try it again
                    continue;
                }
                Err(e) => {
                    error!("Unable to read audio file: {}", e);
                    return None;
                }
            };

            let duration_ms = audio_item.duration_ms;
            // Don't try to seek past the track's duration.
            // If the position is invalid just start from
            // the beginning of the track.
            let position_ms = if position_ms > duration_ms {
                warn!("Invalid start position of {} ms exceeds track's duration of {} ms, starting track from the beginning", position_ms, duration_ms);
                0
            } else {
                position_ms
            };

            // Ensure the starting position. Even when we want to play from the beginning,
            // the cursor may have been moved by parsing normalisation data. This may not
            // matter for playback (but won't hurt either), but may be useful for the
            // passthrough decoder.
            let stream_position_ms = match decoder.seek(position_ms) {
                Ok(new_position_ms) => new_position_ms,
                Err(e) => {
                    error!(
                        "PlayerTrackLoader::load_track error seeking to starting position {}: {}",
                        position_ms, e
                    );
                    return None;
                }
            };

            // Ensure streaming mode now that we are ready to play from the requested position.
            stream_loader_controller.set_stream_mode();

            let is_explicit = audio_item.is_explicit;

            info!("<{}> ({} ms) loaded", audio_item.name, duration_ms);

            return Some(PlayerLoadedTrackData {
                decoder,
                normalisation_data,
                stream_loader_controller,
                audio_item,
                bytes_per_second,
                duration_ms,
                stream_position_ms,
                is_explicit,
            });
        }
    }
}

impl Future for PlayerInternal {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        // While this is written as a future, it still contains blocking code.
        // It must be run on its own thread.
        let passthrough = self.config.passthrough;

        loop {
            let mut all_futures_completed_or_not_ready = true;

            // process commands that were sent to us
            let cmd = match self.commands.poll_recv(cx) {
                Poll::Ready(None) => return Poll::Ready(()), // client has disconnected - shut down.
                Poll::Ready(Some(cmd)) => {
                    all_futures_completed_or_not_ready = false;
                    Some(cmd)
                }
                _ => None,
            };

            if let Some(cmd) = cmd {
                if let Err(e) = self.handle_command(cmd) {
                    error!("Error handling command: {}", e);
                }
            }

            // Handle loading of a new track to play
            if let PlayerState::Loading {
                ref mut loader,
                track_id,
                start_playback,
                play_request_id,
            } = self.state
            {
                // The loader may be terminated if we are trying to load the same track
                // as before, and that track failed to open before.
                if !loader.as_mut().is_terminated() {
                    match loader.as_mut().poll(cx) {
                        Poll::Ready(Ok(loaded_track)) => {
                            self.start_playback(
                                track_id,
                                play_request_id,
                                loaded_track,
                                start_playback,
                            );
                            if let PlayerState::Loading { .. } = self.state {
                                error!("The state wasn't changed by start_playback()");
                                exit(1);
                            }
                        }
                        Poll::Ready(Err(e)) => {
                            error!(
                                "Skipping to next track, unable to load track <{:?}>: {:?}",
                                track_id, e
                            );
                            self.send_event(PlayerEvent::Unavailable {
                                track_id,
                                play_request_id,
                            })
                        }
                        Poll::Pending => (),
                    }
                }
            }

            // handle pending preload requests.
            if let PlayerPreload::Loading {
                ref mut loader,
                track_id,
            } = self.preload
            {
                match loader.as_mut().poll(cx) {
                    Poll::Ready(Ok(loaded_track)) => {
                        self.send_event(PlayerEvent::Preloading { track_id });
                        self.preload = PlayerPreload::Ready {
                            track_id,
                            loaded_track: Box::new(loaded_track),
                        };
                    }
                    Poll::Ready(Err(_)) => {
                        debug!("Unable to preload {:?}", track_id);
                        self.preload = PlayerPreload::None;
                        // Let Spirc know that the track was unavailable.
                        if let PlayerState::Playing {
                            play_request_id, ..
                        }
                        | PlayerState::Paused {
                            play_request_id, ..
                        } = self.state
                        {
                            self.send_event(PlayerEvent::Unavailable {
                                track_id,
                                play_request_id,
                            });
                        }
                    }
                    Poll::Pending => (),
                }
            }

            if self.state.is_playing() {
                self.ensure_sink_running();

                let sample_pipeline_latency_ms = self.sample_pipeline.get_latency_ms();

                if let PlayerState::Playing {
                    track_id,
                    play_request_id,
                    ref mut decoder,
                    ref mut stream_position_ms,
                    ref mut reported_nominal_start_time,
                    ..
                } = self.state
                {
                    match decoder.next_packet() {
                        Ok(result) => {
                            if let Some((ref packet_position, ref packet)) = result {
                                let decoder_position_ms = packet_position.position_ms;

                                *stream_position_ms =
                                    decoder_position_ms.saturating_sub(sample_pipeline_latency_ms);

                                if !passthrough {
                                    match packet.samples() {
                                        Ok(_) => {
                                            let now = Instant::now();

                                            let notify_about_position = {
                                                // It would be so much easier to use elapsed but elapsed could
                                                // potentially panic is rare cases.
                                                // See:
                                                // https://doc.rust-lang.org/std/time/struct.Instant.html#monotonicity
                                                //
                                                // Otherwise this is pretty straight forward. If anything fails getting
                                                // expected_position_ms it will return 0 which will trigger a notify if
                                                // either stream_position_ms or decoder_position_ms is > 1000. If all goes
                                                // well it's simply a matter of calculating the max delta of expected_position_ms
                                                // and stream_position_ms and expected_position_ms and decoder_position_ms.
                                                // So if the decoder or the sample pipeline are off by more than 1 sec we notify.
                                                let expected_position_ms = now
                                                    .checked_duration_since(
                                                        reported_nominal_start_time.unwrap_or(now),
                                                    )
                                                    .unwrap_or(Duration::ZERO)
                                                    .as_millis();

                                                let max_expected_position_delta_ms =
                                                    expected_position_ms
                                                        .abs_diff(*stream_position_ms as u128)
                                                        .max(
                                                            expected_position_ms.abs_diff(
                                                                decoder_position_ms as u128,
                                                            ),
                                                        );

                                                max_expected_position_delta_ms > 1000
                                            };

                                            if notify_about_position {
                                                let position_ms = *stream_position_ms;

                                                *reported_nominal_start_time = now.checked_sub(
                                                    Duration::from_millis(position_ms as u64),
                                                );

                                                self.send_event(PlayerEvent::PositionCorrection {
                                                    play_request_id,
                                                    track_id,
                                                    position_ms,
                                                });
                                            }
                                        }
                                        Err(e) => {
                                            error!("Skipping to next track, unable to decode samples for track <{:?}>: {:?}", track_id, e);
                                            self.send_event(PlayerEvent::EndOfTrack {
                                                track_id,
                                                play_request_id,
                                            })
                                        }
                                    }
                                }
                            }

                            self.handle_packet(result);
                        }
                        Err(e) => {
                            error!("Skipping to next track, unable to get next packet for track <{:?}>: {:?}", track_id, e);
                            self.send_event(PlayerEvent::EndOfTrack {
                                track_id,
                                play_request_id,
                            })
                        }
                    }
                } else {
                    error!("PlayerInternal poll: Invalid PlayerState");
                    exit(1);
                };
            }

            if let PlayerState::Playing {
                track_id,
                play_request_id,
                duration_ms,
                stream_position_ms,
                ref mut stream_loader_controller,
                ref mut suggested_to_preload_next_track,
                ..
            }
            | PlayerState::Paused {
                track_id,
                play_request_id,
                duration_ms,
                stream_position_ms,
                ref mut stream_loader_controller,
                ref mut suggested_to_preload_next_track,
                ..
            } = self.state
            {
                if (!*suggested_to_preload_next_track)
                    && ((duration_ms as i64 - stream_position_ms as i64)
                        < PRELOAD_NEXT_TRACK_BEFORE_END_DURATION_MS as i64)
                    && stream_loader_controller.range_to_end_available()
                {
                    *suggested_to_preload_next_track = true;
                    self.send_event(PlayerEvent::TimeToPreloadNextTrack {
                        track_id,
                        play_request_id,
                    });
                }
            }

            if self.session.is_invalid() {
                return Poll::Ready(());
            }

            if (!self.state.is_playing()) && all_futures_completed_or_not_ready {
                return Poll::Pending;
            }
        }
    }
}

impl PlayerInternal {
    fn ensure_sink_running(&mut self) {
        if self.sink_status != SinkStatus::Running {
            trace!("== Starting sink ==");
            if let Some(callback) = &mut self.sink_event_callback {
                callback(SinkStatus::Running);
            }
            match self.sample_pipeline.start() {
                Ok(()) => self.sink_status = SinkStatus::Running,
                Err(e) => {
                    error!("{}", e);
                    self.handle_pause();
                }
            }
        }
    }

    fn ensure_sink_stopped(&mut self, temporarily: bool) {
        match self.sink_status {
            SinkStatus::Running => {
                trace!("== Stopping sink ==");
                match self.sample_pipeline.stop() {
                    Ok(()) => {
                        self.sink_status = if temporarily {
                            SinkStatus::TemporarilyClosed
                        } else {
                            SinkStatus::Closed
                        };
                        if let Some(callback) = &mut self.sink_event_callback {
                            callback(self.sink_status);
                        }
                    }
                    Err(e) => {
                        error!("{}", e);
                        exit(1);
                    }
                }
            }
            SinkStatus::TemporarilyClosed => {
                if !temporarily {
                    self.sink_status = SinkStatus::Closed;
                    if let Some(callback) = &mut self.sink_event_callback {
                        callback(SinkStatus::Closed);
                    }
                }
            }
            SinkStatus::Closed => (),
        }
    }

    fn handle_player_stop(&mut self) {
        match self.state {
            PlayerState::Playing {
                track_id,
                play_request_id,
                ..
            }
            | PlayerState::Paused {
                track_id,
                play_request_id,
                ..
            }
            | PlayerState::EndOfTrack {
                track_id,
                play_request_id,
                ..
            }
            | PlayerState::Loading {
                track_id,
                play_request_id,
                ..
            } => {
                self.ensure_sink_stopped(false);
                self.send_event(PlayerEvent::Stopped {
                    track_id,
                    play_request_id,
                });
                self.state = PlayerState::Stopped;
            }
            PlayerState::Stopped => (),
            PlayerState::Invalid => {
                error!("PlayerInternal::handle_player_stop in invalid state");
                exit(1);
            }
        }
    }

    fn handle_play(&mut self) {
        match self.state {
            PlayerState::Paused {
                track_id,
                play_request_id,
                stream_position_ms,
                ..
            } => {
                self.state.paused_to_playing();
                self.send_event(PlayerEvent::Playing {
                    track_id,
                    play_request_id,
                    position_ms: stream_position_ms,
                });
                self.ensure_sink_running();
            }
            PlayerState::Loading {
                ref mut start_playback,
                ..
            } => {
                *start_playback = true;
            }
            _ => error!("Player::play called from invalid state: {:?}", self.state),
        }
    }

    fn handle_pause(&mut self) {
        match self.state {
            PlayerState::Paused { .. } => self.ensure_sink_stopped(false),
            PlayerState::Playing {
                track_id,
                play_request_id,
                stream_position_ms,
                ..
            } => {
                self.state.playing_to_paused();

                self.ensure_sink_stopped(false);
                self.send_event(PlayerEvent::Paused {
                    track_id,
                    play_request_id,
                    position_ms: stream_position_ms,
                });
            }
            PlayerState::Loading {
                ref mut start_playback,
                ..
            } => {
                *start_playback = false;
            }
            _ => error!("Player::pause called from invalid state: {:?}", self.state),
        }
    }

    fn handle_packet(&mut self, packet: Option<(AudioPacketPosition, AudioPacket)>) {
        match packet {
            Some((_, packet)) => {
                if !packet.is_empty() {
                    if let Err(e) = self.sample_pipeline.write(packet) {
                        error!("{}", e);
                        self.handle_pause();
                    }
                }
            }
            None => {
                self.state.playing_to_end_of_track();
                if let PlayerState::EndOfTrack {
                    track_id,
                    play_request_id,
                    ..
                } = self.state
                {
                    self.send_event(PlayerEvent::EndOfTrack {
                        track_id,
                        play_request_id,
                    })
                } else {
                    error!("PlayerInternal handle_packet: Invalid PlayerState");
                    exit(1);
                }
            }
        }
    }

    fn start_playback(
        &mut self,
        track_id: SpotifyId,
        play_request_id: u64,
        loaded_track: PlayerLoadedTrackData,
        start_playback: bool,
    ) {
        let audio_item = Box::new(loaded_track.audio_item.clone());

        self.send_event(PlayerEvent::TrackChanged { audio_item });

        let position_ms = loaded_track.stream_position_ms;

        self.sample_pipeline.update_normalisation_data(
            self.auto_normalise_as_album,
            loaded_track.normalisation_data,
        );

        if start_playback {
            self.ensure_sink_running();
            self.send_event(PlayerEvent::Playing {
                track_id,
                play_request_id,
                position_ms,
            });

            self.state = PlayerState::Playing {
                track_id,
                play_request_id,
                decoder: loaded_track.decoder,
                audio_item: loaded_track.audio_item,
                normalisation_data: loaded_track.normalisation_data,
                stream_loader_controller: loaded_track.stream_loader_controller,
                duration_ms: loaded_track.duration_ms,
                bytes_per_second: loaded_track.bytes_per_second,
                stream_position_ms: loaded_track.stream_position_ms,
                reported_nominal_start_time: Instant::now()
                    .checked_sub(Duration::from_millis(position_ms as u64)),
                suggested_to_preload_next_track: false,
                is_explicit: loaded_track.is_explicit,
            };
        } else {
            self.ensure_sink_stopped(false);

            self.state = PlayerState::Paused {
                track_id,
                play_request_id,
                decoder: loaded_track.decoder,
                audio_item: loaded_track.audio_item,
                normalisation_data: loaded_track.normalisation_data,
                stream_loader_controller: loaded_track.stream_loader_controller,
                duration_ms: loaded_track.duration_ms,
                bytes_per_second: loaded_track.bytes_per_second,
                stream_position_ms: loaded_track.stream_position_ms,
                suggested_to_preload_next_track: false,
                is_explicit: loaded_track.is_explicit,
            };

            self.send_event(PlayerEvent::Paused {
                track_id,
                play_request_id,
                position_ms,
            });
        }
    }

    fn handle_command_load(
        &mut self,
        track_id: SpotifyId,
        play_request_id: u64,
        play: bool,
        position_ms: u32,
    ) -> PlayerResult {
        if !self.config.gapless {
            self.ensure_sink_stopped(play);
        }

        if matches!(self.state, PlayerState::Invalid { .. }) {
            return Err(Error::internal(format!(
                "Player::handle_command_load called from invalid state: {:?}",
                self.state
            )));
        }

        // Now we check at different positions whether we already have a pre-loaded version
        // of this track somewhere. If so, use it and return.

        // Check if there's a matching loaded track in the EndOfTrack player state.
        // This is the case if we're repeating the same track again.
        if let PlayerState::EndOfTrack {
            track_id: previous_track_id,
            ..
        } = self.state
        {
            if previous_track_id == track_id {
                let mut loaded_track = match mem::replace(&mut self.state, PlayerState::Invalid) {
                    PlayerState::EndOfTrack { loaded_track, .. } => loaded_track,
                    _ => {
                        return Err(Error::internal(format!("PlayerInternal::handle_command_load repeating the same track: invalid state: {:?}", self.state)));
                    }
                };

                if position_ms != loaded_track.stream_position_ms {
                    // This may be blocking.
                    loaded_track.stream_position_ms = loaded_track.decoder.seek(position_ms)?;
                }
                self.preload = PlayerPreload::None;
                self.start_playback(track_id, play_request_id, loaded_track, play);
                if let PlayerState::Invalid = self.state {
                    return Err(Error::internal(format!("PlayerInternal::handle_command_load repeating the same track: start_playback() did not transition to valid player state: {:?}", self.state)));
                }
                return Ok(());
            }
        }

        // Check if we are already playing the track. If so, just do a seek and update our info.
        if let PlayerState::Playing {
            track_id: current_track_id,
            ref mut stream_position_ms,
            ref mut decoder,
            ..
        }
        | PlayerState::Paused {
            track_id: current_track_id,
            ref mut stream_position_ms,
            ref mut decoder,
            ..
        } = self.state
        {
            if current_track_id == track_id {
                // we can use the current decoder. Ensure it's at the correct position.
                if position_ms != *stream_position_ms {
                    // This may be blocking.
                    *stream_position_ms = decoder.seek(position_ms)?;
                }

                // Move the info from the current state into a PlayerLoadedTrackData so we can use
                // the usual code path to start playback.
                let old_state = mem::replace(&mut self.state, PlayerState::Invalid);

                if let PlayerState::Playing {
                    stream_position_ms,
                    decoder,
                    audio_item,
                    stream_loader_controller,
                    bytes_per_second,
                    duration_ms,
                    normalisation_data,
                    is_explicit,
                    ..
                }
                | PlayerState::Paused {
                    stream_position_ms,
                    decoder,
                    audio_item,
                    stream_loader_controller,
                    bytes_per_second,
                    duration_ms,
                    normalisation_data,
                    is_explicit,
                    ..
                } = old_state
                {
                    let loaded_track = PlayerLoadedTrackData {
                        decoder,
                        normalisation_data,
                        stream_loader_controller,
                        audio_item,
                        bytes_per_second,
                        duration_ms,
                        stream_position_ms,
                        is_explicit,
                    };

                    self.preload = PlayerPreload::None;
                    self.start_playback(track_id, play_request_id, loaded_track, play);

                    if let PlayerState::Invalid = self.state {
                        return Err(Error::internal(format!("PlayerInternal::handle_command_load already playing this track: start_playback() did not transition to valid player state: {:?}", self.state)));
                    }

                    return Ok(());
                } else {
                    return Err(Error::internal(format!("PlayerInternal::handle_command_load already playing this track: invalid state: {:?}", self.state)));
                }
            }
        }

        // Check if the requested track has been preloaded already. If so use the preloaded data.
        if let PlayerPreload::Ready {
            track_id: loaded_track_id,
            ..
        } = self.preload
        {
            if track_id == loaded_track_id {
                let preload = std::mem::replace(&mut self.preload, PlayerPreload::None);
                if let PlayerPreload::Ready {
                    track_id,
                    mut loaded_track,
                } = preload
                {
                    if position_ms != loaded_track.stream_position_ms {
                        // This may be blocking
                        loaded_track.stream_position_ms = loaded_track.decoder.seek(position_ms)?;
                    }
                    self.start_playback(track_id, play_request_id, *loaded_track, play);
                    return Ok(());
                } else {
                    return Err(Error::internal(format!("PlayerInternal::handle_command_loading preloaded track: invalid state: {:?}", self.state)));
                }
            }
        }

        // We need to load the track - either from scratch or by completing a preload.
        // In any case we go into a Loading state to load the track.
        self.ensure_sink_stopped(play);

        self.send_event(PlayerEvent::Loading {
            track_id,
            play_request_id,
            position_ms,
        });

        // Try to extract a pending loader from the preloading mechanism
        let loader = if let PlayerPreload::Loading {
            track_id: loaded_track_id,
            ..
        } = self.preload
        {
            if (track_id == loaded_track_id) && (position_ms == 0) {
                let mut preload = PlayerPreload::None;
                std::mem::swap(&mut preload, &mut self.preload);
                if let PlayerPreload::Loading { loader, .. } = preload {
                    Some(loader)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        self.preload = PlayerPreload::None;

        // If we don't have a loader yet, create one from scratch.
        let loader = loader.unwrap_or_else(|| Box::pin(self.load_track(track_id, position_ms)));

        // Set ourselves to a loading state.
        self.state = PlayerState::Loading {
            track_id,
            play_request_id,
            start_playback: play,
            loader,
        };

        Ok(())
    }

    fn handle_command_preload(&mut self, track_id: SpotifyId) {
        debug!("Preloading track");
        let mut preload_track = true;
        // check whether the track is already loaded somewhere or being loaded.
        if let PlayerPreload::Loading {
            track_id: currently_loading,
            ..
        }
        | PlayerPreload::Ready {
            track_id: currently_loading,
            ..
        } = self.preload
        {
            if currently_loading == track_id {
                // we're already preloading the requested track.
                preload_track = false;
            } else {
                // we're preloading something else - cancel it.
                self.preload = PlayerPreload::None;
            }
        }

        if let PlayerState::Playing {
            track_id: current_track_id,
            ..
        }
        | PlayerState::Paused {
            track_id: current_track_id,
            ..
        }
        | PlayerState::EndOfTrack {
            track_id: current_track_id,
            ..
        } = self.state
        {
            if current_track_id == track_id {
                // we already have the requested track loaded.
                preload_track = false;
            }
        }

        // schedule the preload of the current track if desired.
        if preload_track {
            let loader = self.load_track(track_id, 0);
            self.preload = PlayerPreload::Loading {
                track_id,
                loader: Box::pin(loader),
            }
        }
    }

    fn handle_command_seek(&mut self, position_ms: u32) -> PlayerResult {
        // When we are still loading, the user may immediately ask to
        // seek to another position yet the decoder won't be ready for
        // that. In this case just restart the loading process but
        // with the requested position.
        if let PlayerState::Loading {
            track_id,
            play_request_id,
            start_playback,
            ..
        } = self.state
        {
            return self.handle_command_load(
                track_id,
                play_request_id,
                start_playback,
                position_ms,
            );
        }

        if let Some(decoder) = self.state.decoder() {
            match decoder.seek(position_ms) {
                Ok(new_position_ms) => {
                    if let PlayerState::Playing {
                        ref mut stream_position_ms,
                        track_id,
                        play_request_id,
                        ..
                    }
                    | PlayerState::Paused {
                        ref mut stream_position_ms,
                        track_id,
                        play_request_id,
                        ..
                    } = self.state
                    {
                        *stream_position_ms = new_position_ms;

                        self.send_event(PlayerEvent::Seeked {
                            play_request_id,
                            track_id,
                            position_ms: new_position_ms,
                        });
                    }
                }
                Err(e) => error!("PlayerInternal::handle_command_seek error: {}", e),
            }
        } else {
            error!("Player::seek called from invalid state: {:?}", self.state);
        }

        // ensure we have a bit of a buffer of downloaded data
        self.preload_data_before_playback()?;

        if let PlayerState::Playing {
            ref mut reported_nominal_start_time,
            ..
        } = self.state
        {
            *reported_nominal_start_time =
                Instant::now().checked_sub(Duration::from_millis(position_ms as u64));
        }

        Ok(())
    }

    fn handle_command(&mut self, cmd: PlayerCommand) -> PlayerResult {
        debug!("command={:?}", cmd);
        match cmd {
            PlayerCommand::Load {
                track_id,
                play_request_id,
                play,
                position_ms,
            } => self.handle_command_load(track_id, play_request_id, play, position_ms)?,

            PlayerCommand::Preload { track_id } => self.handle_command_preload(track_id),

            PlayerCommand::Seek(position_ms) => self.handle_command_seek(position_ms)?,

            PlayerCommand::Play => self.handle_play(),

            PlayerCommand::Pause => self.handle_pause(),

            PlayerCommand::Stop => self.handle_player_stop(),

            PlayerCommand::AddEventSender(sender) => self.event_senders.push(sender),

            PlayerCommand::SetSinkEventCallback(callback) => self.sink_event_callback = callback,

            PlayerCommand::EmitVolumeChangedEvent(volume) => {
                self.send_event(PlayerEvent::VolumeChanged { volume })
            }

            PlayerCommand::EmitRepeatChangedEvent(repeat) => {
                self.send_event(PlayerEvent::RepeatChanged { repeat })
            }

            PlayerCommand::EmitShuffleChangedEvent(shuffle) => {
                self.send_event(PlayerEvent::ShuffleChanged { shuffle })
            }

            PlayerCommand::EmitAutoPlayChangedEvent(auto_play) => {
                self.send_event(PlayerEvent::AutoPlayChanged { auto_play })
            }

            PlayerCommand::EmitSessionClientChangedEvent {
                client_id,
                client_name,
                client_brand_name,
                client_model_name,
            } => self.send_event(PlayerEvent::SessionClientChanged {
                client_id,
                client_name,
                client_brand_name,
                client_model_name,
            }),

            PlayerCommand::EmitSessionConnectedEvent {
                connection_id,
                user_name,
            } => self.send_event(PlayerEvent::SessionConnected {
                connection_id,
                user_name,
            }),

            PlayerCommand::EmitSessionDisconnectedEvent {
                connection_id,
                user_name,
            } => self.send_event(PlayerEvent::SessionDisconnected {
                connection_id,
                user_name,
            }),

            PlayerCommand::SetAutoNormaliseAsAlbum(setting) => {
                self.auto_normalise_as_album = setting
            }

            PlayerCommand::EmitFilterExplicitContentChangedEvent(filter) => {
                self.send_event(PlayerEvent::FilterExplicitContentChanged { filter });

                if filter {
                    if let PlayerState::Playing {
                        track_id,
                        play_request_id,
                        is_explicit,
                        ..
                    }
                    | PlayerState::Paused {
                        track_id,
                        play_request_id,
                        is_explicit,
                        ..
                    } = self.state
                    {
                        if is_explicit {
                            warn!("Currently loaded track is explicit, which client setting forbids -- skipping to next track.");
                            self.send_event(PlayerEvent::EndOfTrack {
                                track_id,
                                play_request_id,
                            })
                        }
                    }
                }
            }
        };

        Ok(())
    }

    fn send_event(&mut self, event: PlayerEvent) {
        self.event_senders
            .retain(|sender| sender.send(event.clone()).is_ok());
    }

    fn load_track(
        &mut self,
        spotify_id: SpotifyId,
        position_ms: u32,
    ) -> impl FusedFuture<Output = Result<PlayerLoadedTrackData, ()>> + Send + 'static {
        // This method creates a future that returns the loaded stream and associated info.
        // Ideally all work should be done using asynchronous code. However, seek() on the
        // audio stream is implemented in a blocking fashion. Thus, we can't turn it into future
        // easily. Instead we spawn a thread to do the work and return a one-shot channel as the
        // future to work with.

        let loader = PlayerTrackLoader {
            session: self.session.clone(),
            config: self.config.clone(),
        };

        let (result_tx, result_rx) = oneshot::channel();

        let load_handles_clone = self.load_handles.clone();
        let handle = tokio::runtime::Handle::current();

        // The player increments the player id when it gets it...
        let thread_name = format!(
            "loader:{}:{}",
            PLAYER_COUNTER.load(Ordering::Relaxed).saturating_sub(1),
            spotify_id.to_uri().unwrap_or_default()
        );

        let builder = thread::Builder::new().name(thread_name.clone());

        let load_handle = match builder.spawn(move || {
            let data = handle.block_on(loader.load_track(spotify_id, position_ms));
            if let Some(data) = data {
                let _ = result_tx.send(data);
            }

            let mut load_handles = load_handles_clone.lock();
            load_handles.remove(&thread::current().id());

            match thread::current().name() {
                Some(name) => debug!("<PlayerInternal> [{name}] thread finished"),
                None => debug!("<PlayerInternal> [loader] thread finished"),
            }
        }) {
            Ok(handle) => {
                debug!("Created <PlayerInternal> [{thread_name}] thread");
                handle
            }
            Err(e) => {
                error!("Error creating <PlayerInternal> [{thread_name}] thread: {e}");
                exit(1);
            }
        };

        let mut load_handles = self.load_handles.lock();
        load_handles.insert(load_handle.thread().id(), load_handle);

        result_rx.map_err(|_| ())
    }

    fn preload_data_before_playback(&mut self) -> PlayerResult {
        if let PlayerState::Playing {
            bytes_per_second,
            ref mut stream_loader_controller,
            ..
        } = self.state
        {
            // Request our read ahead range
            let request_data_length =
                (READ_AHEAD_DURING_PLAYBACK.as_secs_f32() * bytes_per_second as f32) as usize;

            // Request the part we want to wait for blocking. This effectively means we wait for the previous request to partially complete.
            let wait_for_data_length =
                (READ_AHEAD_BEFORE_PLAYBACK.as_secs_f32() * bytes_per_second as f32) as usize;

            stream_loader_controller
                .fetch_next_and_wait(request_data_length, wait_for_data_length)
                .map_err(Into::into)
        } else {
            Ok(())
        }
    }
}

impl Drop for PlayerInternal {
    fn drop(&mut self) {
        debug!("drop PlayerInternal[{}]", self.player_id);

        let handles: Vec<thread::JoinHandle<()>> = {
            // waiting for the thread while holding the mutex would result in a deadlock
            let mut load_handles = self.load_handles.lock();

            load_handles
                .drain()
                .map(|(_thread_id, handle)| handle)
                .collect()
        };

        for handle in handles {
            let _ = handle.join();
        }
    }
}

impl fmt::Debug for PlayerCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlayerCommand::Load {
                track_id,
                play,
                position_ms,
                ..
            } => f
                .debug_tuple("Load")
                .field(&track_id)
                .field(&play)
                .field(&position_ms)
                .finish(),
            PlayerCommand::Preload { track_id } => {
                f.debug_tuple("Preload").field(&track_id).finish()
            }
            PlayerCommand::Play => f.debug_tuple("Play").finish(),
            PlayerCommand::Pause => f.debug_tuple("Pause").finish(),
            PlayerCommand::Stop => f.debug_tuple("Stop").finish(),
            PlayerCommand::Seek(position) => f.debug_tuple("Seek").field(&position).finish(),
            PlayerCommand::AddEventSender(_) => f.debug_tuple("AddEventSender").finish(),
            PlayerCommand::SetSinkEventCallback(_) => {
                f.debug_tuple("SetSinkEventCallback").finish()
            }
            PlayerCommand::EmitVolumeChangedEvent(volume) => f
                .debug_tuple("EmitVolumeChangedEvent")
                .field(&volume)
                .finish(),
            PlayerCommand::SetAutoNormaliseAsAlbum(setting) => f
                .debug_tuple("SetAutoNormaliseAsAlbum")
                .field(&setting)
                .finish(),
            PlayerCommand::EmitFilterExplicitContentChangedEvent(filter) => f
                .debug_tuple("EmitFilterExplicitContentChangedEvent")
                .field(&filter)
                .finish(),
            PlayerCommand::EmitSessionConnectedEvent {
                connection_id,
                user_name,
            } => f
                .debug_tuple("EmitSessionConnectedEvent")
                .field(&connection_id)
                .field(&user_name)
                .finish(),
            PlayerCommand::EmitSessionDisconnectedEvent {
                connection_id,
                user_name,
            } => f
                .debug_tuple("EmitSessionDisconnectedEvent")
                .field(&connection_id)
                .field(&user_name)
                .finish(),
            PlayerCommand::EmitSessionClientChangedEvent {
                client_id,
                client_name,
                client_brand_name,
                client_model_name,
            } => f
                .debug_tuple("EmitSessionClientChangedEvent")
                .field(&client_id)
                .field(&client_name)
                .field(&client_brand_name)
                .field(&client_model_name)
                .finish(),
            PlayerCommand::EmitShuffleChangedEvent(shuffle) => f
                .debug_tuple("EmitShuffleChangedEvent")
                .field(&shuffle)
                .finish(),
            PlayerCommand::EmitRepeatChangedEvent(repeat) => f
                .debug_tuple("EmitRepeatChangedEvent")
                .field(&repeat)
                .finish(),
            PlayerCommand::EmitAutoPlayChangedEvent(auto_play) => f
                .debug_tuple("EmitAutoPlayChangedEvent")
                .field(&auto_play)
                .finish(),
        }
    }
}

impl fmt::Debug for PlayerState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use PlayerState::*;
        match *self {
            Stopped => f.debug_struct("Stopped").finish(),
            Loading {
                track_id,
                play_request_id,
                ..
            } => f
                .debug_struct("Loading")
                .field("track_id", &track_id)
                .field("play_request_id", &play_request_id)
                .finish(),
            Paused {
                track_id,
                play_request_id,
                ..
            } => f
                .debug_struct("Paused")
                .field("track_id", &track_id)
                .field("play_request_id", &play_request_id)
                .finish(),
            Playing {
                track_id,
                play_request_id,
                ..
            } => f
                .debug_struct("Playing")
                .field("track_id", &track_id)
                .field("play_request_id", &play_request_id)
                .finish(),
            EndOfTrack {
                track_id,
                play_request_id,
                ..
            } => f
                .debug_struct("EndOfTrack")
                .field("track_id", &track_id)
                .field("play_request_id", &play_request_id)
                .finish(),
            Invalid => f.debug_struct("Invalid").finish(),
        }
    }
}

struct Subfile<T: Read + Seek> {
    stream: T,
    offset: u64,
    length: u64,
}

impl<T: Read + Seek> Subfile<T> {
    pub fn new(mut stream: T, offset: u64, length: u64) -> Result<Subfile<T>, io::Error> {
        let target = SeekFrom::Start(offset);
        stream.seek(target)?;

        Ok(Subfile {
            stream,
            offset,
            length,
        })
    }
}

impl<T: Read + Seek> Read for Subfile<T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.stream.read(buf)
    }
}

impl<T: Read + Seek> Seek for Subfile<T> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        let pos = match pos {
            SeekFrom::Start(offset) => SeekFrom::Start(offset + self.offset),
            SeekFrom::End(offset) => {
                if (self.length as i64 - offset) < self.offset as i64 {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "newpos would be < self.offset",
                    ));
                }
                pos
            }
            _ => pos,
        };

        let newpos = self.stream.seek(pos)?;
        Ok(newpos - self.offset)
    }
}

impl<R> MediaSource for Subfile<R>
where
    R: Read + Seek + Send + Sync,
{
    fn is_seekable(&self) -> bool {
        true
    }

    fn byte_len(&self) -> Option<u64> {
        Some(self.length)
    }
}
