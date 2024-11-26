pub(super) mod context;
mod handle;
mod metadata;
mod options;
pub(super) mod provider;
mod restrictions;
mod tracks;
mod transfer;

use crate::model::SpircPlayStatus;
use crate::state::context::{ContextType, StateContext};
use crate::state::metadata::Metadata;
use crate::state::provider::{IsProvider, Provider};
use librespot_core::config::DeviceType;
use librespot_core::date::Date;
use librespot_core::dealer::protocol::Request;
use librespot_core::spclient::SpClientResult;
use librespot_core::{version, Error, Session, SpotifyId};
use librespot_protocol::connect::{
    Capabilities, Device, DeviceInfo, MemberType, PutStateReason, PutStateRequest,
};
use librespot_protocol::player::{
    ContextIndex, ContextPage, ContextPlayerOptions, PlayOrigin, PlayerState, ProvidedTrack,
    Suppressions,
};
use log::LevelFilter;
use protobuf::{EnumOrUnknown, MessageField};
use std::collections::{hash_map::DefaultHasher, VecDeque};
use std::hash::{Hash, Hasher};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use thiserror::Error;

// these limitations are essential, otherwise to many tracks will overload the web-player
const SPOTIFY_MAX_PREV_TRACKS_SIZE: usize = 10;
const SPOTIFY_MAX_NEXT_TRACKS_SIZE: usize = 80;

#[derive(Debug, Error)]
pub enum StateError {
    #[error("the current track couldn't be resolved from the transfer state")]
    CouldNotResolveTrackFromTransfer,
    #[error("message field {0} was not available")]
    MessageFieldNone(String),
    #[error("context is not available. shuffle: {0:?}")]
    NoContext(ContextType),
    #[error("could not find track {0:?} in context of {1}")]
    CanNotFindTrackInContext(Option<usize>, usize),
    #[error("currently {action} is not allowed because {reason}")]
    CurrentlyDisallowed { action: String, reason: String },
    #[error("the provided context has no tracks")]
    ContextHasNoTracks,
    #[error("playback of local files is not supported")]
    UnsupportedLocalPlayBack,
}

impl From<StateError> for Error {
    fn from(err: StateError) -> Self {
        use StateError::*;
        match err {
            CouldNotResolveTrackFromTransfer
            | MessageFieldNone(_)
            | NoContext(_)
            | CanNotFindTrackInContext(_, _)
            | ContextHasNoTracks => Error::failed_precondition(err),
            CurrentlyDisallowed { .. } | UnsupportedLocalPlayBack => Error::unavailable(err),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConnectStateConfig {
    pub session_id: String,
    pub initial_volume: u32,
    pub name: String,
    pub device_type: DeviceType,
    pub zeroconf_enabled: bool,
    pub volume_steps: i32,
    pub is_group: bool,
}

impl Default for ConnectStateConfig {
    fn default() -> Self {
        Self {
            session_id: String::new(),
            initial_volume: u32::from(u16::MAX) / 2,
            name: "librespot".to_string(),
            device_type: DeviceType::Speaker,
            zeroconf_enabled: false,
            volume_steps: 64,
            is_group: false,
        }
    }
}

#[derive(Default, Debug)]
pub struct ConnectState {
    pub session_id: String,
    pub active: bool,
    pub active_since: Option<SystemTime>,

    pub has_been_playing_for: Option<Instant>,

    pub device: DeviceInfo,

    unavailable_uri: Vec<String>,

    /// index: 0 based, so the first track is index 0
    player: PlayerState,

    // we don't work directly on the track lists of the player state, because
    // we mostly need to push and pop at the beginning of them
    /// bottom => top, aka the last track of the list is the prev track
    prev_tracks: VecDeque<ProvidedTrack>,
    /// top => bottom, aka the first track of the list is the next track
    next_tracks: VecDeque<ProvidedTrack>,

    pub active_context: ContextType,
    /// the context from which we play, is used to top up prev and next tracks
    /// the index is used to keep track which tracks are already loaded into next tracks
    pub context: Option<StateContext>,
    /// upcoming contexts, usually directly provided by the context-resolver
    pub next_contexts: Vec<ContextPage>,
    /// a context to keep track of our shuffled context, should be only available when option.shuffling_context is true
    pub shuffle_context: Option<StateContext>,
    /// a context to keep track of the autoplay context
    pub autoplay_context: Option<StateContext>,

    pub queue_count: u64,

    pub last_command: Option<Request>,
}

impl ConnectState {
    pub fn new(cfg: ConnectStateConfig, session: &Session) -> Self {
        let mut state = Self {
            session_id: cfg.session_id,
            device: DeviceInfo {
                can_play: true,
                volume: cfg.initial_volume,
                name: cfg.name,
                device_id: session.device_id().to_string(),
                device_type: EnumOrUnknown::new(cfg.device_type.into()),
                device_software_version: version::SEMVER.to_string(),
                spirc_version: version::SPOTIFY_SPIRC_VERSION.to_string(),
                client_id: session.client_id(),
                is_group: cfg.is_group,
                capabilities: MessageField::some(Capabilities {
                    volume_steps: cfg.volume_steps,
                    hidden: false, // could be exposed later to only observe the playback
                    gaia_eq_connect_id: true,
                    can_be_player: true,

                    needs_full_player_state: true,

                    is_observable: true,
                    is_controllable: true,

                    supports_gzip_pushes: true,
                    supports_logout: cfg.zeroconf_enabled,
                    supported_types: vec!["audio/episode".into(), "audio/track".into()],
                    supports_playlist_v2: true,
                    supports_transfer_command: true,
                    supports_command_request: true,
                    supports_set_options_command: true,

                    is_voice_enabled: false,
                    restrict_to_local: false,
                    disable_volume: false,
                    connect_disabled: false,
                    supports_rename: false,
                    supports_external_episodes: false,
                    supports_set_backend_metadata: false,
                    supports_hifi: MessageField::none(),

                    command_acks: true,
                    ..Default::default()
                }),
                ..Default::default()
            },
            // + 1, so that we have a buffer where we can swap elements
            prev_tracks: VecDeque::with_capacity(SPOTIFY_MAX_PREV_TRACKS_SIZE + 1),
            next_tracks: VecDeque::with_capacity(SPOTIFY_MAX_NEXT_TRACKS_SIZE + 1),
            ..Default::default()
        };
        state.reset();
        state
    }

    pub fn reset(&mut self) {
        self.set_active(false);
        self.queue_count = 0;

        self.player = PlayerState {
            session_id: self.session_id.clone(),
            is_system_initiated: true,
            playback_speed: 1.,
            play_origin: MessageField::some(PlayOrigin::new()),
            suppressions: MessageField::some(Suppressions::new()),
            options: MessageField::some(ContextPlayerOptions::new()),
            ..Default::default()
        }
    }

    pub fn set_active(&mut self, value: bool) {
        if value {
            if self.active {
                return;
            }

            self.active = true;
            self.active_since = Some(SystemTime::now())
        } else {
            self.active = false;
            self.active_since = None
        }
    }

    pub fn set_origin(&mut self, origin: PlayOrigin) {
        self.player.play_origin = MessageField::some(origin)
    }

    pub fn set_session_id(&mut self, session_id: String) {
        self.session_id = session_id.clone();
        self.player.session_id = session_id;
    }

    pub(crate) fn set_status(&mut self, status: &SpircPlayStatus) {
        self.player.is_paused = matches!(
            status,
            SpircPlayStatus::LoadingPause { .. }
                | SpircPlayStatus::Paused { .. }
                | SpircPlayStatus::Stopped
        );

        // desktop and mobile want all 'states' set to true, when we are paused,
        // otherwise the play button (desktop) is grayed out or the preview (mobile) can't be opened
        self.player.is_buffering = self.player.is_paused
            || matches!(
                status,
                SpircPlayStatus::LoadingPause { .. } | SpircPlayStatus::LoadingPlay { .. }
            );
        self.player.is_playing = self.player.is_paused
            || matches!(
                status,
                SpircPlayStatus::LoadingPlay { .. } | SpircPlayStatus::Playing { .. }
            );

        debug!(
            "updated connect play status playing: {}, paused: {}, buffering: {}",
            self.player.is_playing, self.player.is_paused, self.player.is_buffering
        );

        self.update_restrictions()
    }

    pub fn update_current_index(&mut self, f: impl Fn(&mut ContextIndex)) {
        match self.player.index.as_mut() {
            Some(player_index) => f(player_index),
            None => {
                let mut new_index = ContextIndex::new();
                f(&mut new_index);
                self.player.index = MessageField::some(new_index)
            }
        }
    }

    pub fn update_position(&mut self, position_ms: u32, timestamp: i64) {
        self.player.position_as_of_timestamp = position_ms.into();
        self.player.timestamp = timestamp;
    }

    pub fn update_duration(&mut self, duration: u32) {
        self.player.duration = duration.into()
    }

    pub fn update_queue_revision(&mut self) {
        let mut state = DefaultHasher::new();
        self.next_tracks.iter().for_each(|t| t.uri.hash(&mut state));
        self.player.queue_revision = state.finish().to_string()
    }

    pub fn reset_playback_to_position(&mut self, new_index: Option<usize>) -> Result<(), Error> {
        let new_index = new_index.unwrap_or(0);
        self.update_current_index(|i| i.track = new_index as u32);

        self.update_context_index(new_index + 1)?;

        debug!("reset playback state to {new_index}");

        if !self.player.track.is_queue() {
            self.set_current_track(new_index)?;
        }

        self.prev_tracks.clear();

        if new_index > 0 {
            let context = self.get_current_context()?;

            let before_new_track = context.tracks.len() - new_index;
            self.prev_tracks = context
                .tracks
                .iter()
                .rev()
                .skip(before_new_track)
                .take(SPOTIFY_MAX_PREV_TRACKS_SIZE)
                .rev()
                .cloned()
                .collect();
            debug!("has {} prev tracks", self.prev_tracks.len())
        }

        self.clear_next_tracks(true);
        self.fill_up_next_tracks()?;
        self.update_restrictions();

        Ok(())
    }

    pub fn add_to_queue(&mut self, mut track: ProvidedTrack, rev_update: bool) {
        track.uid = format!("q{}", self.queue_count);
        self.queue_count += 1;

        track.set_provider(Provider::Queue);
        if !track.is_queued() {
            track.add_queued();
        }

        if let Some(next_not_queued_track) =
            self.next_tracks.iter().position(|track| !track.is_queue())
        {
            self.next_tracks.insert(next_not_queued_track, track);
        } else {
            self.next_tracks.push_back(track)
        }

        while self.next_tracks.len() > SPOTIFY_MAX_NEXT_TRACKS_SIZE {
            self.next_tracks.pop_back();
        }

        if rev_update {
            self.update_queue_revision();
        }
        self.update_restrictions();
    }

    pub fn mark_unavailable(&mut self, id: SpotifyId) -> Result<(), Error> {
        let uri = id.to_uri()?;

        debug!("marking {uri} as unavailable");

        for next_track in &mut self.next_tracks {
            Self::mark_as_unavailable_for_match(next_track, &uri)
        }

        for prev_track in &mut self.prev_tracks {
            Self::mark_as_unavailable_for_match(prev_track, &uri)
        }

        if self.player.track.uri != uri {
            while let Some(pos) = self.next_tracks.iter().position(|t| t.uri == uri) {
                let _ = self.next_tracks.remove(pos);
            }

            while let Some(pos) = self.prev_tracks.iter().position(|t| t.uri == uri) {
                let _ = self.prev_tracks.remove(pos);
            }

            self.unavailable_uri.push(uri);
            self.fill_up_next_tracks()?;
            self.update_queue_revision();
        }

        Ok(())
    }

    fn mark_as_unavailable_for_match(track: &mut ProvidedTrack, uri: &str) {
        if track.uri == uri {
            debug!("Marked <{}:{}> as unavailable", track.provider, track.uri);
            track.set_provider(Provider::Unavailable);
        }
    }

    pub fn update_position_in_relation(&mut self, timestamp: i64) {
        let diff = timestamp - self.player.timestamp;
        self.player.position_as_of_timestamp += diff;

        if log::max_level() >= LevelFilter::Debug {
            let pos = Duration::from_millis(self.player.position_as_of_timestamp as u64);
            let time = Date::from_timestamp_ms(timestamp)
                .map(|d| d.time().to_string())
                .unwrap_or_else(|_| timestamp.to_string());

            let sec = pos.as_secs();
            let (min, sec) = (sec / 60, sec % 60);
            debug!("update position to {min}:{sec:0>2} at {time}");
        }

        self.player.timestamp = timestamp;
    }

    pub async fn became_inactive(&mut self, session: &Session) -> SpClientResult {
        self.reset();
        self.reset_context(None);

        session.spclient().put_connect_state_inactive(false).await
    }

    /// Updates the connect state for the connect session
    ///
    /// Prepares a [PutStateRequest] from the current connect state
    pub async fn update_state(&self, session: &Session, reason: PutStateReason) -> SpClientResult {
        if matches!(reason, PutStateReason::BECAME_INACTIVE) {
            warn!("should use <ConnectState::became_inactive> instead")
        }

        let now = SystemTime::now();
        let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

        let client_side_timestamp = u64::try_from(since_the_epoch.as_millis())?;
        let member_type = EnumOrUnknown::new(MemberType::CONNECT_STATE);
        let put_state_reason = EnumOrUnknown::new(reason);

        let mut player_state = self.player.clone();
        // we copy the player state, which only contains the infos, not the next and prev tracks
        // cloning seems to be fine, because the cloned lists get dropped after the method call
        player_state.next_tracks = self.next_tracks.clone().into();
        player_state.prev_tracks = self.prev_tracks.clone().into();

        if let Some(context_uri) = player_state.track.get_context_uri() {
            player_state.context_uri = context_uri.to_owned();
            player_state.context_url = format!("context://{context_uri}");
        }

        let is_active = self.active;
        let device = MessageField::some(Device {
            device_info: MessageField::some(self.device.clone()),
            player_state: MessageField::some(player_state),
            ..Default::default()
        });

        let mut put_state = PutStateRequest {
            client_side_timestamp,
            member_type,
            put_state_reason,
            is_active,
            device,
            ..Default::default()
        };

        if let Some(has_been_playing_for) = self.has_been_playing_for {
            match has_been_playing_for.elapsed().as_millis().try_into() {
                Ok(ms) => put_state.has_been_playing_for_ms = ms,
                Err(why) => warn!("couldn't update has been playing for because {why}"),
            }
        }

        if let Some(active_since) = self.active_since {
            if let Ok(active_since_duration) = active_since.duration_since(UNIX_EPOCH) {
                match active_since_duration.as_millis().try_into() {
                    Ok(active_since_ms) => put_state.started_playing_at = active_since_ms,
                    Err(why) => warn!("couldn't update active since because {why}"),
                }
            }
        }

        if let Some(request) = self.last_command.clone() {
            put_state.last_command_message_id = request.message_id;
            put_state.last_command_sent_by_device_id = request.sent_by_device_id;
        }

        session
            .spclient()
            .put_connect_state_request(put_state)
            .await
    }
}
