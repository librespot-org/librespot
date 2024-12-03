pub use crate::model::{PlayingTrack, SpircLoadCommand};
use crate::state::{context::ResetContext, metadata::Metadata};
use crate::{
    core::{
        authentication::Credentials,
        dealer::{
            manager::{Reply, RequestReply},
            protocol::{Command, Message, Request},
        },
        session::UserAttributes,
        Error, Session, SpotifyId,
    },
    playback::{
        mixer::Mixer,
        player::{Player, PlayerEvent, PlayerEventChannel},
    },
    protocol::{
        autoplay_context_request::AutoplayContextRequest,
        connect::{Cluster, ClusterUpdate, LogoutCommand, SetVolumeCommand},
        explicit_content_pubsub::UserAttributesUpdate,
        player::{Context, TransferState},
        playlist4_external::PlaylistModificationInfo,
        social_connect_v2::{session::_host_active_device_id, SessionUpdate},
        user_attributes::UserAttributesMutation,
    },
};
use crate::{
    model::{ResolveContext, SpircPlayStatus},
    state::{
        context::{ContextType, LoadNext, UpdateContext},
        provider::IsProvider,
        {ConnectState, ConnectStateConfig},
    },
};
use futures_util::{Stream, StreamExt};
use protobuf::MessageField;
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
    #[error("message pushed for another URI")]
    InvalidUri(String),
    #[error("tried resolving not allowed context: {0:?}")]
    NotAllowedContext(String),
    #[error("failed to put connect state for new device")]
    FailedDealerSetup,
    #[error("unknown endpoint: {0:#?}")]
    UnknownEndpoint(serde_json::Value),
}

impl From<SpircError> for Error {
    fn from(err: SpircError) -> Self {
        use SpircError::*;
        match err {
            NoData | NotAllowedContext(_) => Error::unavailable(err),
            InvalidUri(_) | FailedDealerSetup => Error::aborted(err),
            UnknownEndpoint(_) => Error::unimplemented(err),
        }
    }
}

type BoxedStream<T> = Pin<Box<dyn Stream<Item = T> + Send>>;
type BoxedStreamResult<T> = BoxedStream<Result<T, Error>>;

struct SpircTask {
    player: Arc<Player>,
    mixer: Arc<dyn Mixer>,

    connect_state: ConnectState,

    play_request_id: Option<u64>,
    play_status: SpircPlayStatus,

    connection_id_update: BoxedStreamResult<String>,
    connect_state_update: BoxedStreamResult<ClusterUpdate>,
    connect_state_volume_update: BoxedStreamResult<SetVolumeCommand>,
    connect_state_logout_request: BoxedStreamResult<LogoutCommand>,
    playlist_update: BoxedStreamResult<PlaylistModificationInfo>,
    session_update: BoxedStreamResult<SessionUpdate>,
    connect_state_command: BoxedStream<RequestReply>,
    user_attributes_update: BoxedStreamResult<UserAttributesUpdate>,
    user_attributes_mutation: BoxedStreamResult<UserAttributesMutation>,

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
enum SpircCommand {
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

const CONTEXT_FETCH_THRESHOLD: usize = 2;

const VOLUME_STEP_SIZE: u16 = 1024; // (u16::MAX + 1) / VOLUME_STEPS

// delay to resolve a bundle of context updates, delaying the update prevents duplicate context updates of the same type
const RESOLVE_CONTEXT_DELAY: Duration = Duration::from_millis(500);
// delay to update volume after a certain amount of time, instead on each update request
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
                .map(Message::from_raw),
        );

        let connect_state_volume_update = Box::pin(
            session
                .dealer()
                .listen_for("hm://connect-state/v1/connect/volume")?
                .map(Message::from_raw),
        );

        let connect_state_logout_request = Box::pin(
            session
                .dealer()
                .listen_for("hm://connect-state/v1/connect/logout")?
                .map(Message::from_raw),
        );

        let playlist_update = Box::pin(
            session
                .dealer()
                .listen_for("hm://playlist/v2/playlist/")?
                .map(Message::from_raw),
        );

        let session_update = Box::pin(
            session
                .dealer()
                .listen_for("social-connect/v2/session_update")?
                .map(Message::from_json),
        );

        let connect_state_command = Box::pin(
            session
                .dealer()
                .handle_for("hm://connect-state/v1/player/command")
                .map(UnboundedReceiverStream::new)?,
        );

        let user_attributes_update = Box::pin(
            session
                .dealer()
                .listen_for("spotify:user:attributes:update")?
                .map(Message::from_raw),
        );

        // can be trigger by toggling autoplay in a desktop client
        let user_attributes_mutation = Box::pin(
            session
                .dealer()
                .listen_for("spotify:user:attributes:mutated")?
                .map(Message::from_raw),
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
            connect_state_logout_request,
            playlist_update,
            session_update,
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

        let initial_volume = task.connect_state.device_info().volume;
        task.connect_state.set_volume(0);

        match initial_volume.try_into() {
            Ok(volume) => {
                task.set_volume(volume);
                // we don't want to update the volume initially,
                // we just want to set the mixer to the correct volume
                task.update_volume = false;
            }
            Err(why) => error!("failed to update initial volume: {why}"),
        };

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
        // simplify unwrapping of received item or parsed result
        macro_rules! unwrap {
            ( $next:expr, |$some:ident| $use_some:expr ) => {
                match $next {
                    Some($some) => $use_some,
                    None => {
                        error!("{} selected, but none received", stringify!($next));
                        break;
                    }
                }
            };
            ( $next:expr, match |$ok:ident| $use_ok:expr ) => {
                unwrap!($next, |$ok| match $ok {
                    Ok($ok) => $use_ok,
                    Err(why) => error!("could not parse {}: {}", stringify!($ok), why),
                })
            };
        }

        if let Err(why) = self.session.dealer().start().await {
            error!("starting dealer failed: {why}");
            return;
        }

        while !self.session.is_invalid() && !self.shutdown {
            let commands = self.commands.as_mut();
            let player_events = self.player_events.as_mut();

            tokio::select! {
                // startup of the dealer requires a connection_id, which is retrieved at the very beginning
                connection_id_update = self.connection_id_update.next() => unwrap! {
                    connection_id_update,
                    match |connection_id| if let Err(why) = self.handle_connection_id_update(connection_id).await {
                        error!("failed handling connection id update: {why}");
                        break;
                    }
                },
                // main dealer update of any remote device updates
                cluster_update = self.connect_state_update.next() => unwrap! {
                    cluster_update,
                    match |cluster_update| if let Err(e) = self.handle_cluster_update(cluster_update).await {
                        error!("could not dispatch connect state update: {}", e);
                    }
                },
                // main dealer request handling (dealer expects an answer)
                request = self.connect_state_command.next() => unwrap! {
                    request,
                    |request| if let Err(e) = self.handle_connect_state_request(request).await {
                        error!("couldn't handle connect state command: {}", e);
                    }
                },
                // volume request handling is send separately (it's more like a fire forget)
                volume_update = self.connect_state_volume_update.next() => unwrap! {
                    volume_update,
                    match |volume_update| match volume_update.volume.try_into() {
                        Ok(volume) => self.set_volume(volume),
                        Err(why) => error!("can't update volume, failed to parse i32 to u16: {why}")
                    }
                },
                logout_request = self.connect_state_logout_request.next() => unwrap! {
                    logout_request,
                    |logout_request| {
                        error!("received logout request, currently not supported: {logout_request:#?}");
                        // todo: call logout handling
                    }
                },
                playlist_update = self.playlist_update.next() => unwrap! {
                    playlist_update,
                    match |playlist_update| if let Err(why) = self.handle_playlist_modification(playlist_update) {
                        error!("failed to handle playlist modification: {why}")
                    }
                },
                user_attributes_update = self.user_attributes_update.next() => unwrap! {
                    user_attributes_update,
                    match |attributes| self.handle_user_attributes_update(attributes)
                },
                user_attributes_mutation = self.user_attributes_mutation.next() => unwrap! {
                    user_attributes_mutation,
                    match |attributes| self.handle_user_attributes_mutation(attributes)
                },
                session_update = self.session_update.next() => unwrap! {
                    session_update,
                    match |session_update| self.handle_session_update(session_update)
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
                _ = async { sleep(RESOLVE_CONTEXT_DELAY).await }, if !self.resolve_context.is_empty() => {
                    if let Err(why) = self.handle_resolve_context().await {
                        error!("ContextError: {why}")
                    }
                },
                _ = async { sleep(VOLUME_UPDATE_DELAY).await }, if self.update_volume => {
                    self.update_volume = false;

                    info!("delayed volume update for all devices: volume is now {}", self.connect_state.device_info().volume);
                    if let Err(why) = self.connect_state.notify_volume_changed(&self.session).await {
                        error!("error updating connect state for volume update: {why}")
                    }

                    // for some reason the web-player does need two separate updates, so that the
                    // position of the current track is retained, other clients also send a state
                    // update before they send the volume update
                    if let Err(why) = self.notify().await {
                        error!("error updating connect state for volume update: {why}")
                    }
                },
                else => break
            }
        }

        if !self.shutdown && self.connect_state.is_active() {
            if let Err(why) = self.notify().await {
                warn!("notify before unexpected shutdown couldn't be send: {why}")
            }
        }

        // clears the session id, leaving an empty state
        if let Err(why) = self.session.spclient().delete_connect_state_request().await {
            warn!("deleting connect_state failed before unexpected shutdown: {why}")
        }
        self.session.dealer().close().await;
    }

    async fn handle_resolve_context(&mut self) -> Result<(), Error> {
        let mut last_resolve = None::<ResolveContext>;
        while let Some(resolve) = self.resolve_context.pop() {
            if matches!(last_resolve, Some(ref last_resolve) if last_resolve == &resolve) {
                debug!("did already update the context for {resolve}");
                continue;
            } else {
                last_resolve = Some(resolve.clone());

                let resolve_uri = match resolve.resolve_uri() {
                    Some(resolve) => resolve,
                    None => {
                        warn!("tried to resolve context without resolve_uri: {resolve}");
                        return Ok(());
                    }
                };

                debug!("resolving: {resolve}");
                // the autoplay endpoint can return a 404, when it tries to retrieve an
                // autoplay context for an empty playlist as it seems
                if let Err(why) = self
                    .resolve_context(
                        resolve_uri,
                        resolve.context_uri(),
                        resolve.autoplay(),
                        resolve.update(),
                    )
                    .await
                {
                    error!("failed resolving context <{resolve}>: {why}");
                    self.connect_state.reset_context(ResetContext::Completely);
                    self.handle_stop()
                }

                self.connect_state.merge_context(Some(resolve.into()));
            }
        }

        if let Some(transfer_state) = self.transfer_state.take() {
            self.connect_state.finish_transfer(transfer_state)?
        }

        if matches!(self.connect_state.active_context, ContextType::Default) {
            let ctx = self.connect_state.context.as_ref();
            if matches!(ctx, Some(ctx) if ctx.tracks.is_empty()) {
                self.connect_state.clear_next_tracks(true);
                self.handle_next(None)?;
            }
        }

        self.connect_state.fill_up_next_tracks()?;
        self.connect_state.update_restrictions();
        self.connect_state.update_queue_revision();

        self.preload_autoplay_when_required();

        self.notify().await
    }

    async fn resolve_context(
        &mut self,
        resolve_uri: &str,
        context_uri: &str,
        autoplay: bool,
        update: bool,
    ) -> Result<(), Error> {
        if !autoplay {
            let mut ctx = self.session.spclient().get_context(resolve_uri).await?;

            if update {
                ctx.uri = context_uri.to_string();
                ctx.url = format!("context://{context_uri}");

                self.connect_state
                    .update_context(ctx, UpdateContext::Default)?
            } else if matches!(ctx.pages.first(), Some(p) if !p.tracks.is_empty()) {
                debug!(
                    "update context from single page, context {} had {} pages",
                    ctx.uri,
                    ctx.pages.len()
                );
                self.connect_state
                    .fill_context_from_page(ctx.pages.remove(0))?;
            } else {
                error!("resolving context should only update the tracks, but had no page, or track. {ctx:#?}");
            };

            if let Err(why) = self.notify().await {
                error!("failed to update connect state, after updating the context: {why}")
            }

            return Ok(());
        }

        if resolve_uri.contains("spotify:show:") || resolve_uri.contains("spotify:episode:") {
            // autoplay is not supported for podcasts
            Err(SpircError::NotAllowedContext(resolve_uri.to_string()))?
        }

        let previous_tracks = self.connect_state.prev_autoplay_track_uris();

        debug!(
            "requesting autoplay context <{resolve_uri}> with {} previous tracks",
            previous_tracks.len()
        );

        let ctx_request = AutoplayContextRequest {
            context_uri: Some(resolve_uri.to_string()),
            recent_track_uri: previous_tracks,
            ..Default::default()
        };

        let context = self
            .session
            .spclient()
            .get_autoplay_context(&ctx_request)
            .await?;

        self.connect_state
            .update_context(context, UpdateContext::Autoplay)
    }

    // todo: time_delta still necessary?
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
        } else if self.connect_state.is_active() {
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

    async fn handle_connection_id_update(&mut self, connection_id: String) -> Result<(), Error> {
        trace!("Received connection ID update: {:?}", connection_id);
        self.session.set_connection_id(&connection_id);

        let cluster = match self
            .connect_state
            .notify_new_device_appeared(&self.session)
            .await
        {
            Ok(res) => Cluster::parse_from_bytes(&res).ok(),
            Err(why) => {
                error!("{why:?}");
                None
            }
        }
        .ok_or(SpircError::FailedDealerSetup)?;

        debug!(
            "successfully put connect state for {} with connection-id {connection_id}",
            self.session.device_id()
        );

        if !cluster.active_device_id.is_empty() || !cluster.player_state.session_id.is_empty() {
            info!(
                "active device is <{}> with session <{}>",
                cluster.active_device_id, cluster.player_state.session_id
            );
            return Ok(());
        } else if cluster.transfer_data.is_empty() {
            debug!("got empty transfer state, do nothing");
            return Ok(());
        } else {
            info!(
                "trying to take over control automatically, session_id: {}",
                cluster.player_state.session_id
            )
        }

        use protobuf::Message;

        // todo: handle received pages from transfer, important to not always shuffle the first 10 tracks
        //  also important when the dealer is restarted, currently we just shuffle again, but at least
        //  the 10 tracks provided should be used and after that the new shuffle context
        match TransferState::parse_from_bytes(&cluster.transfer_data) {
            Ok(transfer_state) => self.handle_transfer(transfer_state)?,
            Err(why) => error!("failed to take over control: {why}"),
        }

        Ok(())
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

                    self.preload_autoplay_when_required()
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
        let reason = cluster_update.update_reason.enum_value();

        let device_ids = cluster_update.devices_that_changed.join(", ");
        debug!(
            "cluster update: {reason:?} from {device_ids}, active device: {}",
            cluster_update.cluster.active_device_id
        );

        if let Some(cluster) = cluster_update.cluster.take() {
            let became_inactive = self.connect_state.is_active()
                && cluster.active_device_id != self.session.device_id();
            if became_inactive {
                info!("device became inactive");
                self.connect_state.became_inactive(&self.session).await?;
                self.handle_stop()
            } else if self.connect_state.is_active() {
                // fixme: workaround fix, because of missing information why it behaves like it does
                //  background: when another device sends a connect-state update, some player's position de-syncs
                //  tried: providing session_id, playback_id, track-metadata "track_player"
                self.notify().await?;
            }
        }

        Ok(())
    }

    async fn handle_connect_state_request(
        &mut self,
        (request, sender): RequestReply,
    ) -> Result<(), Error> {
        self.connect_state.set_last_command(request.clone());

        debug!(
            "handling: '{}' from {}",
            request.command, request.sent_by_device_id
        );

        let response = match self.handle_request(request).await {
            Ok(_) => Reply::Success,
            Err(why) => {
                error!("failed to handle request: {why}");
                Reply::Failure
            }
        };

        sender.send(response).map_err(Into::into)
    }

    async fn handle_request(&mut self, request: Request) -> Result<(), Error> {
        use Command::*;

        match request.command {
            // errors and unknown commands
            Transfer(transfer) if transfer.data.is_none() => {
                warn!("transfer endpoint didn't contain any data to transfer");
                Err(SpircError::NoData)?
            }
            Unknown(unknown) => Err(SpircError::UnknownEndpoint(unknown))?,
            // implicit update of the connect_state
            UpdateContext(update_context) => {
                if &update_context.context.uri != self.connect_state.context_uri() {
                    debug!(
                        "ignoring context update for <{}>, because it isn't the current context <{}>",
                        update_context.context.uri, self.connect_state.context_uri()
                    )
                } else {
                    self.resolve_context
                        .push(ResolveContext::from_context(update_context.context, false));
                }
                return Ok(());
            }
            // modification and update of the connect_state
            Transfer(transfer) => {
                self.handle_transfer(transfer.data.expect("by condition checked"))?
            }
            Play(play) => {
                let shuffle = play
                    .options
                    .player_options_override
                    .as_ref()
                    .map(|o| o.shuffling_context)
                    .unwrap_or_else(|| self.connect_state.shuffling_context());
                let repeat = play
                    .options
                    .player_options_override
                    .as_ref()
                    .map(|o| o.repeating_context)
                    .unwrap_or_else(|| self.connect_state.repeat_context());
                let repeat_track = play
                    .options
                    .player_options_override
                    .as_ref()
                    .map(|o| o.repeating_track)
                    .unwrap_or_else(|| self.connect_state.repeat_track());

                self.handle_load(
                    SpircLoadCommand {
                        context_uri: play.context.uri.clone(),
                        start_playing: true,
                        seek_to: play.options.seek_to.unwrap_or_default(),
                        playing_track: play.options.skip_to.into(),
                        shuffle,
                        repeat,
                        repeat_track,
                    },
                    Some(play.context),
                )
                .await?;

                self.connect_state.set_origin(play.play_origin)
            }
            Pause(_) => self.handle_pause(),
            SeekTo(seek_to) => {
                // for some reason the position is stored in value, not in position
                trace!("seek to {seek_to:?}");
                self.handle_seek(seek_to.value)
            }
            SetShufflingContext(shuffle) => self.connect_state.handle_shuffle(shuffle.value)?,
            SetRepeatingContext(repeat_context) => self
                .connect_state
                .handle_set_repeat(Some(repeat_context.value), None)?,
            SetRepeatingTrack(repeat_track) => self
                .connect_state
                .handle_set_repeat(None, Some(repeat_track.value))?,
            AddToQueue(add_to_queue) => self.connect_state.add_to_queue(add_to_queue.track, true),
            SetQueue(set_queue) => self.connect_state.handle_set_queue(set_queue),
            SetOptions(set_options) => {
                let context = set_options.repeating_context;
                let track = set_options.repeating_track;
                self.connect_state.handle_set_repeat(context, track)?;

                let shuffle = set_options.shuffling_context;
                if let Some(shuffle) = shuffle {
                    self.connect_state.handle_shuffle(shuffle)?;
                }
            }
            SkipNext(skip_next) => self.handle_next(skip_next.track.map(|t| t.uri))?,
            SkipPrev(_) => self.handle_prev()?,
            Resume(_) if matches!(self.play_status, SpircPlayStatus::Stopped) => {
                self.load_track(true, 0)?
            }
            Resume(_) => self.handle_play(),
        }

        self.notify().await
    }

    fn handle_transfer(&mut self, mut transfer: TransferState) -> Result<(), Error> {
        self.connect_state
            .reset_context(ResetContext::WhenDifferent(
                &transfer.current_session.context.uri,
            ));

        let mut ctx_uri = transfer.current_session.context.uri.clone();

        debug!("trying to find initial track");
        match self.connect_state.current_track_from_transfer(&transfer) {
            Err(why) => warn!("{why}"),
            Ok(track) => {
                debug!("found initial track");
                self.connect_state.set_track(track)
            }
        };

        let autoplay = self.connect_state.current_track(|t| t.is_from_autoplay());
        if autoplay {
            ctx_uri = ctx_uri.replace("station:", "");
        }

        let fallback = self.connect_state.current_track(|t| &t.uri).clone();

        debug!("async resolve context for <{}>", ctx_uri);
        self.resolve_context
            .push(ResolveContext::from_uri(ctx_uri.clone(), &fallback, false));

        let timestamp = self.now_ms();
        let state = &mut self.connect_state;

        state.set_active(true);
        state.handle_initial_transfer(&mut transfer, ctx_uri.clone());

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

        if self.connect_state.current_track(|t| t.is_autoplay()) || autoplay {
            debug!("currently in autoplay context, async resolving autoplay for {ctx_uri}");

            self.resolve_context
                .push(ResolveContext::from_uri(ctx_uri, fallback, true))
        }

        self.transfer_state = Some(transfer);

        self.load_track(is_playing, position.try_into()?)
    }

    async fn handle_disconnect(&mut self) -> Result<(), Error> {
        self.handle_stop();

        self.play_status = SpircPlayStatus::Stopped {};
        self.connect_state
            .update_position_in_relation(self.now_ms());
        self.notify().await?;

        self.connect_state.became_inactive(&self.session).await?;

        self.player
            .emit_session_disconnected_event(self.session.connection_id(), self.session.username());

        Ok(())
    }

    fn handle_stop(&mut self) {
        self.player.stop();
        self.connect_state.update_position(0, self.now_ms());
        self.connect_state.clear_next_tracks(true);

        if let Err(why) = self.connect_state.fill_up_next_tracks() {
            warn!("failed filling up next_track during stopping: {why}")
        }
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
            .emit_volume_changed_event(self.connect_state.device_info().volume as u16);

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
        self.connect_state
            .reset_context(ResetContext::WhenDifferent(&cmd.context_uri));

        if !self.connect_state.is_active() {
            self.handle_activate();
        }

        let current_context_uri = self.connect_state.context_uri();
        let fallback = if let Some(ref ctx) = context {
            match ConnectState::get_context_uri_from_context(ctx) {
                Some(ctx_uri) => ctx_uri,
                None => Err(SpircError::InvalidUri(cmd.context_uri.clone()))?,
            }
        } else {
            &cmd.context_uri
        }
        .clone();

        if current_context_uri == &cmd.context_uri && fallback == cmd.context_uri {
            debug!("context <{current_context_uri}> didn't change, no resolving required")
        } else {
            debug!("resolving context for load command");
            self.resolve_context(&fallback, &cmd.context_uri, false, true)
                .await?;
        }

        // for play commands with skip by uid, the context of the command contains
        // tracks with uri and uid, so we merge the new context with the resolved/existing context
        self.connect_state.merge_context(context);
        self.connect_state.clear_next_tracks(false);

        debug!("play track <{:?}>", cmd.playing_track);

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

        debug!(
            "loading with shuffle: <{}>, repeat track: <{}> context: <{}>",
            cmd.shuffle, cmd.repeat, cmd.repeat_track
        );

        self.connect_state.set_shuffle(cmd.shuffle);
        self.connect_state.set_repeat_context(cmd.repeat);

        if cmd.shuffle {
            self.connect_state.set_current_track(index)?;
            self.connect_state.shuffle()?;
        } else {
            // manually overwrite a possible current queued track
            self.connect_state.set_current_track(index)?;
            self.connect_state.reset_playback_to_position(Some(index))?;
        }

        self.connect_state.set_repeat_track(cmd.repeat_track);

        if self.connect_state.current_track(MessageField::is_some) {
            self.load_track(cmd.start_playing, cmd.seek_to)?;
        } else {
            info!("No active track, stopping");
            self.handle_stop()
        }

        self.preload_autoplay_when_required();

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

    fn preload_autoplay_when_required(&mut self) {
        let require_load_new = !self
            .connect_state
            .has_next_tracks(Some(CONTEXT_FETCH_THRESHOLD));

        if !require_load_new {
            return;
        }

        match self.connect_state.try_load_next_context() {
            Err(why) => error!("failed loading next context: {why}"),
            Ok(next) => {
                match next {
                    LoadNext::Done => info!("loaded next context"),
                    LoadNext::PageUrl(page_url) => {
                        self.resolve_context
                            .push(ResolveContext::from_page_url(page_url));
                    }
                    LoadNext::Empty if self.session.autoplay() => {
                        let current_context = self.connect_state.context_uri();
                        let fallback = self.connect_state.current_track(|t| &t.uri);
                        let resolve = ResolveContext::from_uri(current_context, fallback, true);

                        // When in autoplay, keep topping up the playlist when it nears the end
                        debug!("Preloading autoplay: {resolve}");
                        // resolve the next autoplay context
                        self.resolve_context.push(resolve);
                    }
                    LoadNext::Empty => {
                        debug!("next context is empty and autoplay isn't enabled, no preloading required")
                    }
                }
            }
        }
    }

    fn is_playing(&self) -> bool {
        matches!(
            self.play_status,
            SpircPlayStatus::Playing { .. } | SpircPlayStatus::LoadingPlay { .. }
        )
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

        self.preload_autoplay_when_required();

        if has_next_track {
            self.load_track(continue_playing, 0)
        } else {
            info!("Not playing next track because there are no more tracks left in queue.");
            self.connect_state.reset_playback_to_position(None)?;
            self.handle_stop();
            Ok(())
        }
    }

    fn handle_prev(&mut self) -> Result<(), Error> {
        // Previous behaves differently based on the position
        // Under 3s it goes to the previous song (starts playing)
        // Over 3s it seeks to zero (retains previous play status)
        if self.position() < 3000 {
            let repeat_context = self.connect_state.repeat_context();
            match self.connect_state.prev_track()? {
                None if repeat_context => self.connect_state.reset_playback_to_position(None)?,
                None => {
                    self.connect_state.reset_playback_to_position(None)?;
                    self.handle_stop()
                }
                Some(_) => self.load_track(self.is_playing(), 0)?,
            }
        } else {
            self.handle_seek(0);
        }

        Ok(())
    }

    fn handle_volume_up(&mut self) {
        let volume =
            (self.connect_state.device_info().volume as u16).saturating_add(VOLUME_STEP_SIZE);
        self.set_volume(volume);
    }

    fn handle_volume_down(&mut self) {
        let volume =
            (self.connect_state.device_info().volume as u16).saturating_sub(VOLUME_STEP_SIZE);
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

    fn handle_playlist_modification(
        &mut self,
        playlist_modification_info: PlaylistModificationInfo,
    ) -> Result<(), Error> {
        let uri = playlist_modification_info.uri.ok_or(SpircError::NoData)?;
        let uri = String::from_utf8(uri)?;

        if self.connect_state.context_uri() != &uri {
            debug!("ignoring playlist modification update for playlist <{uri}>, because it isn't the current context");
            return Ok(());
        }

        debug!("playlist modification for current context: {uri}");
        self.resolve_context.push(ResolveContext::from_uri(
            uri,
            self.connect_state.current_track(|t| &t.uri),
            false,
        ));

        Ok(())
    }

    fn handle_session_update(&mut self, mut session_update: SessionUpdate) {
        let reason = session_update.reason.enum_value();

        let mut session = match session_update.session.take() {
            None => return,
            Some(session) => session,
        };

        let active_device = session._host_active_device_id.take().map(|id| match id {
            _host_active_device_id::HostActiveDeviceId(id) => id,
            other => {
                warn!("unexpected active device id {other:?}");
                String::new()
            }
        });

        if matches!(active_device, Some(ref device) if device == self.session.device_id()) {
            info!(
                "session update: <{:?}> for self, current session_id {}, new session_id {}",
                reason,
                self.session.session_id(),
                session.session_id
            );

            if self.session.session_id() != session.session_id {
                self.session.set_session_id(session.session_id.clone());
                self.connect_state.set_session_id(session.session_id);
            }
        } else {
            debug!("session update: <{reason:?}> from active session host: <{active_device:?}>");
        }

        // this seems to be used for jams or handling the current session_id
        //
        // handling this event was intended to keep the playback when other clients (primarily
        // mobile) connects, otherwise they would steel the current playback when there was no
        // session_id provided on the initial PutStateReason::NEW_DEVICE state update
        //
        // by generating an initial session_id from the get-go we prevent that behavior and
        // currently don't need to handle this event, might still be useful for later "jam" support
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
            debug!("current track is none, stopping playback");
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

        self.connect_state.set_now(self.now_ms() as u64);

        self.connect_state
            .send_state(&self.session)
            .await
            .map(|_| ())
    }

    fn set_volume(&mut self, volume: u16) {
        let old_volume = self.connect_state.device_info().volume;
        let new_volume = volume as u32;
        if old_volume != new_volume || self.mixer.volume() != volume {
            self.update_volume = true;

            self.connect_state.set_volume(new_volume);
            self.mixer.set_volume(volume);
            if let Some(cache) = self.session.cache() {
                cache.save_volume(volume)
            }
            if self.connect_state.is_active() {
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
