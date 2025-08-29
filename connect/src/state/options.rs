use crate::{
    core::Error,
    protocol::player::ContextPlayerOptions,
    state::{
        ConnectState, StateError,
        context::{ContextType, ResetContext},
        metadata::Metadata,
    },
};
use protobuf::MessageField;
use rand::Rng;

#[derive(Default, Debug)]
pub(crate) struct ShuffleState {
    pub seed: u64,
    pub initial_track: String,
}

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

    pub fn shuffle(&mut self, shuffle_state: Option<ShuffleState>) -> Result<(), Error> {
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

        let (seed, initial_track) = match shuffle_state {
            Some(state) => (state.seed, Some(state.initial_track)),
            None => (
                rand::rng().random_range(100_000_000_000..1_000_000_000_000),
                None,
            ),
        };

        let current_track = self.current_track(|t| t.uri.clone());
        let first_track = initial_track.as_ref().unwrap_or(&current_track);

        self.reset_context(ResetContext::DefaultIndex);

        let ctx = self.get_context_mut(ContextType::Default)?;
        ctx.tracks
            .shuffle_with_seed(seed, |f| f.uri == *first_track);
        ctx.set_initial_track(first_track);
        ctx.set_shuffle_seed(seed);

        let start_at =
            initial_track.and_then(|initial| ctx.tracks.iter().position(|t| t.uri == initial));

        let start_at = start_at.unwrap_or_default();
        self.update_current_index(|i| i.track = start_at as u32);
        self.update_context_index(ContextType::Default, start_at + 1)?;

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
