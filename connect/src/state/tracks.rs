use crate::state::consts::IDENTIFIER_DELIMITER;
use crate::state::context::ContextType;
use crate::state::provider::{IsProvider, Provider};
use crate::state::{
    ConnectState, StateError, SPOTIFY_MAX_NEXT_TRACKS_SIZE, SPOTIFY_MAX_PREV_TRACKS_SIZE,
};
use librespot_core::Error;
use librespot_protocol::player::ProvidedTrack;
use protobuf::MessageField;
use std::collections::HashMap;

impl ConnectState {
    fn new_delimiter(iteration: i64) -> ProvidedTrack {
        const HIDDEN: &str = "hidden";
        const ITERATION: &str = "iteration";

        let mut metadata = HashMap::new();
        metadata.insert(HIDDEN.to_string(), true.to_string());
        metadata.insert(ITERATION.to_string(), iteration.to_string());

        ProvidedTrack {
            uri: format!("spotify:{IDENTIFIER_DELIMITER}"),
            uid: format!("{IDENTIFIER_DELIMITER}{iteration}"),
            provider: Provider::Context.to_string(),
            metadata,
            ..Default::default()
        }
    }

    pub fn set_current_track(&mut self, index: usize) -> Result<(), Error> {
        let context = self.get_current_context()?;

        let new_track = context
            .tracks
            .get(index)
            .ok_or(StateError::CanNotFindTrackInContext(
                Some(index),
                context.tracks.len(),
            ))?;

        debug!(
            "set track to: {} at {} of {} tracks",
            index + 1,
            new_track.uri,
            context.tracks.len()
        );

        self.player.track = MessageField::some(new_track.clone());

        Ok(())
    }

    /// Move to the next track
    ///
    /// Updates the current track to the next track. Adds the old track
    /// to prev tracks and fills up the next tracks from the current context
    pub fn next_track(&mut self) -> Result<Option<u32>, StateError> {
        // when we skip in repeat track, we don't repeat the current track anymore
        if self.player.options.repeating_track {
            self.set_repeat_track(false);
        }

        let old_track = self.player.track.take();

        if let Some(old_track) = old_track {
            // only add songs from our context to our previous tracks
            if old_track.is_context() || old_track.is_autoplay() {
                // add old current track to prev tracks, while preserving a length of 10
                if self.prev_tracks.len() >= SPOTIFY_MAX_PREV_TRACKS_SIZE {
                    _ = self.prev_tracks.pop_front();
                }
                self.prev_tracks.push_back(old_track);
            }
        }

        let new_track = match self.next_tracks.pop_front() {
            Some(next) if next.uid.starts_with(IDENTIFIER_DELIMITER) => {
                if self.prev_tracks.len() >= SPOTIFY_MAX_PREV_TRACKS_SIZE {
                    _ = self.prev_tracks.pop_front();
                }
                self.prev_tracks.push_back(next);
                self.next_tracks.pop_front()
            }
            Some(next) if next.is_unavailable() => self.next_tracks.pop_front(),
            other => other,
        };

        let new_track = match new_track {
            None => return Ok(None),
            Some(t) => t,
        };

        self.fill_up_next_tracks()?;

        let is_queue_or_autoplay = new_track.is_queue() || new_track.is_autoplay();
        let update_index = if is_queue_or_autoplay && self.player.index.is_some() {
            // the index isn't send when we are a queued track, but we have to preserve it for later
            self.player_index = self.player.index.take();
            None
        } else if is_queue_or_autoplay {
            None
        } else {
            let ctx = self.context.as_ref();
            let new_index = Self::find_index_in_context(ctx, |c| c.uri == new_track.uri);
            match new_index {
                Ok(new_index) => Some(new_index as u32),
                Err(why) => {
                    error!("didn't find the track in the current context: {why}");
                    None
                }
            }
        };

        if let Some(update_index) = update_index {
            if let Some(index) = self.player.index.as_mut() {
                index.track = update_index
            } else {
                debug!("next: index can't be updated, no index available")
            }
        }

        self.player.track = MessageField::some(new_track);

        self.update_restrictions();

        Ok(Some(self.player.index.track))
    }

    /// Move to the prev track
    ///
    /// Updates the current track to the prev track. Adds the old track
    /// to next tracks (when from the context) and fills up the prev tracks from the
    /// current context
    pub fn prev_track(&mut self) -> Result<Option<&MessageField<ProvidedTrack>>, StateError> {
        let old_track = self.player.track.take();

        if let Some(old_track) = old_track {
            if old_track.is_context() || old_track.is_autoplay() {
                self.next_tracks.push_front(old_track);
            }
        }

        // handle possible delimiter
        if matches!(self.prev_tracks.back(), Some(prev) if prev.uid.starts_with(IDENTIFIER_DELIMITER))
        {
            let delimiter = self
                .prev_tracks
                .pop_back()
                .expect("item that was prechecked");
            if self.next_tracks.len() >= SPOTIFY_MAX_NEXT_TRACKS_SIZE {
                _ = self.next_tracks.pop_back();
            }
            self.next_tracks.push_front(delimiter)
        }

        while self.next_tracks.len() > SPOTIFY_MAX_NEXT_TRACKS_SIZE {
            let _ = self.next_tracks.pop_back();
        }

        let new_track = match self.prev_tracks.pop_back() {
            None => return Ok(None),
            Some(t) => t,
        };

        if matches!(self.active_context, ContextType::Autoplay if new_track.is_context()) {
            // transition back to default context
            self.active_context = ContextType::Default;
        }

        self.fill_up_next_tracks()?;

        self.player.track = MessageField::some(new_track);

        if self.player.index.track == 0 {
            warn!("prev: trying to skip into negative, index update skipped")
        } else if let Some(index) = self.player.index.as_mut() {
            index.track -= 1;
        } else {
            debug!("prev: index can't be decreased, no index available")
        }

        self.update_restrictions();

        Ok(Some(&self.player.track))
    }

    pub(super) fn clear_next_tracks(&mut self) {
        // respect queued track and don't throw them out of our next played tracks
        let first_non_queued_track = self
            .next_tracks
            .iter()
            .enumerate()
            .find(|(_, track)| !track.is_queue());

        if let Some((non_queued_track, _)) = first_non_queued_track {
            while self.next_tracks.len() > non_queued_track && self.next_tracks.pop_back().is_some()
            {
            }
        }
    }

    pub fn fill_up_next_tracks(&mut self) -> Result<(), StateError> {
        let ctx = self.get_current_context()?;
        let mut new_index = ctx.index.track as usize;
        let mut iteration = ctx.index.page;

        while self.next_tracks.len() < SPOTIFY_MAX_NEXT_TRACKS_SIZE {
            let ctx = self.get_current_context()?;
            let track = match ctx.tracks.get(new_index) {
                None if self.player.options.repeating_context => {
                    let delimiter = Self::new_delimiter(iteration.into());
                    iteration += 1;
                    new_index = 0;
                    delimiter
                }
                None if self.autoplay_context.is_some() => {
                    // transitional to autoplay as active context
                    self.active_context = ContextType::Autoplay;

                    match self.get_current_context()?.tracks.get(new_index) {
                        None => break,
                        Some(ct) => {
                            new_index += 1;
                            ct.clone()
                        }
                    }
                }
                None => break,
                Some(ct) if ct.is_unavailable() => {
                    new_index += 1;
                    continue;
                }
                Some(ct) => {
                    new_index += 1;
                    ct.clone()
                }
            };

            self.next_tracks.push_back(track);
        }

        self.update_context_index(new_index)?;

        // the web-player needs a revision update, otherwise the queue isn't updated in the ui
        self.player.queue_revision = self.new_queue_revision();

        Ok(())
    }
}
