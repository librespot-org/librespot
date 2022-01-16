use std::{
    convert::TryFrom,
    future::Future,
    pin::Pin,
    time::{SystemTime, UNIX_EPOCH},
};

use futures_util::{
    future::{self, FusedFuture},
    stream::FusedStream,
    FutureExt, StreamExt,
};

use protobuf::{self, Message};
use rand::seq::SliceRandom;
use thiserror::Error;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::{
    config::ConnectConfig,
    context::StationContext,
    core::{
        authentication::Credentials,
        mercury::{MercuryError, MercurySender},
        session::UserAttributes,
        util::SeqGenerator,
        version, Error, Session, SpotifyId,
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
    #[error("message addressed at another ident: {0}")]
    Ident(String),
    #[error("message pushed for another URI")]
    InvalidUri(String),
}

impl From<SpircError> for Error {
    fn from(err: SpircError) -> Self {
        match err {
            SpircError::NoData => Error::unavailable(err),
            SpircError::Ident(_) => Error::aborted(err),
            SpircError::InvalidUri(_) => Error::aborted(err),
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

type BoxedFuture<T> = Pin<Box<dyn FusedFuture<Output = T> + Send>>;
type BoxedStream<T> = Pin<Box<dyn FusedStream<Item = T> + Send>>;

struct SpircTask {
    player: Player,
    mixer: Box<dyn Mixer>,

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
    context_fut: BoxedFuture<Result<serde_json::Value, Error>>,
    autoplay_fut: BoxedFuture<Result<String, Error>>,
    context: Option<StationContext>,
}

pub enum SpircCommand {
    Play,
    PlayPause,
    Pause,
    Prev,
    Next,
    VolumeUp,
    VolumeDown,
    Shutdown,
    Shuffle,
}

const CONTEXT_TRACKS_HISTORY: usize = 10;
const CONTEXT_FETCH_THRESHOLD: u32 = 5;

const VOLUME_STEPS: i64 = 64;
const VOLUME_STEP_SIZE: u16 = 1024; // (u16::MAX + 1) / VOLUME_STEPS

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

fn initial_device_state(config: ConnectConfig) -> DeviceState {
    {
        let mut msg = DeviceState::new();
        msg.set_sw_version(version::SEMVER.to_string());
        msg.set_is_active(false);
        msg.set_can_play(true);
        msg.set_volume(0);
        msg.set_name(config.name);
        {
            let repeated = msg.mut_capabilities();
            {
                let msg = repeated.push_default();
                msg.set_typ(protocol::spirc::CapabilityType::kCanBePlayer);
                {
                    let repeated = msg.mut_intValue();
                    repeated.push(1)
                };
                msg
            };
            {
                let msg = repeated.push_default();
                msg.set_typ(protocol::spirc::CapabilityType::kDeviceType);
                {
                    let repeated = msg.mut_intValue();
                    repeated.push(config.device_type as i64)
                };
                msg
            };
            {
                let msg = repeated.push_default();
                msg.set_typ(protocol::spirc::CapabilityType::kGaiaEqConnectId);
                {
                    let repeated = msg.mut_intValue();
                    repeated.push(1)
                };
                msg
            };
            {
                let msg = repeated.push_default();
                msg.set_typ(protocol::spirc::CapabilityType::kSupportsLogout);
                {
                    let repeated = msg.mut_intValue();
                    repeated.push(0)
                };
                msg
            };
            {
                let msg = repeated.push_default();
                msg.set_typ(protocol::spirc::CapabilityType::kIsObservable);
                {
                    let repeated = msg.mut_intValue();
                    repeated.push(1)
                };
                msg
            };
            {
                let msg = repeated.push_default();
                msg.set_typ(protocol::spirc::CapabilityType::kVolumeSteps);
                {
                    let repeated = msg.mut_intValue();
                    if config.has_volume_ctrl {
                        repeated.push(VOLUME_STEPS)
                    } else {
                        repeated.push(0)
                    }
                };
                msg
            };
            {
                let msg = repeated.push_default();
                msg.set_typ(protocol::spirc::CapabilityType::kSupportsPlaylistV2);
                {
                    let repeated = msg.mut_intValue();
                    repeated.push(1)
                };
                msg
            };
            {
                let msg = repeated.push_default();
                msg.set_typ(protocol::spirc::CapabilityType::kSupportedContexts);
                {
                    let repeated = msg.mut_stringValue();
                    repeated.push(::std::convert::Into::into("album"));
                    repeated.push(::std::convert::Into::into("playlist"));
                    repeated.push(::std::convert::Into::into("search"));
                    repeated.push(::std::convert::Into::into("inbox"));
                    repeated.push(::std::convert::Into::into("toplist"));
                    repeated.push(::std::convert::Into::into("starred"));
                    repeated.push(::std::convert::Into::into("publishedstarred"));
                    repeated.push(::std::convert::Into::into("track"))
                };
                msg
            };
            {
                let msg = repeated.push_default();
                msg.set_typ(protocol::spirc::CapabilityType::kSupportedTypes);
                {
                    let repeated = msg.mut_stringValue();
                    repeated.push(::std::convert::Into::into("audio/local"));
                    repeated.push(::std::convert::Into::into("audio/track"));
                    repeated.push(::std::convert::Into::into("audio/episode"));
                    repeated.push(::std::convert::Into::into("local"));
                    repeated.push(::std::convert::Into::into("track"))
                };
                msg
            };
        };
        msg
    }
}

fn url_encode(bytes: impl AsRef<[u8]>) -> String {
    form_urlencoded::byte_serialize(bytes.as_ref()).collect()
}

impl Spirc {
    pub async fn new(
        config: ConnectConfig,
        session: Session,
        credentials: Credentials,
        player: Player,
        mixer: Box<dyn Mixer>,
    ) -> Result<(Spirc, impl Future<Output = ()>), Error> {
        debug!("new Spirc[{}]", session.session_id());

        let ident = session.device_id().to_owned();

        let remote_update = Box::pin(
            session
                .mercury()
                .listen_for("hm://remote/user/")
                .map(UnboundedReceiverStream::new)
                .flatten_stream()
                .map(|response| -> Result<(String, Frame), Error> {
                    let uri_split: Vec<&str> = response.uri.split('/').collect();
                    let username = match uri_split.get(uri_split.len() - 2) {
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
        session.connect(credentials).await?;

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

            context_fut: Box::pin(future::pending()),
            autoplay_fut: Box::pin(future::pending()),
            context: None,
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
    pub fn shuffle(&self) -> Result<(), Error> {
        Ok(self.commands.send(SpircCommand::Shuffle)?)
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
                                error!("could not dispatch remote update: frame was intended for {}", username);
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
                        error!("could not dispatch command: {}", e);
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
                context = &mut self.context_fut, if !self.context_fut.is_terminated() => {
                    match context {
                        Ok(value) => {
                            let r_context = serde_json::from_value::<StationContext>(value);
                            self.context = match r_context {
                                Ok(context) => {
                                    info!(
                                        "Resolved {:?} tracks from <{:?}>",
                                        context.tracks.len(),
                                        self.state.get_context_uri(),
                                    );
                                    Some(context)
                                }
                                Err(e) => {
                                    error!("Unable to parse JSONContext {:?}", e);
                                    None
                                }
                            };
                            // It needn't be so verbose - can be as simple as
                            // if let Some(ref context) = r_context {
                            //     info!("Got {:?} tracks from <{}>", context.tracks.len(), context.uri);
                            // }
                            // self.context = r_context;
                        },
                        Err(err) => {
                            error!("ContextError: {:?}", err)
                        }
                    }
                },
                autoplay = &mut self.autoplay_fut, if !self.autoplay_fut.is_terminated() => {
                    match autoplay {
                        Ok(autoplay_station_uri) => {
                            info!("Autoplay uri resolved to <{:?}>", autoplay_station_uri);
                            self.context_fut = self.resolve_station(&autoplay_station_uri);
                        },
                        Err(err) => {
                            error!("AutoplayError: {:?}", err)
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
        let active = self.device.get_is_active();
        match cmd {
            SpircCommand::Play => {
                if active {
                    self.handle_play();
                    self.notify(None)
                } else {
                    CommandSender::new(self, MessageType::kMessageTypePlay).send()
                }
            }
            SpircCommand::PlayPause => {
                if active {
                    self.handle_play_pause();
                    self.notify(None)
                } else {
                    CommandSender::new(self, MessageType::kMessageTypePlayPause).send()
                }
            }
            SpircCommand::Pause => {
                if active {
                    self.handle_pause();
                    self.notify(None)
                } else {
                    CommandSender::new(self, MessageType::kMessageTypePause).send()
                }
            }
            SpircCommand::Prev => {
                if active {
                    self.handle_prev();
                    self.notify(None)
                } else {
                    CommandSender::new(self, MessageType::kMessageTypePrev).send()
                }
            }
            SpircCommand::Next => {
                if active {
                    self.handle_next();
                    self.notify(None)
                } else {
                    CommandSender::new(self, MessageType::kMessageTypeNext).send()
                }
            }
            SpircCommand::VolumeUp => {
                if active {
                    self.handle_volume_up();
                    self.notify(None)
                } else {
                    CommandSender::new(self, MessageType::kMessageTypeVolumeUp).send()
                }
            }
            SpircCommand::VolumeDown => {
                if active {
                    self.handle_volume_down();
                    self.notify(None)
                } else {
                    CommandSender::new(self, MessageType::kMessageTypeVolumeDown).send()
                }
            }
            SpircCommand::Shutdown => {
                CommandSender::new(self, MessageType::kMessageTypeGoodbye).send()?;
                self.shutdown = true;
                if let Some(rx) = self.commands.as_mut() {
                    rx.close()
                }
                Ok(())
            }
            SpircCommand::Shuffle => {
                CommandSender::new(self, MessageType::kMessageTypeShuffle).send()
            }
        }
    }

    fn handle_player_event(&mut self, event: PlayerEvent) -> Result<(), Error> {
        // we only process events if the play_request_id matches. If it doesn't, it is
        // an event that belongs to a previous track and only arrives now due to a race
        // condition. In this case we have updated the state already and don't want to
        // mess with it.
        if let Some(play_request_id) = event.get_play_request_id() {
            if Some(play_request_id) == self.play_request_id {
                match event {
                    PlayerEvent::EndOfTrack { .. } => self.handle_end_of_track(),
                    PlayerEvent::Loading { .. } => {
                        trace!("==> kPlayStatusLoading");
                        self.state.set_status(PlayStatus::kPlayStatusLoading);
                        self.notify(None)
                    }
                    PlayerEvent::Playing { position_ms, .. } => {
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
                            SpircPlayStatus::Paused {
                                ref mut position_ms,
                                ..
                            } => {
                                if *position_ms != new_position_ms {
                                    *position_ms = new_position_ms;
                                    self.update_state_position(new_position_ms);
                                    self.notify(None)
                                } else {
                                    Ok(())
                                }
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
            .get_pairs()
            .iter()
            .map(|pair| (pair.get_key().to_owned(), pair.get_value().to_owned()))
            .collect();
        self.session.set_user_attributes(attributes)
    }

    fn handle_user_attributes_mutation(&mut self, mutation: UserAttributesMutation) {
        for attribute in mutation.get_fields().iter() {
            let key = attribute.get_name();
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
                    self.player.skip_explicit_content();
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
        let ident = update.get_ident();
        if ident == device_id
            || (!update.get_recipient().is_empty() && !update.get_recipient().contains(device_id))
        {
            return Err(SpircError::Ident(ident.to_string()).into());
        }

        for entry in update.get_device_state().get_metadata().iter() {
            if entry.get_field_type() == "client_id" {
                self.session.set_client_id(entry.get_metadata());
                break;
            }
        }

        match update.get_typ() {
            MessageType::kMessageTypeHello => self.notify(Some(ident)),

            MessageType::kMessageTypeLoad => {
                if !self.device.get_is_active() {
                    let now = self.now_ms();
                    self.device.set_is_active(true);
                    self.device.set_became_active_at(now);
                }

                self.update_tracks(&update);

                if !self.state.get_track().is_empty() {
                    let start_playing =
                        update.get_state().get_status() == PlayStatus::kPlayStatusPlay;
                    self.load_track(start_playing, update.get_state().get_position_ms());
                } else {
                    info!("No more tracks left in queue");
                    self.handle_stop();
                }

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
                self.state.set_repeat(update.get_state().get_repeat());
                self.notify(None)
            }

            MessageType::kMessageTypeShuffle => {
                self.state.set_shuffle(update.get_state().get_shuffle());
                if self.state.get_shuffle() {
                    let current_index = self.state.get_playing_track_index();
                    {
                        let tracks = self.state.mut_track();
                        tracks.swap(0, current_index as usize);
                        if let Some((_, rest)) = tracks.split_first_mut() {
                            let mut rng = rand::thread_rng();
                            rest.shuffle(&mut rng);
                        }
                    }
                    self.state.set_playing_track_index(0);
                } else {
                    let context = self.state.get_context_uri();
                    debug!("{:?}", context);
                }
                self.notify(None)
            }

            MessageType::kMessageTypeSeek => {
                self.handle_seek(update.get_position());
                self.notify(None)
            }

            MessageType::kMessageTypeReplace => {
                self.update_tracks(&update);

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
                self.set_volume(update.get_volume() as u16);
                self.notify(None)
            }

            MessageType::kMessageTypeNotify => {
                if self.device.get_is_active()
                    && update.get_device_state().get_is_active()
                    && self.device.get_became_active_at()
                        <= update.get_device_state().get_became_active_at()
                {
                    self.device.set_is_active(false);
                    self.handle_stop();
                }
                Ok(())
            }

            _ => Ok(()),
        }
    }

    fn handle_stop(&mut self) {
        self.player.stop();
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
                    nominal_start_time: self.now_ms() as i64 - position_ms as i64,
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

    fn consume_queued_track(&mut self) -> usize {
        // Removes current track if it is queued
        // Returns the index of the next track
        let current_index = self.state.get_playing_track_index() as usize;
        if (current_index < self.state.get_track().len())
            && self.state.get_track()[current_index].get_queued()
        {
            self.state.mut_track().remove(current_index);
            current_index
        } else {
            current_index + 1
        }
    }

    fn preview_next_track(&mut self) -> Option<SpotifyId> {
        self.get_track_id_to_play_from_playlist(self.state.get_playing_track_index() + 1)
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
            debug_assert_eq!(self.state.get_track()[index].get_gid(), track_id.to_raw());
            let mut unplayable_track_ref = TrackRef::new();
            unplayable_track_ref.set_gid(self.state.get_track()[index].get_gid().to_vec());
            // Misuse context field to flag the track
            unplayable_track_ref.set_context(String::from("NonPlayable"));
            std::mem::swap(
                &mut self.state.mut_track()[index],
                &mut unplayable_track_ref,
            );
            debug!(
                "Marked <{:?}> at {:?} as NonPlayable",
                self.state.get_track()[index],
                index,
            );
        }
        self.handle_preload_next_track();
    }

    fn handle_next(&mut self) {
        let mut new_index = self.consume_queued_track() as u32;
        let mut continue_playing = true;
        let tracks_len = self.state.get_track().len() as u32;
        debug!(
            "At track {:?} of {:?} <{:?}> update [{}]",
            new_index + 1,
            tracks_len,
            self.state.get_context_uri(),
            tracks_len - new_index < CONTEXT_FETCH_THRESHOLD
        );
        let context_uri = self.state.get_context_uri().to_owned();
        if (context_uri.starts_with("spotify:station:")
            || context_uri.starts_with("spotify:dailymix:")
            // spotify:user:xxx:collection
            || context_uri.starts_with(&format!("spotify:user:{}:collection",url_encode(&self.session.username()))))
            && ((self.state.get_track().len() as u32) - new_index) < CONTEXT_FETCH_THRESHOLD
        {
            self.context_fut = self.resolve_station(&context_uri);
            self.update_tracks_from_context();
        }

        if new_index >= tracks_len {
            let autoplay = self
                .session
                .get_user_attribute("autoplay")
                .unwrap_or_else(|| {
                    warn!(
                        "Unable to get autoplay user attribute. Continuing with autoplay disabled."
                    );
                    "0".into()
                });

            if autoplay == "1" {
                // Extend the playlist
                debug!("Extending playlist <{}>", context_uri);
                self.update_tracks_from_context();
                self.player.set_auto_normalise_as_album(false);
            } else {
                new_index = 0;
                continue_playing = self.state.get_repeat();
                debug!(
                    "Looping around back to start, repeat is {}",
                    continue_playing
                );
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
                let tracks = self.state.mut_track();
                while queue_index < tracks.len() && tracks[queue_index].get_queued() {
                    queue_tracks.push(tracks.remove(queue_index));
                }
            }
            let current_index = self.state.get_playing_track_index();
            let new_index = if current_index > 0 {
                current_index - 1
            } else if self.state.get_repeat() {
                self.state.get_track().len() as u32 - 1
            } else {
                0
            };
            // Reinsert queued tracks after the new playing track.
            let mut pos = (new_index + 1) as usize;
            for track in queue_tracks {
                self.state.mut_track().insert(pos, track);
                pos += 1;
            }

            self.state.set_playing_track_index(new_index);

            self.load_track(true, 0);
        } else {
            self.handle_seek(0);
        }
    }

    fn handle_volume_up(&mut self) {
        let volume = (self.device.get_volume() as u16).saturating_add(VOLUME_STEP_SIZE);
        self.set_volume(volume);
    }

    fn handle_volume_down(&mut self) {
        let volume = (self.device.get_volume() as u16).saturating_sub(VOLUME_STEP_SIZE);
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

    fn resolve_station(&self, uri: &str) -> BoxedFuture<Result<serde_json::Value, Error>> {
        let radio_uri = format!("hm://radio-apollo/v3/stations/{}", uri);

        self.resolve_uri(&radio_uri)
    }

    fn resolve_autoplay_uri(&self, uri: &str) -> BoxedFuture<Result<String, Error>> {
        let query_uri = format!("hm://autoplay-enabled/query?uri={}", uri);
        let request = self.session.mercury().get(query_uri);
        Box::pin(
            async {
                let response = request?.await?;

                if response.status_code == 200 {
                    let data = response.payload.first().ok_or(SpircError::NoData)?.to_vec();
                    Ok(String::from_utf8(data)?)
                } else {
                    warn!("No autoplay_uri found");
                    Err(MercuryError::Response(response).into())
                }
            }
            .fuse(),
        )
    }

    fn resolve_uri(&self, uri: &str) -> BoxedFuture<Result<serde_json::Value, Error>> {
        let request = self.session.mercury().get(uri);

        Box::pin(
            async move {
                let response = request?.await?;

                let data = response.payload.first().ok_or(SpircError::NoData)?;
                let response: serde_json::Value = serde_json::from_slice(data)?;

                Ok(response)
            }
            .fuse(),
        )
    }

    fn update_tracks_from_context(&mut self) {
        if let Some(ref context) = self.context {
            self.context_fut = self.resolve_uri(&context.next_page_url);

            let new_tracks = &context.tracks;
            debug!("Adding {:?} tracks from context to frame", new_tracks.len());
            let mut track_vec = self.state.take_track().into_vec();
            if let Some(head) = track_vec.len().checked_sub(CONTEXT_TRACKS_HISTORY) {
                track_vec.drain(0..head);
            }
            track_vec.extend_from_slice(new_tracks);
            self.state
                .set_track(protobuf::RepeatedField::from_vec(track_vec));

            // Update playing index
            if let Some(new_index) = self
                .state
                .get_playing_track_index()
                .checked_sub(CONTEXT_TRACKS_HISTORY as u32)
            {
                self.state.set_playing_track_index(new_index);
            }
        } else {
            warn!("No context to update from!");
        }
    }

    fn update_tracks(&mut self, frame: &protocol::spirc::Frame) {
        trace!("State: {:#?}", frame.get_state());

        let index = frame.get_state().get_playing_track_index();
        let context_uri = frame.get_state().get_context_uri().to_owned();
        let tracks = frame.get_state().get_track();

        trace!("Frame has {:?} tracks", tracks.len());

        if context_uri.starts_with("spotify:station:")
            || context_uri.starts_with("spotify:dailymix:")
        {
            self.context_fut = self.resolve_station(&context_uri);
        } else if let Some(autoplay) = self.session.get_user_attribute("autoplay") {
            if &autoplay == "1" {
                info!("Fetching autoplay context uri");
                // Get autoplay_station_uri for regular playlists
                self.autoplay_fut = self.resolve_autoplay_uri(&context_uri);
            }
        }

        self.player
            .set_auto_normalise_as_album(context_uri.starts_with("spotify:album:"));

        self.state.set_playing_track_index(index);
        self.state.set_track(tracks.iter().cloned().collect());
        self.state.set_context_uri(context_uri);
        // has_shuffle/repeat seem to always be true in these replace msgs,
        // but to replicate the behaviour of the Android client we have to
        // ignore false values.
        let state = frame.get_state();
        if state.get_repeat() {
            self.state.set_repeat(true);
        }
        if state.get_shuffle() {
            self.state.set_shuffle(true);
        }
    }

    // Helper to find corresponding index(s) for track_id
    fn get_track_index_for_spotify_id(
        &self,
        track_id: &SpotifyId,
        start_index: usize,
    ) -> Vec<usize> {
        let index: Vec<usize> = self.state.get_track()[start_index..]
            .iter()
            .enumerate()
            .filter(|&(_, track_ref)| track_ref.get_gid() == track_id.to_raw())
            .map(|(idx, _)| start_index + idx)
            .collect();
        // Sanity check
        debug_assert!(!index.is_empty());
        index
    }

    // Broken out here so we can refactor this later when we move to SpotifyObjectID or similar
    fn track_ref_is_unavailable(&self, track_ref: &TrackRef) -> bool {
        track_ref.get_context() == "NonPlayable"
    }

    fn get_track_id_to_play_from_playlist(&self, index: u32) -> Option<(SpotifyId, u32)> {
        let tracks_len = self.state.get_track().len();

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

        let mut track_ref = self.state.get_track()[new_playlist_index].clone();
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
            track_ref = self.state.get_track()[new_playlist_index].clone();
            track_id = SpotifyId::try_from(&track_ref);
        }

        match track_id {
            Ok(track_id) => Some((track_id, new_playlist_index as u32)),
            Err(_) => None,
        }
    }

    fn load_track(&mut self, start_playing: bool, position_ms: u32) {
        let index = self.state.get_playing_track_index();

        match self.get_track_id_to_play_from_playlist(index) {
            Some((track, index)) => {
                self.state.set_playing_track_index(index);

                self.play_request_id = Some(self.player.load(track, start_playing, position_ms));

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
        let status_string = match self.state.get_status() {
            PlayStatus::kPlayStatusLoading => "kPlayStatusLoading",
            PlayStatus::kPlayStatusPause => "kPlayStatusPause",
            PlayStatus::kPlayStatusStop => "kPlayStatusStop",
            PlayStatus::kPlayStatusPlay => "kPlayStatusPlay",
        };
        trace!("Sending status to server: [{}]", status_string);
        let mut cs = CommandSender::new(self, MessageType::kMessageTypeNotify);
        if let Some(s) = recipient {
            cs = cs.recipient(s);
        }
        cs.send()
    }

    fn set_volume(&mut self, volume: u16) {
        self.device.set_volume(volume as u32);
        self.mixer.set_volume(volume);
        if let Some(cache) = self.session.cache() {
            cache.save_volume(volume)
        }
        self.player.emit_volume_set_event(volume);
    }
}

impl Drop for SpircTask {
    fn drop(&mut self) {
        debug!("drop Spirc[{}]", self.session.session_id());
    }
}

struct CommandSender<'a> {
    spirc: &'a mut SpircTask,
    frame: protocol::spirc::Frame,
}

impl<'a> CommandSender<'a> {
    fn new(spirc: &'a mut SpircTask, cmd: MessageType) -> CommandSender {
        let mut frame = protocol::spirc::Frame::new();
        frame.set_version(1);
        frame.set_protocol_version(::std::convert::Into::into("2.0.0"));
        frame.set_ident(spirc.ident.clone());
        frame.set_seq_nr(spirc.sequence.get());
        frame.set_typ(cmd);
        frame.set_device_state(spirc.device.clone());
        frame.set_state_update_id(spirc.now_ms());
        CommandSender { spirc, frame }
    }

    fn recipient(mut self, recipient: &'a str) -> CommandSender {
        self.frame.mut_recipient().push(recipient.to_owned());
        self
    }

    #[allow(dead_code)]
    fn state(mut self, state: protocol::spirc::State) -> CommandSender<'a> {
        self.frame.set_state(state);
        self
    }

    fn send(mut self) -> Result<(), Error> {
        if !self.frame.has_state() && self.spirc.device.get_is_active() {
            self.frame.set_state(self.spirc.state.clone());
        }

        self.spirc.sender.send(self.frame.write_to_bytes()?)
    }
}
