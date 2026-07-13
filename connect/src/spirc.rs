use crate::{
    LoadContextOptions, LoadRequestOptions, PlayContext,
    context_resolver::{ContextAction, ContextResolver, ResolveContext},
    core::{
        Error, Session, SpotifyUri,
        authentication::Credentials,
        dealer::{
            manager::{BoxedStream, BoxedStreamResult, Reply, RequestReply},
            protocol::{Command, FallbackWrapper, Message, Request},
        },
        session::UserAttributes,
        spclient::TransferRequest,
    },
    model::{LoadRequest, PlayingTrack, SpircPlayStatus},
    playback::{
        mixer::Mixer,
        player::{Player, PlayerEvent, PlayerEventChannel, QueueTrack},
    },
    protocol::{
        connect::{
            Cluster, ClusterUpdate, ClusterUpdateReason as ServerClusterUpdateReason,
            LogoutCommand, SetVolumeCommand,
        },
        context::Context,
        devices::DeviceType,
        explicit_content_pubsub::UserAttributesUpdate,
        player::ProvidedTrack,
        playlist4_external::PlaylistModificationInfo,
        social_connect_v2::SessionUpdate,
        transfer_state::TransferState,
        user_attributes::UserAttributesMutation,
        {context_page::ContextPage, player::PlayerState},
    },
    state::{
        context::{ContextType, ResetContext},
        provider::IsProvider,
        {ConnectConfig, ConnectState},
    },
};
use futures_util::StreamExt;
use protobuf::MessageField;
use std::{
    collections::HashMap,
    future::Future,
    sync::Arc,
    sync::atomic::{AtomicUsize, Ordering},
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use thiserror::Error;
use tokio::{
    sync::{broadcast, mpsc, watch},
    time::sleep,
};

#[derive(Debug, Error)]
enum SpircError {
    #[error("response payload empty")]
    NoData,
    #[error("{0} had no uri")]
    NoUri(&'static str),
    #[error("message pushed for another URI")]
    InvalidUri(String),
    #[error("failed to put connect state for new device")]
    FailedDealerSetup,
    #[error("unknown endpoint: {0:#?}")]
    UnknownEndpoint(serde_json::Value),
}

impl From<SpircError> for Error {
    fn from(err: SpircError) -> Self {
        use SpircError::*;
        match err {
            NoData | NoUri(_) => Error::unavailable(err),
            InvalidUri(_) | FailedDealerSetup => Error::aborted(err),
            UnknownEndpoint(_) => Error::unimplemented(err),
        }
    }
}

/// Information about a device in the cluster
#[derive(Debug, Clone)]
pub struct DeviceInfo {
    /// Unique device identifier
    pub device_id: String,
    /// Human-readable device name
    pub device_alias: String,
    /// Device type (e.g., speaker, phone)
    pub device_type: DeviceType,
    /// Volume level 0-100
    pub volume: u32,
    /// Whether this is the currently active device
    pub is_active: bool,
}

/// Current state of the device cluster (all known devices)
#[derive(Debug, Clone)]
pub struct ClusterState {
    /// Map of all known devices by device_id
    pub devices: HashMap<String, DeviceInfo>,
    /// Currently active device ID (if any)
    pub active_device_id: Option<String>,
}

/// Queue information (previous and next tracks)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueueList {
    /// Previous tracks in the queue (as URIs)
    pub prev_tracks: Vec<String>,
    /// Next tracks in the queue (as URIs)
    pub next_tracks: Vec<String>,
}

/// Semantic reason for cluster updates
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ClusterUpdateReason {
    /// A device joined the cluster
    DeviceAppeared,
    /// A device left the cluster
    DeviceDisappeared,
    /// Active device switched
    ActiveDeviceChanged,
    /// Device state changed
    DeviceStateChanged,
    /// Device info changed
    DeviceInfoChanged,
}

/// Event emitted when cluster state changes
#[derive(Debug, Clone)]
pub struct ClusterUpdateEvent {
    /// Device that changed, or `None` if `ActiveDeviceChanged` means nothing is active
    pub device_id: Option<String>,
    /// Reason for the update
    pub reason: ClusterUpdateReason,
}

/// Semantic reasons for queue updates
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum QueueUpdateReason {
    /// Previous tracks changed
    PrevTracksChanged,
    /// Next tracks changed
    NextTracksChanged,
}

/// Event emitted when queue changes
#[derive(Debug, Clone)]
pub struct QueueUpdateEvent {
    /// Reason for the queue update
    pub reason: QueueUpdateReason,
}

/// Semantic reasons for player state updates
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum PlayerUpdateReason {
    /// Track changed
    TrackChanged,
    /// Play/pause state changed
    PlayPauseChanged,
    /// Shuffle mode changed
    ShuffleChanged,
    /// Repeat mode changed
    RepeatChanged,
    /// Context changed
    ContextChanged,
    /// Queue was set or reloaded
    QueueChanged,
    /// Seek detected
    SeekChanged,
    /// Other state change
    Other,
}

/// Emitted when player state changes
#[derive(Debug, Clone)]
pub struct PlayerUpdateEvent {
    /// Reason for the player update
    pub reason: PlayerUpdateReason,
}

struct SpircTask {
    player: Arc<Player>,
    mixer: Arc<dyn Mixer>,

    /// the state management object
    connect_state: ConnectState,
    connect_established: bool,

    play_request_id: Option<u64>,
    play_status: SpircPlayStatus,

    connection_id_update: BoxedStreamResult<String>,
    connect_state_update: BoxedStreamResult<ClusterUpdate>,
    connect_state_volume_update: BoxedStreamResult<SetVolumeCommand>,
    connect_state_logout_request: BoxedStreamResult<LogoutCommand>,
    playlist_update: BoxedStreamResult<PlaylistModificationInfo>,
    session_update: BoxedStreamResult<FallbackWrapper<SessionUpdate>>,
    connect_state_command: BoxedStream<RequestReply>,
    user_attributes_update: BoxedStreamResult<UserAttributesUpdate>,
    user_attributes_mutation: BoxedStreamResult<UserAttributesMutation>,

    commands: Option<mpsc::UnboundedReceiver<SpircCommand>>,
    player_events: Option<PlayerEventChannel>,

    context_resolver: ContextResolver,

    emit_set_queue_events: bool,

    shutdown: bool,
    session: Session,

    /// is set when transferring, and used after resolving the contexts to finish the transfer
    pub transfer_state: Option<TransferState>,

    /// when set to true, it will update the volume after [VOLUME_UPDATE_DELAY],
    /// when no other future resolves, otherwise resets the delay
    update_volume: bool,

    /// when set to true, it will update the volume after [UPDATE_STATE_DELAY],
    /// when no other future resolves, otherwise resets the delay
    update_state: bool,

    player_update_sender: broadcast::Sender<PlayerUpdateEvent>,
    cluster_update_sender: broadcast::Sender<ClusterUpdateEvent>,
    queue_update_sender: broadcast::Sender<QueueUpdateEvent>,
    player_state_sender: watch::Sender<Option<PlayerState>>,
    cluster_state_sender: watch::Sender<ClusterState>,
    queue_list_sender: watch::Sender<QueueList>,
    last_active_device_id: Option<String>,
    last_queue_list: QueueList,
    last_player_state: Option<PlayerState>,

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
    Disconnect { pause: bool },
    SetPosition(u32),
    SetVolume(u16),
    Activate,
    Transfer(Option<TransferRequest>),
    Load(LoadRequest),
    AddToQueue(SpotifyUri),
}

const CONTEXT_FETCH_THRESHOLD: usize = 2;

// delay to update volume after a certain amount of time, instead on each update request
const VOLUME_UPDATE_DELAY: Duration = Duration::from_millis(500);
// to reduce updates to remote, we group some request by waiting for a set amount of time
const UPDATE_STATE_DELAY: Duration = Duration::from_millis(200);
// a single cluster update can emit several of these back-to-back (e.g. multiple devices
// changing at once) with no await point in between for a receiver to drain the channel,
// so capacity 1 would silently drop all but the last one
const UPDATE_EVENT_CHANNEL_CAPACITY: usize = 16;

/// The spotify connect handle
pub struct Spirc {
    commands: mpsc::UnboundedSender<SpircCommand>,
    player_update_sender: broadcast::Sender<PlayerUpdateEvent>,
    cluster_update_sender: broadcast::Sender<ClusterUpdateEvent>,
    queue_update_sender: broadcast::Sender<QueueUpdateEvent>,
    player_state_sender: watch::Sender<Option<PlayerState>>,
    cluster_state_sender: watch::Sender<ClusterState>,
    queue_list_sender: watch::Sender<QueueList>,
}

impl Spirc {
    /// Initializes a new spotify connect device
    ///
    /// The returned tuple consists out of a handle to the [`Spirc`] that
    /// can control the local connect device when active. And a [`Future`]
    /// which represents the [`Spirc`] event loop that processes the whole
    /// connect device logic.
    pub async fn new(
        config: ConnectConfig,
        session: Session,
        credentials: Credentials,
        player: Arc<Player>,
        mixer: Arc<dyn Mixer>,
    ) -> Result<(Spirc, impl Future<Output = ()>), Error> {
        fn extract_connection_id(msg: Message) -> Result<String, Error> {
            let connection_id = msg
                .headers
                .get("Spotify-Connection-Id")
                .ok_or_else(|| SpircError::InvalidUri(msg.uri.clone()))?;
            Ok(connection_id.to_owned())
        }

        let spirc_id = SPIRC_COUNTER.fetch_add(1, Ordering::AcqRel);
        debug!("new Spirc[{spirc_id}]");

        let emit_set_queue_events = config.emit_set_queue_events;
        let connect_state = ConnectState::new(config, &session);

        let connection_id_update = session
            .dealer()
            .listen_for("hm://pusher/v1/connections/", extract_connection_id)?;

        let connect_state_update = session
            .dealer()
            .listen_for("hm://connect-state/v1/cluster", Message::from_raw)?;

        let connect_state_volume_update = session
            .dealer()
            .listen_for("hm://connect-state/v1/connect/volume", Message::from_raw)?;

        let connect_state_logout_request = session
            .dealer()
            .listen_for("hm://connect-state/v1/connect/logout", Message::from_raw)?;

        let playlist_update = session
            .dealer()
            .listen_for("hm://playlist/v2/playlist/", Message::from_raw)?;

        let session_update = session
            .dealer()
            .listen_for("social-connect/v2/session_update", Message::try_from_json)?;

        let user_attributes_update = session
            .dealer()
            .listen_for("spotify:user:attributes:update", Message::from_raw)?;

        // can be trigger by toggling autoplay in a desktop client
        let user_attributes_mutation = session
            .dealer()
            .listen_for("spotify:user:attributes:mutated", Message::from_raw)?;

        let connect_state_command = session
            .dealer()
            .handle_for("hm://connect-state/v1/player/command")?;

        // pre-acquire client_token, preventing multiple request while running
        let _ = session.spclient().client_token().await?;

        // Connect *after* all message listeners are registered
        session.connect(credentials, true).await?;

        // pre-acquire access_token (we need to be authenticated to retrieve a token)
        let _ = session.login5().auth_token().await?;

        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();
        let (player_update_sender_tx, _) = broadcast::channel(UPDATE_EVENT_CHANNEL_CAPACITY);
        let (cluster_update_sender_tx, _) = broadcast::channel(UPDATE_EVENT_CHANNEL_CAPACITY);
        let (queue_update_sender_tx, _) = broadcast::channel(UPDATE_EVENT_CHANNEL_CAPACITY);
        let (player_state_sender_tx, _) = watch::channel(None);
        let (cluster_state_sender_tx, _) = watch::channel(ClusterState {
            devices: HashMap::new(),
            active_device_id: None,
        });
        let (queue_list_sender_tx, _) = watch::channel(QueueList {
            prev_tracks: Vec::new(),
            next_tracks: Vec::new(),
        });

        let player_events = player.get_player_event_channel();

        let mut task = SpircTask {
            player,
            mixer,

            connect_state,
            connect_established: false,

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

            context_resolver: ContextResolver::new(session.clone()),

            emit_set_queue_events,

            shutdown: false,
            session,

            transfer_state: None,
            update_volume: false,
            update_state: false,

            player_update_sender: player_update_sender_tx.clone(),
            cluster_update_sender: cluster_update_sender_tx.clone(),
            queue_update_sender: queue_update_sender_tx.clone(),
            player_state_sender: player_state_sender_tx.clone(),
            cluster_state_sender: cluster_state_sender_tx.clone(),
            queue_list_sender: queue_list_sender_tx.clone(),
            last_active_device_id: None,
            last_queue_list: QueueList {
                prev_tracks: Vec::new(),
                next_tracks: Vec::new(),
            },
            last_player_state: None,

            spirc_id,
        };

        let spirc = Spirc {
            commands: cmd_tx,
            player_update_sender: player_update_sender_tx,
            cluster_update_sender: cluster_update_sender_tx,
            queue_update_sender: queue_update_sender_tx,
            player_state_sender: player_state_sender_tx,
            cluster_state_sender: cluster_state_sender_tx,
            queue_list_sender: queue_list_sender_tx,
        };

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

    /// Safely shutdowns the spirc.
    ///
    /// This pauses the playback, disconnects the connect device and
    /// bring the future initially returned to an end.
    pub fn shutdown(&self) -> Result<(), Error> {
        Ok(self.commands.send(SpircCommand::Shutdown)?)
    }

    /// Resumes the playback
    ///
    /// Does nothing if we are not the active device, or it isn't paused.
    pub fn play(&self) -> Result<(), Error> {
        Ok(self.commands.send(SpircCommand::Play)?)
    }

    /// Resumes or pauses the playback
    ///
    /// Does nothing if we are not the active device.
    pub fn play_pause(&self) -> Result<(), Error> {
        Ok(self.commands.send(SpircCommand::PlayPause)?)
    }

    /// Pauses the playback
    ///
    /// Does nothing if we are not the active device, or if it isn't playing.
    pub fn pause(&self) -> Result<(), Error> {
        Ok(self.commands.send(SpircCommand::Pause)?)
    }

    /// Seeks to the beginning or skips to the previous track.
    ///
    /// Seeks to the beginning when the current track position
    /// is greater than 3 seconds.
    ///
    /// Does nothing if we are not the active device.
    pub fn prev(&self) -> Result<(), Error> {
        Ok(self.commands.send(SpircCommand::Prev)?)
    }

    /// Skips to the next track.
    ///
    /// Does nothing if we are not the active device.
    pub fn next(&self) -> Result<(), Error> {
        Ok(self.commands.send(SpircCommand::Next)?)
    }

    /// Increases the volume by configured steps of [ConnectConfig].
    ///
    /// Does nothing if we are not the active device.
    pub fn volume_up(&self) -> Result<(), Error> {
        Ok(self.commands.send(SpircCommand::VolumeUp)?)
    }

    /// Decreases the volume by configured steps of [ConnectConfig].
    ///
    /// Does nothing if we are not the active device.
    pub fn volume_down(&self) -> Result<(), Error> {
        Ok(self.commands.send(SpircCommand::VolumeDown)?)
    }

    /// Shuffles the playback according to the value.
    ///
    /// If true shuffles/reshuffles the playback. Otherwise, does
    /// nothing (if not shuffled) or unshuffles the playback while
    /// resuming at the position of the current track.
    ///
    /// Does nothing if we are not the active device.
    pub fn shuffle(&self, shuffle: bool) -> Result<(), Error> {
        Ok(self.commands.send(SpircCommand::Shuffle(shuffle))?)
    }

    /// Repeats the playback context according to the value.
    ///
    /// Does nothing if we are not the active device.
    pub fn repeat(&self, repeat: bool) -> Result<(), Error> {
        Ok(self.commands.send(SpircCommand::Repeat(repeat))?)
    }

    /// Repeats the current track if true.
    ///
    /// Does nothing if we are not the active device.
    ///
    /// Skipping to the next track disables the repeating.
    pub fn repeat_track(&self, repeat: bool) -> Result<(), Error> {
        Ok(self.commands.send(SpircCommand::RepeatTrack(repeat))?)
    }

    /// Update the volume to the given value.
    ///
    /// Does nothing if we are not the active device.
    pub fn set_volume(&self, volume: u16) -> Result<(), Error> {
        Ok(self.commands.send(SpircCommand::SetVolume(volume))?)
    }

    /// Updates the position to the given value.
    ///
    /// Does nothing if we are not the active device.
    ///
    /// If value is greater than the track duration,
    /// the update is ignored.
    pub fn set_position_ms(&self, position_ms: u32) -> Result<(), Error> {
        Ok(self.commands.send(SpircCommand::SetPosition(position_ms))?)
    }

    /// Load a new context and replace the current.
    ///
    /// Does nothing if we are not the active device.
    ///
    /// Does not overwrite the queue.
    pub fn load(&self, command: LoadRequest) -> Result<(), Error> {
        Ok(self.commands.send(SpircCommand::Load(command))?)
    }

    /// Adds a track, episode, album or playlist to the queue.
    ///
    /// Does nothing if we are not the active device.
    ///
    /// For albums and playlists, all tracks/episodes are resolved and added to the queue.
    pub fn add_to_queue(&self, uri: SpotifyUri) -> Result<(), Error> {
        if !matches!(
            uri,
            SpotifyUri::Track { .. }
                | SpotifyUri::Episode { .. }
                | SpotifyUri::Album { .. }
                | SpotifyUri::Playlist { .. }
        ) {
            return Err(Error::invalid_argument("uri"));
        }
        Ok(self.commands.send(SpircCommand::AddToQueue(uri))?)
    }

    /// Disconnects the current device and pauses the playback according the value.
    ///
    /// Does nothing if we are not the active device.
    pub fn disconnect(&self, pause: bool) -> Result<(), Error> {
        Ok(self.commands.send(SpircCommand::Disconnect { pause })?)
    }

    /// Acquires the control as active connect device.
    ///
    /// Does not [Spirc::transfer] the playback. Does nothing if we are not the active device.
    pub fn activate(&self) -> Result<(), Error> {
        Ok(self.commands.send(SpircCommand::Activate)?)
    }

    /// Acquires the control as active connect device over the transfer flow.
    ///
    /// Does nothing if we are not the active device.
    pub fn transfer(&self, transfer_request: Option<TransferRequest>) -> Result<(), Error> {
        Ok(self
            .commands
            .send(SpircCommand::Transfer(transfer_request))?)
    }

    /// Get a channel which sends lightweight playback state updates.
    pub fn get_player_update_channel(&self) -> broadcast::Receiver<PlayerUpdateEvent> {
        self.player_update_sender.subscribe()
    }

    /// Get a channel which sends device topology changes (devices appearing/disappearing, active device changes).
    pub fn get_cluster_update_channel(&self) -> broadcast::Receiver<ClusterUpdateEvent> {
        self.cluster_update_sender.subscribe()
    }

    /// Get a channel which sends queue change events when prev/next tracks differ.
    pub fn get_queue_update_channel(&self) -> broadcast::Receiver<QueueUpdateEvent> {
        self.queue_update_sender.subscribe()
    }

    /// Watch the current player state (full PlayerState)
    pub fn watch_player_state(&self) -> watch::Receiver<Option<PlayerState>> {
        self.player_state_sender.subscribe()
    }

    /// Watch the current cluster state (all devices and active device)
    pub fn watch_cluster_state(&self) -> watch::Receiver<ClusterState> {
        self.cluster_state_sender.subscribe()
    }

    /// Watch the current queue list (previous and next tracks)
    pub fn watch_queue_list(&self) -> watch::Receiver<QueueList> {
        self.queue_list_sender.subscribe()
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

            // when state and volume update have a higher priority than context resolving
            // because of that the context resolving has to wait, so that the other tasks can finish
            let allow_context_resolving = !self.update_state && !self.update_volume;

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
                        error!("could not dispatch connect state update: {e}");
                    }
                },
                // main dealer request handling (dealer expects an answer)
                request = self.connect_state_command.next() => unwrap! {
                    request,
                    |request| if let Err(e) = self.handle_connect_state_request(request).await {
                        error!("couldn't handle connect state command: {e}");
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
                cmd = async { commands?.recv().await }, if commands.is_some() && self.connect_established => if let Some(cmd) = cmd {
                    if let Err(e) = self.handle_command(cmd).await {
                        debug!("could not dispatch command: {e}");
                    }
                },
                event = async { player_events?.recv().await }, if player_events.is_some() => if let Some(event) = event {
                    if let Err(e) = self.handle_player_event(event) {
                        error!("could not dispatch player event: {e}");
                    }
                },
                _ = async { sleep(UPDATE_STATE_DELAY).await }, if self.update_state => {
                    self.update_state = false;

                    if let Err(why) = self.notify().await {
                        error!("state update: {why}")
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
                // context resolver handling, the idea/reason behind it the following:
                //
                // when we request a context that has multiple pages (for example an artist)
                // resolving all pages at once can take around ~1-30sec, when we resolve
                // everything at once that would block our main loop for that time
                //
                // to circumvent this behavior, we request each context separately here and
                // finish after we received our last item of a type
                next_context = async {
                    self.context_resolver.get_next_context(|| {
                        // Sending local file URIs to this endpoint results in a Bad Request status.
                        // It's likely appropriate to filter them out anyway; Spotify's backend
                        // has no knowledge about these tracks and so can't do anything with them.
                        self.connect_state.recent_track_uris()
                            .into_iter()
                            .filter(|t| !t.starts_with("spotify:local"))
                            .collect::<Vec<_>>()
                    }).await
                }, if allow_context_resolving && self.context_resolver.has_next() => {
                    let update_state = self.handle_next_context(next_context);
                    if update_state {
                        if let Err(why) = self.notify().await {
                            error!("update after context resolving failed: {why}")
                        }
                    }
                },
                else => break
            }
        }

        if !self.shutdown && self.connect_state.is_active() {
            warn!("unexpected shutdown");
            if let Err(why) = self.handle_disconnect().await {
                error!("error during disconnecting: {why}")
            }
            self.publish_local_activation(false);
        }

        // this should clear the active session id, leaving an empty state
        if let Err(why) = self.session.spclient().delete_connect_state_request().await {
            error!("error during connect state deletion: {why}")
        };

        self.session.dealer().close().await;
    }

    fn handle_next_context(&mut self, next_context: Result<Context, Error>) -> bool {
        let next_context = match next_context {
            Err(why) => {
                self.context_resolver.mark_next_unavailable();
                self.context_resolver.remove_used_and_invalid();
                error!("{why}");
                return false;
            }
            Ok(ctx) => ctx,
        };

        debug!("handling next context {:?}", next_context.uri);

        match self
            .context_resolver
            .apply_next_context(&mut self.connect_state, next_context)
        {
            Ok(remaining) => {
                if let Some(remaining) = remaining {
                    self.context_resolver.add_list(remaining)
                }
            }
            Err(why) => {
                error!("{why}")
            }
        }

        let update_state = if self
            .context_resolver
            .try_finish(&mut self.connect_state, &mut self.transfer_state)
        {
            self.add_autoplay_resolving_when_required();
            true
        } else {
            false
        };

        // Fire set queue event if context was successfully loaded
        if update_state {
            self.emit_set_queue_event();
        }

        self.context_resolver.remove_used_and_invalid();
        update_state
    }

    /// Emit set queue event via PlayerEvent
    fn emit_set_queue_event(&self) {
        if !self.emit_set_queue_events {
            return;
        }

        let state_player = self.connect_state.player();

        let current_track = state_player.track.as_ref().map(|t| QueueTrack {
            uri: t.uri.clone(),
            provider: t.provider.clone(),
        });

        let next_tracks: Vec<_> = state_player
            .next_tracks
            .iter()
            .map(|t| QueueTrack {
                uri: t.uri.clone(),
                provider: t.provider.clone(),
            })
            .collect();

        let prev_tracks: Vec<_> = state_player
            .prev_tracks
            .iter()
            .map(|t| QueueTrack {
                uri: t.uri.clone(),
                provider: t.provider.clone(),
            })
            .collect();

        let context_uri = self.connect_state.context_uri().clone();

        self.player
            .emit_set_queue_event(context_uri, current_track, next_tracks, prev_tracks);
    }

    // todo: is the time_delta still necessary?
    fn now_ms(&self) -> i64 {
        let dur = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|err| err.duration());

        dur.as_millis() as i64 + 1000 * self.session.time_delta()
    }

    async fn handle_command(&mut self, cmd: SpircCommand) -> Result<(), Error> {
        trace!("Received SpircCommand::{cmd:?}");
        match cmd {
            SpircCommand::Shutdown => {
                trace!("Received SpircCommand::Shutdown");
                self.handle_pause();
                self.handle_disconnect().await?;
                self.publish_local_activation(false);
                self.shutdown = true;
                if let Some(rx) = self.commands.as_mut() {
                    rx.close()
                }
            }
            SpircCommand::Transfer(request) if !self.connect_state.is_active() => {
                let device_id = self.session.device_id();
                self.session
                    .spclient()
                    .transfer(device_id, device_id, request.as_ref())
                    .await?;
                return Ok(());
            }
            SpircCommand::Activate if !self.connect_state.is_active() => {
                trace!("Received SpircCommand::{cmd:?}");
                self.handle_activate();
                return self.notify().await;
            }
            SpircCommand::Transfer(..) | SpircCommand::Activate => {
                warn!("SpircCommand::{cmd:?} will be ignored while already active")
            }
            _ if !self.connect_state.is_active() => {
                warn!("SpircCommand::{cmd:?} will be ignored while Not Active")
            }
            SpircCommand::Disconnect { pause } => {
                if pause {
                    self.handle_pause()
                }
                let result = self.handle_disconnect().await;
                self.publish_local_activation(false);
                return result;
            }
            SpircCommand::Play => self.handle_play(),
            SpircCommand::PlayPause => self.handle_play_pause(),
            SpircCommand::Pause => self.handle_pause(),
            SpircCommand::Prev => self.handle_prev()?,
            SpircCommand::Next => self.handle_next(None)?,
            SpircCommand::VolumeUp => self.handle_volume_up(),
            SpircCommand::VolumeDown => self.handle_volume_down(),
            SpircCommand::Shuffle(shuffle) => self.handle_shuffle(shuffle)?,
            SpircCommand::Repeat(repeat) => self.handle_repeat_context(repeat)?,
            SpircCommand::RepeatTrack(repeat) => self.handle_repeat_track(repeat),
            SpircCommand::SetPosition(position) => self.handle_seek(position),
            SpircCommand::SetVolume(volume) => self.set_volume(volume),
            SpircCommand::Load(command) => self.handle_load(command, None, None).await?,
            SpircCommand::AddToQueue(uri) => self.handle_add_to_queue(uri).await,
        };

        self.notify().await
    }

    fn handle_player_event(&mut self, event: PlayerEvent) -> Result<(), Error> {
        let player_update_reason = match &event {
            PlayerEvent::TrackChanged { .. } => Some(PlayerUpdateReason::TrackChanged),
            PlayerEvent::Playing { .. }
            | PlayerEvent::Paused { .. }
            | PlayerEvent::Stopped { .. } => Some(PlayerUpdateReason::PlayPauseChanged),
            PlayerEvent::Seeked { .. } | PlayerEvent::PositionCorrection { .. } => {
                Some(PlayerUpdateReason::SeekChanged)
            }
            PlayerEvent::ShuffleChanged { .. } => Some(PlayerUpdateReason::ShuffleChanged),
            PlayerEvent::RepeatChanged { .. } => Some(PlayerUpdateReason::RepeatChanged),
            PlayerEvent::SetQueue { .. } => Some(PlayerUpdateReason::QueueChanged),
            _ => None,
        };

        if let PlayerEvent::TrackChanged { audio_item } = &event {
            self.connect_state.update_duration(audio_item.duration_ms);
            self.update_state = true;
        }

        // update play_request_id
        if let PlayerEvent::PlayRequestIdChanged { play_request_id } = event {
            self.play_request_id = Some(play_request_id);
            return Ok(());
        }

        let should_gate_by_play_request_id = matches!(
            event,
            PlayerEvent::Loading { .. }
                | PlayerEvent::Seeked { .. }
                | PlayerEvent::PositionCorrection { .. }
                | PlayerEvent::Playing { .. }
                | PlayerEvent::Paused { .. }
                | PlayerEvent::Stopped { .. }
                | PlayerEvent::TimeToPreloadNextTrack { .. }
                | PlayerEvent::EndOfTrack { .. }
                | PlayerEvent::Unavailable { .. }
        );

        // we only process events if the play_request_id matches. If it doesn't, it is
        // an event that belongs to a previous track and only arrives now due to a race
        // condition. In this case we have updated the state already and don't want to
        // mess with it.
        if should_gate_by_play_request_id
            && !matches! {
                (event.get_play_request_id(), self.play_request_id),
                (Some(event_id), Some(current_id)) if event_id == current_id
            }
        {
            return Ok(());
        }

        match event {
            PlayerEvent::EndOfTrack { .. } => {
                let next_track = self
                    .connect_state
                    .repeat_track()
                    .then(|| self.connect_state.current_track(|t| t.uri.clone()));

                self.handle_next(next_track)?
            }
            PlayerEvent::Loading { .. } => match self.play_status {
                SpircPlayStatus::LoadingPlay { position_ms } => {
                    self.connect_state
                        .update_position(position_ms, self.now_ms());
                    trace!("==> LoadingPlay");
                }
                SpircPlayStatus::LoadingPause { position_ms } => {
                    self.connect_state
                        .update_position(position_ms, self.now_ms());
                    trace!("==> LoadingPause");
                }
                _ => {
                    self.connect_state.update_position(0, self.now_ms());
                    trace!("==> Loading");
                }
            },
            PlayerEvent::Seeked { position_ms, .. } => {
                trace!("==> Seeked");
                self.connect_state
                    .update_position(position_ms, self.now_ms())
            }
            PlayerEvent::Playing { position_ms, .. }
            | PlayerEvent::PositionCorrection { position_ms, .. } => {
                trace!("==> Playing");
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
                        } else {
                            return Ok(());
                        }
                    }
                    SpircPlayStatus::LoadingPlay { .. } | SpircPlayStatus::LoadingPause { .. } => {
                        self.connect_state
                            .update_position(position_ms, self.now_ms());
                        self.play_status = SpircPlayStatus::Playing {
                            nominal_start_time: new_nominal_start_time,
                            preloading_of_next_track_triggered: false,
                        };
                    }
                    _ => return Ok(()),
                }
            }
            PlayerEvent::Paused {
                position_ms: new_position_ms,
                ..
            } => {
                trace!("==> Paused");
                match self.play_status {
                    SpircPlayStatus::Paused { .. } | SpircPlayStatus::Playing { .. } => {
                        self.connect_state
                            .update_position(new_position_ms, self.now_ms());
                        self.play_status = SpircPlayStatus::Paused {
                            position_ms: new_position_ms,
                            preloading_of_next_track_triggered: false,
                        };
                    }
                    SpircPlayStatus::LoadingPlay { .. } | SpircPlayStatus::LoadingPause { .. } => {
                        self.connect_state
                            .update_position(new_position_ms, self.now_ms());
                        self.play_status = SpircPlayStatus::Paused {
                            position_ms: new_position_ms,
                            preloading_of_next_track_triggered: false,
                        };
                    }
                    _ => return Ok(()),
                }
            }
            PlayerEvent::Stopped { .. } => {
                trace!("==> Stopped");
                match self.play_status {
                    SpircPlayStatus::Stopped => return Ok(()),
                    _ => self.play_status = SpircPlayStatus::Stopped,
                }
            }
            PlayerEvent::TimeToPreloadNextTrack { .. } => {
                self.handle_preload_next_track();
                return Ok(());
            }
            PlayerEvent::TrackChanged { .. }
            | PlayerEvent::ShuffleChanged { .. }
            | PlayerEvent::RepeatChanged { .. }
            | PlayerEvent::SetQueue { .. } => {}
            PlayerEvent::VolumeChanged { .. }
            | PlayerEvent::SessionConnected { .. }
            | PlayerEvent::SessionDisconnected { .. }
            | PlayerEvent::SessionClientChanged { .. }
            | PlayerEvent::AutoPlayChanged { .. }
            | PlayerEvent::FilterExplicitContentChanged { .. } => return Ok(()),
            PlayerEvent::Unavailable { track_id, .. } => {
                self.handle_unavailable(&track_id)?;
                if self.connect_state.current_track(|t| &t.uri) == &track_id.to_uri() {
                    self.handle_next(None)?
                }
            }
            _ => return Ok(()),
        }

        self.update_state = true;

        if self.connect_state.is_active() {
            // sync play_status now instead of waiting on the debounced notify()
            self.connect_state.set_status(&self.play_status);
            self.publish_active_state(player_update_reason);
        }

        Ok(())
    }

    async fn handle_connection_id_update(&mut self, connection_id: String) -> Result<(), Error> {
        trace!("Received connection ID update: {connection_id:?}");
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

        self.connect_established = true;

        let same_session = cluster.player_state.session_id == self.session.session_id()
            || cluster.player_state.session_id.is_empty();
        if !cluster.active_device_id.is_empty() || !same_session {
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

        match TransferState::parse_from_bytes(&cluster.transfer_data) {
            Ok(transfer_state) => self.handle_transfer(transfer_state)?,
            Err(why) => error!("failed to take over control: {why}"),
        }

        Ok(())
    }

    fn handle_user_attributes_update(&mut self, update: UserAttributesUpdate) {
        trace!("Received attributes update: {update:#?}");
        let attributes: UserAttributes = update
            .pairs
            .iter()
            .map(|(key, value)| (key.to_owned(), value.to_owned()))
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

                trace!("Received attribute mutation, {key} was {old_value} is now {new_value}");

                if key == "filter-explicit-content" && new_value == "1" {
                    self.player
                        .emit_filter_explicit_content_changed_event(matches!(new_value, "1"));
                }

                if key == "autoplay" && old_value != new_value {
                    self.player
                        .emit_auto_play_changed_event(matches!(new_value, "1"));

                    self.add_autoplay_resolving_when_required()
                }
            } else {
                trace!("Received attribute mutation for {key} but key was not found!");
            }
        }
    }

    /// Remote snapshots carry no event metadata, so reasons are inferred by diffing here.
    /// Local updates get theirs directly from the originating `PlayerEvent` instead.
    fn emit_player_update(&self, state: Option<PlayerState>, last_state: Option<&PlayerState>) {
        if self.player_update_sender.receiver_count() == 0
            && self.player_state_sender.receiver_count() == 0
        {
            return;
        }

        let state = state.unwrap_or_else(|| self.connect_state.player().clone());
        let reasons = classify_player_update_reasons(&state, last_state);

        let _ = self.player_state_sender.send(Some(state));
        for reason in reasons {
            self.emit_player_update_event(PlayerUpdateEvent { reason });
        }
    }

    /// No-op without subscribers, so local playback doesn't warn when nobody's listening
    fn emit_player_update_event(&self, event: PlayerUpdateEvent) {
        if self.player_update_sender.receiver_count() == 0 {
            return;
        }

        if let Err(why) = self.player_update_sender.send(event) {
            warn!("couldn't emit player update because: {why}")
        }
    }

    fn emit_cluster_update(&self, event: ClusterUpdateEvent) {
        if self.cluster_update_sender.receiver_count() == 0 {
            return;
        }

        if let Err(why) = self.cluster_update_sender.send(event) {
            warn!("couldn't emit cluster transition because: {why}")
        }
    }

    fn emit_queue_update(&self, event: QueueUpdateEvent) {
        if self.queue_update_sender.receiver_count() == 0 {
            return;
        }

        if let Err(why) = self.queue_update_sender.send(event) {
            warn!("couldn't emit queue update because: {why}")
        }
    }

    fn queue_list_from_player_state(player_state: &PlayerState) -> QueueList {
        QueueList {
            prev_tracks: player_state
                .prev_tracks
                .iter()
                .map(|t| t.uri.clone())
                .collect(),
            next_tracks: player_state
                .next_tracks
                .iter()
                .map(|t| t.uri.clone())
                .collect(),
        }
    }

    fn publish_queue_snapshot(&mut self, player_state: &PlayerState) {
        let queue_list = Self::queue_list_from_player_state(player_state);
        let prev_queue_list = std::mem::replace(&mut self.last_queue_list, queue_list.clone());

        let (prev_changed, next_changed) = queue_lists_changed(&prev_queue_list, &queue_list);

        if prev_changed || next_changed {
            let _ = self.queue_list_sender.send(queue_list);

            if prev_changed {
                self.emit_queue_update(QueueUpdateEvent {
                    reason: QueueUpdateReason::PrevTracksChanged,
                });
            }
            if next_changed {
                self.emit_queue_update(QueueUpdateEvent {
                    reason: QueueUpdateReason::NextTracksChanged,
                });
            }
        }
    }

    fn publish_active_state(&mut self, reason: Option<PlayerUpdateReason>) {
        let player_state = self.connect_state.player().clone();
        let _ = self.player_state_sender.send(Some(player_state.clone()));
        self.publish_queue_snapshot(&player_state);
        self.last_player_state = Some(player_state);

        if let Some(reason) = reason {
            self.emit_player_update_event(PlayerUpdateEvent { reason });
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

        if let Some(mut cluster) = cluster_update.cluster.take() {
            // published before any broadcast below, so a receiver woken by one of those
            // events can trust watch_cluster_state() already reflects it
            let _ = self
                .cluster_state_sender
                .send(build_cluster_state(&cluster));

            if let Ok(reason_enum) = reason {
                match reason_enum {
                    ServerClusterUpdateReason::DEVICE_NEW_CONNECTION
                    | ServerClusterUpdateReason::NEW_DEVICE_APPEARED => {
                        for device_id in &cluster_update.devices_that_changed {
                            self.emit_cluster_update(ClusterUpdateEvent {
                                reason: ClusterUpdateReason::DeviceAppeared,
                                device_id: Some(device_id.clone()),
                            });
                        }
                    }
                    ServerClusterUpdateReason::DEVICES_DISAPPEARED => {
                        for device_id in &cluster_update.devices_that_changed {
                            self.emit_cluster_update(ClusterUpdateEvent {
                                reason: ClusterUpdateReason::DeviceDisappeared,
                                device_id: Some(device_id.clone()),
                            });
                        }
                    }
                    ServerClusterUpdateReason::DEVICE_ALIAS_CHANGED
                    | ServerClusterUpdateReason::DEVICE_VOLUME_CHANGED => {
                        for device_id in &cluster_update.devices_that_changed {
                            self.emit_cluster_update(ClusterUpdateEvent {
                                reason: ClusterUpdateReason::DeviceInfoChanged,
                                device_id: Some(device_id.clone()),
                            });
                        }
                    }
                    ServerClusterUpdateReason::DEVICE_STATE_CHANGED => {
                        for device_id in &cluster_update.devices_that_changed {
                            self.emit_cluster_update(ClusterUpdateEvent {
                                reason: ClusterUpdateReason::DeviceStateChanged,
                                device_id: Some(device_id.clone()),
                            });
                        }
                    }
                    _ => {}
                }
            }

            // Check for active device changes
            let new_active_device_id = if cluster.active_device_id.is_empty() {
                None
            } else {
                Some(cluster.active_device_id.clone())
            };
            if new_active_device_id != self.last_active_device_id {
                self.emit_cluster_update(ClusterUpdateEvent {
                    reason: ClusterUpdateReason::ActiveDeviceChanged,
                    device_id: new_active_device_id.clone(),
                });
                self.last_active_device_id = new_active_device_id;
            }

            let became_inactive = self.connect_state.is_active()
                && cluster.active_device_id != self.session.device_id();
            if became_inactive {
                info!("device became inactive");
                self.handle_disconnect().await?;
                self.handle_stop();
            } else if self.connect_state.is_active() {
                // fixme: workaround fix, because of missing information why it behaves like it does
                //  background: when another device sends a connect-state update, some player's position de-syncs
                //  tried: providing session_id, playback_id, track-metadata "track_player"
                self.update_state = true;
            } else if let Some(state) = cluster.player_state.take() {
                self.publish_queue_snapshot(&state);
                self.emit_player_update(Some(state.clone()), self.last_player_state.as_ref());
                self.last_player_state = Some(state);
            }
        } else if self.connect_state.is_active() {
            self.connect_state.became_inactive(&self.session).await?;
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
                if matches!(update_context.context.uri, Some(ref uri) if uri != self.connect_state.context_uri())
                {
                    debug!(
                        "ignoring context update for <{:?}>, because it isn't the current context <{}>",
                        update_context.context.uri,
                        self.connect_state.context_uri()
                    )
                } else {
                    self.context_resolver.add(ResolveContext::from_context(
                        update_context.context,
                        ContextType::Default,
                        ContextAction::Replace,
                    ))
                }
                return Ok(());
            }
            // modification and update of the connect_state
            Transfer(transfer) => {
                self.handle_transfer(transfer.data.expect("by condition checked"))?;
                return self.notify().await;
            }
            Play(mut play) => {
                if !self.connect_state.is_active() {
                    self.handle_activate()
                }

                let context = match play.context.uri {
                    Some(s) => PlayContext::Uri(s),
                    None if !play.context.pages.is_empty() => PlayContext::Tracks(
                        play.context
                            .pages
                            .iter()
                            .cloned()
                            .flat_map(|p| p.tracks)
                            .flat_map(|t| t.uri)
                            .collect(),
                    ),
                    None => Err(SpircError::NoUri("context"))?,
                };

                let context_options = play
                    .options
                    .player_options_override
                    .map(Into::into)
                    .map(LoadContextOptions::Options);

                let fallback_index = play
                    .options
                    .skip_to
                    .as_ref()
                    .and_then(|s| s.track_index)
                    .map(|i| i as usize);

                self.handle_load(
                    LoadRequest {
                        context,
                        options: LoadRequestOptions {
                            start_playing: true,
                            seek_to: play.options.seek_to.unwrap_or_default(),
                            playing_track: play.options.skip_to.and_then(|s| s.try_into().ok()),
                            context_options,
                        },
                    },
                    play.context.pages.pop(),
                    fallback_index,
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
            SetShufflingContext(shuffle) => self.handle_shuffle(shuffle.value)?,
            SetRepeatingContext(repeat_context) => {
                self.handle_repeat_context(repeat_context.value)?
            }
            SetRepeatingTrack(repeat_track) => self.handle_repeat_track(repeat_track.value),
            AddToQueue(add_to_queue) => {
                self.connect_state.add_to_queue(add_to_queue.track, true);
                self.emit_set_queue_event();
            }
            SetQueue(set_queue) => {
                self.connect_state.handle_set_queue(set_queue);
                self.emit_set_queue_event();
            }
            SetOptions(set_options) => {
                if let Some(repeat_context) = set_options.repeating_context {
                    self.handle_repeat_context(repeat_context)?
                }

                if let Some(repeat_track) = set_options.repeating_track {
                    self.handle_repeat_track(repeat_track)
                }

                let shuffle = set_options.shuffling_context;
                if let Some(shuffle) = shuffle {
                    self.handle_shuffle(shuffle)?;
                }
            }
            SkipNext(skip_next) => self.handle_next(skip_next.track.map(|t| t.uri))?,
            SkipPrev(_) => self.handle_prev()?,
            Resume(_) if matches!(self.play_status, SpircPlayStatus::Stopped) => {
                self.load_track(true, 0)?
            }
            Resume(_) => self.handle_play(),
        }

        self.update_state = true;
        Ok(())
    }

    fn handle_transfer(&mut self, mut transfer: TransferState) -> Result<(), Error> {
        let mut ctx_uri = match transfer.current_session.context.uri {
            None => Err(SpircError::NoUri("transfer context"))?,
            // can apparently happen when a state is transferred and was started with "uris" via the api
            Some(ref uri) if uri == "-" || uri.is_empty() => None,
            Some(ref uri) => Some(uri.clone()),
        };

        self.connect_state.reset_context(
            ctx_uri
                .as_deref()
                .map(ResetContext::WhenDifferent)
                .unwrap_or(ResetContext::Completely),
        );

        match self.connect_state.current_track_from_transfer(&transfer) {
            Err(why) => warn!("didn't find initial track: {why}"),
            Ok(track) => {
                debug!("found initial track <{}>", track.uri);
                self.connect_state.set_track(track)
            }
        };

        let autoplay = self.connect_state.current_track(|t| t.is_autoplay());
        if autoplay {
            ctx_uri = ctx_uri.map(|c| c.replace("station:", ""));
        }

        let fallback = self.connect_state.current_track(|t| &t.uri).clone();
        let load_from_context_uri = ctx_uri.is_some();

        match ctx_uri {
            Some(ref uri) => {
                self.context_resolver.add(ResolveContext::from_uri(
                    uri.clone(),
                    &fallback,
                    ContextType::Default,
                    ContextAction::Replace,
                ));
            }
            None => {
                let all_tracks = transfer
                    .current_session
                    .context
                    .pages
                    .iter()
                    .cloned()
                    .flat_map(|p| p.tracks)
                    .collect::<Vec<_>>();

                if !all_tracks.is_empty() {
                    self.load_context_from_tracks(all_tracks)?;
                } else {
                    warn!(
                        "tried to transfer with an invalid state, using fallback as ctx_uri ({fallback})"
                    );
                    ctx_uri = Some(fallback.clone())
                }
            }
        };

        self.handle_activate();

        let timestamp = self.now_ms();
        let state = &mut self.connect_state;
        state.handle_initial_transfer(&mut transfer, ctx_uri.clone());

        // adjust active context, so resolve knows for which context it should set up the state
        state.active_context = if autoplay {
            ContextType::Autoplay
        } else {
            ContextType::Default
        };

        // update position if the track continued playing
        let transfer_timestamp = transfer.playback.timestamp.unwrap_or_default();
        let position = match transfer.playback.position_as_of_timestamp {
            Some(position) if transfer.playback.is_paused.unwrap_or_default() => position.into(),
            // update position if the track continued playing
            Some(position) if position > 0 => {
                let time_since_position_update = timestamp - transfer_timestamp;
                i64::from(position) + time_since_position_update
            }
            _ => 0,
        };

        let is_playing = !transfer.playback.is_paused();

        if self.connect_state.current_track(|t| t.is_autoplay()) || autoplay {
            if let Some(ctx_uri) = ctx_uri {
                debug!("currently in autoplay context, async resolving autoplay for {ctx_uri}");
                self.context_resolver.add(ResolveContext::from_uri(
                    ctx_uri,
                    fallback,
                    ContextType::Autoplay,
                    ContextAction::Replace,
                ))
            } else {
                warn!("couldn't resolve autoplay context without a context uri");
            }
        }

        if load_from_context_uri {
            self.transfer_state = Some(transfer);
        } else {
            match self.connect_state.get_context(ContextType::Default) {
                Err(why) => {
                    warn!("continuing transfer in an unknown state. {why}");
                    self.transfer_state = Some(transfer);
                }
                Ok(ctx) => {
                    let idx = ConnectState::find_index_in_context(ctx, |pt| {
                        self.connect_state.current_track(|t| pt.uri == t.uri)
                    })?;
                    self.connect_state.reset_playback_to_position(Some(idx))?;
                }
            }
        }

        self.load_track(is_playing, position.try_into()?)
    }

    async fn handle_disconnect(&mut self) -> Result<(), Error> {
        self.context_resolver.clear();

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
        self.connect_state.clear_next_tracks();

        if let Err(why) = self.connect_state.reset_playback_to_position(None) {
            warn!("failed filling up next_track during stopping: {why}")
        }
    }

    fn handle_activate(&mut self) {
        self.connect_state.set_active(true);
        self.publish_local_activation(true);
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
        cmd: LoadRequest,
        page: Option<ContextPage>,
        fallback_index: Option<usize>,
    ) -> Result<(), Error> {
        self.connect_state
            .reset_context(if let PlayContext::Uri(ref uri) = cmd.context {
                ResetContext::WhenDifferent(uri)
            } else {
                ResetContext::Completely
            });

        self.connect_state.reset_options();

        let autoplay = matches!(cmd.context_options, Some(LoadContextOptions::Autoplay));
        match cmd.context {
            PlayContext::Uri(uri) => {
                self.load_context_from_uri(uri, page.as_ref(), autoplay)
                    .await?
            }
            PlayContext::Tracks(tracks) => self.load_context_from_tracks(tracks)?,
        }

        let cmd_options = cmd.options;

        self.connect_state.set_active_context(ContextType::Default);

        // for play commands with skip by uid, the context of the command contains
        // tracks with uri and uid, so we merge the new context with the resolved/existing context
        self.connect_state.merge_context(page);

        // load here, so that we clear the queue only after we definitely retrieved a new context
        self.connect_state.clear_next_tracks();
        self.connect_state.clear_restrictions();

        debug!("play track <{:?}>", cmd_options.playing_track);

        let index = match cmd_options.playing_track {
            None => None,
            Some(ref playing_track) => Some(match playing_track {
                PlayingTrack::Index(i) => Ok(*i as usize),
                PlayingTrack::Uri(uri) => {
                    let ctx = self.connect_state.get_context(ContextType::Default)?;
                    ConnectState::find_index_in_context(ctx, |t| &t.uri == uri)
                }
                PlayingTrack::Uid(uid) => {
                    let ctx = self.connect_state.get_context(ContextType::Default)?;
                    ConnectState::find_index_in_context(ctx, |t| &t.uid == uid)
                }
            }),
        }
        .map(|i| {
            i.unwrap_or_else(|why| {
                warn!(
                    "Failed to resolve index by {:?}, using fallback index: {:?} (Error: {why})",
                    cmd_options.playing_track, fallback_index
                );
                fallback_index.unwrap_or_default()
            })
        });

        if let Some(LoadContextOptions::Options(ref options)) = cmd_options.context_options {
            debug!(
                "loading with shuffle: <{}>, repeat track: <{}> context: <{}>",
                options.shuffle, options.repeat, options.repeat_track
            );

            self.connect_state.set_shuffle(options.shuffle);
            self.connect_state.set_repeat_context(options.repeat);
            self.connect_state.set_repeat_track(options.repeat_track);
        }

        if matches!(cmd_options.context_options, Some(LoadContextOptions::Options(ref o)) if o.shuffle)
        {
            if let Some(index) = index {
                self.connect_state.set_current_track(index)?;
            } else {
                self.connect_state.set_current_track_random()?;
            }

            if self.context_resolver.has_next() {
                self.connect_state.update_queue_revision()
            } else {
                self.connect_state.shuffle_new()?;
                self.add_autoplay_resolving_when_required();
            }
        } else {
            self.connect_state
                .set_current_track(index.unwrap_or_default())?;
            self.connect_state.reset_playback_to_position(index)?;
            self.add_autoplay_resolving_when_required();
        }

        if self.connect_state.current_track(MessageField::is_some) {
            self.load_track(cmd_options.start_playing, cmd_options.seek_to)?;
        } else {
            info!("No active track, stopping");
            self.handle_stop()
        }

        Ok(())
    }

    async fn load_context_from_uri(
        &mut self,
        context_uri: String,
        page: Option<&ContextPage>,
        autoplay: bool,
    ) -> Result<(), Error> {
        if !self.connect_state.is_active() {
            self.handle_activate();
        }

        let update_context = if autoplay {
            ContextType::Autoplay
        } else {
            ContextType::Default
        };

        self.connect_state.set_active_context(update_context);

        let fallback = match page {
            // check that the uri is valid or the page has a valid uri that can be used
            Some(page) => match ConnectState::find_valid_uri(Some(&context_uri), Some(page)) {
                Some(ctx_uri) => ctx_uri,
                None => return Err(SpircError::InvalidUri(context_uri).into()),
            },
            // when there is no page, the uri should be valid
            None => &context_uri,
        };

        let current_context_uri = self.connect_state.context_uri();

        if current_context_uri == &context_uri && fallback == context_uri {
            debug!("context <{current_context_uri}> didn't change, no resolving required")
        } else {
            debug!("resolving context for load command");
            self.context_resolver.clear();
            self.context_resolver.add(ResolveContext::from_uri(
                &context_uri,
                fallback,
                update_context,
                ContextAction::Replace,
            ));
            let context = self.context_resolver.get_next_context(Vec::new).await;
            self.handle_next_context(context);
        }

        Ok(())
    }

    fn load_context_from_tracks(&mut self, tracks: impl Into<ContextPage>) -> Result<(), Error> {
        const WEB_API_URI: &str = "spotify:web-api";
        let ctx = Context {
            // by providing values for uri/url the player in the official client's isn't frozen
            uri: Some(WEB_API_URI.into()),
            url: Some(format!("context://{WEB_API_URI}")),
            pages: vec![tracks.into()],
            ..Default::default()
        };

        let _ = self
            .connect_state
            .update_context(ctx, ContextType::Default)?;

        self.emit_set_queue_event();

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
        let duration = self.connect_state.player().duration;
        if i64::from(position_ms) > duration {
            warn!("tried to seek to {position_ms}ms of {duration}ms");
            return;
        }

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

    fn handle_shuffle(&mut self, shuffle: bool) -> Result<(), Error> {
        self.player.emit_shuffle_changed_event(shuffle);
        self.connect_state.handle_shuffle(shuffle)
    }

    fn handle_repeat_context(&mut self, repeat: bool) -> Result<(), Error> {
        self.player
            .emit_repeat_changed_event(repeat, self.connect_state.repeat_track());
        self.connect_state.handle_set_repeat_context(repeat)
    }

    fn handle_repeat_track(&mut self, repeat: bool) {
        self.player
            .emit_repeat_changed_event(self.connect_state.repeat_context(), repeat);
        self.connect_state.set_repeat_track(repeat);
    }

    async fn handle_add_to_queue(&mut self, uri: SpotifyUri) {
        let track_uris: Vec<String> = match uri {
            SpotifyUri::Track { .. } | SpotifyUri::Episode { .. } => vec![uri.to_uri()],
            SpotifyUri::Album { .. } | SpotifyUri::Playlist { .. } => {
                match self.session.spclient().get_context(&uri.to_uri()).await {
                    Ok(context) => context
                        .pages
                        .iter()
                        .flat_map(|page| page.tracks.iter())
                        .filter_map(|track| track.uri.clone())
                        .collect(),
                    Err(e) => {
                        error!("failed to resolve context for {}: {e}", uri.item_type());
                        return;
                    }
                }
            }
            _ => return,
        };

        for track_uri in track_uris {
            let track = ProvidedTrack {
                uri: track_uri.clone(),
                ..Default::default()
            };
            self.connect_state.add_to_queue(track, true);
        }
        self.emit_set_queue_event();
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
    fn handle_unavailable(&mut self, track_id: &SpotifyUri) -> Result<(), Error> {
        self.connect_state.mark_unavailable(track_id)?;
        self.handle_preload_next_track();

        Ok(())
    }

    fn add_autoplay_resolving_when_required(&mut self) {
        let require_load_new = !self
            .connect_state
            .has_next_tracks(Some(CONTEXT_FETCH_THRESHOLD))
            && self.session.autoplay()
            && !self.connect_state.context_uri().is_empty();

        if !require_load_new {
            return;
        }

        let current_context = self.connect_state.context_uri();
        let fallback = self.connect_state.current_track(|t| &t.uri);

        let has_tracks = self
            .connect_state
            .get_context(ContextType::Autoplay)
            .map(|c| !c.tracks.is_empty())
            .unwrap_or_default();

        let resolve = ResolveContext::from_uri(
            current_context,
            fallback,
            ContextType::Autoplay,
            if has_tracks {
                ContextAction::Append
            } else {
                ContextAction::Replace
            },
        );

        self.context_resolver.add(resolve);
    }

    fn handle_next(&mut self, track_uri: Option<String>) -> Result<(), Error> {
        let continue_playing = self.connect_state.is_playing();

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

        if has_next_track {
            self.add_autoplay_resolving_when_required();
            self.load_track(continue_playing, 0)
        } else {
            info!("Not playing next track because there are no more tracks left in queue.");
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
                Some(_) => self.load_track(self.connect_state.is_playing(), 0)?,
            }
        } else {
            self.handle_seek(0);
        }

        Ok(())
    }

    fn handle_volume_up(&mut self) {
        let volume = (self.connect_state.device_info().volume as u16)
            .saturating_add(self.connect_state.volume_step_size);

        self.set_volume(volume);
    }

    fn handle_volume_down(&mut self) {
        let volume = (self.connect_state.device_info().volume as u16)
            .saturating_sub(self.connect_state.volume_step_size);

        self.set_volume(volume);
    }

    fn handle_playlist_modification(
        &mut self,
        playlist_modification_info: PlaylistModificationInfo,
    ) -> Result<(), Error> {
        let uri = playlist_modification_info
            .uri
            .ok_or(SpircError::NoUri("playlist modification"))?;
        let uri = String::from_utf8(uri)?;

        if self.connect_state.context_uri() != &uri {
            debug!(
                "ignoring playlist modification update for playlist <{uri}>, because it isn't the current context"
            );
            return Ok(());
        }

        debug!("playlist modification for current context: {uri}");
        self.context_resolver.add(ResolveContext::from_uri(
            uri,
            self.connect_state.current_track(|t| &t.uri),
            ContextType::Default,
            ContextAction::Replace,
        ));

        Ok(())
    }

    fn handle_session_update(&mut self, session_update: FallbackWrapper<SessionUpdate>) {
        // we know that this enum value isn't present in our current proto definitions, by that
        // the json parsing fails because the enum isn't known as proto representation
        const WBC: &str = "WIFI_BROADCAST_CHANGED";

        let mut session_update = match session_update {
            FallbackWrapper::Inner(update) => update,
            FallbackWrapper::Fallback(value) => {
                let fallback_inner = value.to_string();
                if fallback_inner.contains(WBC) {
                    log::debug!("Received SessionUpdate::{WBC}");
                } else {
                    log::warn!("SessionUpdate couldn't be parse correctly: {value:?}");
                }
                return;
            }
        };

        let reason = session_update.reason.enum_value();

        let mut session = match session_update.session.take() {
            None => return,
            Some(session) => session,
        };

        let active_device = session.host_active_device_id.take();
        if matches!(active_device, Some(ref device) if device == self.session.device_id()) {
            info!(
                "session update: <{:?}> for self, current session_id {}, new session_id {}",
                reason,
                self.session.session_id(),
                session.session_id
            );

            if self.session.session_id() != session.session_id {
                self.session.set_session_id(&session.session_id);
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
        let id = SpotifyUri::from_uri(current_uri)?;
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

        if self.connect_state.is_playing() {
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
        debug!("SpircTask::set_volume({volume})");

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

            // reflect locally now; the server only echoes this back on its own schedule
            let (device_id, info) = self.own_device_info(self.connect_state.is_active());
            self.cluster_state_sender.send_modify(|state| {
                state.devices.insert(device_id.clone(), info);
            });
            self.emit_cluster_update(ClusterUpdateEvent {
                reason: ClusterUpdateReason::DeviceInfoChanged,
                device_id: Some(device_id),
            });
        }
    }

    fn own_device_info(&self, is_active: bool) -> (String, DeviceInfo) {
        let device_id = self.session.device_id().to_string();
        let device_info = self.connect_state.device_info();
        let info = DeviceInfo {
            device_id: device_id.clone(),
            device_alias: device_info.name.clone(),
            device_type: device_info.device_type.enum_value_or_default(),
            volume: device_info.volume,
            is_active,
        };
        (device_id, info)
    }

    /// Local echo of an activation change, same rationale as `set_volume`'s
    fn publish_local_activation(&mut self, active: bool) {
        let (device_id, info) = self.own_device_info(active);
        let new_active_device_id = active.then(|| device_id.clone());

        self.cluster_state_sender
            .send_modify(|state| apply_local_activation(state, device_id, info, active));

        if new_active_device_id != self.last_active_device_id {
            self.last_active_device_id = new_active_device_id.clone();
            self.emit_cluster_update(ClusterUpdateEvent {
                reason: ClusterUpdateReason::ActiveDeviceChanged,
                device_id: new_active_device_id,
            });
        }
    }
}

impl Drop for SpircTask {
    fn drop(&mut self) {
        debug!("drop Spirc[{}]", self.spirc_id);
    }
}

/// Returns whether the prev/next track lists differ between two `QueueList`s
fn queue_lists_changed(prev: &QueueList, next: &QueueList) -> (bool, bool) {
    (
        prev.prev_tracks != next.prev_tracks,
        prev.next_tracks != next.next_tracks,
    )
}

/// Upserts one device's entry into a `ClusterState`, clearing `is_active` on whichever
/// device previously held it
fn apply_local_activation(
    state: &mut ClusterState,
    device_id: String,
    info: DeviceInfo,
    active: bool,
) {
    if let Some(prev_id) = &state.active_device_id {
        if let Some(prev) = state.devices.get_mut(prev_id) {
            prev.is_active = false;
        }
    }
    state.active_device_id = active.then(|| device_id.clone());
    state.devices.insert(device_id, info);
}

/// Builds the device map and active device id from a server `Cluster` snapshot
fn build_cluster_state(cluster: &Cluster) -> ClusterState {
    let devices = cluster
        .device
        .values()
        .map(|device| {
            let info = DeviceInfo {
                device_id: device.device_id.clone(),
                device_alias: device.name.clone(),
                device_type: device.device_type.enum_value_or_default(),
                volume: device.volume,
                is_active: device.device_id == cluster.active_device_id,
            };
            (info.device_id.clone(), info)
        })
        .collect();

    let active_device_id = if cluster.active_device_id.is_empty() {
        None
    } else {
        Some(cluster.active_device_id.clone())
    };

    ClusterState {
        devices,
        active_device_id,
    }
}

/// Every way a remote `PlayerState` snapshot differs from the last one seen
fn classify_player_update_reasons(
    state: &PlayerState,
    last_state: Option<&PlayerState>,
) -> Vec<PlayerUpdateReason> {
    let Some(last) = last_state else {
        return vec![PlayerUpdateReason::Other];
    };

    let mut reasons = Vec::new();

    let new_track_uri = state.track.as_ref().map(|t| t.uri.as_str());
    let old_track_uri = last.track.as_ref().map(|t| t.uri.as_str());
    let track_changed = new_track_uri != old_track_uri;
    if track_changed {
        reasons.push(PlayerUpdateReason::TrackChanged);
    }

    let new_is_playing = state.is_playing && !state.is_paused;
    let old_is_playing = last.is_playing && !last.is_paused;
    if new_is_playing != old_is_playing {
        reasons.push(PlayerUpdateReason::PlayPauseChanged);
    }

    let shuffle = |s: &PlayerState| s.options.as_ref().map(|o| o.shuffling_context);
    if shuffle(state) != shuffle(last) {
        reasons.push(PlayerUpdateReason::ShuffleChanged);
    }

    let repeat = |s: &PlayerState| {
        s.options
            .as_ref()
            .map(|o| (o.repeating_context, o.repeating_track))
    };
    if repeat(state) != repeat(last) {
        reasons.push(PlayerUpdateReason::RepeatChanged);
    }

    if state.context_uri != last.context_uri {
        reasons.push(PlayerUpdateReason::ContextChanged);
    }

    // seek doesn't apply across a track change: position naturally resets then
    if !track_changed {
        let time_diff = state.timestamp.saturating_sub(last.timestamp);
        let expected_position = if state.is_playing {
            last.position_as_of_timestamp + time_diff
        } else {
            last.position_as_of_timestamp
        };
        let position_delta = (state.position_as_of_timestamp - expected_position).abs();

        if position_delta > 5000 {
            reasons.push(PlayerUpdateReason::SeekChanged);
        }
    }

    if reasons.is_empty() {
        reasons.push(PlayerUpdateReason::Other);
    }

    reasons
}

#[cfg(test)]
mod tests {
    use super::*;

    fn state_with(f: impl FnOnce(&mut PlayerState)) -> PlayerState {
        let mut state = PlayerState::default();
        f(&mut state);
        state
    }

    fn track(uri: &str) -> ProvidedTrack {
        let mut t = ProvidedTrack::new();
        t.uri = uri.to_string();
        t
    }

    #[test]
    fn no_previous_state_is_other() {
        let state = PlayerState::default();
        assert_eq!(
            classify_player_update_reasons(&state, None),
            vec![PlayerUpdateReason::Other]
        );
    }

    #[test]
    fn track_change_alone() {
        let last = state_with(|s| {
            s.track = MessageField::some(track("spotify:track:a"));
            s.is_playing = true;
        });
        let next = state_with(|s| {
            s.track = MessageField::some(track("spotify:track:b"));
            s.is_playing = true;
        });
        assert_eq!(
            classify_player_update_reasons(&next, Some(&last)),
            vec![PlayerUpdateReason::TrackChanged]
        );
    }

    #[test]
    fn track_and_play_pause_both_reported() {
        let last = state_with(|s| {
            s.track = MessageField::some(track("spotify:track:a"));
            s.is_playing = true;
        });
        let next = state_with(|s| {
            s.track = MessageField::some(track("spotify:track:b"));
            s.is_playing = false;
        });
        assert_eq!(
            classify_player_update_reasons(&next, Some(&last)),
            vec![
                PlayerUpdateReason::TrackChanged,
                PlayerUpdateReason::PlayPauseChanged,
            ]
        );
    }

    #[test]
    fn play_pause_change_detected() {
        let last = state_with(|s| s.is_playing = false);
        let next = state_with(|s| s.is_playing = true);
        assert_eq!(
            classify_player_update_reasons(&next, Some(&last)),
            vec![PlayerUpdateReason::PlayPauseChanged]
        );
    }

    #[test]
    fn seek_detected_beyond_natural_progress() {
        let last = state_with(|s| {
            s.is_playing = true;
            s.timestamp = 1_000;
            s.position_as_of_timestamp = 10_000;
        });
        let next = state_with(|s| {
            s.is_playing = true;
            s.timestamp = 2_000;
            s.position_as_of_timestamp = 50_000;
        });
        assert_eq!(
            classify_player_update_reasons(&next, Some(&last)),
            vec![PlayerUpdateReason::SeekChanged]
        );
    }

    #[test]
    fn natural_progress_is_not_a_seek() {
        let last = state_with(|s| {
            s.is_playing = true;
            s.timestamp = 1_000;
            s.position_as_of_timestamp = 10_000;
        });
        let next = state_with(|s| {
            s.is_playing = true;
            s.timestamp = 2_000;
            s.position_as_of_timestamp = 11_000;
        });
        assert_eq!(
            classify_player_update_reasons(&next, Some(&last)),
            vec![PlayerUpdateReason::Other]
        );
    }

    #[test]
    fn track_change_suppresses_seek() {
        // position naturally resets on a track change; that's not a seek
        let last = state_with(|s| {
            s.track = MessageField::some(track("spotify:track:a"));
            s.is_playing = true;
            s.timestamp = 1_000;
            s.position_as_of_timestamp = 100_000;
        });
        let next = state_with(|s| {
            s.track = MessageField::some(track("spotify:track:b"));
            s.is_playing = true;
            s.timestamp = 2_000;
            s.position_as_of_timestamp = 0;
        });
        assert_eq!(
            classify_player_update_reasons(&next, Some(&last)),
            vec![PlayerUpdateReason::TrackChanged]
        );
    }

    fn queue_list(prev: &[&str], next: &[&str]) -> QueueList {
        QueueList {
            prev_tracks: prev.iter().map(ToString::to_string).collect(),
            next_tracks: next.iter().map(ToString::to_string).collect(),
        }
    }

    #[test]
    fn queue_lists_changed_detects_no_change() {
        let a = queue_list(&["a"], &["b"]);
        let b = queue_list(&["a"], &["b"]);
        assert_eq!(queue_lists_changed(&a, &b), (false, false));
    }

    #[test]
    fn queue_lists_changed_detects_prev_and_next_independently() {
        let a = queue_list(&["a"], &["b"]);
        assert_eq!(
            queue_lists_changed(&a, &queue_list(&["z"], &["b"])),
            (true, false)
        );
        assert_eq!(
            queue_lists_changed(&a, &queue_list(&["a"], &["z"])),
            (false, true)
        );
        assert_eq!(
            queue_lists_changed(&a, &queue_list(&["z"], &["z"])),
            (true, true)
        );
    }

    fn device(
        id: &str,
        name: &str,
        volume: u32,
        device_type: DeviceType,
    ) -> crate::protocol::connect::DeviceInfo {
        let mut d = crate::protocol::connect::DeviceInfo::new();
        d.device_id = id.to_string();
        d.name = name.to_string();
        d.volume = volume;
        d.device_type = ::protobuf::EnumOrUnknown::new(device_type);
        d
    }

    #[test]
    fn build_cluster_state_maps_devices_and_marks_active() {
        let mut cluster = Cluster::new();
        cluster.active_device_id = "device-1".to_string();
        cluster.device.insert(
            "device-1".to_string(),
            device("device-1", "Kitchen", 50, DeviceType::SPEAKER),
        );
        cluster.device.insert(
            "device-2".to_string(),
            device("device-2", "Phone", 80, DeviceType::SMARTPHONE),
        );

        let state = build_cluster_state(&cluster);

        assert_eq!(state.active_device_id.as_deref(), Some("device-1"));
        assert_eq!(state.devices.len(), 2);
        assert!(state.devices["device-1"].is_active);
        assert!(!state.devices["device-2"].is_active);
        assert_eq!(state.devices["device-2"].volume, 80);
        assert_eq!(
            state.devices["device-2"].device_type,
            DeviceType::SMARTPHONE
        );
    }

    #[test]
    fn build_cluster_state_no_active_device_is_none() {
        let cluster = Cluster::new();
        let state = build_cluster_state(&cluster);
        assert_eq!(state.active_device_id, None);
        assert!(state.devices.is_empty());
    }

    fn device_info(device_id: &str, is_active: bool) -> DeviceInfo {
        DeviceInfo {
            device_id: device_id.to_string(),
            device_alias: "test device".to_string(),
            device_type: DeviceType::COMPUTER,
            volume: 50,
            is_active,
        }
    }

    #[test]
    fn apply_local_activation_marks_device_active_and_clears_previous() {
        let mut state = ClusterState {
            devices: [("old".to_string(), device_info("old", true))].into(),
            active_device_id: Some("old".to_string()),
        };

        apply_local_activation(
            &mut state,
            "new".to_string(),
            device_info("new", true),
            true,
        );

        assert_eq!(state.active_device_id.as_deref(), Some("new"));
        assert!(!state.devices["old"].is_active);
        assert!(state.devices["new"].is_active);
    }

    #[test]
    fn apply_local_activation_deactivation_clears_active_device_id() {
        let mut state = ClusterState {
            devices: [("me".to_string(), device_info("me", true))].into(),
            active_device_id: Some("me".to_string()),
        };

        apply_local_activation(
            &mut state,
            "me".to_string(),
            device_info("me", false),
            false,
        );

        assert_eq!(state.active_device_id, None);
        assert!(!state.devices["me"].is_active);
    }
}
