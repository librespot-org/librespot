use crate::state::{ConnectState, PROVIDER_AUTOPLAY};
use librespot_protocol::player::Restrictions;
use protobuf::MessageField;

impl ConnectState {
    pub fn update_restrictions(&mut self) {
        const NO_PREV: &str = "no previous tracks";
        const NO_NEXT: &str = "no next tracks";
        const AUTOPLAY: &str = "autoplay";
        const ENDLESS_CONTEXT: &str = "endless_context";

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

            if self.player.track.provider == PROVIDER_AUTOPLAY {
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
}
