use std::{
    future::Future,
    pin::Pin,
    sync::atomic::{AtomicUsize, Ordering},
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::state::{ConnectState, ConnectStateConfig, ConnectStateError};
use crate::{
    context::PageContext,
    core::{authentication::Credentials, session::UserAttributes, Error, Session, SpotifyId},
    playback::{
        mixer::Mixer,
        player::{Player, PlayerEvent, PlayerEventChannel},
    },
    protocol::{
        explicit_content_pubsub::UserAttributesUpdate, user_attributes::UserAttributesMutation,
    },
};
use futures_util::{FutureExt, Stream, StreamExt};
use librespot_core::dealer::manager::{Reply, RequestReply};
use librespot_core::dealer::protocol::{PayloadValue, RequestCommand};
use librespot_protocol::connect::{Cluster, ClusterUpdate, PutStateReason, SetVolumeCommand};
use librespot_protocol::player::{Context, TransferState};
use protobuf::Message;
use thiserror::Error;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

#[derive(Debug, Error)]
pub enum SpircError {
    #[error("response payload empty")]
    NoData,
    #[error("received unexpected data {0:#?}")]
    UnexpectedData(PayloadValue),
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
            NoData | UnsupportedLocalPlayBack | UnexpectedData(_) => Error::unavailable(err),
            Ident(_) | InvalidUri(_) => Error::aborted(err),
        }
    }
}

#[derive(Debug)]
pub(crate) enum SpircPlayStatus {
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

type BoxedStream<T> = Pin<Box<dyn Stream<Item = T> + Send>>;

struct SpircTask {
    player: Arc<Player>,
    mixer: Arc<dyn Mixer>,

    connect_state: ConnectState,

    play_request_id: Option<u64>,
    play_status: SpircPlayStatus,

    connection_id_update: BoxedStream<Result<String, Error>>,
    connect_state_update: BoxedStream<Result<ClusterUpdate, Error>>,
    connect_state_volume_update: BoxedStream<Result<SetVolumeCommand, Error>>,
    connect_state_command: BoxedStream<RequestReply>,
    user_attributes_update: BoxedStream<Result<UserAttributesUpdate, Error>>,
    user_attributes_mutation: BoxedStream<Result<UserAttributesMutation, Error>>,

    commands: Option<mpsc::UnboundedReceiver<SpircCommand>>,
    player_events: Option<PlayerEventChannel>,

    shutdown: bool,
    session: Session,
    resolve_context: Option<String>,
    autoplay_context: bool,

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
    RepeatTrack(bool),
    Disconnect,
    SetPosition(u32),
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
    pub repeat_track: bool,
    pub playing_track_index: u32,
}

const CONTEXT_FETCH_THRESHOLD: u32 = 5;

const VOLUME_STEP_SIZE: u16 = 1024; // (u16::MAX + 1) / VOLUME_STEPS

pub struct Spirc {
    commands: mpsc::UnboundedSender<SpircCommand>,
}

impl Spirc {
    pub async fn new(
        config: ConnectStateConfig,
        session: Session,
        credentials: Credentials,
        player: Arc<Player>,
        mixer: Arc<dyn Mixer>,
    ) -> Result<(Spirc, impl Future<Output = ()>), Error> {
        let spirc_id = SPIRC_COUNTER.fetch_add(1, Ordering::AcqRel);
        debug!("new Spirc[{}]", spirc_id);

        let initial_volume = config.initial_volume;
        let connect_state = ConnectState::new(config, &session);

        let connection_id_update = Box::pin(
            session
                .dealer()
                .listen_for("hm://pusher/v1/connections/")?
                .map(|response| -> Result<String, Error> {
                    let connection_id = response
                        .headers
                        .get("Spotify-Connection-Id")
                        .ok_or_else(|| SpircError::InvalidUri(response.uri.clone()))?;
                    Ok(connection_id.to_owned())
                }),
        );

        let connect_state_update = Box::pin(
            session
                .dealer()
                .listen_for("hm://connect-state/v1/cluster")?
                .map(|msg| -> Result<ClusterUpdate, Error> {
                    match msg.payload {
                        PayloadValue::Raw(bytes) => ClusterUpdate::parse_from_bytes(&bytes)
                            .map_err(Error::failed_precondition),
                        other => Err(SpircError::UnexpectedData(other).into()),
                    }
                }),
        );

        let connect_state_volume_update = Box::pin(
            session
                .dealer()
                .listen_for("hm://connect-state/v1/connect/volume")?
                .map(|msg| match msg.payload {
                    PayloadValue::Raw(bytes) => SetVolumeCommand::parse_from_bytes(&bytes)
                        .map_err(Error::failed_precondition),
                    other => Err(SpircError::UnexpectedData(other).into()),
                }),
        );

        let connect_state_command = Box::pin(
            session
                .dealer()
                .handle_for("hm://connect-state/v1/player/command")
                .map(UnboundedReceiverStream::new)?,
        );

        // todo: remove later? probably have to find the equivalent for the dealer
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

        // todo: remove later? probably have to find the equivalent for the dealer
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
        session.dealer().start().await?;

        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();

        let player_events = player.get_player_event_channel();

        let mut task = SpircTask {
            player,
            mixer,

            connect_state,

            play_request_id: None,
            play_status: SpircPlayStatus::Stopped,

            connection_id_update,
            connect_state_update,
            connect_state_volume_update,
            connect_state_command,
            user_attributes_update,
            user_attributes_mutation,
            commands: Some(cmd_rx),
            player_events: Some(player_events),

            shutdown: false,
            session,

            resolve_context: None,
            autoplay_context: false,

            spirc_id,
        };

        let spirc = Spirc { commands: cmd_tx };
        task.set_volume(initial_volume as u16 - 1);

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
    pub fn repeat_track(&self, repeat: bool) -> Result<(), Error> {
        Ok(self.commands.send(SpircCommand::RepeatTrack(repeat))?)
    }
    pub fn set_volume(&self, volume: u16) -> Result<(), Error> {
        Ok(self.commands.send(SpircCommand::SetVolume(volume))?)
    }
    pub fn set_position_ms(&self, position_ms: u32) -> Result<(), Error> {
        Ok(self.commands.send(SpircCommand::SetPosition(position_ms))?)
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
                cluster_update = self.connect_state_update.next() => match cluster_update {
                    Some(result) => match result {
                        Ok(cluster_update) => {
                            if let Err(e) = self.handle_cluster_update(cluster_update).await {
                                error!("could not dispatch connect state update: {}", e);
                            }
                        },
                        Err(e) => error!("could not parse connect state update: {}", e),
                    }
                    None => {
                        error!("connect state update selected, but none received");
                        break;
                    }
                },
                volume_update = self.connect_state_volume_update.next() => match volume_update {
                    Some(result) => match result {
                        Ok(volume_update) => self.handle_set_volume(volume_update).await,
                        Err(e) => error!("could not parse set volume update request: {}", e),
                    }
                    None => {
                        error!("volume update selected, but none received");
                        break;
                    }
                },
                connect_state_command = self.connect_state_command.next() => match connect_state_command {
                    Some(request) => if let Err(e) = self.handle_connect_state_command(request).await {
                        error!("could handle connect state command: {}", e);
                    },
                    None => {
                        error!("connect state command selected, but none received");
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
                        Ok(connection_id) => self.handle_connection_id_update(connection_id).await,
                        Err(e) => error!("could not parse connection ID update: {}", e),
                    }
                    None => {
                        error!("connection ID update selected, but none received");
                        break;
                    }
                },
                cmd = async { commands?.recv().await }, if commands.is_some() => if let Some(cmd) = cmd {
                    if let Err(e) = self.handle_command(cmd).await {
                        debug!("could not dispatch command: {}", e);
                    }
                },
                event = async { player_events?.recv().await }, if player_events.is_some() => if let Some(event) = event {
                    if let Err(e) = self.handle_player_event(event).await {
                        error!("could not dispatch player event: {}", e);
                    }
                },
                context_uri = async { self.resolve_context.take() }, if self.resolve_context.is_some() => {
                    let context_uri = context_uri.unwrap(); // guaranteed above
                    if context_uri.contains("spotify:show:") || context_uri.contains("spotify:episode:") {
                        continue; // not supported by apollo stations
                    }

                    let context = if context_uri.starts_with("hm://") {
                        self.session.spclient().get_next_page(&context_uri).await
                    } else {
                        let previous_tracks = self
                            .connect_state
                            .player.prev_tracks
                            .iter()
                            .map(SpotifyId::try_from)
                            .filter_map(Result::ok)
                            .collect();

                        let scope = if self.autoplay_context {
                            "stations" // this returns a `StationContext` but we deserialize it into a `PageContext`
                        } else {
                            "tracks" // this returns a `PageContext`
                        };

                        self.session.spclient().get_apollo_station(scope, &context_uri, None, previous_tracks, self.autoplay_context).await
                    };

                    match context {
                        Ok(value) => {
                            let context = match serde_json::from_slice::<PageContext>(&value) {
                                Ok(context) => {
                                    info!(
                                        "Resolved {:?} tracks from <{:?}>",
                                        context.tracks.len(),
                                        self.connect_state.player.context_uri,
                                    );
                                    Some(context.into())
                                }
                                Err(e) => {
                                    error!("Unable to parse JSONContext {:?}", e);
                                    None
                                }
                            };
                            self.connect_state.update_context(context)
                        },
                        Err(err) => {
                            error!("ContextError: {:?}", err)
                        }
                    }
                },
                else => break
            }
        }

        self.session.dealer().close().await;
    }

    fn now_ms(&self) -> i64 {
        let dur = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|err| err.duration());

        dur.as_millis() as i64 + 1000 * self.session.time_delta()
    }

    fn update_state_position(&mut self, position_ms: u32) {
        self.connect_state.player.position_as_of_timestamp = position_ms.into();
        self.connect_state.player.timestamp = self.now_ms();
    }

    async fn handle_command(&mut self, cmd: SpircCommand) -> Result<(), Error> {
        if matches!(cmd, SpircCommand::Shutdown) {
            trace!("Received SpircCommand::Shutdown");
            self.handle_disconnect().await?;
            self.shutdown = true;
            if let Some(rx) = self.commands.as_mut() {
                rx.close()
            }
            Ok(())
        } else if self.connect_state.active {
            trace!("Received SpircCommand::{:?}", cmd);
            match cmd {
                SpircCommand::Play => {
                    self.handle_play();
                    self.notify().await
                }
                SpircCommand::PlayPause => {
                    self.handle_play_pause();
                    self.notify().await
                }
                SpircCommand::Pause => {
                    self.handle_pause();
                    self.notify().await
                }
                SpircCommand::Prev => {
                    self.handle_prev()?;
                    self.notify().await
                }
                SpircCommand::Next => {
                    self.handle_next()?;
                    self.notify().await
                }
                SpircCommand::VolumeUp => {
                    self.handle_volume_up();
                    self.notify().await
                }
                SpircCommand::VolumeDown => {
                    self.handle_volume_down();
                    self.notify().await
                }
                SpircCommand::Disconnect => {
                    self.handle_disconnect().await?;
                    self.notify().await
                }
                SpircCommand::Shuffle(shuffle) => {
                    self.connect_state.set_shuffle(shuffle)?;
                    self.notify().await
                }
                SpircCommand::Repeat(repeat) => {
                    self.connect_state.set_repeat_context(repeat);
                    self.notify().await
                }
                SpircCommand::RepeatTrack(repeat) => {
                    self.connect_state.set_repeat_track(repeat);
                    self.notify().await
                }
                SpircCommand::SetPosition(position) => {
                    self.handle_seek(position);
                    self.notify().await
                }
                SpircCommand::SetVolume(volume) => {
                    self.set_volume(volume);
                    self.notify().await
                }
                SpircCommand::Load(command) => {
                    self.handle_load(command).await?;
                    self.notify().await
                }
                _ => Ok(()),
            }
        } else {
            match cmd {
                SpircCommand::Activate => {
                    trace!("Received SpircCommand::{:?}", cmd);
                    self.handle_activate();
                    self.notify().await
                }
                _ => {
                    warn!("SpircCommand::{:?} will be ignored while Not Active", cmd);
                    Ok(())
                }
            }
        }
    }

    async fn handle_player_event(&mut self, event: PlayerEvent) -> Result<(), Error> {
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
                    PlayerEvent::EndOfTrack { .. } => self.handle_end_of_track().await,
                    PlayerEvent::Loading { .. } => {
                        match self.play_status {
                            SpircPlayStatus::LoadingPlay { position_ms } => {
                                self.update_state_position(position_ms);
                                trace!("==> kPlayStatusPlay");
                            }
                            SpircPlayStatus::LoadingPause { position_ms } => {
                                self.update_state_position(position_ms);
                                trace!("==> kPlayStatusPause");
                            }
                            _ => {
                                self.update_state_position(0);
                                trace!("==> kPlayStatusLoading");
                            }
                        }
                        self.notify().await
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
                                    self.notify().await
                                } else {
                                    Ok(())
                                }
                            }
                            SpircPlayStatus::LoadingPlay { .. }
                            | SpircPlayStatus::LoadingPause { .. } => {
                                self.update_state_position(position_ms);
                                self.play_status = SpircPlayStatus::Playing {
                                    nominal_start_time: new_nominal_start_time,
                                    preloading_of_next_track_triggered: false,
                                };
                                self.notify().await
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
                                self.update_state_position(new_position_ms);
                                self.play_status = SpircPlayStatus::Paused {
                                    position_ms: new_position_ms,
                                    preloading_of_next_track_triggered: false,
                                };
                                self.notify().await
                            }
                            SpircPlayStatus::LoadingPlay { .. }
                            | SpircPlayStatus::LoadingPause { .. } => {
                                self.update_state_position(new_position_ms);
                                self.play_status = SpircPlayStatus::Paused {
                                    position_ms: new_position_ms,
                                    preloading_of_next_track_triggered: false,
                                };
                                self.notify().await
                            }
                            _ => Ok(()),
                        }
                    }
                    PlayerEvent::Stopped { .. } => {
                        trace!("==> kPlayStatusStop");
                        match self.play_status {
                            SpircPlayStatus::Stopped => Ok(()),
                            _ => {
                                self.play_status = SpircPlayStatus::Stopped;
                                self.notify().await
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

    async fn handle_connection_id_update(&mut self, connection_id: String) {
        trace!("Received connection ID update: {:?}", connection_id);
        self.session.set_connection_id(&connection_id);

        let response = match self
            .connect_state
            .update_state(&self.session, PutStateReason::NEW_DEVICE)
            .await
        {
            Ok(res) => Cluster::parse_from_bytes(&res).ok(),
            Err(why) => {
                error!("{why:?}");
                None
            }
        };

        if let Some(mut cluster) = response {
            debug!(
                "successfully put connect state for {} with connection-id {connection_id}",
                self.session.device_id()
            );
            info!("active device is {:?}", cluster.active_device_id);

            if let Some(player_state) = cluster.player_state.take() {
                self.connect_state.player = player_state;
            } else {
                warn!("couldn't take player state from cluster")
            }
        }
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

    async fn handle_cluster_update(
        &mut self,
        mut cluster_update: ClusterUpdate,
    ) -> Result<(), Error> {
        let reason = cluster_update.update_reason.enum_value_or_default();

        let device_ids = cluster_update.devices_that_changed.join(", ");
        let devices = cluster_update.cluster.device.len();

        let prev_tracks = cluster_update.cluster.player_state.prev_tracks.len();
        let next_tracks = cluster_update.cluster.player_state.next_tracks.len();

        info!("cluster update! {reason:?} for {device_ids} from {devices} has {prev_tracks:?} previous tracks and {next_tracks} next tracks");

        let state = &mut self.connect_state;

        if let Some(mut cluster) = cluster_update.cluster.take() {
            if let Some(player_state) = cluster.player_state.take() {
                state.player = player_state;
            }

            let became_inactive =
                self.connect_state.active && cluster.active_device_id != self.session.device_id();
            if became_inactive {
                info!("device became inactive");
                self.handle_stop();
                self.connect_state.reset();
                let _ = self
                    .connect_state
                    .update_state(&self.session, PutStateReason::BECAME_INACTIVE)
                    .await?;
            }
        }

        Ok(())
    }

    async fn handle_set_volume(&mut self, set_volume_command: SetVolumeCommand) {

        let volume_difference = set_volume_command.volume - self.connect_state.device.volume as i32;
        if volume_difference < self.connect_state.device.capabilities.volume_steps {
            return;
        }

        self.set_volume(set_volume_command.volume as u16);
        if let Err(why) = self.notify().await {
            error!("couldn't notify after updating the volume: {why}")
        }
    }

    async fn handle_connect_state_command(
        &mut self,
        (request, sender): RequestReply,
    ) -> Result<(), Error> {
        self.connect_state.last_command = Some(request.clone());

        debug!(
            "handling: '{}' from {}",
            request.command, request.sent_by_device_id
        );

        let response = match request.command {
            RequestCommand::Transfer(transfer) if transfer.data.is_some() => {
                self.handle_transfer(transfer.data.expect("by condition checked"))
                    .await?;
                self.notify().await?;

                Reply::Success
            }
            RequestCommand::Transfer(_) => {
                warn!("transfer endpoint didn't contain any data to transfer");
                Reply::Failure
            }
            RequestCommand::Play(play) => {
                self.handle_load(SpircLoadCommand {
                    context_uri: play.context.uri,
                    start_playing: true,
                    playing_track_index: play.options.skip_to.track_index,
                    shuffle: false,
                    repeat: false,
                    repeat_track: false,
                })
                .await?;
                self.notify().await.map(|_| Reply::Success)?
            }
            RequestCommand::Pause(_) => {
                self.handle_pause();
                self.notify().await.map(|_| Reply::Success)?
            }
            RequestCommand::SeekTo(seek_to) => {
                self.handle_seek(seek_to.position);
                self.notify().await.map(|_| Reply::Success)?
            }
            RequestCommand::SetShufflingContext(shuffle) => {
                self.connect_state.set_shuffle(shuffle.value)?;
                self.notify().await.map(|_| Reply::Success)?
            }
            RequestCommand::SkipNext(_) => {
                self.handle_next()?;
                self.notify().await.map(|_| Reply::Success)?
            }
            RequestCommand::SkipPrev(_) => {
                self.handle_prev()?;
                self.notify().await.map(|_| Reply::Success)?
            }
            RequestCommand::Resume(_) => {
                self.handle_play();
                self.notify().await.map(|_| Reply::Success)?
            }
            RequestCommand::Unknown(unknown) => {
                warn!("unknown request command: {unknown}");
                // we just don't handle the command, by that we don't lose our connect state
                Reply::Success
            }
        };

        sender.send(response).map_err(Into::into)
    }

    async fn handle_transfer(&mut self, mut transfer: TransferState) -> Result<(), Error> {
        if let Some(session) = transfer.current_session.as_mut() {
            if let Some(ctx) = session.context.as_mut() {
                self.resolve_context(ctx).await?;
                self.connect_state.update_context(ctx.pages.pop())
            }
        }

        let timestamp = self.now_ms();
        let state = &mut self.connect_state;

        state.set_active(true);
        state.player.is_buffering = false;

        state.player.options = transfer.options;
        state.player.is_paused = transfer.playback.is_paused;
        state.player.is_playing = !transfer.playback.is_paused;

        if transfer.playback.playback_speed != 0. {
            state.player.playback_speed = transfer.playback.playback_speed
        } else {
            state.player.playback_speed = 1.;
        }

        state.player.play_origin = transfer.current_session.play_origin.clone();
        state.player.context_uri = transfer.current_session.context.uri.clone();
        state.player.context_url = transfer.current_session.context.url.clone();
        state.player.context_restrictions = transfer.current_session.context.restrictions.clone();
        state.player.suppressions = transfer.current_session.suppressions.clone();

        for (key, value) in &transfer.current_session.context.metadata {
            state
                .player
                .context_metadata
                .insert(key.clone(), value.clone());
        }

        if let Some((context, _)) = &state.context {
            for (key, value) in &context.metadata {
                state
                    .player
                    .context_metadata
                    .insert(key.clone(), value.clone());
            }
        }

        if state.player.track.is_none() {
            // todo: now we need to resolve this stuff, we can ignore this to some degree for now if we come from an already running context
            todo!("resolving player_state required")
        }

        // update position if the track continued playing
        let position = if transfer.playback.is_paused {
            state.player.position_as_of_timestamp
        } else {
            let time_since_position_update = timestamp - state.player.timestamp;
            state.player.position_as_of_timestamp + time_since_position_update
        };

        if self.connect_state.player.options.shuffling_context {
            self.connect_state.set_shuffle(true)?;
        }

        self.load_track(self.connect_state.player.is_playing, position.try_into()?)
    }

    async fn resolve_context(&mut self, ctx: &mut Context) -> Result<(), Error> {
        if ctx.uri.starts_with("spotify:local-files") {
            return Err(SpircError::UnsupportedLocalPlayBack.into());
        }

        if !ctx.pages.is_empty() {
            debug!("context already contains pages to use");
            return Ok(());
        }

        debug!("context didn't had any tracks, resolving tracks...");
        let resolved_ctx = self.session.spclient().get_context(&ctx.uri).await?;

        debug!(
            "context was resolved {} pages and {} tracks",
            resolved_ctx.pages.len(),
            resolved_ctx
                .pages
                .iter()
                .map(|p| p.tracks.len())
                .sum::<usize>()
        );

        ctx.pages = resolved_ctx.pages;
        ctx.metadata = resolved_ctx.metadata;
        ctx.restrictions = resolved_ctx.restrictions;
        ctx.loading = resolved_ctx.loading;
        ctx.special_fields = resolved_ctx.special_fields;

        Ok(())
    }

    async fn handle_disconnect(&mut self) -> Result<(), Error> {
        self.connect_state.set_active(false);
        self.notify().await?;
        self.handle_stop();

        self.player
            .emit_session_disconnected_event(self.session.connection_id(), self.session.username());

        Ok(())
    }

    fn handle_stop(&mut self) {
        self.player.stop();
    }

    fn handle_activate(&mut self) {
        self.connect_state.set_active(true);
        self.player
            .emit_session_connected_event(self.session.connection_id(), self.session.username());
        self.player.emit_session_client_changed_event(
            self.session.client_id(),
            self.session.client_name(),
            self.session.client_brand_name(),
            self.session.client_model_name(),
        );

        self.player
            .emit_volume_changed_event(self.connect_state.device.volume as u16);

        self.player
            .emit_auto_play_changed_event(self.session.autoplay());

        self.player
            .emit_filter_explicit_content_changed_event(self.session.filter_explicit_content());

        let options = &self.connect_state.player.options;
        self.player
            .emit_shuffle_changed_event(options.shuffling_context);

        self.player
            .emit_repeat_changed_event(options.repeating_context, options.repeating_track);
    }

    async fn handle_load(&mut self, cmd: SpircLoadCommand) -> Result<(), Error> {
        if !self.connect_state.active {
            self.handle_activate();
        }

        let mut ctx = Context {
            uri: cmd.context_uri,
            ..Default::default()
        };

        self.resolve_context(&mut ctx).await?;
        self.connect_state.update_context(ctx.pages.pop());
        self.connect_state
            .reset_playback_context(Some(cmd.playing_track_index as usize))?;

        self.connect_state.set_shuffle(cmd.shuffle)?;
        self.connect_state.set_repeat_context(cmd.repeat);
        self.connect_state.set_repeat_track(cmd.repeat_track);

        if !self.connect_state.player.next_tracks.is_empty() {
            self.load_track(self.connect_state.player.is_playing, 0)?;
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

    fn preview_next_track(&mut self) -> Option<SpotifyId> {
        let next = self.connect_state.player.next_tracks.first()?;
        SpotifyId::try_from(next).ok()
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
        self.connect_state.mark_all_as_unavailable(track_id);
        self.handle_preload_next_track();
    }

    fn handle_next(&mut self) -> Result<(), Error> {
        let context_uri = self.connect_state.player.context_uri.to_owned();
        let mut continue_playing = self.connect_state.player.is_playing;

        let new_track_index = match self.connect_state.move_to_next_track() {
            Ok(index) => Some(index),
            Err(ConnectStateError::NoNextTrack) => None,
            Err(why) => return Err(why.into()),
        };

        let (ctx, ctx_index) = self
            .connect_state
            .context
            .as_ref()
            .ok_or(ConnectStateError::NoContext)?;
        let context_length = ctx.tracks.len() as u32;
        let context_index = ctx_index.track;

        let update_tracks =
            self.autoplay_context && context_length - context_index < CONTEXT_FETCH_THRESHOLD;

        debug!(
            "At context track {:?} of {:?} <{:?}> update [{}]",
            context_index + 1,
            context_length,
            context_uri,
            update_tracks,
        );

        // When in autoplay, keep topping up the playlist when it nears the end
        if update_tracks {
            if let Some((ctx, _)) = self.connect_state.context.as_ref() {
                self.resolve_context = Some(ctx.next_page_url.to_owned());
            }
            todo!("update tracks from context: preloading");
        }

        // When not in autoplay, either start autoplay or loop back to the start
        if matches!(new_track_index, Some(i) if i >= context_length) || new_track_index.is_none() {
            // for some contexts there is no autoplay, such as shows and episodes
            // in such cases there is no context in librespot.
            if self.connect_state.context.is_some() && self.session.autoplay() {
                // Extend the playlist
                debug!("Starting autoplay for <{}>", context_uri);
                // force reloading the current context with an autoplay context
                self.autoplay_context = true;
                self.resolve_context = Some(context_uri);
                todo!("update tracks from context: autoplay");
                self.player.set_auto_normalise_as_album(false);
            } else {
                if self.connect_state.player.options.shuffling_context {
                    self.connect_state.shuffle()?
                } else {
                    self.connect_state.reset_playback_context(None)?;
                }

                continue_playing &= self.connect_state.player.options.repeating_context;
                debug!(
                    "Looping back to start, repeating_context is {}",
                    continue_playing
                );
            }
        }

        if context_length > 0 {
            self.load_track(continue_playing, 0)
        } else {
            info!("Not playing next track because there are no more tracks left in queue.");
            self.connect_state.reset_playback_context(None)?;
            self.handle_stop();
            Ok(())
        }
    }

    fn handle_prev(&mut self) -> Result<(), Error> {
        // Previous behaves differently based on the position
        // Under 3s it goes to the previous song (starts playing)
        // Over 3s it seeks to zero (retains previous play status)
        if self.position() < 3000 {
            let new_track_index = match self.connect_state.move_to_prev_track() {
                Ok(index) => Some(index),
                Err(ConnectStateError::NoPrevTrack) => None,
                Err(why) => return Err(why.into()),
            };

            if new_track_index.is_none() && self.connect_state.player.options.repeating_context {
                self.connect_state.reset_playback_context(None)?
            }

            self.load_track(self.connect_state.player.is_playing, 0)
        } else {
            self.handle_seek(0);
            Ok(())
        }
    }

    fn handle_volume_up(&mut self) {
        let volume = (self.connect_state.device.volume as u16).saturating_add(VOLUME_STEP_SIZE);
        self.set_volume(volume);
    }

    fn handle_volume_down(&mut self) {
        let volume = (self.connect_state.device.volume as u16).saturating_sub(VOLUME_STEP_SIZE);
        self.set_volume(volume);
    }

    async fn handle_end_of_track(&mut self) -> Result<(), Error> {
        self.handle_next()?;
        self.notify().await
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

    fn load_track(&mut self, start_playing: bool, position_ms: u32) -> Result<(), Error> {
        let track_to_load = match self.connect_state.player.track.as_ref() {
            None => {
                self.handle_stop();
                return Ok(());
            }
            Some(track) => track,
        };

        let id = SpotifyId::try_from(track_to_load)?;
        self.player.load(id, start_playing, position_ms);

        self.update_state_position(position_ms);
        if start_playing {
            self.play_status = SpircPlayStatus::LoadingPlay { position_ms };
        } else {
            self.play_status = SpircPlayStatus::LoadingPause { position_ms };
        }
        self.connect_state.set_status(&self.play_status);

        Ok(())
    }

    async fn notify(&mut self) -> Result<(), Error> {
        self.connect_state.set_status(&self.play_status);
        self.connect_state
            .update_state(&self.session, PutStateReason::PLAYER_STATE_CHANGED)
            .await
            .map(|_| ())
    }

    fn set_volume(&mut self, volume: u16) {
        let old_volume = self.connect_state.device.volume;
        let new_volume = volume as u32;
        if old_volume != new_volume {
            self.connect_state.device.volume = new_volume;
            self.mixer.set_volume(volume);
            if let Some(cache) = self.session.cache() {
                cache.save_volume(volume)
            }
            if self.connect_state.active {
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
