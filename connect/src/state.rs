use std::collections::{hash_map::DefaultHasher, HashMap, VecDeque};
use std::hash::Hasher;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use crate::spirc::SpircPlayStatus;
use librespot_core::config::DeviceType;
use librespot_core::dealer::protocol::Request;
use librespot_core::spclient::SpClientResult;
use librespot_core::{version, Error, Session, SpotifyId};
use librespot_protocol::connect::{
    Capabilities, Device, DeviceInfo, MemberType, PutStateReason, PutStateRequest,
};
use librespot_protocol::player::{
    Context, ContextIndex, ContextPlayerOptions, ContextTrack, PlayOrigin, PlayerState,
    ProvidedTrack, Restrictions, Suppressions, TransferState,
};
use protobuf::{EnumOrUnknown, Message, MessageField};
use rand::prelude::SliceRandom;
use thiserror::Error;

// these limitations are essential, otherwise to many tracks will overload the web-player
const SPOTIFY_MAX_PREV_TRACKS_SIZE: usize = 10;
const SPOTIFY_MAX_NEXT_TRACKS_SIZE: usize = 80;

// provider used by spotify
pub const CONTEXT_PROVIDER: &str = "context";
pub const QUEUE_PROVIDER: &str = "queue";
pub const AUTOPLAY_PROVIDER: &str = "autoplay";

pub const DELIMITER_IDENTIFIER: &str = "delimiter";

// our own provider to flag tracks as a specific states
// todo: we might just need to remove tracks that are unavailable to play, will have to see how the official clients handle this provider
//  it seems like spotify just knows that the track isn't available, currently i didn't found
//  a solution to do the same, so we stay with the old solution for now
pub const UNAVAILABLE_PROVIDER: &str = "unavailable";

pub const METADATA_CONTEXT_URI: &str = "context_uri";
pub const METADATA_ENTITY_URI: &str = "entity_uri";
pub const METADATA_IS_QUEUED: &str = "is_queued";

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
    #[error("Currently {action} is not allowed because {reason}")]
    CurrentlyDisallowed { action: String, reason: String },
}

impl From<StateError> for Error {
    fn from(err: StateError) -> Self {
        Error::failed_precondition(err)
    }
}

#[derive(Debug, Clone)]
pub struct ConnectStateConfig {
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
            initial_volume: u32::from(u16::MAX) / 2,
            name: "librespot".to_string(),
            device_type: DeviceType::Speaker,
            zeroconf_enabled: false,
            volume_steps: 64,
            is_group: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StateContext {
    pub tracks: Vec<ProvidedTrack>,
    pub metadata: HashMap<String, String>,
    pub index: ContextIndex,
}

#[derive(Default, Debug, Copy, Clone)]
pub enum ContextType {
    #[default]
    Default,
    Shuffle,
    Autoplay,
}

#[derive(Default, Debug)]
pub struct ConnectState {
    pub active: bool,
    pub active_since: Option<SystemTime>,

    pub has_been_playing_for: Option<Instant>,

    pub device: DeviceInfo,

    unavailable_uri: Vec<String>,
    /// is only some when we're playing a queued item and have to preserve the index
    player_index: Option<ContextIndex>,

    /// index: 0 based, so the first track is index 0
    /// prev_track: bottom => top, aka the last track of the list is the prev track
    /// next_track: top => bottom, aka the first track of the list is the next track
    pub player: PlayerState,

    /// we don't work directly on the lists of the player state, because
    /// we mostly need to push and pop at the beginning of both
    pub prev_tracks: VecDeque<ProvidedTrack>,
    pub next_tracks: VecDeque<ProvidedTrack>,

    pub active_context: ContextType,
    /// the context from which we play, is used to top up prev and next tracks
    /// the index is used to keep track which tracks are already loaded into next tracks
    pub context: Option<StateContext>,
    /// a context to keep track of our shuffled context, should be only available when option.shuffling_context is true
    pub shuffle_context: Option<StateContext>,
    /// a context to keep track of the autoplay context
    pub autoplay_context: Option<StateContext>,

    // is set when we receive a transfer state and are loading the context asynchronously
    pub transfer_state: Option<TransferState>,

    pub last_command: Option<Request>,
}

impl ConnectState {
    pub fn new(cfg: ConnectStateConfig, session: &Session) -> Self {
        let mut state = Self {
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
        self.player = PlayerState {
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

        if let Some(restrictions) = self.player.restrictions.as_mut() {
            if self.player.is_playing {
                restrictions.disallow_pausing_reasons.clear();
                restrictions.disallow_resuming_reasons = vec!["not_paused".to_string()]
            }

            if self.player.is_paused {
                restrictions.disallow_resuming_reasons.clear();
                restrictions.disallow_pausing_reasons = vec!["not_playing".to_string()]
            }
        }
    }

    // todo: is there maybe a better or more efficient way to calculate the hash?
    pub fn new_queue_revision(&self) -> String {
        let mut hasher = DefaultHasher::new();
        for track in &self.next_tracks {
            if let Ok(bytes) = track.write_to_bytes() {
                hasher.write(&bytes)
            }
        }

        hasher.finish().to_string()
    }

    // region options (shuffle, repeat)

    fn add_options_if_empty(&mut self) {
        if self.player.options.is_none() {
            self.player.options = MessageField::some(ContextPlayerOptions::new())
        }
    }

    pub fn set_repeat_context(&mut self, repeat: bool) {
        self.add_options_if_empty();
        if let Some(options) = self.player.options.as_mut() {
            options.repeating_context = repeat;
        }
    }

    pub fn set_repeat_track(&mut self, repeat: bool) {
        self.add_options_if_empty();
        if let Some(options) = self.player.options.as_mut() {
            options.repeating_track = repeat;
        }
    }

    pub fn set_shuffle(&mut self, shuffle: bool) {
        self.add_options_if_empty();
        if let Some(options) = self.player.options.as_mut() {
            options.shuffling_context = shuffle;
        }
    }

    pub fn shuffle(&mut self) -> Result<(), Error> {
        if let Some(reason) = self
            .player
            .restrictions
            .disallow_toggling_shuffle_reasons
            .first()
        {
            return Err(StateError::CurrentlyDisallowed {
                action: "shuffle".to_string(),
                reason: reason.clone(),
            }
            .into());
        }

        self.prev_tracks.clear();
        self.clear_next_tracks();

        let current_uri = &self.player.track.uri;

        let ctx = self
            .context
            .as_mut()
            .ok_or(StateError::NoContext(ContextType::Default))?;
        let current_track = Self::find_index_in_context(Some(ctx), |t| &t.uri == current_uri)?;

        let mut shuffle_context = ctx.clone();
        // we don't need to include the current track, because it is already being played
        shuffle_context.tracks.remove(current_track);

        let mut rng = rand::thread_rng();
        shuffle_context.tracks.shuffle(&mut rng);
        shuffle_context.index = ContextIndex::new();

        self.shuffle_context = Some(shuffle_context);
        self.active_context = ContextType::Shuffle;
        self.fill_up_next_tracks()?;

        Ok(())
    }

    // endregion

    pub fn set_current_track(&mut self, index: usize) -> Result<(), Error> {
        let context = self.get_current_context()?;

        let new_track = context
            .tracks
            .get(index)
            .ok_or(StateError::CanNotFindTrackInContext(
                Some(index),
                context.tracks.len(),
            ))?;

        debug!(
            "set track to: {} at {} of {} tracks",
            index + 1,
            new_track.uri,
            context.tracks.len()
        );

        self.player.track = MessageField::some(new_track.clone());

        Ok(())
    }

    /// Move to the next track
    ///
    /// Updates the current track to the next track. Adds the old track
    /// to prev tracks and fills up the next tracks from the current context
    pub fn next_track(&mut self) -> Result<Option<u32>, StateError> {
        // when we skip in repeat track, we don't repeat the current track anymore
        if self.player.options.repeating_track {
            self.set_repeat_track(false);
        }

        let old_track = self.player.track.take();

        if let Some(old_track) = old_track {
            // only add songs from our context to our previous tracks
            if old_track.provider == CONTEXT_PROVIDER || old_track.provider == AUTOPLAY_PROVIDER {
                // add old current track to prev tracks, while preserving a length of 10
                if self.prev_tracks.len() >= SPOTIFY_MAX_PREV_TRACKS_SIZE {
                    _ = self.prev_tracks.pop_front();
                }
                self.prev_tracks.push_back(old_track);
            }
        }

        let new_track = match self.next_tracks.pop_front() {
            Some(next) if next.uid.starts_with(DELIMITER_IDENTIFIER) => {
                if self.prev_tracks.len() >= SPOTIFY_MAX_PREV_TRACKS_SIZE {
                    _ = self.prev_tracks.pop_front();
                }
                self.prev_tracks.push_back(next);
                self.next_tracks.pop_front()
            }
            Some(next) if next.provider == UNAVAILABLE_PROVIDER => self.next_tracks.pop_front(),
            other => other,
        };

        let new_track = match new_track {
            None => return Ok(None),
            Some(t) => t,
        };

        self.fill_up_next_tracks()?;

        let is_queued_track = new_track.provider == QUEUE_PROVIDER;
        let is_autoplay = new_track.provider == AUTOPLAY_PROVIDER;
        let update_index = if (is_queued_track || is_autoplay) && self.player.index.is_some() {
            // the index isn't send when we are a queued track, but we have to preserve it for later
            self.player_index = self.player.index.take();
            None
        } else if is_autoplay || is_queued_track {
            None
        } else {
            let ctx = self.context.as_ref();
            let new_index = Self::find_index_in_context(ctx, |c| c.uri == new_track.uri);
            match new_index {
                Ok(new_index) => Some(new_index as u32),
                Err(why) => {
                    error!("didn't find the track in the current context: {why}");
                    None
                }
            }
        };

        if let Some(update_index) = update_index {
            if let Some(index) = self.player.index.as_mut() {
                index.track = update_index
            } else {
                debug!("next: index can't be updated, no index available")
            }
        }

        self.player.track = MessageField::some(new_track);

        self.update_restrictions();

        Ok(Some(self.player.index.track))
    }

    /// Move to the prev track
    ///
    /// Updates the current track to the prev track. Adds the old track
    /// to next tracks (when from the context) and fills up the prev tracks from the
    /// current context
    pub fn prev_track(&mut self) -> Result<Option<&MessageField<ProvidedTrack>>, StateError> {
        let old_track = self.player.track.take();

        if let Some(old_track) = old_track {
            if old_track.provider == CONTEXT_PROVIDER || old_track.provider == AUTOPLAY_PROVIDER {
                self.next_tracks.push_front(old_track);
            }
        }

        // handle possible delimiter
        if matches!(self.prev_tracks.back(), Some(prev) if prev.uid.starts_with(DELIMITER_IDENTIFIER))
        {
            let delimiter = self
                .prev_tracks
                .pop_back()
                .expect("item that was prechecked");
            if self.next_tracks.len() >= SPOTIFY_MAX_NEXT_TRACKS_SIZE {
                _ = self.next_tracks.pop_back();
            }
            self.next_tracks.push_front(delimiter)
        }

        while self.next_tracks.len() > SPOTIFY_MAX_NEXT_TRACKS_SIZE {
            let _ = self.next_tracks.pop_back();
        }

        let new_track = match self.prev_tracks.pop_back() {
            None => return Ok(None),
            Some(t) => t,
        };

        if matches!(self.active_context, ContextType::Autoplay if new_track.provider == CONTEXT_PROVIDER)
        {
            // transition back to default context
            self.active_context = ContextType::Default;
        }

        self.fill_up_next_tracks()?;

        self.player.track = MessageField::some(new_track);

        if self.player.index.track == 0 {
            warn!("prev: trying to skip into negative, index update skipped")
        } else if let Some(index) = self.player.index.as_mut() {
            index.track -= 1;
        } else {
            debug!("prev: index can't be decreased, no index available")
        }

        self.update_restrictions();

        Ok(Some(&self.player.track))
    }

    fn update_context_index(&mut self, new_index: usize) -> Result<(), StateError> {
        let context = match self.active_context {
            ContextType::Default => self.context.as_mut(),
            ContextType::Shuffle => self.shuffle_context.as_mut(),
            ContextType::Autoplay => self.autoplay_context.as_mut(),
        }
        .ok_or(StateError::NoContext(self.active_context))?;

        context.index.track = new_index as u32;
        Ok(())
    }

    pub fn reset_context(&mut self, new_context: Option<&str>) {
        self.active_context = ContextType::Default;

        self.autoplay_context = None;
        self.shuffle_context = None;

        if matches!(new_context, Some(ctx) if self.player.context_uri != ctx) {
            self.context = None;
        } else if let Some(ctx) = self.context.as_mut() {
            ctx.index.track = 0;
            ctx.index.page = 0;
        }

        self.update_restrictions()
    }

    pub fn reset_playback_context(&mut self, new_index: Option<usize>) -> Result<(), Error> {
        let new_index = new_index.unwrap_or(0);
        if let Some(player_index) = self.player.index.as_mut() {
            player_index.track = new_index as u32;
        }

        self.update_context_index(new_index + 1)?;

        debug!("reset playback state to {new_index}");

        if self.player.track.provider != QUEUE_PROVIDER {
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

        self.clear_next_tracks();
        self.fill_up_next_tracks()?;
        self.update_restrictions();

        Ok(())
    }

    pub fn add_to_queue(&mut self, mut track: ProvidedTrack, rev_update: bool) {
        track.provider = QUEUE_PROVIDER.to_string();
        if !track.metadata.contains_key(METADATA_IS_QUEUED) {
            track
                .metadata
                .insert(METADATA_IS_QUEUED.to_string(), true.to_string());
        }

        if let Some(next_not_queued_track) = self
            .next_tracks
            .iter()
            .position(|track| track.provider != QUEUE_PROVIDER)
        {
            self.next_tracks.insert(next_not_queued_track, track);
        } else {
            self.next_tracks.push_back(track)
        }

        while self.next_tracks.len() > SPOTIFY_MAX_NEXT_TRACKS_SIZE {
            self.next_tracks.pop_back();
        }

        if rev_update {
            self.player.queue_revision = self.new_queue_revision();
        }
        self.update_restrictions();
    }

    pub fn update_context(&mut self, mut context: Context) -> Result<(), Error> {
        debug!("context: {}, {}", context.uri, context.url);
        let page = context
            .pages
            .pop()
            .ok_or(StateError::NoContext(ContextType::Default))?;

        let tracks = page
            .tracks
            .iter()
            .flat_map(|track| {
                match self.context_to_provided_track(track, context.uri.clone(), None) {
                    Ok(t) => Some(t),
                    Err(_) => {
                        error!("couldn't convert {track:#?} into ProvidedTrack");
                        None
                    }
                }
            })
            .collect();

        self.context = Some(StateContext {
            tracks,
            metadata: page.metadata,
            index: ContextIndex::new(),
        });

        self.player.context_url = format!("context://{}", context.uri);
        self.player.context_uri = context.uri;

        if context.restrictions.is_some() {
            self.player.context_restrictions = context.restrictions;
        }

        if !context.metadata.is_empty() {
            self.player.context_metadata = context.metadata;
        }

        if let Some(transfer_state) = self.transfer_state.take() {
            self.setup_current_state(transfer_state)?
        }

        Ok(())
    }

    pub fn update_autoplay_context(&mut self, mut context: Context) -> Result<(), Error> {
        debug!(
            "autoplay-context: {}, pages: {}",
            context.uri,
            context.pages.len()
        );
        let page = context
            .pages
            .pop()
            .ok_or(StateError::NoContext(ContextType::Autoplay))?;
        debug!("autoplay-context size: {}", page.tracks.len());

        let tracks = page
            .tracks
            .iter()
            .flat_map(|track| {
                match self.context_to_provided_track(
                    track,
                    context.uri.clone(),
                    Some(AUTOPLAY_PROVIDER),
                ) {
                    Ok(t) => Some(t),
                    Err(_) => {
                        error!("couldn't convert {track:#?} into ProvidedTrack");
                        None
                    }
                }
            })
            .collect::<Vec<_>>();

        // add the tracks to the context if we already have an autoplay context
        if let Some(autoplay_context) = self.autoplay_context.as_mut() {
            for track in tracks {
                autoplay_context.tracks.push(track)
            }
        } else {
            self.autoplay_context = Some(StateContext {
                tracks,
                metadata: page.metadata,
                index: ContextIndex::new(),
            })
        }

        Ok(())
    }

    pub fn try_get_current_track_from_transfer(
        &self,
        transfer: &TransferState,
    ) -> Result<ProvidedTrack, Error> {
        let track = if transfer.queue.is_playing_queue {
            transfer.queue.tracks.first()
        } else {
            transfer.playback.current_track.as_ref()
        }
        .ok_or(StateError::CouldNotResolveTrackFromTransfer)?;

        self.context_to_provided_track(
            track,
            transfer.current_session.context.uri.clone(),
            transfer.queue.is_playing_queue.then_some(QUEUE_PROVIDER),
        )
    }

    pub fn setup_current_state(&mut self, transfer: TransferState) -> Result<(), Error> {
        let track = match self.player.track.as_ref() {
            None => self.try_get_current_track_from_transfer(&transfer)?,
            Some(track) => track.clone(),
        };

        let ctx = self.get_current_context().ok();

        let current_index =
            Self::find_index_in_context(ctx, |c| c.uri == track.uri || c.uid == track.uid);

        debug!(
            "active track is <{}> with index {current_index:?} in {:?} context, has {} tracks",
            track.uri,
            self.active_context,
            ctx.map(|c| c.tracks.len()).unwrap_or_default()
        );

        if self.player.track.is_none() {
            self.player.track = MessageField::some(track);
        }

        let current_index = current_index.ok();
        if let Some(current_index) = current_index {
            if let Some(index) = self.player.index.as_mut() {
                index.track = current_index as u32;
            } else {
                self.player.index = MessageField::some(ContextIndex {
                    page: 0,
                    track: current_index as u32,
                    ..Default::default()
                })
            }
        }

        debug!(
            "setting up next and prev: index is at {current_index:?} while shuffle {}",
            self.player.options.shuffling_context
        );

        if self.player.options.shuffling_context {
            self.set_current_track(current_index.unwrap_or_default())?;
            self.set_shuffle(true);
            self.shuffle()?;
        } else {
            // todo: it seems like, if we play a queued track and transfer we will reset that queued track...
            self.reset_playback_context(current_index)?;
        }

        self.update_restrictions();

        Ok(())
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
                _ = self.next_tracks.remove(pos);
            }

            while let Some(pos) = self.prev_tracks.iter().position(|t| t.uri == uri) {
                _ = self.prev_tracks.remove(pos);
            }

            self.unavailable_uri.push(uri);
            self.fill_up_next_tracks()?;
            self.player.queue_revision = self.new_queue_revision();
        }

        Ok(())
    }

    pub fn update_restrictions(&mut self) {
        const NO_PREV: &str = "no previous tracks";
        const NO_NEXT: &str = "no next tracks";
        const AUTOPLAY: &str = "autoplay";
        const ENDLESS_CONTEXT: &str = "endless_context";

        if self.player.restrictions.is_none() {
            self.player.restrictions = MessageField::some(Restrictions::new())
        }

        if let Some(restrictions) = self.player.restrictions.as_mut() {
            if self.prev_tracks.is_empty() {
                restrictions.disallow_peeking_prev_reasons = vec![NO_PREV.to_string()];
                restrictions.disallow_skipping_prev_reasons = vec![NO_PREV.to_string()];
            } else {
                restrictions.disallow_peeking_prev_reasons.clear();
                restrictions.disallow_skipping_prev_reasons.clear();
            }

            if self.next_tracks.is_empty() {
                restrictions.disallow_peeking_next_reasons = vec![NO_NEXT.to_string()];
                restrictions.disallow_skipping_next_reasons = vec![NO_NEXT.to_string()];
            } else {
                restrictions.disallow_peeking_next_reasons.clear();
                restrictions.disallow_skipping_next_reasons.clear();
            }

            if self.player.track.provider == AUTOPLAY_PROVIDER {
                restrictions.disallow_toggling_shuffle_reasons = vec![AUTOPLAY.to_string()];
                restrictions.disallow_toggling_repeat_context_reasons = vec![AUTOPLAY.to_string()];
                restrictions.disallow_toggling_repeat_track_reasons = vec![AUTOPLAY.to_string()];
            } else if self.player.options.repeating_context {
                restrictions.disallow_toggling_shuffle_reasons = vec![ENDLESS_CONTEXT.to_string()]
            } else {
                restrictions.disallow_toggling_shuffle_reasons.clear();
                restrictions
                    .disallow_toggling_repeat_context_reasons
                    .clear();
                restrictions.disallow_toggling_repeat_track_reasons.clear();
            }
        }
    }

    fn clear_next_tracks(&mut self) {
        // respect queued track and don't throw them out of our next played tracks
        let first_non_queued_track = self
            .next_tracks
            .iter()
            .enumerate()
            .find(|(_, track)| track.provider != QUEUE_PROVIDER);

        if let Some((non_queued_track, _)) = first_non_queued_track {
            while self.next_tracks.len() > non_queued_track && self.next_tracks.pop_back().is_some()
            {
            }
        }
    }

    fn get_current_context(&self) -> Result<&StateContext, StateError> {
        match self.active_context {
            ContextType::Default => self.context.as_ref(),
            ContextType::Shuffle => self.shuffle_context.as_ref(),
            ContextType::Autoplay => self.autoplay_context.as_ref(),
        }
        .ok_or(StateError::NoContext(self.active_context))
    }

    pub fn fill_up_next_tracks(&mut self) -> Result<(), StateError> {
        let ctx = self.get_current_context()?;
        let mut new_index = ctx.index.track as usize;
        let mut iteration = ctx.index.page;

        while self.next_tracks.len() < SPOTIFY_MAX_NEXT_TRACKS_SIZE {
            let ctx = self.get_current_context()?;
            let track = match ctx.tracks.get(new_index) {
                None if self.player.options.repeating_context => {
                    let delimiter = Self::delimiter(iteration.into());
                    iteration += 1;
                    new_index = 0;
                    delimiter
                }
                None if self.autoplay_context.is_some() => {
                    // transitional to autoplay as active context
                    self.active_context = ContextType::Autoplay;

                    match self.get_current_context()?.tracks.get(new_index) {
                        None => break,
                        Some(ct) => {
                            new_index += 1;
                            ct.clone()
                        }
                    }
                }
                None => break,
                Some(ct) if ct.provider == UNAVAILABLE_PROVIDER => {
                    new_index += 1;
                    continue;
                }
                Some(ct) => {
                    new_index += 1;
                    ct.clone()
                }
            };

            self.next_tracks.push_back(track);
        }

        self.update_context_index(new_index)?;

        // the web-player needs a revision update, otherwise the queue isn't updated in the ui
        self.player.queue_revision = self.new_queue_revision();

        Ok(())
    }

    pub fn find_index_in_context<F: Fn(&ProvidedTrack) -> bool>(
        context: Option<&StateContext>,
        f: F,
    ) -> Result<usize, StateError> {
        let ctx = context
            .as_ref()
            .ok_or(StateError::NoContext(ContextType::Default))?;

        ctx.tracks
            .iter()
            .position(f)
            .ok_or(StateError::CanNotFindTrackInContext(None, ctx.tracks.len()))
    }

    fn mark_as_unavailable_for_match(track: &mut ProvidedTrack, uri: &str) {
        if track.uri == uri {
            debug!("Marked <{}:{}> as unavailable", track.provider, track.uri);
            track.provider = UNAVAILABLE_PROVIDER.to_string();
        }
    }

    fn delimiter(iteration: i64) -> ProvidedTrack {
        const HIDDEN: &str = "hidden";
        const ITERATION: &str = "iteration";

        let mut metadata = HashMap::new();
        metadata.insert(HIDDEN.to_string(), true.to_string());
        metadata.insert(ITERATION.to_string(), iteration.to_string());

        ProvidedTrack {
            uri: format!("spotify:{DELIMITER_IDENTIFIER}"),
            uid: format!("{DELIMITER_IDENTIFIER}{iteration}"),
            provider: CONTEXT_PROVIDER.to_string(),
            metadata,
            ..Default::default()
        }
    }

    pub fn context_to_provided_track(
        &self,
        ctx_track: &ContextTrack,
        context_uri: String,
        provider: Option<&str>,
    ) -> Result<ProvidedTrack, Error> {
        let provider = if self.unavailable_uri.contains(&ctx_track.uri) {
            UNAVAILABLE_PROVIDER
        } else {
            provider.unwrap_or(CONTEXT_PROVIDER)
        };

        let id = if !ctx_track.uri.is_empty() {
            SpotifyId::from_uri(&ctx_track.uri)
        } else if !ctx_track.gid.is_empty() {
            SpotifyId::from_raw(&ctx_track.gid)
        } else {
            return Err(Error::unavailable("track not available"));
        }?;

        let mut metadata = HashMap::new();
        metadata.insert(METADATA_CONTEXT_URI.to_string(), context_uri.to_string());
        metadata.insert(METADATA_ENTITY_URI.to_string(), context_uri.to_string());

        if !ctx_track.metadata.is_empty() {
            for (k, v) in &ctx_track.metadata {
                metadata.insert(k.to_string(), v.to_string());
            }
        }

        let uid = if !ctx_track.uid.is_empty() {
            ctx_track.uid.clone()
        } else {
            String::from_utf8(id.to_raw().to_vec()).unwrap_or_else(|_| String::new())
        };

        Ok(ProvidedTrack {
            uri: id.to_uri()?.replace("unknown", "track"),
            uid,
            metadata,
            provider: provider.to_string(),
            ..Default::default()
        })
    }

    pub fn update_position_in_relation(&mut self, timestamp: i64) {
        let diff = timestamp - self.player.timestamp;
        self.player.position_as_of_timestamp += diff;

        debug!(
            "update position to {} at {timestamp}",
            self.player.position_as_of_timestamp
        );
        self.player.timestamp = timestamp;
    }

    // todo: i would like to refrain from copying the next and prev track lists... will have to see what we can come up with
    /// Updates the connect state for the connect session
    ///
    /// Prepares a [PutStateRequest] from the current connect state
    pub async fn update_state(&self, session: &Session, reason: PutStateReason) -> SpClientResult {
        if matches!(reason, PutStateReason::BECAME_INACTIVE) {
            return session.spclient().put_connect_state_inactive(false).await;
        }

        let now = SystemTime::now();
        let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let client_side_timestamp = u64::try_from(since_the_epoch.as_millis())?;

        let member_type = EnumOrUnknown::new(MemberType::CONNECT_STATE);
        let put_state_reason = EnumOrUnknown::new(reason);

        let mut player_state = self.player.clone();
        player_state.next_tracks = self.next_tracks.clone().into();
        player_state.prev_tracks = self.prev_tracks.clone().into();

        if let Some(context_uri) = player_state.track.metadata.get(METADATA_CONTEXT_URI) {
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
