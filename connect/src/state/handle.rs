use crate::state::ConnectState;
use librespot_core::dealer::protocol::SetQueueCommand;
use librespot_core::Error;
use protobuf::MessageField;

impl ConnectState {
    pub fn handle_shuffle(&mut self, shuffle: bool) -> Result<(), Error> {
        self.set_shuffle(shuffle);

        if shuffle {
            return self.shuffle();
        }

        self.reset_context(None);

        if self.current_track(MessageField::is_none) {
            return Ok(());
        }

        let ctx = self.context.as_ref();
        let current_index =
            ConnectState::find_index_in_context(ctx, |c| self.current_track(|t| c.uri == t.uri))?;

        self.reset_playback_context(Some(current_index))
    }

    pub fn handle_set_queue(&mut self, set_queue: SetQueueCommand) {
        self.set_next_tracks(set_queue.next_tracks);
        self.set_prev_tracks(set_queue.prev_tracks);
        self.update_queue_revision();
    }

    pub fn handle_set_repeat(
        &mut self,
        context: Option<bool>,
        track: Option<bool>,
    ) -> Result<(), Error> {
        // doesn't need any state updates, because it should only change how the current song is played
        if let Some(track) = track {
            self.set_repeat_track(track);
        }

        if matches!(context, Some(context) if self.repeat_context() == context) {
            return Ok(());
        }

        if let Some(context) = context {
            self.set_repeat_context(context);
        }

        if self.repeat_context() {
            self.set_shuffle(false);
            self.reset_context(None);

            let ctx = self.context.as_ref();
            let current_track = ConnectState::find_index_in_context(ctx, |t| {
                self.current_track(|t| &t.uri) == &t.uri
            })?;
            self.reset_playback_context(Some(current_track))
        } else {
            self.update_restrictions();
            Ok(())
        }
    }
}
