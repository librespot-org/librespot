use std::{
    future::Future,
    pin::Pin,
    sync::atomic::{AtomicUsize, Ordering},
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use futures_util::{stream::FusedStream, FutureExt, StreamExt};

use protobuf::Message;
use rand::prelude::SliceRandom;
use thiserror::Error;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::{
    config::ConnectConfig,
    context::PageContext,
    core::{
        authentication::Credentials, mercury::MercurySender, session::UserAttributes,
        util::SeqGenerator, version, Error, Session, SpotifyId,
    },
    playback::{
        mixer::Mixer,
        player::{Player, PlayerEvent, PlayerEventChannel},
    },
    protocol::{
        self,
        explicit_content_pubsub::UserAttributesUpdate,
        spirc::{DeviceState, Frame, MessageType, PlayStatus, State, TrackRef},
        user_attributes::UserAttributesMutation,
    },
};

#[derive(Debug, Error)]
pub enum SpircError {
    #[error("response payload empty")]
    NoData,
    #[error("playback of local files is not supported")]
    UnsupportedLocalPlayBack,
    #[error("message addressed at another ident: {0}")]
    Ident(String),
    #[error("message pushed for another URI")]
    InvalidUri(String),
}

impl From<SpircError> for Error {
    fn from(err: SpircError) -> Self {
        use SpircError::*;
        match err {
            NoData | UnsupportedLocalPlayBack => Error::unavailable(err),
            Ident(_) | InvalidUri(_) => Error::aborted(err),
        }
    }
}

#[derive(Debug)]
enum SpircPlayStatus {
    Stopped,
    LoadingPlay {
        position_ms: u32,
    },
    LoadingPause {
        position_ms: u32,
    },
    Playing {
        nominal_start_time: i64,
        preloading_of_next_track_triggered: bool,
    },
    Paused {
        position_ms: u32,
        preloading_of_next_track_triggered: bool,
    },
}

type BoxedStream<T> = Pin<Box<dyn FusedStream<Item = T> + Send>>;

struct SpircTask {
    player: Arc<Player>,
    mixer: Arc<dyn Mixer>,

    sequence: SeqGenerator<u32>,

    ident: String,
    device: DeviceState,
    state: State,
    play_request_id: Option<u64>,
    play_status: SpircPlayStatus,

    remote_update: BoxedStream<Result<(String, Frame), Error>>,
    connection_id_update: BoxedStream<Result<String, Error>>,
    user_attributes_update: BoxedStream<Result<UserAttributesUpdate, Error>>,
    user_attributes_mutation: BoxedStream<Result<UserAttributesMutation, Error>>,
    sender: MercurySender,
    commands: Option<mpsc::UnboundedReceiver<SpircCommand>>,
    player_events: Option<PlayerEventChannel>,

    shutdown: bool,
    session: Session,
    resolve_context: Option<String>,
    autoplay_context: bool,
    context: Option<PageContext>,

    spirc_id: usize,
}

static SPIRC_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug)]
pub enum SpircCommand {
    Play,
    PlayPause,
    Pause,
    Prev,
    Next,
    VolumeUp,
    VolumeDown,
    Shutdown,
    Shuffle(bool),
    Repeat(bool),
    Disconnect,
    SetPosition(u32),
    SeekOffset(i32),
    SetVolume(u16),
    Activate,
    Load(SpircLoadCommand),
}

#[derive(Debug)]
pub struct SpircLoadCommand {
    pub context_uri: String,
    /// Whether the given tracks should immediately start playing, or just be initially loaded.
    pub start_playing: bool,
    pub shuffle: bool,
    pub repeat: bool,
    pub playing_track_index: u32,
    pub tracks: Vec<TrackRef>,
}

impl From<SpircLoadCommand> for State {
    fn from(command: SpircLoadCommand) -> Self {
        let mut state = State::new();
        state.set_context_uri(command.context_uri);
        state.set_status(if command.start_playing {
            PlayStatus::kPlayStatusPlay
        } else {
            PlayStatus::kPlayStatusStop
        });
        state.set_shuffle(command.shuffle);
        state.set_repeat(command.repeat);
        state.set_playing_track_index(command.playing_track_index);
        state.track = command.tracks;
        state
    }
}

const CONTEXT_TRACKS_HISTORY: usize = 10;
const CONTEXT_FETCH_THRESHOLD: u32 = 5;

const VOLUME_STEPS: i64 = 64;
const VOLUME_STEP_SIZE: u16 = 1024; // (u16::MAX + 1) / VOLUME_STEPS

#[derive(Clone)]
pub struct Spirc {
    commands: mpsc::UnboundedSender<SpircCommand>,
}

fn initial_state() -> State {
    let mut frame = protocol::spirc::State::new();
    frame.set_repeat(false);
    frame.set_shuffle(false);
    frame.set_status(PlayStatus::kPlayStatusStop);
    frame.set_position_ms(0);
    frame.set_position_measured_at(0);
    frame
}

fn int_capability(typ: protocol::spirc::CapabilityType, val: i64) -> protocol::spirc::Capability {
    let mut cap = protocol::spirc::Capability::new();
    cap.set_typ(typ);
    cap.intValue.push(val);
    cap
}

fn initial_device_state(config: ConnectConfig) -> DeviceState {
    let mut msg = DeviceState::new();
    msg.set_sw_version(version::SEMVER.to_string());
    msg.set_is_active(false);
    msg.set_can_play(true);
    msg.set_volume(0);
    msg.set_name(config.name);
    msg.capabilities.push(int_capability(
        protocol::spirc::CapabilityType::kCanBePlayer,
        1,
    ));
    msg.capabilities.push(int_capability(
        protocol::spirc::CapabilityType::kDeviceType,
        config.device_type as i64,
    ));
    msg.capabilities.push(int_capability(
        protocol::spirc::CapabilityType::kGaiaEqConnectId,
        1,
    ));
    // TODO: implement logout
    msg.capabilities.push(int_capability(
        protocol::spirc::CapabilityType::kSupportsLogout,
        0,
    ));
    msg.capabilities.push(int_capability(
        protocol::spirc::CapabilityType::kIsObservable,
        1,
    ));
    msg.capabilities.push(int_capability(
        protocol::spirc::CapabilityType::kVolumeSteps,
        if config.has_volume_ctrl {
            VOLUME_STEPS
        } else {
            0
        },
    ));
    msg.capabilities.push(int_capability(
        protocol::spirc::CapabilityType::kSupportsPlaylistV2,
        1,
    ));
    msg.capabilities.push(int_capability(
        protocol::spirc::CapabilityType::kSupportsExternalEpisodes,
        1,
    ));
    // TODO: how would such a rename command be triggered? Handle it.
    msg.capabilities.push(int_capability(
        protocol::spirc::CapabilityType::kSupportsRename,
        1,
    ));
    msg.capabilities.push(int_capability(
        protocol::spirc::CapabilityType::kCommandAcks,
        0,
    ));
    // TODO: does this mean local files or the local network?
    // LAN may be an interesting privacy toggle.
    msg.capabilities.push(int_capability(
        protocol::spirc::CapabilityType::kRestrictToLocal,
        0,
    ));
    // TODO: what does this hide, or who do we hide from?
    // May be an interesting privacy toggle.
    msg.capabilities
        .push(int_capability(protocol::spirc::CapabilityType::kHidden, 0));
    let mut supported_types = protocol::spirc::Capability::new();
    supported_types.set_typ(protocol::spirc::CapabilityType::kSupportedTypes);
    supported_types
        .stringValue
        .push("audio/episode".to_string());
    supported_types
        .stringValue
        .push("audio/episode+track".to_string());
    supported_types.stringValue.push("audio/track".to_string());
    // other known types:
    // - "audio/ad"
    // - "audio/interruption"
    // - "audio/local"
    // - "video/ad"
    // - "video/episode"
    msg.capabilities.push(supported_types);
    msg
}

fn url_encode(bytes: impl AsRef<[u8]>) -> String {
    form_urlencoded::byte_serialize(bytes.as_ref()).collect()
}

impl Spirc {
    pub async fn new(
        config: ConnectConfig,
        session: Session,
        credentials: Credentials,
        player: Arc<Player>,
        mixer: Arc<dyn Mixer>,
    ) -> Result<(Spirc, impl Future<Output = ()>), Error> {
        let spirc_id = SPIRC_COUNTER.fetch_add(1, Ordering::AcqRel);
        debug!("new Spirc[{}]", spirc_id);

        let ident = session.device_id().to_owned();

        let remote_update = Box::pin(
            session
                .mercury()
                .listen_for("hm://remote/user/")
                .map(UnboundedReceiverStream::new)
                .flatten_stream()
                .map(|response| -> Result<(String, Frame), Error> {
                    let uri_split: Vec<&str> = response.uri.split('/').collect();
                    let username = match uri_split.get(4) {
                        Some(s) => s.to_string(),
                        None => String::new(),
                    };

                    let data = response.payload.first().ok_or(SpircError::NoData)?;
                    Ok((username, Frame::parse_from_bytes(data)?))
                }),
        );

        let connection_id_update = Box::pin(
            session
                .mercury()
                .listen_for("hm://pusher/v1/connections/")
                .map(UnboundedReceiverStream::new)
                .flatten_stream()
                .map(|response| -> Result<String, Error> {
                    let connection_id = response
                        .uri
                        .strip_prefix("hm://pusher/v1/connections/")
                        .ok_or_else(|| SpircError::InvalidUri(response.uri.clone()))?;
                    Ok(connection_id.to_owned())
                }),
        );

        let user_attributes_update = Box::pin(
            session
                .mercury()
                .listen_for("spotify:user:attributes:update")
                .map(UnboundedReceiverStream::new)
                .flatten_stream()
                .map(|response| -> Result<UserAttributesUpdate, Error> {
                    let data = response.payload.first().ok_or(SpircError::NoData)?;
                    Ok(UserAttributesUpdate::parse_from_bytes(data)?)
                }),
        );

        let user_attributes_mutation = Box::pin(
            session
                .mercury()
                .listen_for("spotify:user:attributes:mutated")
                .map(UnboundedReceiverStream::new)
                .flatten_stream()
                .map(|response| -> Result<UserAttributesMutation, Error> {
                    let data = response.payload.first().ok_or(SpircError::NoData)?;
                    Ok(UserAttributesMutation::parse_from_bytes(data)?)
                }),
        );

        // Connect *after* all message listeners are registered
        session.connect(credentials, true).await?;

        let canonical_username = &session.username();
        debug!("canonical_username: {}", canonical_username);
        let sender_uri = format!("hm://remote/user/{}/", url_encode(canonical_username));

        let sender = session.mercury().sender(sender_uri);

        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();

        let initial_volume = config.initial_volume;

        let device = initial_device_state(config);

        let player_events = player.get_player_event_channel();

        let mut task = SpircTask {
            player,
            mixer,

            sequence: SeqGenerator::new(1),

            ident,

            device,
            state: initial_state(),
            play_request_id: None,
            play_status: SpircPlayStatus::Stopped,

            remote_update,
            connection_id_update,
            user_attributes_update,
            user_attributes_mutation,
            sender,
            commands: Some(cmd_rx),
            player_events: Some(player_events),

            shutdown: false,
            session,

            resolve_context: None,
            autoplay_context: false,
            context: None,

            spirc_id,
        };

        if let Some(volume) = initial_volume {
            task.set_volume(volume);
        } else {
            let current_volume = task.mixer.volume();
            task.set_volume(current_volume);
        }

        let spirc = Spirc { commands: cmd_tx };

        task.hello()?;

        Ok((spirc, task.run()))
    }

    pub fn play(&self) -> Result<(), Error> {
        Ok(self.commands.send(SpircCommand::Play)?)
    }
    pub fn play_pause(&self) -> Result<(), Error> {
        Ok(self.commands.send(SpircCommand::PlayPause)?)
    }
    pub fn pause(&self) -> Result<(), Error> {
        Ok(self.commands.send(SpircCommand::Pause)?)
    }
    pub fn prev(&self) -> Result<(), Error> {
        Ok(self.commands.send(SpircCommand::Prev)?)
    }
    pub fn next(&self) -> Result<(), Error> {
        Ok(self.commands.send(SpircCommand::Next)?)
    }
    pub fn volume_up(&self) -> Result<(), Error> {
        Ok(self.commands.send(SpircCommand::VolumeUp)?)
    }
    pub fn volume_down(&self) -> Result<(), Error> {
        Ok(self.commands.send(SpircCommand::VolumeDown)?)
    }
    pub fn shutdown(&self) -> Result<(), Error> {
        Ok(self.commands.send(SpircCommand::Shutdown)?)
    }
    pub fn shuffle(&self, shuffle: bool) -> Result<(), Error> {
        Ok(self.commands.send(SpircCommand::Shuffle(shuffle))?)
    }
    pub fn repeat(&self, repeat: bool) -> Result<(), Error> {
        Ok(self.commands.send(SpircCommand::Repeat(repeat))?)
    }
    pub fn set_volume(&self, volume: u16) -> Result<(), Error> {
        Ok(self.commands.send(SpircCommand::SetVolume(volume))?)
    }
    pub fn set_position_ms(&self, position_ms: u32) -> Result<(), Error> {
        Ok(self.commands.send(SpircCommand::SetPosition(position_ms))?)
    }
    pub fn seek_offset(&self, offset_ms: i32) -> Result<(), Error> {
        Ok(self.commands.send(SpircCommand::SeekOffset(offset_ms))?)
    }
    pub fn disconnect(&self) -> Result<(), Error> {
        Ok(self.commands.send(SpircCommand::Disconnect)?)
    }
    pub fn activate(&self) -> Result<(), Error> {
        Ok(self.commands.send(SpircCommand::Activate)?)
    }
    pub fn load(&self, command: SpircLoadCommand) -> Result<(), Error> {
        Ok(self.commands.send(SpircCommand::Load(command))?)
    }
}

impl SpircTask {
    async fn run(mut self) {
        while !self.session.is_invalid() && !self.shutdown {
            let commands = self.commands.as_mut();
            let player_events = self.player_events.as_mut();
            tokio::select! {
                remote_update = self.remote_update.next() => match remote_update {
                    Some(result) => match result {
                        Ok((username, frame)) => {
                            if username != self.session.username() {
                                warn!("could not dispatch remote update: frame was intended for {}", username);
                            } else if let Err(e) = self.handle_remote_update(frame) {
                                error!("could not dispatch remote update: {}", e);
                            }
                        },
                        Err(e) => error!("could not parse remote update: {}", e),
                    }
                    None => {
                        error!("remote update selected, but none received");
                        break;
                    }
                },
                user_attributes_update = self.user_attributes_update.next() => match user_attributes_update {
                    Some(result) => match result {
                        Ok(attributes) => self.handle_user_attributes_update(attributes),
                        Err(e) => error!("could not parse user attributes update: {}", e),
                    }
                    None => {
                        error!("user attributes update selected, but none received");
                        break;
                    }
                },
                user_attributes_mutation = self.user_attributes_mutation.next() => match user_attributes_mutation {
                    Some(result) => match result {
                        Ok(attributes) => self.handle_user_attributes_mutation(attributes),
                        Err(e) => error!("could not parse user attributes mutation: {}", e),
                    }
                    None => {
                        error!("user attributes mutation selected, but none received");
                        break;
                    }
                },
                connection_id_update = self.connection_id_update.next() => match connection_id_update {
                    Some(result) => match result {
                        Ok(connection_id) => self.handle_connection_id_update(connection_id),
                        Err(e) => error!("could not parse connection ID update: {}", e),
                    }
                    None => {
                        error!("connection ID update selected, but none received");
                        break;
                    }
                },
                cmd = async { commands?.recv().await }, if commands.is_some() => if let Some(cmd) = cmd {
                    if let Err(e) = self.handle_command(cmd) {
                        debug!("could not dispatch command: {}", e);
                    }
                },
                event = async { player_events?.recv().await }, if player_events.is_some() => if let Some(event) = event {
                    if let Err(e) = self.handle_player_event(event) {
                        error!("could not dispatch player event: {}", e);
                    }
                },
                result = self.sender.flush(), if !self.sender.is_flushed() => if result.is_err() {
                    error!("Cannot flush spirc event sender.");
                    break;
                },
                context_uri = async { self.resolve_context.take() }, if self.resolve_context.is_some() => {
                    let context_uri = context_uri.unwrap(); // guaranteed above
                    if context_uri.contains("spotify:show:") || context_uri.contains("spotify:episode:") {
                        continue; // not supported by apollo stations
                    }

                    let context = if context_uri.starts_with("hm://") {
                        self.session.spclient().get_next_page(&context_uri).await
                    } else {
                        // only send previous tracks that were before the current playback position
                        let current_position = self.state.playing_track_index() as usize;
                        let previous_tracks = self.state.track[..current_position].iter().filter_map(|t| SpotifyId::try_from(t).ok()).collect();

                        let scope = if self.autoplay_context {
                            "stations" // this returns a `StationContext` but we deserialize it into a `PageContext`
                        } else {
                            "tracks" // this returns a `PageContext`
                        };

                        self.session.spclient().get_apollo_station(scope, &context_uri, None, previous_tracks, self.autoplay_context).await
                    };

                    match context {
                        Ok(value) => {
                            self.context = match serde_json::from_slice::<PageContext>(&value) {
                                Ok(context) => {
                                    info!(
                                        "Resolved {:?} tracks from <{:?}>",
                                        context.tracks.len(),
                                        self.state.context_uri(),
                                    );
                                    Some(context)
                                }
                                Err(e) => {
                                    error!("Unable to parse JSONContext {:?}", e);
                                    None
                                }
                            };
                        },
                        Err(err) => {
                            error!("ContextError: {:?}", err)
                        }
                    }
                },
                else => break
            }
        }

        if self.sender.flush().await.is_err() {
            warn!("Cannot flush spirc event sender when done.");
        }
    }

    fn now_ms(&mut self) -> i64 {
        let dur = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(dur) => dur,
            Err(err) => err.duration(),
        };

        dur.as_millis() as i64 + 1000 * self.session.time_delta()
    }

    fn update_state_position(&mut self, position_ms: u32) {
        let now = self.now_ms();
        self.state.set_position_measured_at(now as u64);
        self.state.set_position_ms(position_ms);
    }

    fn handle_command(&mut self, cmd: SpircCommand) -> Result<(), Error> {
        if matches!(cmd, SpircCommand::Shutdown) {
            trace!("Received SpircCommand::Shutdown");
            CommandSender::new(self, MessageType::kMessageTypeGoodbye).send()?;
            self.handle_disconnect();
            self.shutdown = true;
            if let Some(rx) = self.commands.as_mut() {
                rx.close()
            }
            Ok(())
        } else if self.device.is_active() {
            trace!("Received SpircCommand::{:?}", cmd);
            match cmd {
                SpircCommand::Play => {
                    self.handle_play();
                    self.notify(None)
                }
                SpircCommand::PlayPause => {
                    self.handle_play_pause();
                    self.notify(None)
                }
                SpircCommand::Pause => {
                    self.handle_pause();
                    self.notify(None)
                }
                SpircCommand::Prev => {
                    self.handle_prev();
                    self.notify(None)
                }
                SpircCommand::Next => {
                    self.handle_next();
                    self.notify(None)
                }
                SpircCommand::VolumeUp => {
                    self.handle_volume_up();
                    self.notify(None)
                }
                SpircCommand::VolumeDown => {
                    self.handle_volume_down();
                    self.notify(None)
                }
                SpircCommand::Disconnect => {
                    self.handle_disconnect();
                    self.notify(None)
                }
                SpircCommand::Shuffle(shuffle) => {
                    self.state.set_shuffle(shuffle);
                    self.notify(None)
                }
                SpircCommand::Repeat(repeat) => {
                    self.state.set_repeat(repeat);
                    self.notify(None)
                }
                SpircCommand::SetPosition(position) => {
                    self.handle_seek(position);
                    self.notify(None)
                }
                SpircCommand::SeekOffset(offset) => {
                    self.handle_seek_offset(offset);
                    self.notify(None)
                }
                SpircCommand::SetVolume(volume) => {
                    self.set_volume(volume);
                    self.notify(None)
                }
                SpircCommand::Load(command) => {
                    self.handle_load(&command.into())?;
                    self.notify(None)
                }
                _ => Ok(()),
            }
        } else {
            match cmd {
                SpircCommand::Activate => {
                    trace!("Received SpircCommand::{:?}", cmd);
                    self.handle_activate();
                    self.notify(None)
                }
                _ => {
                    warn!("SpircCommand::{:?} will be ignored while Not Active", cmd);
                    Ok(())
                }
            }
        }
    }

    fn handle_player_event(&mut self, event: PlayerEvent) -> Result<(), Error> {
        // update play_request_id
        if let PlayerEvent::PlayRequestIdChanged { play_request_id } = event {
            self.play_request_id = Some(play_request_id);
            return Ok(());
        }
        // we only process events if the play_request_id matches. If it doesn't, it is
        // an event that belongs to a previous track and only arrives now due to a race
        // condition. In this case we have updated the state already and don't want to
        // mess with it.
        if let Some(play_request_id) = event.get_play_request_id() {
            if Some(play_request_id) == self.play_request_id {
                match event {
                    PlayerEvent::EndOfTrack { .. } => self.handle_end_of_track(),
                    PlayerEvent::Loading { .. } => {
                        match self.play_status {
                            SpircPlayStatus::LoadingPlay { position_ms } => {
                                self.update_state_position(position_ms);
                                self.state.set_status(PlayStatus::kPlayStatusPlay);
                                trace!("==> kPlayStatusPlay");
                            }
                            SpircPlayStatus::LoadingPause { position_ms } => {
                                self.update_state_position(position_ms);
                                self.state.set_status(PlayStatus::kPlayStatusPause);
                                trace!("==> kPlayStatusPause");
                            }
                            _ => {
                                self.state.set_status(PlayStatus::kPlayStatusLoading);
                                self.update_state_position(0);
                                trace!("==> kPlayStatusLoading");
                            }
                        }
                        self.notify(None)
                    }
                    PlayerEvent::Playing { position_ms, .. }
                    | PlayerEvent::PositionCorrection { position_ms, .. }
                    | PlayerEvent::Seeked { position_ms, .. } => {
                        trace!("==> kPlayStatusPlay");
                        let new_nominal_start_time = self.now_ms() - position_ms as i64;
                        match self.play_status {
                            SpircPlayStatus::Playing {
                                ref mut nominal_start_time,
                                ..
                            } => {
                                if (*nominal_start_time - new_nominal_start_time).abs() > 100 {
                                    *nominal_start_time = new_nominal_start_time;
                                    self.update_state_position(position_ms);
                                    self.notify(None)
                                } else {
                                    Ok(())
                                }
                            }
                            SpircPlayStatus::LoadingPlay { .. }
                            | SpircPlayStatus::LoadingPause { .. } => {
                                self.state.set_status(PlayStatus::kPlayStatusPlay);
                                self.update_state_position(position_ms);
                                self.play_status = SpircPlayStatus::Playing {
                                    nominal_start_time: new_nominal_start_time,
                                    preloading_of_next_track_triggered: false,
                                };
                                self.notify(None)
                            }
                            _ => Ok(()),
                        }
                    }
                    PlayerEvent::Paused {
                        position_ms: new_position_ms,
                        ..
                    } => {
                        trace!("==> kPlayStatusPause");
                        match self.play_status {
                            SpircPlayStatus::Paused { .. } | SpircPlayStatus::Playing { .. } => {
                                self.state.set_status(PlayStatus::kPlayStatusPause);
                                self.update_state_position(new_position_ms);
                                self.play_status = SpircPlayStatus::Paused {
                                    position_ms: new_position_ms,
                                    preloading_of_next_track_triggered: false,
                                };
                                self.notify(None)
                            }
                            SpircPlayStatus::LoadingPlay { .. }
                            | SpircPlayStatus::LoadingPause { .. } => {
                                self.state.set_status(PlayStatus::kPlayStatusPause);
                                self.update_state_position(new_position_ms);
                                self.play_status = SpircPlayStatus::Paused {
                                    position_ms: new_position_ms,
                                    preloading_of_next_track_triggered: false,
                                };
                                self.notify(None)
                            }
                            _ => Ok(()),
                        }
                    }
                    PlayerEvent::Stopped { .. } => {
                        trace!("==> kPlayStatusStop");
                        match self.play_status {
                            SpircPlayStatus::Stopped => Ok(()),
                            _ => {
                                self.state.set_status(PlayStatus::kPlayStatusStop);
                                self.play_status = SpircPlayStatus::Stopped;
                                self.notify(None)
                            }
                        }
                    }
                    PlayerEvent::TimeToPreloadNextTrack { .. } => {
                        self.handle_preload_next_track();
                        Ok(())
                    }
                    PlayerEvent::Unavailable { track_id, .. } => {
                        self.handle_unavailable(track_id);
                        Ok(())
                    }
                    _ => Ok(()),
                }
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }

    fn handle_connection_id_update(&mut self, connection_id: String) {
        trace!("Received connection ID update: {:?}", connection_id);
        self.session.set_connection_id(&connection_id);
    }

    fn handle_user_attributes_update(&mut self, update: UserAttributesUpdate) {
        trace!("Received attributes update: {:#?}", update);
        let attributes: UserAttributes = update
            .pairs
            .iter()
            .map(|pair| (pair.key().to_owned(), pair.value().to_owned()))
            .collect();
        self.session.set_user_attributes(attributes)
    }

    fn handle_user_attributes_mutation(&mut self, mutation: UserAttributesMutation) {
        for attribute in mutation.fields.iter() {
            let key = &attribute.name;

            if key == "autoplay" && self.session.config().autoplay.is_some() {
                trace!("Autoplay override active. Ignoring mutation.");
                continue;
            }

            if let Some(old_value) = self.session.user_data().attributes.get(key) {
                let new_value = match old_value.as_ref() {
                    "0" => "1",
                    "1" => "0",
                    _ => old_value,
                };
                self.session.set_user_attribute(key, new_value);

                trace!(
                    "Received attribute mutation, {} was {} is now {}",
                    key,
                    old_value,
                    new_value
                );

                if key == "filter-explicit-content" && new_value == "1" {
                    self.player
                        .emit_filter_explicit_content_changed_event(matches!(new_value, "1"));
                }

                if key == "autoplay" && old_value != new_value {
                    self.player
                        .emit_auto_play_changed_event(matches!(new_value, "1"));
                }
            } else {
                trace!(
                    "Received attribute mutation for {} but key was not found!",
                    key
                );
            }
        }
    }

    fn handle_remote_update(&mut self, update: Frame) -> Result<(), Error> {
        trace!("Received update frame: {:#?}", update);

        // First see if this update was intended for us.
        let device_id = &self.ident;
        let ident = update.ident();
        if ident == device_id
            || (!update.recipient.is_empty() && !update.recipient.contains(device_id))
        {
            return Err(SpircError::Ident(ident.to_string()).into());
        }

        let old_client_id = self.session.client_id();

        for entry in update.device_state.metadata.iter() {
            match entry.type_() {
                "client_id" => self.session.set_client_id(entry.metadata()),
                "brand_display_name" => self.session.set_client_brand_name(entry.metadata()),
                "model_display_name" => self.session.set_client_model_name(entry.metadata()),
                _ => (),
            }
        }

        self.session.set_client_name(update.device_state.name());

        let new_client_id = self.session.client_id();

        if self.device.is_active() && new_client_id != old_client_id {
            self.player.emit_session_client_changed_event(
                new_client_id,
                self.session.client_name(),
                self.session.client_brand_name(),
                self.session.client_model_name(),
            );
        }

        match update.typ() {
            MessageType::kMessageTypeHello => self.notify(Some(ident)),

            MessageType::kMessageTypeLoad => {
                self.handle_load(update.state.get_or_default())?;
                self.notify(None)
            }

            MessageType::kMessageTypePlay => {
                self.handle_play();
                self.notify(None)
            }

            MessageType::kMessageTypePlayPause => {
                self.handle_play_pause();
                self.notify(None)
            }

            MessageType::kMessageTypePause => {
                self.handle_pause();
                self.notify(None)
            }

            MessageType::kMessageTypeNext => {
                self.handle_next();
                self.notify(None)
            }

            MessageType::kMessageTypePrev => {
                self.handle_prev();
                self.notify(None)
            }

            MessageType::kMessageTypeVolumeUp => {
                self.handle_volume_up();
                self.notify(None)
            }

            MessageType::kMessageTypeVolumeDown => {
                self.handle_volume_down();
                self.notify(None)
            }

            MessageType::kMessageTypeRepeat => {
                let repeat = update.state.repeat();
                self.state.set_repeat(repeat);

                self.player.emit_repeat_changed_event(repeat);

                self.notify(None)
            }

            MessageType::kMessageTypeShuffle => {
                let shuffle = update.state.shuffle();
                self.state.set_shuffle(shuffle);
                if shuffle {
                    let current_index = self.state.playing_track_index();
                    let tracks = &mut self.state.track;
                    if !tracks.is_empty() {
                        tracks.swap(0, current_index as usize);
                        if let Some((_, rest)) = tracks.split_first_mut() {
                            let mut rng = rand::thread_rng();
                            rest.shuffle(&mut rng);
                        }
                        self.state.set_playing_track_index(0);
                    }
                }
                self.player.emit_shuffle_changed_event(shuffle);

                self.notify(None)
            }

            MessageType::kMessageTypeSeek => {
                self.handle_seek(update.position());
                self.notify(None)
            }

            MessageType::kMessageTypeReplace => {
                let context_uri = update.state.context_uri().to_owned();

                // completely ignore local playback.
                if context_uri.starts_with("spotify:local-files") {
                    self.notify(None)?;
                    return Err(SpircError::UnsupportedLocalPlayBack.into());
                }

                self.update_tracks(update.state.get_or_default());

                if let SpircPlayStatus::Playing {
                    preloading_of_next_track_triggered,
                    ..
                }
                | SpircPlayStatus::Paused {
                    preloading_of_next_track_triggered,
                    ..
                } = self.play_status
                {
                    if preloading_of_next_track_triggered {
                        // Get the next track_id in the playlist
                        if let Some(track_id) = self.preview_next_track() {
                            self.player.preload(track_id);
                        }
                    }
                }

                self.notify(None)
            }

            MessageType::kMessageTypeVolume => {
                self.set_volume(update.volume() as u16);
                self.notify(None)
            }

            MessageType::kMessageTypeNotify => {
                if self.device.is_active()
                    && update.device_state.is_active()
                    && self.device.became_active_at() <= update.device_state.became_active_at()
                {
                    self.handle_disconnect();
                }
                self.notify(None)
            }

            _ => Ok(()),
        }
    }

    fn handle_disconnect(&mut self) {
        self.device.set_is_active(false);
        self.handle_stop();

        self.player
            .emit_session_disconnected_event(self.session.connection_id(), self.session.username());
    }

    fn handle_stop(&mut self) {
        self.player.stop();
    }

    fn handle_activate(&mut self) {
        let now = self.now_ms();
        self.device.set_is_active(true);
        self.device.set_became_active_at(now);
        self.player
            .emit_session_connected_event(self.session.connection_id(), self.session.username());
        self.player.emit_session_client_changed_event(
            self.session.client_id(),
            self.session.client_name(),
            self.session.client_brand_name(),
            self.session.client_model_name(),
        );

        self.player
            .emit_volume_changed_event(self.device.volume() as u16);

        self.player
            .emit_auto_play_changed_event(self.session.autoplay());

        self.player
            .emit_filter_explicit_content_changed_event(self.session.filter_explicit_content());

        self.player.emit_shuffle_changed_event(self.state.shuffle());

        self.player.emit_repeat_changed_event(self.state.repeat());
    }

    fn handle_load(&mut self, state: &State) -> Result<(), Error> {
        if !self.device.is_active() {
            self.handle_activate();
        }

        let context_uri = state.context_uri().to_owned();

        // completely ignore local playback.
        if context_uri.starts_with("spotify:local-files") {
            self.notify(None)?;
            return Err(SpircError::UnsupportedLocalPlayBack.into());
        }

        self.update_tracks(state);

        if !self.state.track.is_empty() {
            let start_playing = state.status() == PlayStatus::kPlayStatusPlay;
            self.load_track(start_playing, state.position_ms());
        } else {
            info!("No more tracks left in queue");
            self.handle_stop();
        }
        Ok(())
    }

    fn handle_play(&mut self) {
        match self.play_status {
            SpircPlayStatus::Paused {
                position_ms,
                preloading_of_next_track_triggered,
            } => {
                self.player.play();
                self.state.set_status(PlayStatus::kPlayStatusPlay);
                self.update_state_position(position_ms);
                self.play_status = SpircPlayStatus::Playing {
                    nominal_start_time: self.now_ms() - position_ms as i64,
                    preloading_of_next_track_triggered,
                };
            }
            SpircPlayStatus::LoadingPause { position_ms } => {
                self.player.play();
                self.play_status = SpircPlayStatus::LoadingPlay { position_ms };
            }
            _ => return,
        }

        // Synchronize the volume from the mixer. This is useful on
        // systems that can switch sources from and back to librespot.
        let current_volume = self.mixer.volume();
        self.set_volume(current_volume);
    }

    fn handle_play_pause(&mut self) {
        match self.play_status {
            SpircPlayStatus::Paused { .. } | SpircPlayStatus::LoadingPause { .. } => {
                self.handle_play()
            }
            SpircPlayStatus::Playing { .. } | SpircPlayStatus::LoadingPlay { .. } => {
                self.handle_pause()
            }
            _ => (),
        }
    }

    fn handle_pause(&mut self) {
        match self.play_status {
            SpircPlayStatus::Playing {
                nominal_start_time,
                preloading_of_next_track_triggered,
            } => {
                self.player.pause();
                self.state.set_status(PlayStatus::kPlayStatusPause);
                let position_ms = (self.now_ms() - nominal_start_time) as u32;
                self.update_state_position(position_ms);
                self.play_status = SpircPlayStatus::Paused {
                    position_ms,
                    preloading_of_next_track_triggered,
                };
            }
            SpircPlayStatus::LoadingPlay { position_ms } => {
                self.player.pause();
                self.play_status = SpircPlayStatus::LoadingPause { position_ms };
            }
            _ => (),
        }
    }

    fn handle_seek(&mut self, position_ms: u32) {
        self.update_state_position(position_ms);
        self.player.seek(position_ms);
        let now = self.now_ms();
        match self.play_status {
            SpircPlayStatus::Stopped => (),
            SpircPlayStatus::LoadingPause {
                position_ms: ref mut position,
            }
            | SpircPlayStatus::LoadingPlay {
                position_ms: ref mut position,
            }
            | SpircPlayStatus::Paused {
                position_ms: ref mut position,
                ..
            } => *position = position_ms,
            SpircPlayStatus::Playing {
                ref mut nominal_start_time,
                ..
            } => *nominal_start_time = now - position_ms as i64,
        };
    }

    fn handle_seek_offset(&mut self, offset_ms: i32) {
        let position_ms = match self.play_status {
            SpircPlayStatus::Stopped => return,
            SpircPlayStatus::LoadingPause { position_ms }
            | SpircPlayStatus::LoadingPlay { position_ms }
            | SpircPlayStatus::Paused { position_ms, .. } => position_ms,
            SpircPlayStatus::Playing {
                nominal_start_time, ..
            } => {
                let now = self.now_ms();
                (now - nominal_start_time) as u32
            }
        };

        let position_ms = ((position_ms as i32) + offset_ms).max(0) as u32;

        self.handle_seek(position_ms);
    }

    fn consume_queued_track(&mut self) -> usize {
        // Removes current track if it is queued
        // Returns the index of the next track
        let current_index = self.state.playing_track_index() as usize;
        if (current_index < self.state.track.len()) && self.state.track[current_index].queued() {
            self.state.track.remove(current_index);
            current_index
        } else {
            current_index + 1
        }
    }

    fn preview_next_track(&mut self) -> Option<SpotifyId> {
        self.get_track_id_to_play_from_playlist(self.state.playing_track_index() + 1)
            .map(|(track_id, _)| track_id)
    }

    fn handle_preload_next_track(&mut self) {
        // Requests the player thread to preload the next track
        match self.play_status {
            SpircPlayStatus::Paused {
                ref mut preloading_of_next_track_triggered,
                ..
            }
            | SpircPlayStatus::Playing {
                ref mut preloading_of_next_track_triggered,
                ..
            } => {
                *preloading_of_next_track_triggered = true;
            }
            _ => (),
        }

        if let Some(track_id) = self.preview_next_track() {
            self.player.preload(track_id);
        } else {
            self.handle_stop();
        }
    }

    // Mark unavailable tracks so we can skip them later
    fn handle_unavailable(&mut self, track_id: SpotifyId) {
        let unavailables = self.get_track_index_for_spotify_id(&track_id, 0);
        for &index in unavailables.iter() {
            let mut unplayable_track_ref = TrackRef::new();
            unplayable_track_ref.set_gid(self.state.track[index].gid().to_vec());
            // Misuse context field to flag the track
            unplayable_track_ref.set_context(String::from("NonPlayable"));
            std::mem::swap(&mut self.state.track[index], &mut unplayable_track_ref);
            debug!(
                "Marked <{:?}> at {:?} as NonPlayable",
                self.state.track[index], index,
            );
        }
        self.handle_preload_next_track();
    }

    fn handle_next(&mut self) {
        let context_uri = self.state.context_uri().to_owned();
        let mut tracks_len = self.state.track.len() as u32;
        let mut new_index = self.consume_queued_track() as u32;
        let mut continue_playing = self.state.status() == PlayStatus::kPlayStatusPlay;

        let update_tracks =
            self.autoplay_context && tracks_len - new_index < CONTEXT_FETCH_THRESHOLD;

        debug!(
            "At track {:?} of {:?} <{:?}> update [{}]",
            new_index + 1,
            tracks_len,
            context_uri,
            update_tracks,
        );

        // When in autoplay, keep topping up the playlist when it nears the end
        if update_tracks {
            if let Some(ref context) = self.context {
                self.resolve_context = Some(context.next_page_url.to_owned());
                self.update_tracks_from_context();
                tracks_len = self.state.track.len() as u32;
            }
        }

        // When not in autoplay, either start autoplay or loop back to the start
        if new_index >= tracks_len {
            // for some contexts there is no autoplay, such as shows and episodes
            // in such cases there is no context in librespot.
            if self.context.is_some() && self.session.autoplay() {
                // Extend the playlist
                debug!("Starting autoplay for <{}>", context_uri);
                // force reloading the current context with an autoplay context
                self.autoplay_context = true;
                self.resolve_context = Some(self.state.context_uri().to_owned());
                self.update_tracks_from_context();
                self.player.set_auto_normalise_as_album(false);
            } else {
                new_index = 0;
                continue_playing &= self.state.repeat();
                debug!("Looping back to start, repeat is {}", continue_playing);
            }
        }

        if tracks_len > 0 {
            self.state.set_playing_track_index(new_index);
            self.load_track(continue_playing, 0);
        } else {
            info!("Not playing next track because there are no more tracks left in queue.");
            self.state.set_playing_track_index(0);
            self.handle_stop();
        }
    }

    fn handle_prev(&mut self) {
        // Previous behaves differently based on the position
        // Under 3s it goes to the previous song (starts playing)
        // Over 3s it seeks to zero (retains previous play status)
        if self.position() < 3000 {
            // Queued tracks always follow the currently playing track.
            // They should not be considered when calculating the previous
            // track so extract them beforehand and reinsert them after it.
            let mut queue_tracks = Vec::new();
            {
                let queue_index = self.consume_queued_track();
                let tracks = &mut self.state.track;
                while queue_index < tracks.len() && tracks[queue_index].queued() {
                    queue_tracks.push(tracks.remove(queue_index));
                }
            }
            let current_index = self.state.playing_track_index();
            let new_index = if current_index > 0 {
                current_index - 1
            } else if self.state.repeat() {
                self.state.track.len() as u32 - 1
            } else {
                0
            };
            // Reinsert queued tracks after the new playing track.
            let mut pos = (new_index + 1) as usize;
            for track in queue_tracks {
                self.state.track.insert(pos, track);
                pos += 1;
            }

            self.state.set_playing_track_index(new_index);

            let start_playing = self.state.status() == PlayStatus::kPlayStatusPlay;
            self.load_track(start_playing, 0);
        } else {
            self.handle_seek(0);
        }
    }

    fn handle_volume_up(&mut self) {
        let volume = (self.device.volume() as u16).saturating_add(VOLUME_STEP_SIZE);
        self.set_volume(volume);
    }

    fn handle_volume_down(&mut self) {
        let volume = (self.device.volume() as u16).saturating_sub(VOLUME_STEP_SIZE);
        self.set_volume(volume);
    }

    fn handle_end_of_track(&mut self) -> Result<(), Error> {
        self.handle_next();
        self.notify(None)
    }

    fn position(&mut self) -> u32 {
        match self.play_status {
            SpircPlayStatus::Stopped => 0,
            SpircPlayStatus::LoadingPlay { position_ms }
            | SpircPlayStatus::LoadingPause { position_ms }
            | SpircPlayStatus::Paused { position_ms, .. } => position_ms,
            SpircPlayStatus::Playing {
                nominal_start_time, ..
            } => (self.now_ms() - nominal_start_time) as u32,
        }
    }

    fn update_tracks_from_context(&mut self) {
        if let Some(ref context) = self.context {
            let new_tracks = &context.tracks;

            debug!("Adding {:?} tracks from context to frame", new_tracks.len());

            let mut track_vec = self.state.track.clone();
            if let Some(head) = track_vec.len().checked_sub(CONTEXT_TRACKS_HISTORY) {
                track_vec.drain(0..head);
            }
            track_vec.extend_from_slice(new_tracks);
            self.state.track = track_vec;

            // Update playing index
            if let Some(new_index) = self
                .state
                .playing_track_index()
                .checked_sub(CONTEXT_TRACKS_HISTORY as u32)
            {
                self.state.set_playing_track_index(new_index);
            }
        } else {
            warn!("No context to update from!");
        }
    }

    fn update_tracks(&mut self, state: &State) {
        trace!("State: {:#?}", state);

        let index = state.playing_track_index();
        let context_uri = state.context_uri();
        let tracks = &state.track;

        trace!("Frame has {:?} tracks", tracks.len());

        // First the tracks from the requested context, without autoplay.
        // We will transition into autoplay after the latest track of this context.
        self.autoplay_context = false;
        self.resolve_context = Some(context_uri.to_owned());

        self.player
            .set_auto_normalise_as_album(context_uri.starts_with("spotify:album:"));

        self.state.set_playing_track_index(index);
        self.state.track = tracks.to_vec();
        self.state.set_context_uri(context_uri.to_owned());
        // has_shuffle/repeat seem to always be true in these replace msgs,
        // but to replicate the behaviour of the Android client we have to
        // ignore false values.
        if state.repeat() {
            self.state.set_repeat(true);
        }
        if state.shuffle() {
            self.state.set_shuffle(true);
        }
    }

    // Helper to find corresponding index(s) for track_id
    fn get_track_index_for_spotify_id(
        &self,
        track_id: &SpotifyId,
        start_index: usize,
    ) -> Vec<usize> {
        let index: Vec<usize> = self.state.track[start_index..]
            .iter()
            .enumerate()
            .filter(|&(_, track_ref)| track_ref.gid() == track_id.to_raw())
            .map(|(idx, _)| start_index + idx)
            .collect();
        index
    }

    // Broken out here so we can refactor this later when we move to SpotifyObjectID or similar
    fn track_ref_is_unavailable(&self, track_ref: &TrackRef) -> bool {
        track_ref.context() == "NonPlayable"
    }

    fn get_track_id_to_play_from_playlist(&self, index: u32) -> Option<(SpotifyId, u32)> {
        let tracks_len = self.state.track.len();

        // Guard against tracks_len being zero to prevent
        // 'index out of bounds: the len is 0 but the index is 0'
        // https://github.com/librespot-org/librespot/issues/226#issuecomment-971642037
        if tracks_len == 0 {
            warn!("No playable track found in state: {:?}", self.state);
            return None;
        }

        let mut new_playlist_index = index as usize;

        if new_playlist_index >= tracks_len {
            new_playlist_index = 0;
        }

        let start_index = new_playlist_index;

        // Cycle through all tracks, break if we don't find any playable tracks
        // tracks in each frame either have a gid or uri (that may or may not be a valid track)
        // E.g - context based frames sometimes contain tracks with <spotify:meta:page:>

        let mut track_ref = self.state.track[new_playlist_index].clone();
        let mut track_id = SpotifyId::try_from(&track_ref);
        while self.track_ref_is_unavailable(&track_ref) || track_id.is_err() {
            warn!(
                "Skipping track <{:?}> at position [{}] of {}",
                track_ref, new_playlist_index, tracks_len
            );

            new_playlist_index += 1;
            if new_playlist_index >= tracks_len {
                new_playlist_index = 0;
            }

            if new_playlist_index == start_index {
                warn!("No playable track found in state: {:?}", self.state);
                return None;
            }
            track_ref = self.state.track[new_playlist_index].clone();
            track_id = SpotifyId::try_from(&track_ref);
        }

        match track_id {
            Ok(track_id) => Some((track_id, new_playlist_index as u32)),
            Err(_) => None,
        }
    }

    fn load_track(&mut self, start_playing: bool, position_ms: u32) {
        let index = self.state.playing_track_index();

        match self.get_track_id_to_play_from_playlist(index) {
            Some((track, index)) => {
                self.state.set_playing_track_index(index);

                self.player.load(track, start_playing, position_ms);

                self.update_state_position(position_ms);
                if start_playing {
                    self.state.set_status(PlayStatus::kPlayStatusPlay);
                    self.play_status = SpircPlayStatus::LoadingPlay { position_ms };
                } else {
                    self.state.set_status(PlayStatus::kPlayStatusPause);
                    self.play_status = SpircPlayStatus::LoadingPause { position_ms };
                }
            }
            None => {
                self.handle_stop();
            }
        }
    }

    fn hello(&mut self) -> Result<(), Error> {
        CommandSender::new(self, MessageType::kMessageTypeHello).send()
    }

    fn notify(&mut self, recipient: Option<&str>) -> Result<(), Error> {
        let status = self.state.status();

        // When in loading state, the Spotify UI is disabled for interaction.
        // On desktop this isn't so bad but on mobile it means that the bottom
        // control disappears entirely. This is very confusing, so don't notify
        // in this case.
        if status == PlayStatus::kPlayStatusLoading {
            return Ok(());
        }

        trace!("Sending status to server: [{:?}]", status);
        let mut cs = CommandSender::new(self, MessageType::kMessageTypeNotify);
        if let Some(s) = recipient {
            cs = cs.recipient(s);
        }
        cs.send()
    }

    fn set_volume(&mut self, volume: u16) {
        let old_volume = self.device.volume();
        let new_volume = volume as u32;
        if old_volume != new_volume {
            self.device.set_volume(new_volume);
            self.mixer.set_volume(volume);
            if let Some(cache) = self.session.cache() {
                cache.save_volume(volume)
            }
            if self.device.is_active() {
                self.player.emit_volume_changed_event(volume);
            }
        }
    }
}

impl Drop for SpircTask {
    fn drop(&mut self) {
        debug!("drop Spirc[{}]", self.spirc_id);
    }
}

struct CommandSender<'a> {
    spirc: &'a mut SpircTask,
    frame: protocol::spirc::Frame,
}

impl<'a> CommandSender<'a> {
    fn new(spirc: &'a mut SpircTask, cmd: MessageType) -> Self {
        let mut frame = protocol::spirc::Frame::new();
        // frame version
        frame.set_version(1);
        // Latest known Spirc version is 3.2.6, but we need another interface to announce support for Spirc V3.
        // Setting anything higher than 2.0.0 here just seems to limit it to 2.0.0.
        frame.set_protocol_version("2.0.0".to_string());
        frame.set_ident(spirc.ident.clone());
        frame.set_seq_nr(spirc.sequence.get());
        frame.set_typ(cmd);
        *frame.device_state.mut_or_insert_default() = spirc.device.clone();
        frame.set_state_update_id(spirc.now_ms());
        CommandSender { spirc, frame }
    }

    fn recipient(mut self, recipient: &'a str) -> Self {
        self.frame.recipient.push(recipient.to_owned());
        self
    }

    #[allow(dead_code)]
    fn state(mut self, state: protocol::spirc::State) -> Self {
        *self.frame.state.mut_or_insert_default() = state;
        self
    }

    fn send(mut self) -> Result<(), Error> {
        if self.frame.state.is_none() && self.spirc.device.is_active() {
            *self.frame.state.mut_or_insert_default() = self.spirc.state.clone();
        }

        self.spirc.sender.send(self.frame.write_to_bytes()?)
    }
}
