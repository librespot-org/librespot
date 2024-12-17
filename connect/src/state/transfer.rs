use crate::state::context::ContextType;
use crate::state::metadata::Metadata;
use crate::state::provider::{IsProvider, Provider};
use crate::state::{ConnectState, StateError};
use librespot_core::Error;
use librespot_protocol::player::{ProvidedTrack, TransferState};
use protobuf::MessageField;

impl ConnectState {
    pub fn current_track_from_transfer(
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
            Some(&transfer.current_session.context.uri),
            None,
            transfer.queue.is_playing_queue.then_some(Provider::Queue),
        )
    }

    /// handles the initially transferable data
    pub fn handle_initial_transfer(&mut self, transfer: &mut TransferState) {
        let current_context_metadata = self.context.as_ref().map(|c| c.metadata.clone());
        let player = self.player_mut();

        player.is_buffering = false;

        if let Some(options) = transfer.options.take() {
            player.options = MessageField::some(options);
        }
        player.is_paused = transfer.playback.is_paused;
        player.is_playing = !transfer.playback.is_paused;

        if transfer.playback.playback_speed != 0. {
            player.playback_speed = transfer.playback.playback_speed
        } else {
            player.playback_speed = 1.;
        }

        if let Some(session) = transfer.current_session.as_mut() {
            player.play_origin = session.play_origin.take().into();
            player.suppressions = session.suppressions.take().into();

            if let Some(mut ctx) = session.context.take() {
                player.restrictions = ctx.restrictions.take().into();
                for (key, value) in ctx.metadata {
                    player.context_metadata.insert(key, value);
                }
            }
        }

        player.context_url.clear();
        player.context_uri.clear();

        if let Some(metadata) = current_context_metadata {
            for (key, value) in metadata {
                player.context_metadata.insert(key, value);
            }
        }

        self.clear_prev_track();
        self.clear_next_tracks();
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

        let current_index = if track.is_queue() {
            Self::find_index_in_context(ctx, |c| c.uid == transfer.current_session.current_uid)
                .map(|i| if i > 0 { i - 1 } else { i })
        } else {
            Self::find_index_in_context(ctx, |c| c.uri == track.uri || c.uid == track.uid)
        };

        debug!(
            "active track is <{}> with index {current_index:?} in {:?} context, has {} tracks",
            track.uri,
            self.active_context,
            ctx.tracks.len()
        );

        if self.player().track.is_none() {
            self.set_track(track);
        }

        let current_index = current_index.ok();
        if let Some(current_index) = current_index {
            self.update_current_index(|i| i.track = current_index as u32);
        }

        debug!(
            "setting up next and prev: index is at {current_index:?} while shuffle {}",
            self.shuffling_context()
        );

        for (i, track) in transfer.queue.tracks.iter().enumerate() {
            if transfer.queue.is_playing_queue && i == 0 {
                // if we are currently playing from the queue,
                // don't add the first queued item, because we are currently playing that item
                continue;
            }

            if let Ok(queued_track) = self.context_to_provided_track(
                track,
                Some(self.context_uri()),
                None,
                Some(Provider::Queue),
            ) {
                self.add_to_queue(queued_track, false);
            }
        }

        if self.shuffling_context() {
            self.set_current_track(current_index.unwrap_or_default())?;
            self.set_shuffle(true);
            self.shuffle()?;
        } else {
            self.reset_playback_to_position(current_index)?;
        }

        self.update_restrictions();

        Ok(())
    }
}
