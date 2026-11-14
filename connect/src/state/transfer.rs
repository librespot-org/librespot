use crate::{
    core::Error,
    protocol::{player::ProvidedTrack, transfer_state::TransferState},
    state::{
        context::{ContextType, StateContext, WEB_API_IDENTIFIER},
        metadata::Metadata,
        options::ShuffleState,
        provider::{IsProvider, Provider},
        {ConnectState, StateError},
    },
};
use protobuf::MessageField;

impl ConnectState {
    pub fn current_track_from_transfer(
        &self,
        transfer: &TransferState,
    ) -> Result<ProvidedTrack, Error> {
        let track = if transfer.queue.is_playing_queue.unwrap_or_default() {
            debug!("transfer track was used from the queue");
            transfer.queue.tracks.first()
        } else {
            debug!("transfer track was the current track");
            transfer.playback.current_track.as_ref()
        }
        .ok_or(StateError::CouldNotResolveTrackFromTransfer)?;

        let provider = if transfer.queue.is_playing_queue.unwrap_or_default() {
            Some(Provider::Queue)
        } else {
            track.is_from_autoplay().then_some(Provider::Autoplay)
        };

        self.context_to_provided_track(
            track,
            transfer.current_session.context.uri.as_deref(),
            None,
            None,
            provider,
        )
    }

    /// handles the initially transferable data
    pub fn handle_initial_transfer(
        &mut self,
        transfer: &mut TransferState,
        ctx_uri: Option<String>,
    ) {
        let current_context_metadata = self.context.as_ref().map(|c| c.metadata.clone());
        let player = self.player_mut();

        player.is_buffering = false;

        if let Some(options) = transfer.options.take() {
            player.options = MessageField::some(options.into());
        }
        player.is_paused = transfer.playback.is_paused.unwrap_or_default();
        player.is_playing = !player.is_paused;

        match transfer.playback.playback_speed {
            Some(speed) if speed != 0. => player.playback_speed = speed,
            _ => player.playback_speed = 1.,
        }

        let mut shuffle_seed = None;
        let mut initial_track = None;
        if let Some(session) = transfer.current_session.as_mut() {
            player.play_origin = session.play_origin.take().map(Into::into).into();
            player.suppressions = session.suppressions.take().map(Into::into).into();

            // maybe at some point we can use the shuffle seed provided by spotify,
            // but I doubt it, as spotify doesn't use true randomness but rather an algorithm
            // based shuffle
            trace!(
                "shuffle_seed: <{:?}> (spotify), <{:?}> (own)",
                session.shuffle_seed,
                session.context.get_shuffle_seed()
            );

            shuffle_seed = session
                .context
                .get_shuffle_seed()
                .and_then(|seed| seed.parse().ok());

            initial_track = session.context.get_initial_track().cloned();

            if let Some(mut ctx) = session.context.take() {
                player.restrictions = ctx.restrictions.take().map(Into::into).into();
                for (key, value) in ctx.metadata {
                    player.context_metadata.insert(key, value);
                }
            }
        }

        // it's important to always set the url/uri to a value
        // so that the player doesn't go into an inactive state;
        // without a context uri, the transfer's tracks play as those played by their uris
        let uri = ctx_uri.unwrap_or(WEB_API_IDENTIFIER.into());

        player.context_url = format!("context://{uri}");
        player.context_uri = uri;

        if let Some(metadata) = current_context_metadata {
            for (key, value) in metadata {
                player.context_metadata.insert(key, value);
            }
        }

        self.transfer_shuffle = match (shuffle_seed, initial_track) {
            (Some(seed), Some(initial_track)) => Some(ShuffleState {
                seed,
                initial_track,
            }),
            _ => None,
        };

        self.clear_prev_track();
        // the transfer brings its queue, which replaces any queued before
        self.player_mut().next_tracks.clear();
        self.update_queue_revision()
    }

    /// completes the transfer, loading the queue and updating metadata
    pub fn finish_transfer(&mut self, transfer: TransferState) -> Result<(), Error> {
        let track = match self.player().track.as_ref() {
            None => self.current_track_from_transfer(&transfer)?,
            Some(track) => track.clone(),
        };

        let context_ty = if self.current_track(|t| t.is_from_autoplay()) {
            ContextType::Autoplay
        } else {
            ContextType::Default
        };

        self.set_active_context(context_ty);
        self.fill_up_context = context_ty;

        let ctx = self.get_context(self.active_context)?;
        let current_index = Self::transferred_index(ctx, &transfer, &track);

        debug!(
            "active track is <{}> with index {current_index:?} in {:?} context, has {} tracks",
            track.uri,
            self.active_context,
            ctx.tracks.len()
        );

        if self.player().track.is_none() {
            self.set_track(track.clone());
        }

        match current_index {
            Some(current_index) => self.update_current_index(|i| i.track = current_index as u32),
            // as for a queued track, the context has no index for a track outside it
            None => self.player_mut().index.clear(),
        }

        debug!(
            "setting up next and prev: index is at {current_index:?} while shuffle {}",
            self.shuffling_context()
        );

        for (i, track) in transfer.queue.tracks.iter().enumerate() {
            if transfer.queue.is_playing_queue.unwrap_or_default() && i == 0 {
                // if we are currently playing from the queue,
                // don't add the first queued item, because we are currently playing that item
                continue;
            }

            if let Ok(queued_track) = self.context_to_provided_track(
                track,
                Some(self.context_uri()),
                None,
                None,
                Some(Provider::Queue),
            ) {
                self.add_to_queue(queued_track, false);
            }
        }

        // a shuffle is of the default context, which autoplay has left behind
        let set_up = if self.shuffling_context() && matches!(context_ty, ContextType::Default) {
            // the shuffle leaves the context's track at the index out of the next ones
            current_index
                .map_or(Ok(()), |index| self.set_current_track(index))
                .and_then(|()| {
                    self.set_shuffle(true);

                    match self.transfer_shuffle.take() {
                        None => self.shuffle_new(),
                        // the saved order resumes after the transferred track
                        Some(state) => self.shuffle_restore(state).and_then(|()| {
                            let ctx = self.get_context(ContextType::Default)?;
                            match Self::transferred_index(ctx, &transfer, &track) {
                                Some(index) => self.reset_playback_to_position(Some(index)),
                                None => Ok(()),
                            }
                        }),
                    }
                })
        } else if current_index.is_some() {
            self.reset_playback_to_position(current_index)
        } else {
            // the context follows a track outside it from its start
            self.clear_prev_track();
            self.clear_next_tracks();
            self.update_context_index(self.active_context, 0)
                .map_err(Into::into)
                .and_then(|()| self.fill_up_next_tracks())
        };

        // setting up from an index makes the context's track there current, but the
        // transferred track plays already, as a queued one does after that track
        if self.current_track(|t| t.uri != track.uri || t.provider != track.provider) {
            self.set_track(track);
        }
        set_up?;

        self.update_restrictions();

        Ok(())
    }

    /// The index of the context's track that the transferred `track` is, or that a
    /// queued one plays after; none for a track outside the context
    fn transferred_index(
        ctx: &StateContext,
        transfer: &TransferState,
        track: &ProvidedTrack,
    ) -> Option<usize> {
        match transfer.current_session.current_uid.as_ref() {
            // a queued track plays before the context's track with that uid
            Some(uid) if track.is_queue() => Self::find_index_in_context(ctx, |c| &c.uid == uid)
                .ok()
                .and_then(|i| i.checked_sub(1)),
            _ => {
                Self::find_index_in_context(ctx, |c| c.uri == track.uri || c.uid == track.uid).ok()
            }
        }
    }
}
