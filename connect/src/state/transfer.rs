use crate::state::provider::Provider;
use crate::state::{ConnectState, StateError};
use librespot_core::Error;
use librespot_protocol::player::{ContextIndex, ProvidedTrack, TransferState};
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
            transfer.current_session.context.uri.clone(),
            transfer.queue.is_playing_queue.then_some(Provider::Queue),
        )
    }

    pub fn transfer(&mut self, transfer: &mut TransferState) {
        self.player.is_buffering = false;

        if let Some(options) = transfer.options.take() {
            self.player.options = MessageField::some(options);
        }
        self.player.is_paused = transfer.playback.is_paused;
        self.player.is_playing = !transfer.playback.is_paused;

        if transfer.playback.playback_speed != 0. {
            self.player.playback_speed = transfer.playback.playback_speed
        } else {
            self.player.playback_speed = 1.;
        }

        self.player.play_origin = transfer.current_session.play_origin.clone();

        if let Some(suppressions) = transfer.current_session.suppressions.as_ref() {
            self.player.suppressions = MessageField::some(suppressions.clone());
        }

        if let Some(context) = transfer.current_session.context.as_ref() {
            self.player.context_uri = context.uri.clone();
            self.player.context_url = context.url.clone();
            self.player.context_restrictions = context.restrictions.clone();
        }

        for (key, value) in &transfer.current_session.context.metadata {
            self.player
                .context_metadata
                .insert(key.clone(), value.clone());
        }

        if let Some(context) = &self.context {
            for (key, value) in &context.metadata {
                self.player
                    .context_metadata
                    .insert(key.clone(), value.clone());
            }
        }
    }

    pub fn setup_current_state(&mut self, transfer: TransferState) -> Result<(), Error> {
        let track = match self.player.track.as_ref() {
            None => self.current_track_from_transfer(&transfer)?,
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
            self.shuffling_context()
        );

        for track in &transfer.queue.tracks {
            if let Ok(queued_track) = self.context_to_provided_track(
                track,
                self.context_uri().clone(),
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
            // todo: it seems like, if we play a queued track and transfer we will reset that queued track...
            self.reset_playback_context(current_index)?;
        }

        self.update_restrictions();

        Ok(())
    }
}
