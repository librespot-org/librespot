use crate::state::context::ContextType;
use crate::state::provider::IsProvider;
use crate::state::{ConnectState, ConnectStateConfig};
use crate::{
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
use librespot_core::dealer::protocol::{PayloadValue, RequestCommand, SkipTo};
use librespot_protocol::autoplay_context_request::AutoplayContextRequest;
use librespot_protocol::connect::{Cluster, ClusterUpdate, PutStateReason, SetVolumeCommand};
use librespot_protocol::player::{Context, TransferState};
use protobuf::{Message, MessageField};
use std::{
    future::Future,
    pin::Pin,
    sync::atomic::{AtomicUsize, Ordering},
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use thiserror::Error;
use tokio::{sync::mpsc, time::sleep};
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
    #[error("tried resolving not allowed context: {0:?}")]
    NotAllowedContext(ResolveContext),
}

impl From<SpircError> for Error {
    fn from(err: SpircError) -> Self {
        use SpircError::*;
        match err {
            NoData | UnsupportedLocalPlayBack | UnexpectedData(_) | NotAllowedContext(_) => {
                Error::unavailable(err)
            }
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

#[derive(Debug)]
pub struct ResolveContext {
    uri: String,
    autoplay: bool,
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
    resolve_context: Vec<ResolveContext>,
    // is set when we receive a transfer state and are loading the context asynchronously
    pub transfer_state: Option<TransferState>,

    update_volume: bool,

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
    pub playing_track: PlayingTrack,
}

#[derive(Debug)]
pub enum PlayingTrack {
    Index(u32),
    Uri(String),
    Uid(String),
}

impl From<SkipTo> for PlayingTrack {
    fn from(value: SkipTo) -> Self {
        // order is important as it seems that the index can be 0,
        // but there might still be a uid or uri provided, so we try the index as last resort
        if let Some(uri) = value.track_uri {
            PlayingTrack::Uri(uri)
        } else if let Some(uid) = value.track_uid {
            PlayingTrack::Uid(uid)
        } else {
            PlayingTrack::Index(value.track_index.unwrap_or_else(|| {
                warn!("SkipTo didn't provided any point to skip to, falling back to index 0");
                0
            }))
        }
    }
}

const CONTEXT_FETCH_THRESHOLD: usize = 2;

const VOLUME_STEP_SIZE: u16 = 1024; // (u16::MAX + 1) / VOLUME_STEPS
const VOLUME_UPDATE_DELAY: Duration = Duration::from_secs(2);

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

        // pre-acquire client_token, preventing multiple request while running
        let _ = session.spclient().client_token().await?;

        // Connect *after* all message listeners are registered
        session.connect(credentials, true).await?;

        // pre-acquire access_token (we need to be authenticated to retrieve a token)
        let _ = session.login5().auth_token().await?;

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

            resolve_context: Vec::new(),
            transfer_state: None,
            update_volume: false,

            spirc_id,
        };

        let spirc = Spirc { commands: cmd_tx };
        task.set_volume(initial_volume as u16 - 1);
        task.update_volume = false;

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
        if let Err(why) = self.session.dealer().start().await {
            error!("starting dealer failed: {why}");
            return;
        }

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
                        Ok(volume_update) => match volume_update.volume.try_into() {
                            Ok(volume) => self.set_volume(volume),
                            Err(why) => error!("can't update volume, failed to parse i32 to u16: {why}")
                        },
                        Err(e) => error!("could not parse set volume update request: {}", e),
                    }
                    None => {
                        error!("volume update selected, but none received");
                        break;
                    }
                },
                connect_state_command = self.connect_state_command.next() => match connect_state_command {
                    Some(request) => if let Err(e) = self.handle_connect_state_command(request).await {
                        error!("couldn't handle connect state command: {}", e);
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
                _ = async {}, if !self.resolve_context.is_empty() => {
                    if let Err(why) = self.handle_resolve_context().await {
                        error!("ContextError: {why}")
                    }
                },
                _ = async { sleep(VOLUME_UPDATE_DELAY).await }, if self.update_volume => {
                    self.update_volume = false;

                    // for some reason the web-player does need two separate updates, so that the
                    // position of the current track is retained, other clients also send a state
                    // update before they send the volume update
                    if let Err(why) = self.notify().await {
                        error!("error updating connect state for volume update: {why}")
                    }

                    info!("delayed volume update for all devices: volume is now {}", self.connect_state.device.volume);
                    if let Err(why) = self.connect_state.update_state(&self.session, PutStateReason::VOLUME_CHANGED).await {
                        error!("error updating connect state for volume update: {why}")
                    }
                },
                else => break
            }
        }

        if !self.shutdown {
            if let Err(why) = self.notify().await {
                warn!("notify before unexpected shutdown couldn't be send: {why}")
            }
        }

        self.session.dealer().close().await;
    }

    async fn handle_resolve_context(&mut self) -> Result<(), Error> {
        while let Some(resolve) = self.resolve_context.pop() {
            self.resolve_context(resolve.uri, resolve.autoplay).await?;
        }

        if let Some(transfer_state) = self.transfer_state.take() {
            self.connect_state
                .setup_state_from_transfer(transfer_state)?
        }

        self.connect_state.fill_up_next_tracks()?;
        self.connect_state.update_restrictions();
        self.connect_state.update_queue_revision();

        self.preload_autoplay_when_required(self.connect_state.context_uri().clone());

        self.notify().await
    }

    async fn resolve_context(&mut self, context_uri: String, autoplay: bool) -> Result<(), Error> {
        if !autoplay {
            match self.session.spclient().get_context(&context_uri).await {
                Err(why) => error!("failed to resolve context '{context_uri}': {why}"),
                Ok(ctx) => self.connect_state.update_context(ctx)?,
            };
            if let Err(why) = self.notify().await {
                error!("failed to update connect state, after updating the context: {why}")
            }
            return Ok(());
        }

        if context_uri.contains("spotify:show:") || context_uri.contains("spotify:episode:") {
            // autoplay is not supported for podcasts
            return Err(SpircError::NotAllowedContext(ResolveContext {
                uri: context_uri,
                autoplay: true,
            })
            .into());
        }

        let previous_tracks = self.connect_state.prev_autoplay_track_uris();

        debug!(
            "loading autoplay context {context_uri} with {} previous tracks",
            previous_tracks.len()
        );

        let ctx_request = AutoplayContextRequest {
            context_uri: Some(context_uri.to_string()),
            recent_track_uri: previous_tracks,
            ..Default::default()
        };

        let context = self
            .session
            .spclient()
            .get_autoplay_context(&ctx_request)
            .await?;

        self.connect_state.update_autoplay_context(context)
    }

    fn now_ms(&self) -> i64 {
        let dur = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|err| err.duration());

        dur.as_millis() as i64 + 1000 * self.session.time_delta()
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
                    self.handle_next(None)?;
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
                    self.connect_state.handle_shuffle(shuffle)?;
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
                    self.handle_load(command, None).await?;
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
        if let PlayerEvent::TrackChanged { audio_item } = event {
            self.connect_state.update_duration(audio_item.duration_ms);
            return Ok(());
        }

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
                                self.connect_state
                                    .update_position(position_ms, self.now_ms());
                                trace!("==> kPlayStatusPlay");
                            }
                            SpircPlayStatus::LoadingPause { position_ms } => {
                                self.connect_state
                                    .update_position(position_ms, self.now_ms());
                                trace!("==> kPlayStatusPause");
                            }
                            _ => {
                                self.connect_state.update_position(0, self.now_ms());
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
                                    self.connect_state
                                        .update_position(position_ms, self.now_ms());
                                    self.notify().await
                                } else {
                                    Ok(())
                                }
                            }
                            SpircPlayStatus::LoadingPlay { .. }
                            | SpircPlayStatus::LoadingPause { .. } => {
                                self.connect_state
                                    .update_position(position_ms, self.now_ms());
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
                                self.connect_state
                                    .update_position(new_position_ms, self.now_ms());
                                self.play_status = SpircPlayStatus::Paused {
                                    position_ms: new_position_ms,
                                    preloading_of_next_track_triggered: false,
                                };
                                self.notify().await
                            }
                            SpircPlayStatus::LoadingPlay { .. }
                            | SpircPlayStatus::LoadingPause { .. } => {
                                self.connect_state
                                    .update_position(new_position_ms, self.now_ms());
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
                        self.handle_unavailable(track_id)?;
                        if self.connect_state.current_track(|t| &t.uri) == &track_id.to_uri()? {
                            self.handle_next(None)?;
                        }
                        self.notify().await
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

        // todo: handle received pages from transfer, important to not always shuffle the first 10 pages
        //  also important when the dealer is restarted, currently we shuffle again, index should be changed...
        //  maybe lookup current track of actual player
        if let Some(cluster) = response {
            if !cluster.transfer_data.is_empty() {
                if let Ok(transfer_state) = TransferState::parse_from_bytes(&cluster.transfer_data)
                {
                    if !transfer_state.current_session.context.pages.is_empty() {
                        info!("received transfer state with context, trying to take over control again");
                        match self.handle_transfer(transfer_state) {
                            Ok(_) => info!("successfully re-acquired control"),
                            Err(why) => error!("failed handling transfer state: {why}"),
                        }
                    }
                }
            }

            debug!(
                "successfully put connect state for {} with connection-id {connection_id}",
                self.session.device_id()
            );
            info!("active device is {:?}", cluster.active_device_id);
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
        let reason = cluster_update.update_reason.enum_value().ok();

        let device_ids = cluster_update.devices_that_changed.join(", ");
        debug!("cluster update: {reason:?} from {device_ids}");

        if let Some(cluster) = cluster_update.cluster.take() {
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
            } else if self.connect_state.active {
                // fixme: workaround fix, because of missing information why it behaves like it does
                //  background: when another device sends a connect-state update, some player's position de-syncs
                //  tried: providing session_id, playback_id, track-metadata "track_player"
                self.notify().await?;
            }
        }

        Ok(())
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
                self.handle_transfer(transfer.data.expect("by condition checked"))?;
                self.notify().await?;

                Reply::Success
            }
            RequestCommand::Transfer(_) => {
                warn!("transfer endpoint didn't contain any data to transfer");
                Reply::Failure
            }
            RequestCommand::Play(play) => {
                let shuffle = play
                    .options
                    .player_option_overrides
                    .as_ref()
                    .map(|o| o.shuffling_context)
                    .unwrap_or_else(|| self.connect_state.shuffling_context());
                let repeat = play
                    .options
                    .player_option_overrides
                    .as_ref()
                    .map(|o| o.repeating_context)
                    .unwrap_or_else(|| self.connect_state.repeat_context());
                let repeat_track = play
                    .options
                    .player_option_overrides
                    .as_ref()
                    .map(|o| o.repeating_track)
                    .unwrap_or_else(|| self.connect_state.repeat_track());

                self.handle_load(
                    SpircLoadCommand {
                        context_uri: play.context.uri.clone(),
                        start_playing: true,
                        playing_track: play.options.skip_to.into(),
                        shuffle,
                        repeat,
                        repeat_track,
                    },
                    Some(play.context),
                )
                .await?;

                self.connect_state.set_origin(play.play_origin);

                self.notify().await.map(|_| Reply::Success)?
            }
            RequestCommand::Pause(_) => {
                self.handle_pause();
                self.notify().await.map(|_| Reply::Success)?
            }
            RequestCommand::SeekTo(seek_to) => {
                // for some reason the position is stored in value, not in position
                trace!("seek to {seek_to:?}");
                self.handle_seek(seek_to.value);
                self.notify().await.map(|_| Reply::Success)?
            }
            RequestCommand::SetShufflingContext(shuffle) => {
                self.connect_state.handle_shuffle(shuffle.value)?;
                self.notify().await.map(|_| Reply::Success)?
            }
            RequestCommand::SetRepeatingContext(repeat_context) => {
                self.connect_state
                    .handle_set_repeat(Some(repeat_context.value), None)?;
                self.notify().await.map(|_| Reply::Success)?
            }
            RequestCommand::SetRepeatingTrack(repeat_track) => {
                self.connect_state
                    .handle_set_repeat(None, Some(repeat_track.value))?;
                self.notify().await.map(|_| Reply::Success)?
            }
            RequestCommand::AddToQueue(add_to_queue) => {
                self.connect_state.add_to_queue(add_to_queue.track, true);
                self.notify().await.map(|_| Reply::Success)?
            }
            RequestCommand::SetQueue(set_queue) => {
                self.connect_state.handle_set_queue(set_queue);
                self.notify().await.map(|_| Reply::Success)?
            }
            RequestCommand::SetOptions(set_options) => {
                let context = Some(set_options.repeating_context);
                let track = Some(set_options.repeating_track);
                self.connect_state.handle_set_repeat(context, track)?;
                self.notify().await.map(|_| Reply::Success)?
            }
            RequestCommand::SkipNext(skip_next) => {
                self.handle_next(skip_next.track.map(|t| t.uri))?;
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

    fn handle_transfer(&mut self, mut transfer: TransferState) -> Result<(), Error> {
        self.connect_state
            .reset_context(Some(&transfer.current_session.context.uri));

        let mut ctx_uri = transfer.current_session.context.uri.clone();
        let autoplay = ctx_uri.contains("station");

        if autoplay {
            ctx_uri = ctx_uri.replace("station:", "");
            self.connect_state.active_context = ContextType::Autoplay;
        }

        debug!("async resolve context for {}", ctx_uri);
        self.resolve_context.push(ResolveContext {
            autoplay: false,
            uri: ctx_uri.clone(),
        });

        let timestamp = self.now_ms();
        let state = &mut self.connect_state;

        state.set_active(true);
        state.handle_initial_transfer(&mut transfer);

        // update position if the track continued playing
        let position = if transfer.playback.is_paused {
            transfer.playback.position_as_of_timestamp.into()
        } else if transfer.playback.position_as_of_timestamp > 0 {
            let time_since_position_update = timestamp - transfer.playback.timestamp;
            i64::from(transfer.playback.position_as_of_timestamp) + time_since_position_update
        } else {
            0
        };

        let is_playing = !transfer.playback.is_paused;

        if self.connect_state.context.is_some() {
            self.connect_state.setup_state_from_transfer(transfer)?;
        } else {
            debug!("trying to find initial track");
            match self.connect_state.current_track_from_transfer(&transfer) {
                Err(why) => warn!("{why}"),
                Ok(track) => {
                    debug!("initial track found");
                    self.connect_state.set_track(track)
                }
            }

            if self.connect_state.autoplay_context.is_none()
                && (self.connect_state.current_track(|t| t.is_autoplay()) || autoplay)
            {
                debug!("currently in autoplay context, async resolving autoplay for {ctx_uri}");
                self.resolve_context.push(ResolveContext {
                    uri: ctx_uri,
                    autoplay: true,
                })
            }

            self.transfer_state = Some(transfer);
        }

        self.load_track(is_playing, position.try_into()?)
    }

    async fn handle_disconnect(&mut self) -> Result<(), Error> {
        self.handle_stop();

        self.play_status = SpircPlayStatus::Stopped {};
        self.connect_state
            .update_position_in_relation(self.now_ms());
        self.notify().await?;

        self.connect_state
            .update_state(&self.session, PutStateReason::BECAME_INACTIVE)
            .await?;

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

        self.player
            .emit_shuffle_changed_event(self.connect_state.shuffling_context());

        self.player.emit_repeat_changed_event(
            self.connect_state.repeat_context(),
            self.connect_state.repeat_track(),
        );
    }

    async fn handle_load(
        &mut self,
        cmd: SpircLoadCommand,
        context: Option<Context>,
    ) -> Result<(), Error> {
        self.connect_state.reset_context(Some(&cmd.context_uri));

        if !self.connect_state.active {
            self.handle_activate();
        }

        let current_context_uri = self.connect_state.context_uri();
        if current_context_uri == &cmd.context_uri && self.connect_state.context.is_some() {
            debug!("context <{current_context_uri}> didn't change, no resolving required",)
        } else {
            debug!("resolving context for load command");
            self.resolve_context(cmd.context_uri.clone(), false).await?;
        }

        // for play commands with skip by uid, the context of the command contains
        // tracks with uri and uid, so we merge the new context with the resolved/existing context
        self.connect_state.merge_context(context);
        self.connect_state.clear_next_tracks(false);

        let index = match cmd.playing_track {
            PlayingTrack::Index(i) => i as usize,
            PlayingTrack::Uri(uri) => {
                let ctx = self.connect_state.context.as_ref();
                ConnectState::find_index_in_context(ctx, |t| t.uri == uri)?
            }
            PlayingTrack::Uid(uid) => {
                let ctx = self.connect_state.context.as_ref();
                ConnectState::find_index_in_context(ctx, |t| t.uid == uid)?
            }
        };

        self.connect_state.set_shuffle(cmd.shuffle);
        if cmd.shuffle {
            self.connect_state.active_context = ContextType::Default;
            self.connect_state.set_current_track(index)?;
            self.connect_state.shuffle()?;
        } else {
            // set manually, so that we overwrite a possible current queue track
            self.connect_state.set_current_track(index)?;
            self.connect_state.reset_playback_context(Some(index))?;
        }

        self.connect_state.set_repeat_context(cmd.repeat);
        self.connect_state.set_repeat_track(cmd.repeat_track);

        if self.connect_state.current_track(MessageField::is_some) {
            self.load_track(cmd.start_playing, 0)?;
        } else {
            info!("No active track, stopping");
            self.handle_stop();
        }

        if !self.connect_state.has_next_tracks(None) && self.session.autoplay() {
            self.resolve_context.push(ResolveContext {
                uri: cmd.context_uri,
                autoplay: true,
            })
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
                self.connect_state
                    .update_position(position_ms, self.now_ms());
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
                self.connect_state
                    .update_position(position_ms, self.now_ms());
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
        self.connect_state
            .update_position(position_ms, self.now_ms());
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

        if let Some(track_id) = self.connect_state.preview_next_track() {
            self.player.preload(track_id);
        }
    }

    // Mark unavailable tracks so we can skip them later
    fn handle_unavailable(&mut self, track_id: SpotifyId) -> Result<(), Error> {
        self.connect_state.mark_unavailable(track_id)?;
        self.handle_preload_next_track();

        Ok(())
    }

    fn preload_autoplay_when_required(&mut self, uri: String) {
        let preload_autoplay = self
            .connect_state
            .has_next_tracks(Some(CONTEXT_FETCH_THRESHOLD))
            && self.session.autoplay();

        // When in autoplay, keep topping up the playlist when it nears the end
        if preload_autoplay {
            debug!("Preloading autoplay context for <{}>", uri);
            // resolve the next autoplay context
            self.resolve_context.push(ResolveContext {
                uri,
                autoplay: true,
            });
        }
    }

    fn is_playing(&self) -> bool {
        matches!(self.play_status, SpircPlayStatus::Playing { .. })
    }

    fn handle_next(&mut self, track_uri: Option<String>) -> Result<(), Error> {
        let continue_playing = self.is_playing();

        let current_uri = self.connect_state.current_track(|t| &t.uri);
        let mut has_next_track =
            matches!(track_uri, Some(ref track_uri) if current_uri == track_uri);

        if !has_next_track {
            has_next_track = loop {
                let index = self.connect_state.next_track()?;

                let current_uri = self.connect_state.current_track(|t| &t.uri);
                if matches!(track_uri, Some(ref track_uri) if current_uri != track_uri) {
                    continue;
                } else {
                    break index.is_some();
                }
            };
        };

        self.preload_autoplay_when_required(self.connect_state.context_uri().clone());

        if has_next_track {
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
            let new_track_index = self.connect_state.prev_track()?;

            if new_track_index.is_none() && self.connect_state.repeat_context() {
                self.connect_state.reset_playback_context(None)?
            }

            self.load_track(self.is_playing(), 0)
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
        let next_track = self
            .connect_state
            .repeat_track()
            .then(|| self.connect_state.current_track(|t| t.uri.clone()));

        self.handle_next(next_track)?;
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
        if self.connect_state.current_track(MessageField::is_none) {
            self.handle_stop();
            return Ok(());
        }

        let current_uri = self.connect_state.current_track(|t| &t.uri);
        let id = SpotifyId::from_uri(current_uri)?;
        self.player.load(id, start_playing, position_ms);

        self.connect_state
            .update_position(position_ms, self.now_ms());
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

        if self.is_playing() {
            self.connect_state
                .update_position_in_relation(self.now_ms());
        }

        self.connect_state
            .update_state(&self.session, PutStateReason::PLAYER_STATE_CHANGED)
            .await
            .map(|_| ())
    }

    fn set_volume(&mut self, volume: u16) {
        let old_volume = self.connect_state.device.volume;
        let new_volume = volume as u32;
        if old_volume != new_volume || self.mixer.volume() != volume {
            self.update_volume = true;

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
