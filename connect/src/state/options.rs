use crate::state::context::ContextType;
use crate::state::{ConnectState, StateError};
use librespot_core::Error;
use librespot_protocol::player::{ContextIndex, ContextPlayerOptions};
use protobuf::MessageField;
use rand::prelude::SliceRandom;

impl ConnectState {
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
        self.clear_next_tracks(true);

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
}
