use std::collections::VecDeque;
use std::hash::{DefaultHasher, Hasher};
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
    Context, ContextIndex, ContextPage, ContextPlayerOptions, ContextTrack, PlayOrigin,
    PlayerState, ProvidedTrack, Suppressions, TransferState,
};
use protobuf::{EnumOrUnknown, Message, MessageField};
use rand::prelude::SliceRandom;
use thiserror::Error;

type ContextState = (ContextPage, ContextIndex);

// these limitations are essential, otherwise to many tracks will overload the web-player
const SPOTIFY_MAX_PREV_TRACKS_SIZE: usize = 10;
const SPOTIFY_MAX_NEXT_TRACKS_SIZE: usize = 80;

// provider used by spotify
pub(crate) const CONTEXT_PROVIDER: &str = "context";
const QUEUE_PROVIDER: &str = "queue";
// todo: there is a separator provider which is used to realise repeat

// our own provider to flag tracks as a specific states
// todo: we might just need to remove tracks that are unavailable to play, will have to see how the official clients handle this provider
const UNAVAILABLE_PROVIDER: &str = "unavailable";

#[derive(Debug, Error)]
pub enum StateError {
    #[error("the current track couldn't be resolved from the transfer state")]
    CouldNotResolveTrackFromTransfer,
    #[error("no next track available")]
    NoNextTrack,
    #[error("no prev track available")]
    NoPrevTrack,
    #[error("message field {0} was not available")]
    MessageFieldNone(String),
    #[error("context is not available. shuffle: {0}")]
    NoContext(bool),
    #[error("could not find track {0:?} in context of {1}")]
    CanNotFindTrackInContext(Option<usize>, usize),
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

#[derive(Default, Debug)]
pub struct ConnectState {
    pub active: bool,
    pub active_since: Option<SystemTime>,

    pub has_been_playing_for: Option<Instant>,

    pub device: DeviceInfo,

    unavailable_uri: Vec<String>,
    // is only some when we're playing a queued item and have to preserve the index
    player_index: Option<ContextIndex>,

    // index: 0 based, so the first track is index 0
    // prev_track: bottom => top, aka the last track of the list is the prev track
    // next_track: top => bottom, aka the first track of the list is the next track
    pub player: PlayerState,

    // we don't work directly on the lists of the player state, because
    // we mostly need to push and pop at the beginning of both
    pub prev_tracks: VecDeque<ProvidedTrack>,
    pub next_tracks: VecDeque<ProvidedTrack>,

    // the context from which we play, is used to top up prev and next tracks
    // the index is used to keep track which tracks are already loaded into next tracks
    pub context: Option<ContextState>,
    // a context to keep track of our shuffled context, should be only available when option.shuffling_context is true
    pub shuffle_context: Option<ContextState>,

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

                    supports_logout: cfg.zeroconf_enabled,
                    supported_types: vec!["audio/episode".into(), "audio/track".into()],
                    supports_playlist_v2: true,
                    supports_transfer_command: true,
                    supports_command_request: true,
                    supports_gzip_pushes: true,

                    // todo: not handled yet, repeat missing
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
            prev_tracks: VecDeque::with_capacity(SPOTIFY_MAX_PREV_TRACKS_SIZE),
            next_tracks: VecDeque::with_capacity(SPOTIFY_MAX_NEXT_TRACKS_SIZE),
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
        self.player.is_buffering = matches!(
            status,
            SpircPlayStatus::LoadingPause { .. } | SpircPlayStatus::LoadingPlay { .. }
        );
        self.player.is_playing = matches!(
            status,
            SpircPlayStatus::LoadingPlay { .. } | SpircPlayStatus::Playing { .. }
        );

        debug!(
            "updated connect play status playing: {}, paused: {}, buffering: {}",
            self.player.is_playing, self.player.is_paused, self.player.is_buffering
        );

        if let Some(restrictions) = self.player.restrictions.as_mut() {
            if self.player.is_playing && !self.player.is_paused {
                restrictions.disallow_pausing_reasons.clear();
                restrictions.disallow_resuming_reasons = vec!["not_paused".to_string()]
            }

            if self.player.is_paused && !self.player.is_playing {
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
        self.prev_tracks.clear();
        self.clear_next_tracks();

        let (ctx, _) = self.context.as_mut().ok_or(StateError::NoContext(false))?;

        let mut shuffle_context = ctx.clone();
        let mut rng = rand::thread_rng();
        shuffle_context.tracks.shuffle(&mut rng);

        self.shuffle_context = Some((shuffle_context, ContextIndex::new()));
        self.fill_up_next_tracks()?;

        Ok(())
    }

    // endregion

    pub fn set_current_track(&mut self, index: usize, shuffle_context: bool) -> Result<(), Error> {
        let (context, _) = if shuffle_context {
            self.shuffle_context.as_ref()
        } else {
            self.context.as_ref()
        }
        .ok_or(StateError::NoContext(shuffle_context))?;

        let new_track = context
            .tracks
            .get(index)
            .ok_or(StateError::CanNotFindTrackInContext(
                Some(index),
                context.tracks.len(),
            ))?;

        debug!(
            "set track to: {} at {index} of {} tracks",
            new_track.uri,
            context.tracks.len()
        );

        let new_track = self.context_to_provided_track(new_track)?;
        self.player.track = MessageField::some(new_track);

        Ok(())
    }

    /// Move to the next track
    ///
    /// Updates the current track to the next track. Adds the old track
    /// to prev tracks and fills up the next tracks from the current context
    pub fn next_track(&mut self) -> Result<u32, StateError> {
        let old_track = self.player.track.take();

        if let Some(old_track) = old_track {
            // only add songs from our context to our previous tracks
            if old_track.provider == CONTEXT_PROVIDER {
                // add old current track to prev tracks, while preserving a length of 10
                if self.prev_tracks.len() >= SPOTIFY_MAX_PREV_TRACKS_SIZE {
                    self.prev_tracks.pop_front();
                }
                self.prev_tracks.push_back(old_track);
            }
        }

        let new_track = self
            .next_tracks
            .pop_front()
            .ok_or(StateError::NoNextTrack)?;

        self.fill_up_next_tracks()?;

        let is_queued_track = new_track.provider == QUEUE_PROVIDER;
        let update_index = if is_queued_track {
            // the index isn't send when we are a queued track, but we have to preserve it for later
            self.player_index = self.player.index.take();
            None
        } else {
            let new_index = self.find_index_in_context(|c| c.uri == new_track.uri);
            match new_index {
                Ok(new_index) => Some(new_index as u32),
                Err(why) => {
                    error!("didn't find the shuffled track in the current context: {why}");
                    None
                }
            }
        };

        if let Some(update_index) = update_index {
            if let Some(index) = self.player.index.as_mut() {
                index.track = update_index
            }
        }

        self.player.track = MessageField::some(new_track);

        self.update_restrictions();

        Ok(self.player.index.track)
    }

    /// Move to the prev track
    ///
    /// Updates the current track to the prev track. Adds the old track
    /// to next tracks (when from the context) and fills up the prev tracks from the
    /// current context
    pub fn prev_track(&mut self) -> Result<&MessageField<ProvidedTrack>, StateError> {
        let old_track = self.player.track.take();

        if let Some(old_track) = old_track {
            if old_track.provider == CONTEXT_PROVIDER {
                self.next_tracks.push_front(old_track);
            }
        }

        while self.next_tracks.len() > SPOTIFY_MAX_NEXT_TRACKS_SIZE {
            let _ = self.next_tracks.pop_back();
        }

        let new_track = self.prev_tracks.pop_back().ok_or(StateError::NoPrevTrack)?;

        self.fill_up_next_tracks()?;

        self.player.track = MessageField::some(new_track);
        let index = self
            .player
            .index
            .as_mut()
            .ok_or(StateError::MessageFieldNone("player.index".to_string()))?;

        index.track -= 1;

        self.update_restrictions();

        Ok(&self.player.track)
    }

    fn update_context_index(&mut self, new_index: usize) -> Result<(), StateError> {
        let (_, context_index) = if self.player.options.shuffling_context {
            self.shuffle_context.as_mut()
        } else {
            self.context.as_mut()
        }
        .ok_or(StateError::NoContext(self.player.options.shuffling_context))?;

        context_index.track = new_index as u32;
        Ok(())
    }

    pub fn reset_playback_context(&mut self, new_index: Option<usize>) -> Result<(), Error> {
        let new_index = new_index.unwrap_or(0);
        if let Some(player_index) = self.player.index.as_mut() {
            player_index.track = new_index as u32;
        }

        self.update_context_index(new_index + 1)?;

        debug!("reset playback state to {new_index}");

        if self.player.track.provider != QUEUE_PROVIDER {
            self.set_current_track(new_index, self.player.options.shuffling_context)?;
        }

        self.prev_tracks.clear();

        let (context, _) = self.context.as_ref().ok_or(StateError::NoContext(false))?;
        if new_index > 0 {
            let rev_ctx = context
                .tracks
                .iter()
                .rev()
                .skip(context.tracks.len() - new_index)
                .take(SPOTIFY_MAX_PREV_TRACKS_SIZE);

            for track in rev_ctx {
                self.prev_tracks
                    .push_back(self.context_to_provided_track(track)?)
            }
        }

        self.clear_next_tracks();
        self.fill_up_next_tracks()?;
        self.update_restrictions();

        Ok(())
    }

    pub fn add_to_queue(&mut self, mut track: ProvidedTrack, rev_update: bool) {
        const IS_QUEUED: &str = "is_queued";

        track.provider = QUEUE_PROVIDER.to_string();
        if !track.metadata.contains_key(IS_QUEUED) {
            track
                .metadata
                .insert(IS_QUEUED.to_string(), true.to_string());
        }

        if let Some(next_not_queued_track) = self
            .player
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

    pub fn update_context(&mut self, mut context: Context) {
        debug!("context: {}, {}", context.uri, context.url);
        self.context = context
            .pages
            .pop()
            .map(|ctx| (ctx, ContextIndex::default()));

        self.player.context_url = format!("context://{}", context.uri);
        self.player.context_uri = context.uri;

        if context.restrictions.is_some() {
            self.player.context_restrictions = context.restrictions;
        }

        if !context.metadata.is_empty() {
            self.player.context_metadata = context.metadata;
        }

        if let Some(transfer_state) = self.transfer_state.take() {
            if let Err(why) = self.setup_current_state(transfer_state) {
                error!("setting up current state failed after updating the context: {why}")
            }
        }
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

        self.context_to_provided_track(track)
    }

    pub fn setup_current_state(&mut self, transfer: TransferState) -> Result<(), Error> {
        let track = match self.player.track.as_ref() {
            None => self.try_get_current_track_from_transfer(&transfer)?,
            Some(track) => track.clone(),
        };

        let current_index =
            self.find_index_in_context(|c| c.uri == track.uri || c.uid == track.uid)?;
        if self.player.track.is_none() {
            self.player.track = MessageField::some(track);
        }

        debug!(
            "setting up next and prev: index is at {current_index} while shuffle {}",
            self.player.options.shuffling_context
        );

        if self.player.options.shuffling_context {
            self.set_current_track(current_index, false)?;
            self.set_shuffle(true);
            self.shuffle()?;
        } else {
            // todo: it seems like, if we play a queued track and transfer we will reset that queued track...
            self.reset_playback_context(Some(current_index))?;
        }

        Ok(())
    }

    // todo: for some reason, after we run once into an unavailable track,
    //   a whole batch is marked as unavailable... have to look into that and see why and even how...
    pub fn mark_all_as_unavailable(&mut self, id: SpotifyId) {
        let id = match id.to_uri() {
            Ok(uri) => uri,
            Err(_) => return,
        };

        for next_track in &mut self.next_tracks {
            Self::mark_as_unavailable_for_match(next_track, &id)
        }

        for prev_track in &mut self.prev_tracks {
            Self::mark_as_unavailable_for_match(prev_track, &id)
        }

        self.unavailable_uri.push(id);
    }

    pub fn update_restrictions(&mut self) {
        const NO_PREV: &str = "no previous tracks";
        const NO_NEXT: &str = "no next tracks";

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

    fn fill_up_next_tracks(&mut self) -> Result<(), StateError> {
        let (ctx, ctx_index) = if self.player.options.shuffling_context {
            self.shuffle_context.as_ref()
        } else {
            self.context.as_ref()
        }
        .ok_or(StateError::NoContext(self.player.options.shuffling_context))?;

        let mut new_index = ctx_index.track as usize;
        while self.next_tracks.len() < SPOTIFY_MAX_NEXT_TRACKS_SIZE {
            let track = match ctx.tracks.get(new_index) {
                None => {
                    // todo: what do we do if we can't fill up anymore? autoplay?
                    break;
                }
                Some(ct) => match self.context_to_provided_track(ct) {
                    Err(why) => {
                        error!("bad thing happened: {why}");
                        // todo: handle bad things probably
                        break;
                    }
                    Ok(track) => track,
                },
            };

            new_index += 1;
            self.next_tracks.push_back(track);
        }

        self.update_context_index(new_index)?;

        // the web-player needs a revision update, otherwise the queue isn't updated in the ui
        self.player.queue_revision = self.new_queue_revision();

        Ok(())
    }

    pub fn find_index_in_context<F: Fn(&ContextTrack) -> bool>(
        &self,
        f: F,
    ) -> Result<usize, StateError> {
        let (ctx, _) = self.context.as_ref().ok_or(StateError::NoContext(false))?;

        ctx.tracks
            .iter()
            .position(f)
            .ok_or(StateError::CanNotFindTrackInContext(None, ctx.tracks.len()))
    }

    fn mark_as_unavailable_for_match(track: &mut ProvidedTrack, id: &str) {
        debug!("Marked <{}:{}> as unavailable", track.provider, track.uri);
        if track.uri == id {
            track.provider = UNAVAILABLE_PROVIDER.to_string();
        }
    }

    pub fn context_to_provided_track(
        &self,
        ctx_track: &ContextTrack,
    ) -> Result<ProvidedTrack, Error> {
        let provider = if self.unavailable_uri.contains(&ctx_track.uri) {
            UNAVAILABLE_PROVIDER
        } else {
            CONTEXT_PROVIDER
        };

        let uri = if !ctx_track.uri.is_empty() {
            ctx_track.uri.to_owned()
        } else if !ctx_track.gid.is_empty() {
            SpotifyId::from_raw(&ctx_track.gid)?
                .to_uri()?
                .replace("unknown", "track")
        } else if !ctx_track.uid.is_empty() {
            SpotifyId::from_raw(ctx_track.uid.as_bytes())?
                .to_uri()?
                .replace("unknown", "track")
        } else {
            return Err(Error::unavailable("track not available"));
        };

        Ok(ProvidedTrack {
            uri,
            uid: ctx_track.uid.to_string(),
            metadata: ctx_track.metadata.clone(),
            provider: provider.to_string(),
            ..Default::default()
        })
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
