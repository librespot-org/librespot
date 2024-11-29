use crate::state::context::ContextType;
use crate::state::metadata::Metadata;
use crate::state::provider::{IsProvider, Provider};
use crate::state::{
    ConnectState, StateError, SPOTIFY_MAX_NEXT_TRACKS_SIZE, SPOTIFY_MAX_PREV_TRACKS_SIZE,
};
use librespot_core::{Error, SpotifyId};
use librespot_protocol::player::ProvidedTrack;
use protobuf::MessageField;
use std::collections::{HashMap, VecDeque};

// identifier used as part of the uid
pub const IDENTIFIER_DELIMITER: &str = "delimiter";

impl<'ct> ConnectState {
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
        let context = self.get_context(&self.active_context)?;

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

        self.update_current_index(|i| i.track = index as u32);

        Ok(())
    }

    /// Move to the next track
    ///
    /// Updates the current track to the next track. Adds the old track
    /// to prev tracks and fills up the next tracks from the current context
    pub fn next_track(&mut self) -> Result<Option<u32>, StateError> {
        // when we skip in repeat track, we don't repeat the current track anymore
        if self.repeat_track() {
            self.set_repeat_track(false);
        }

        let old_track = self.player.track.take();

        if let Some(old_track) = old_track {
            // only add songs from our context to our previous tracks
            if old_track.is_context() || old_track.is_autoplay() {
                // add old current track to prev tracks, while preserving a length of 10
                if self.prev_tracks.len() >= SPOTIFY_MAX_PREV_TRACKS_SIZE {
                    let _ = self.prev_tracks.pop_front();
                }
                self.prev_tracks.push_back(old_track);
            }
        }

        let new_track = match self.next_tracks.pop_front() {
            Some(next) if next.uid.starts_with(IDENTIFIER_DELIMITER) => {
                if self.prev_tracks.len() >= SPOTIFY_MAX_PREV_TRACKS_SIZE {
                    let _ = self.prev_tracks.pop_front();
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

        let update_index = if new_track.is_queue() {
            None
        } else if new_track.is_autoplay() {
            self.active_context = ContextType::Autoplay;
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
            self.update_current_index(|i| i.track = update_index)
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
                let _ = self.next_tracks.pop_back();
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
        } else {
            self.update_current_index(|i| i.track -= 1)
        }

        self.update_restrictions();

        Ok(Some(&self.player.track))
    }

    pub fn current_track<F: Fn(&'ct MessageField<ProvidedTrack>) -> R, R>(
        &'ct self,
        access: F,
    ) -> R {
        access(&self.player.track)
    }

    pub fn set_track(&mut self, track: ProvidedTrack) {
        self.player.track = MessageField::some(track)
    }

    pub fn set_next_tracks(&mut self, mut tracks: Vec<ProvidedTrack>) {
        // mobile only sends a set_queue command instead of an add_to_queue command
        // in addition to handling the mobile add_to_queue handling, this should also handle
        // a mass queue addition
        tracks
            .iter_mut()
            .filter(|t| t.is_from_queue())
            .for_each(|t| {
                t.set_provider(Provider::Queue);
                // technically we could preserve the queue-uid here,
                // but it seems to work without that, so we just override it
                t.uid = format!("q{}", self.queue_count);
                self.queue_count += 1;
            });

        self.next_tracks = tracks.into();
    }

    pub fn set_prev_tracks(&mut self, tracks: impl Into<VecDeque<ProvidedTrack>>) {
        self.prev_tracks = tracks.into();
    }

    pub fn clear_next_tracks(&mut self, keep_queued: bool) {
        if !keep_queued {
            self.next_tracks.clear();
            return;
        }

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
        let ctx = self.get_context(&self.fill_up_context)?;
        let mut new_index = ctx.index.track as usize;
        let mut iteration = ctx.index.page;

        while self.next_tracks.len() < SPOTIFY_MAX_NEXT_TRACKS_SIZE {
            let ctx = self.get_context(&self.fill_up_context)?;
            let track = match ctx.tracks.get(new_index) {
                None if self.repeat_context() => {
                    let delimiter = Self::new_delimiter(iteration.into());
                    iteration += 1;
                    new_index = 0;
                    delimiter
                }
                None if self.autoplay_context.is_some() => {
                    // transition to autoplay as fill up context
                    self.fill_up_context = ContextType::Autoplay;

                    match self
                        .get_context(&ContextType::Autoplay)?
                        .tracks
                        .get(new_index)
                    {
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
        self.update_queue_revision();

        Ok(())
    }

    pub fn preview_next_track(&mut self) -> Option<SpotifyId> {
        let next = if self.repeat_track() {
            &self.player.track.uri
        } else {
            &self.next_tracks.front()?.uri
        };

        SpotifyId::from_uri(next).ok()
    }

    pub fn has_next_tracks(&self, min: Option<usize>) -> bool {
        if let Some(min) = min {
            self.next_tracks.len() >= min
        } else {
            !self.next_tracks.is_empty()
        }
    }

    pub fn prev_autoplay_track_uris(&self) -> Vec<String> {
        let mut prev = self
            .prev_tracks
            .iter()
            .flat_map(|t| t.is_autoplay().then_some(t.uri.clone()))
            .collect::<Vec<_>>();

        let current = &self.player.track;
        if current.is_autoplay() {
            prev.push(current.uri.clone());
        }

        prev
    }
}
