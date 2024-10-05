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
    ContextIndex, ContextPage, ContextPlayerOptions, ContextTrack, PlayOrigin, PlayerState,
    ProvidedTrack, Suppressions,
};
use protobuf::{EnumOrUnknown, Message, MessageField};
use rand::prelude::SliceRandom;
use thiserror::Error;

// these limitations are essential, otherwise to many tracks will overload the web-player
const SPOTIFY_MAX_PREV_TRACKS_SIZE: usize = 10;
const SPOTIFY_MAX_NEXT_TRACKS_SIZE: usize = 80;

// provider used by spotify
const CONTEXT_PROVIDER: &str = "context";
const QUEUE_PROVIDER: &str = "queue";
// our own provider to flag tracks as a specific states
// todo: we might just need to remove tracks that are unavailable to play, will have to see how the official clients handle this provider
const UNAVAILABLE_PROVIDER: &str = "unavailable";

#[derive(Debug, Error)]
pub enum ConnectStateError {
    #[error("no next track available")]
    NoNextTrack,
    #[error("no prev track available")]
    NoPrevTrack,
    #[error("message field {0} was not available")]
    MessageFieldNone(String),
    #[error("context is not available")]
    NoContext,
    #[error("not the first context page")]
    NotFirstContextPage,
    #[error("could not find the new track")]
    CanNotFindTrackInContext,
}

impl From<ConnectStateError> for Error {
    fn from(err: ConnectStateError) -> Self {
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

#[derive(Default, Debug, Clone)]
pub struct ConnectState {
    pub active: bool,
    pub active_since: Option<SystemTime>,

    pub has_been_playing_for: Option<Instant>,

    pub device: DeviceInfo,

    unavailable_uri: Vec<String>,
    // is only some when we're playing a queued item and have to preserve the index
    player_index: Option<ContextIndex>,
    // index: 0 based, so the first track is index 0
    // prev_track: bottom => top, aka the last track is the prev track
    // next_track: top => bottom, aka the first track is the next track
    pub player: PlayerState,

    // todo: still a bit jank, have to overhaul the resolving, especially when transferring playback
    // the context from which we play, is used to top up prev and next tracks
    // the index is used to keep track which tracks are already loaded into next tracks
    pub context: Option<(ContextPage, ContextIndex)>,
    // a context to keep track of our shuffled context, should be only available when option.shuffling_context is true
    pub shuffle_context: Option<(ContextPage, ContextIndex)>,

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
                    hidden: false,
                    gaia_eq_connect_id: true,
                    can_be_player: true,

                    needs_full_player_state: true,

                    is_observable: true,
                    is_controllable: true,

                    supports_logout: cfg.zeroconf_enabled,
                    supported_types: vec!["audio/episode".to_string(), "audio/track".to_string()],
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
                    supports_set_backend_metadata: false, // TODO: impl
                    supports_hifi: MessageField::none(),

                    command_acks: true,
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        };
        state.reset();
        state
    }

    // todo: is there maybe a better way to calculate the hash?
    fn new_queue_revision(&self) -> String {
        let mut hasher = DefaultHasher::new();
        for track in &self.player.next_tracks {
            if let Ok(bytes) = track.write_to_bytes() {
                hasher.write(&bytes)
            }
        }

        hasher.finish().to_string()
    }

    pub fn reset(&mut self) {
        self.active = false;
        self.active_since = None;
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

    pub fn set_repeat_context(&mut self, repeat: bool) {
        if let Some(options) = self.player.options.as_mut() {
            options.repeating_context = repeat;
        }
    }

    pub fn set_repeat_track(&mut self, repeat: bool) {
        if let Some(options) = self.player.options.as_mut() {
            options.repeating_track = repeat;
        }
    }

    pub fn set_shuffle(&mut self, shuffle: bool) -> Result<(), Error> {
        if let Some(options) = self.player.options.as_mut() {
            options.shuffling_context = shuffle;
        }

        if !shuffle {
            self.shuffle_context = None;

            let (ctx, _) = self.context.as_mut().ok_or(ConnectStateError::NoContext)?;
            let new_index = Self::find_index_in_context(ctx, &self.player.track.uri)?;

            self.reset_playback_context(Some(new_index))?;
            return Ok(());
        }

        self.shuffle()?;
        self.update_restrictions();

        Ok(())
    }

    pub fn shuffle(&mut self) -> Result<(), Error> {
        self.player.prev_tracks.clear();
        self.clear_next_tracks();

        let (ctx, _) = self.context.as_mut().ok_or(ConnectStateError::NoContext)?;

        let mut shuffle_context = ctx.clone();
        let mut rng = rand::thread_rng();
        shuffle_context.tracks.shuffle(&mut rng);

        self.shuffle_context = Some((shuffle_context, ContextIndex::new()));
        self.fill_up_next_tracks_from_current_context()?;

        Ok(())
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

    pub fn move_to_next_track(&mut self) -> Result<u32, ConnectStateError> {
        let old_track = self.player.track.take();

        if let Some(old_track) = old_track {
            // only add songs not from the queue to our previous tracks
            if old_track.provider != QUEUE_PROVIDER {
                // add old current track to prev tracks, while preserving a length of 10
                if self.player.prev_tracks.len() >= SPOTIFY_MAX_PREV_TRACKS_SIZE {
                    self.player.prev_tracks.remove(0);
                }
                self.player.prev_tracks.push(old_track);
            }
        }

        if self.player.next_tracks.is_empty() {
            return Err(ConnectStateError::NoNextTrack);
        }

        let new_track = self.player.next_tracks.remove(0);
        self.fill_up_next_tracks_from_current_context()?;

        let is_queued_track = new_track.provider == QUEUE_PROVIDER;
        if is_queued_track {
            // the index isn't send when we are a queued track, but we have to preserve it for later
            self.player_index = self.player.index.take();
            self.player.index = MessageField::none()
        } else if let Some(index) = self.player.index.as_mut() {
            if self.player.options.shuffling_context {
                let (ctx, _) = self.context.as_ref().ok_or(ConnectStateError::NoContext)?;
                let new_index = Self::find_index_in_context(ctx, &new_track.uri);
                match new_index {
                    Err(why) => {
                        error!("didn't find the shuffled track in the current context: {why}")
                    }
                    Ok(new_index) => index.track = new_index as u32,
                }
            } else {
                index.track += 1;
            }
        };

        self.player.track = MessageField::some(new_track);

        self.update_restrictions();

        Ok(self.player.index.track)
    }

    pub fn move_to_prev_track(
        &mut self,
    ) -> Result<&MessageField<ProvidedTrack>, ConnectStateError> {
        let old_track = self.player.track.take();

        if let Some(old_track) = old_track {
            if old_track.provider != QUEUE_PROVIDER {
                self.player.next_tracks.insert(0, old_track);
            }
        }

        while self.player.next_tracks.len() > SPOTIFY_MAX_NEXT_TRACKS_SIZE {
            let _ = self.player.next_tracks.pop();
        }

        let new_track = self
            .player
            .prev_tracks
            .pop()
            .ok_or(ConnectStateError::NoPrevTrack)?;

        self.fill_up_next_tracks_from_current_context()?;

        self.player.track = MessageField::some(new_track);
        let index = self
            .player
            .index
            .as_mut()
            .ok_or(ConnectStateError::MessageFieldNone(
                "player.index".to_string(),
            ))?;

        index.track -= 1;

        self.update_restrictions();

        Ok(&self.player.track)
    }

    pub fn reset_playback_context(&mut self, new_index: Option<usize>) -> Result<(), Error> {
        let (context, context_index) = self.context.as_mut().ok_or(ConnectStateError::NoContext)?;
        if context_index.page != 0 {
            // todo: hmm, probably needs to resolve the correct context_page
            return Err(ConnectStateError::NotFirstContextPage.into());
        }

        let new_index = new_index.unwrap_or(0);
        if let Some(player_index) = self.player.index.as_mut() {
            player_index.track = new_index as u32;
        }

        let new_track = context
            .tracks
            .get(new_index)
            .ok_or(ConnectStateError::CanNotFindTrackInContext)?;

        let is_unavailable = self.unavailable_uri.contains(&new_track.uri);
        let new_track = Self::context_to_provided_track(new_track, is_unavailable);
        self.player.track = MessageField::some(new_track);

        context_index.track = new_index as u32 + 1;

        self.player.prev_tracks.clear();

        if new_index > 0 {
            let rev_ctx = context
                .tracks
                .iter()
                .rev()
                .skip(context.tracks.len() - new_index)
                .take(SPOTIFY_MAX_PREV_TRACKS_SIZE);
            for track in rev_ctx {
                let is_unavailable = self.unavailable_uri.contains(&track.uri);
                self.player
                    .prev_tracks
                    .push(Self::context_to_provided_track(track, is_unavailable))
            }
        }

        self.clear_next_tracks();
        self.fill_up_next_tracks_from_current_context()?;
        self.update_restrictions();

        Ok(())
    }

    pub fn update_context(&mut self, context: Option<ContextPage>) {
        self.context = context.map(|ctx| (ctx, ContextIndex::default()))
    }

    pub fn mark_all_as_unavailable(&mut self, id: SpotifyId) {
        let id = match id.to_uri() {
            Ok(uri) => uri,
            Err(_) => return,
        };

        for next_track in &mut self.player.next_tracks {
            Self::mark_as_unavailable_for_match(next_track, &id)
        }

        for prev_track in &mut self.player.prev_tracks {
            Self::mark_as_unavailable_for_match(prev_track, &id)
        }

        self.unavailable_uri.push(id);
    }

    pub async fn update_state(&self, session: &Session, reason: PutStateReason) -> SpClientResult {
        if matches!(reason, PutStateReason::BECAME_INACTIVE) {
            return session.spclient().put_connect_state_inactive(false).await;
        }

        let now = SystemTime::now();
        let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let client_side_timestamp = u64::try_from(since_the_epoch.as_millis())?;

        let member_type = EnumOrUnknown::new(MemberType::CONNECT_STATE);
        let put_state_reason = EnumOrUnknown::new(reason);

        let state = self.clone();

        let is_active = state.active;
        let device = MessageField::some(Device {
            device_info: MessageField::some(state.device),
            player_state: MessageField::some(state.player),
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

        if let Some(has_been_playing_for) = state.has_been_playing_for {
            match has_been_playing_for.elapsed().as_millis().try_into() {
                Ok(ms) => put_state.has_been_playing_for_ms = ms,
                Err(why) => warn!("couldn't update has been playing for because {why}"),
            }
        }

        if let Some(active_since) = state.active_since {
            if let Ok(active_since_duration) = active_since.duration_since(UNIX_EPOCH) {
                match active_since_duration.as_millis().try_into() {
                    Ok(active_since_ms) => put_state.started_playing_at = active_since_ms,
                    Err(why) => warn!("couldn't update active since because {why}"),
                }
            }
        }

        if let Some(request) = state.last_command {
            put_state.last_command_message_id = request.message_id;
            put_state.last_command_sent_by_device_id = request.sent_by_device_id;
        }

        session
            .spclient()
            .put_connect_state_request(put_state)
            .await
    }

    pub fn update_restrictions(&mut self) {
        const NO_PREV: &str = "no previous tracks";
        const NO_NEXT: &str = "no next tracks";

        if let Some(restrictions) = self.player.restrictions.as_mut() {
            if self.player.prev_tracks.is_empty() {
                restrictions.disallow_peeking_prev_reasons = vec![NO_PREV.to_string()];
                restrictions.disallow_skipping_prev_reasons = vec![NO_PREV.to_string()];
            } else {
                restrictions.disallow_peeking_prev_reasons.clear();
                restrictions.disallow_skipping_prev_reasons.clear();
            }

            if self.player.next_tracks.is_empty() {
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
            .player
            .next_tracks
            .iter()
            .enumerate()
            .find(|(_, track)| track.provider != QUEUE_PROVIDER);
        if let Some((non_queued_track, _)) = first_non_queued_track {
            while self.player.next_tracks.len() > non_queued_track
                && self.player.next_tracks.pop().is_some()
            {}
        }
    }

    fn fill_up_next_tracks_from_current_context(&mut self) -> Result<(), ConnectStateError> {
        let current_context = if self.player.options.shuffling_context {
            self.shuffle_context.as_mut()
        } else {
            self.context.as_mut()
        };

        let (ctx, ctx_index) = current_context.ok_or(ConnectStateError::NoContext)?;
        let mut new_index = ctx_index.track as usize;

        while self.player.next_tracks.len() < SPOTIFY_MAX_NEXT_TRACKS_SIZE {
            let track = match ctx.tracks.get(new_index + 1) {
                None => break,
                Some(ct) => Self::context_to_provided_track(ct, false),
            };

            new_index += 1;
            self.player.next_tracks.push(track);
        }

        ctx_index.track = new_index as u32;

        // the web-player needs a revision update, otherwise the queue isn't updated in the ui
        self.player.queue_revision = self.new_queue_revision();

        Ok(())
    }

    fn find_index_in_context(ctx: &ContextPage, uri: &str) -> Result<usize, ConnectStateError> {
        ctx.tracks
            .iter()
            .position(|track| track.uri == uri)
            .ok_or(ConnectStateError::CanNotFindTrackInContext)
    }

    fn mark_as_unavailable_for_match(track: &mut ProvidedTrack, id: &str) {
        debug!("Marked <{}:{}> as unavailable", track.provider, track.uri);
        if track.uri == id {
            track.provider = UNAVAILABLE_PROVIDER.to_string();
        }
    }

    pub fn context_to_provided_track(
        ctx_track: &ContextTrack,
        is_unavailable: bool,
    ) -> ProvidedTrack {
        let provider = if is_unavailable {
            UNAVAILABLE_PROVIDER
        } else {
            CONTEXT_PROVIDER
        };

        ProvidedTrack {
            uri: ctx_track.uri.to_string(),
            uid: ctx_track.uid.to_string(),
            metadata: ctx_track.metadata.clone(),
            provider: provider.to_string(),
            ..Default::default()
        }
    }
}
