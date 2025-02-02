use crate::{
    core::Error,
    protocol::player::ContextPlayerOptions,
    state::{
        context::{ContextType, ResetContext},
        metadata::Metadata,
        ConnectState, StateError,
    },
};
use protobuf::MessageField;
use rand::Rng;

impl ConnectState {
    fn add_options_if_empty(&mut self) {
        if self.player().options.is_none() {
            self.player_mut().options = MessageField::some(ContextPlayerOptions::new())
        }
    }

    pub fn set_repeat_context(&mut self, repeat: bool) {
        self.add_options_if_empty();
        if let Some(options) = self.player_mut().options.as_mut() {
            options.repeating_context = repeat;
        }
    }

    pub fn set_repeat_track(&mut self, repeat: bool) {
        self.add_options_if_empty();
        if let Some(options) = self.player_mut().options.as_mut() {
            options.repeating_track = repeat;
        }
    }

    pub fn set_shuffle(&mut self, shuffle: bool) {
        self.add_options_if_empty();
        if let Some(options) = self.player_mut().options.as_mut() {
            options.shuffling_context = shuffle;
        }
    }

    pub fn reset_options(&mut self) {
        self.set_shuffle(false);
        self.set_repeat_track(false);
        self.set_repeat_context(false);
    }

    pub fn shuffle(&mut self, seed: Option<u64>) -> Result<(), Error> {
        if let Some(reason) = self
            .player()
            .restrictions
            .disallow_toggling_shuffle_reasons
            .first()
        {
            Err(StateError::CurrentlyDisallowed {
                action: "shuffle",
                reason: reason.clone(),
            })?
        }

        self.clear_prev_track();
        self.clear_next_tracks();

        let current_track = self.current_track(|t| t.clone().take());

        self.reset_context(ResetContext::DefaultIndex);
        let ctx = self.get_context_mut(ContextType::Default)?;

        // we don't need to include the current track, because it is already being played
        ctx.skip_track = current_track;

        let seed = seed
            .unwrap_or_else(|| rand::thread_rng().gen_range(100_000_000_000..1_000_000_000_000));

        ctx.tracks.shuffle_with_seed(seed);
        ctx.set_shuffle_seed(seed);

        self.set_active_context(ContextType::Default);
        self.fill_up_context = ContextType::Default;
        self.fill_up_next_tracks()?;

        Ok(())
    }

    pub fn shuffling_context(&self) -> bool {
        self.player().options.shuffling_context
    }

    pub fn repeat_context(&self) -> bool {
        self.player().options.repeating_context
    }

    pub fn repeat_track(&self) -> bool {
        self.player().options.repeating_track
    }
}
