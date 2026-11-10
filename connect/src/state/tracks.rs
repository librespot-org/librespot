use crate::{
    core::{Error, SpotifyUri},
    protocol::player::ProvidedTrack,
    state::{
        ConnectState, SPOTIFY_MAX_NEXT_TRACKS_SIZE, SPOTIFY_MAX_PREV_TRACKS_SIZE, StateError,
        context::ContextType,
        metadata::Metadata,
        provider::{IsProvider, Provider},
    },
};
use protobuf::MessageField;
use rand::Rng;

// identifier used as part of the uid
pub const IDENTIFIER_DELIMITER: &str = "delimiter";

impl<'ct> ConnectState {
    fn new_delimiter(iteration: i64) -> ProvidedTrack {
        let mut delimiter = ProvidedTrack {
            uri: format!("spotify:{IDENTIFIER_DELIMITER}"),
            uid: format!("{IDENTIFIER_DELIMITER}{iteration}"),
            provider: Provider::Context.to_string(),
            ..Default::default()
        };
        delimiter.set_hidden(true);
        delimiter.set_iteration(iteration);

        delimiter
    }

    fn push_prev(&mut self, prev: ProvidedTrack) {
        let prev_tracks = self.prev_tracks_mut();
        // add prev track, while preserving a length of 10
        if prev_tracks.len() >= SPOTIFY_MAX_PREV_TRACKS_SIZE {
            // todo: O(n), but technically only maximal O(SPOTIFY_MAX_PREV_TRACKS_SIZE) aka O(10)
            let _ = prev_tracks.remove(0);
        }
        prev_tracks.push(prev)
    }

    fn get_next_track(&mut self) -> Option<ProvidedTrack> {
        if self.next_tracks().is_empty() {
            None
        } else {
            // todo: O(n), but technically only maximal O(SPOTIFY_MAX_NEXT_TRACKS_SIZE) aka O(80)
            Some(self.next_tracks_mut().remove(0))
        }
    }

    /// bottom => top, aka the last track of the list is the prev track
    fn prev_tracks_mut(&mut self) -> &mut Vec<ProvidedTrack> {
        &mut self.player_mut().prev_tracks
    }

    /// bottom => top, aka the last track of the list is the prev track
    pub(super) fn prev_tracks(&self) -> &Vec<ProvidedTrack> {
        &self.player().prev_tracks
    }

    /// top => bottom, aka the first track of the list is the next track
    fn next_tracks_mut(&mut self) -> &mut Vec<ProvidedTrack> {
        &mut self.player_mut().next_tracks
    }

    /// top => bottom, aka the first track of the list is the next track
    pub(super) fn next_tracks(&self) -> &Vec<ProvidedTrack> {
        &self.player().next_tracks
    }

    pub fn set_current_track_random(&mut self) -> Result<(), Error> {
        let max_tracks = self.get_context(self.active_context)?.tracks.len();
        let rng_track = rand::rng().random_range(0..max_tracks);
        self.set_current_track(rng_track)
    }

    pub fn set_current_track(&mut self, index: usize) -> Result<(), Error> {
        let context = self.get_context(self.active_context)?;

        let new_track = context
            .tracks
            .get(index)
            .ok_or(StateError::CanNotFindTrackInContext(
                Some(index),
                context.tracks.len(),
            ))?;

        debug!(
            "set track to: {} at {} of {} tracks",
            new_track.uri,
            index,
            context.tracks.len()
        );

        self.set_track(new_track.clone());

        self.update_current_index(|i| i.track = index as u32);

        Ok(())
    }

    /// Move to the next track
    ///
    /// Updates the current track to the next track. Adds the old track
    /// to prev tracks and fills up the next tracks from the current context
    pub fn next_track(&mut self) -> Result<Option<u32>, Error> {
        // when we skip in repeat track, we don't repeat the current track anymore
        if self.repeat_track() {
            self.set_repeat_track(false);
        }

        let old_track = self.player_mut().track.take();

        if let Some(old_track) = old_track {
            // only add songs from our context to our previous tracks
            if old_track.is_context() || old_track.is_autoplay() {
                self.push_prev(old_track)
            }
        }

        let new_track = loop {
            match self.get_next_track() {
                Some(next) if next.uid.starts_with(IDENTIFIER_DELIMITER) => {
                    self.push_prev(next);
                    continue;
                }
                Some(next) if next.is_unavailable() => continue,
                other => break other,
            };
        };

        let new_track = match new_track {
            None => return Ok(None),
            Some(t) => t,
        };

        self.fill_up_next_tracks()?;

        let update_index = if new_track.is_queue() {
            None
        } else if new_track.is_autoplay() {
            self.set_active_context(ContextType::Autoplay);
            None
        } else {
            match new_track.get_context_index() {
                Some(new_index) => Some(new_index as u32),
                None => {
                    error!("the given context track had no set context_index");
                    None
                }
            }
        };

        if let Some(update_index) = update_index {
            self.update_current_index(|i| i.track = update_index)
        } else {
            self.player_mut().index.clear()
        }

        self.set_track(new_track);
        self.update_restrictions();

        Ok(Some(self.player().index.track))
    }

    /// Move to the prev track
    ///
    /// Updates the current track to the prev track. Adds the old track
    /// to next tracks (when from the context) and fills up the prev tracks from the
    /// current context
    pub fn prev_track(&mut self) -> Result<Option<&MessageField<ProvidedTrack>>, Error> {
        let old_track = self.player_mut().track.take();

        if let Some(old_track) = old_track {
            if old_track.is_context() || old_track.is_autoplay() {
                // todo: O(n)
                self.next_tracks_mut().insert(0, old_track);
            }
        }

        // handle possible delimiter
        if matches!(self.prev_tracks().last(), Some(prev) if prev.uid.starts_with(IDENTIFIER_DELIMITER))
        {
            let delimiter = self
                .prev_tracks_mut()
                .pop()
                .expect("item that was prechecked");

            let next_tracks = self.next_tracks_mut();
            if next_tracks.len() >= SPOTIFY_MAX_NEXT_TRACKS_SIZE {
                let _ = next_tracks.pop();
            }
            // todo: O(n)
            next_tracks.insert(0, delimiter)
        }

        while self.next_tracks().len() > SPOTIFY_MAX_NEXT_TRACKS_SIZE {
            let _ = self.next_tracks_mut().pop();
        }

        let new_track = match self.prev_tracks_mut().pop() {
            None => return Ok(None),
            Some(t) => t,
        };

        if matches!(self.active_context, ContextType::Autoplay if new_track.is_context()) {
            // transition back to default context
            self.set_active_context(ContextType::Default);
        }

        self.fill_up_next_tracks()?;
        self.set_track(new_track);

        if self.player().index.track == 0 {
            warn!("prev: trying to skip into negative, index update skipped")
        } else {
            self.update_current_index(|i| i.track -= 1)
        }

        self.update_restrictions();

        Ok(Some(self.current_track(|t| t)))
    }

    pub fn current_track<F: Fn(&'ct MessageField<ProvidedTrack>) -> R, R>(
        &'ct self,
        access: F,
    ) -> R {
        access(&self.player().track)
    }

    pub fn set_track(&mut self, track: ProvidedTrack) {
        self.player_mut().track = MessageField::some(track)
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

        // when you drag 'n drop the current track in the queue view into the "Next from: ..."
        // section, it is only send as an empty item with just the provider and metadata, so we have
        // to provide set the uri from the current track manually
        tracks
            .iter_mut()
            .filter(|t| t.uri.is_empty())
            .for_each(|t| t.uri = self.current_track(|ct| ct.uri.clone()));

        self.player_mut().next_tracks = tracks;
    }

    pub fn set_prev_tracks(&mut self, tracks: Vec<ProvidedTrack>) {
        self.player_mut().prev_tracks = tracks;
    }

    pub fn clear_prev_track(&mut self) {
        self.prev_tracks_mut().clear()
    }

    /// Remove all queued tracks
    ///
    /// Keeps the tracks from the context in the next tracks and fills
    /// them up from the current context afterwards
    pub fn clear_queue(&mut self) -> Result<(), Error> {
        self.next_tracks_mut().retain(|t| !t.is_queue());
        self.fill_up_next_tracks()?;
        self.update_queue_revision();
        Ok(())
    }

    /// Remove a single upcoming track
    ///
    /// `index` counts only the visible next tracks: hidden entries (the context
    /// delimiters) are skipped, matching what the Web API and Spotify clients show.
    /// Works for a queued track as well as a track from the context or autoplay.
    ///
    /// Returns `Ok(false)` and leaves the state untouched when there is no visible
    /// track at `index`, or when its uri doesn't match `uri`. Otherwise removes it,
    /// fills the next tracks back up from the context and returns `Ok(true)`.
    pub fn remove_next_track(&mut self, index: usize, uri: &str) -> Result<bool, Error> {
        let position = self
            .next_tracks()
            .iter()
            .enumerate()
            .filter(|(_, t)| !t.is_hidden())
            .nth(index)
            .map(|(position, _)| position);

        let Some(position) = position else {
            return Ok(false);
        };

        if self.next_tracks()[position].uri != uri {
            return Ok(false);
        }

        self.next_tracks_mut().remove(position);
        self.fill_up_next_tracks()?;
        self.update_restrictions();

        Ok(true)
    }

    pub fn clear_next_tracks(&mut self) {
        // respect queued track and don't throw them out of our next played tracks
        let first_non_queued_track = self
            .next_tracks()
            .iter()
            .enumerate()
            .find(|(_, track)| !track.is_queue());

        if let Some((non_queued_track, _)) = first_non_queued_track {
            while self.next_tracks().len() > non_queued_track
                && self.next_tracks_mut().pop().is_some()
            {}
        }
    }

    pub fn fill_up_next_tracks(&mut self) -> Result<(), Error> {
        let ctx = self.get_context(self.fill_up_context)?;
        let mut new_index = ctx.index.track as usize;
        let mut iteration = ctx.index.page;

        while self.next_tracks().len() < SPOTIFY_MAX_NEXT_TRACKS_SIZE {
            let ctx = self.get_context(self.fill_up_context)?;
            let track = match ctx.tracks.get(new_index) {
                None if self.repeat_context() => {
                    let delimiter = Self::new_delimiter(iteration.into());
                    iteration += 1;
                    new_index = 0;
                    delimiter
                }
                None if !matches!(self.fill_up_context, ContextType::Autoplay)
                    && self.autoplay_context.is_some()
                    && !self.repeat_context() =>
                {
                    self.update_context_index(self.fill_up_context, new_index)?;

                    // transition to autoplay as fill up context
                    self.fill_up_context = ContextType::Autoplay;
                    new_index = self.get_context(ContextType::Autoplay)?.index.track as usize;

                    // add delimiter to only display the current context
                    Self::new_delimiter(iteration.into())
                }
                None if self.autoplay_context.is_some() => {
                    match self
                        .get_context(ContextType::Autoplay)?
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
                Some(ct) if ct.is_unavailable() || self.is_skip_track(ct, Some(iteration)) => {
                    debug!(
                        "skipped track {} during fillup as it's unavailable or should be skipped",
                        ct.uri
                    );
                    new_index += 1;
                    continue;
                }
                Some(ct) => {
                    new_index += 1;
                    ct.clone()
                }
            };

            self.next_tracks_mut().push(track);
        }

        debug!(
            "finished filling up next_tracks ({})",
            self.next_tracks().len()
        );

        self.update_context_index(self.fill_up_context, new_index)?;

        // the web-player needs a revision update, otherwise the queue isn't updated in the ui
        self.update_queue_revision();

        Ok(())
    }

    pub fn preview_next_track(&mut self) -> Option<SpotifyUri> {
        let next = if self.repeat_track() {
            self.current_track(|t| &t.uri)
        } else {
            &self.next_tracks().first()?.uri
        };

        SpotifyUri::from_uri(next).ok()
    }

    pub fn has_next_tracks(&self, min: Option<usize>) -> bool {
        if let Some(min) = min {
            self.next_tracks().len() >= min
        } else {
            !self.next_tracks().is_empty()
        }
    }

    pub fn recent_track_uris(&self) -> Vec<String> {
        let mut prev = self
            .prev_tracks()
            .iter()
            .map(|t| t.uri.clone())
            .collect::<Vec<_>>();

        prev.push(self.current_track(|t| t.uri.clone()));
        prev
    }

    pub fn mark_unavailable(&mut self, id: &SpotifyUri) -> Result<(), Error> {
        let uri = id.to_uri();

        debug!("marking {uri} as unavailable");

        let next_tracks = self.next_tracks_mut();
        while let Some(pos) = next_tracks.iter().position(|t| t.uri == uri) {
            let _ = next_tracks.remove(pos);
        }

        for next_track in next_tracks {
            Self::mark_as_unavailable_for_match(next_track, &uri)
        }

        let prev_tracks = self.prev_tracks_mut();
        while let Some(pos) = prev_tracks.iter().position(|t| t.uri == uri) {
            let _ = prev_tracks.remove(pos);
        }

        for prev_track in prev_tracks {
            Self::mark_as_unavailable_for_match(prev_track, &uri)
        }

        self.unavailable_uri.push(uri);
        self.fill_up_next_tracks()?;
        self.update_queue_revision();

        Ok(())
    }

    pub fn add_to_queue(&mut self, mut track: ProvidedTrack, rev_update: bool) {
        track.uid = format!("q{}", self.queue_count);
        self.queue_count += 1;

        track.set_provider(Provider::Queue);
        if !track.is_from_queue() {
            track.set_from_queue(true);
        }

        let next_tracks = self.next_tracks_mut();
        if let Some(next_not_queued_track) = next_tracks.iter().position(|t| !t.is_queue()) {
            next_tracks.insert(next_not_queued_track, track);
        } else {
            next_tracks.push(track)
        }

        while next_tracks.len() > SPOTIFY_MAX_NEXT_TRACKS_SIZE {
            next_tracks.pop();
        }

        if rev_update {
            self.update_queue_revision();
        }
        self.update_restrictions();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        core::{Session, SessionConfig},
        protocol::{context::Context, context_page::ContextPage, context_track::ContextTrack},
        state::ConnectConfig,
    };

    fn new_state() -> ConnectState {
        let session = Session::new(SessionConfig::default(), None);
        ConnectState::new(ConnectConfig::default(), &session)
    }

    fn context_with_tracks(len: usize) -> Context {
        Context {
            uri: Some("spotify:playlist:remove-next-track-test".into()),
            pages: vec![ContextPage {
                tracks: (0..len)
                    .map(|i| ContextTrack {
                        uri: Some(format!("spotify:track:{i:022}")),
                        uid: Some(format!("uid-{i}")),
                        ..Default::default()
                    })
                    .collect(),
                ..Default::default()
            }],
            ..Default::default()
        }
    }

    /// A state with a context of `len` tracks, currently playing `current_index`
    /// and the next tracks already filled up.
    fn state_at(current_index: usize, repeat: bool, len: usize) -> ConnectState {
        let mut state = new_state();
        state
            .update_context(context_with_tracks(len), ContextType::Default)
            .unwrap();
        if repeat {
            state.set_repeat_context(true);
        }
        state.set_current_track(current_index).unwrap();
        state
            .reset_playback_to_position(Some(current_index))
            .unwrap();
        state
    }

    #[tokio::test]
    async fn removing_a_context_track_refills_from_where_it_left_off_and_never_replays_it() {
        let mut state = state_at(0, false, 120);
        let before = state.player().next_tracks.clone();
        assert_eq!(before.len(), SPOTIFY_MAX_NEXT_TRACKS_SIZE);

        let removed = before[5].clone();
        let last_before = before.last().unwrap().get_context_index().unwrap();

        let ok = state.remove_next_track(5, &removed.uri).unwrap();
        assert!(ok);

        let after = state.player().next_tracks.clone();
        assert_eq!(after.len(), SPOTIFY_MAX_NEXT_TRACKS_SIZE);
        assert!(after.iter().all(|t| t.uri != removed.uri));

        // the appended track continues right after the previous last one
        let appended = after.last().unwrap();
        assert_eq!(appended.get_context_index(), Some(last_before + 1));

        // stepping through the rest of the context never yields the removed track again
        let mut seen_removed = false;
        for _ in 0..150 {
            match state.next_track().unwrap() {
                None => break,
                Some(_) => {
                    if state.current_track(|t| t.uri.clone()) == removed.uri {
                        seen_removed = true;
                    }
                }
            }
        }
        assert!(!seen_removed);
    }

    #[tokio::test]
    async fn removing_one_queued_duplicate_keeps_the_others() {
        let mut state = state_at(0, false, 120);

        let queued_uri = "spotify:track:queued-duplicate".to_string();
        for _ in 0..3 {
            state.add_to_queue(
                ProvidedTrack {
                    uri: queued_uri.clone(),
                    ..Default::default()
                },
                true,
            );
        }

        let before = state.player().next_tracks.clone();
        let queued_before: Vec<_> = before.iter().filter(|t| t.uri == queued_uri).collect();
        assert_eq!(queued_before.len(), 3);
        let uids_before: Vec<_> = queued_before.iter().map(|t| t.uid.clone()).collect();

        let revision_before_removal = state.player().queue_revision.clone();

        // the three queued copies are the first three next tracks
        let ok = state.remove_next_track(1, &queued_uri).unwrap();
        assert!(ok);

        let after = state.player().next_tracks.clone();
        let queued_after: Vec<_> = after.iter().filter(|t| t.uri == queued_uri).collect();
        let uids_after: Vec<_> = queued_after.iter().map(|t| t.uid.clone()).collect();
        assert_eq!(
            uids_after,
            vec![uids_before[0].clone(), uids_before[2].clone()]
        );

        assert_ne!(state.player().queue_revision, revision_before_removal);
    }

    #[tokio::test]
    async fn wrong_uri_or_out_of_range_index_leaves_state_untouched() {
        let mut state = state_at(0, false, 120);
        let before = state.player().next_tracks.clone();
        let revision_before = state.player().queue_revision.clone();

        // wrong uri at a valid index
        let ok = state
            .remove_next_track(0, "spotify:track:not-actually-next")
            .unwrap();
        assert!(!ok);
        assert_eq!(state.player().next_tracks, before);
        assert_eq!(state.player().queue_revision, revision_before);

        // out-of-range index
        let ok = state
            .remove_next_track(before.len(), &before[0].uri)
            .unwrap();
        assert!(!ok);
        assert_eq!(state.player().next_tracks, before);
        assert_eq!(state.player().queue_revision, revision_before);
    }

    #[tokio::test]
    async fn repeat_context_delimiter_is_skipped_and_can_never_be_removed() {
        let mut state = state_at(115, true, 120);
        let before = state.player().next_tracks.clone();
        assert_eq!(before.len(), SPOTIFY_MAX_NEXT_TRACKS_SIZE);

        let delimiter_pos = before
            .iter()
            .position(Metadata::is_hidden)
            .expect("a delimiter should be present within the refilled next tracks");
        assert_eq!(before[delimiter_pos].uri, "spotify:delimiter");

        // no visible index maps to the delimiter's raw position: nth() over the
        // non-hidden entries can never land on it
        let visible_positions: Vec<usize> = before
            .iter()
            .enumerate()
            .filter(|(_, t)| !t.is_hidden())
            .map(|(i, _)| i)
            .collect();
        assert!(!visible_positions.contains(&delimiter_pos));

        let visible_index_after_delimiter = before[..delimiter_pos]
            .iter()
            .filter(|t| !t.is_hidden())
            .count();
        let track_after_delimiter = before[delimiter_pos + 1].clone();

        // addressing that visible index with the delimiter's own uri does nothing:
        // the index actually names the visible track after it, not the delimiter
        let ok = state
            .remove_next_track(visible_index_after_delimiter, "spotify:delimiter")
            .unwrap();
        assert!(!ok);
        assert_eq!(state.player().next_tracks, before);

        // removing the track right after the delimiter leaves the delimiter in place
        let ok = state
            .remove_next_track(visible_index_after_delimiter, &track_after_delimiter.uri)
            .unwrap();
        assert!(ok);

        let after = state.player().next_tracks.clone();
        assert_eq!(after.len(), SPOTIFY_MAX_NEXT_TRACKS_SIZE);
        assert!(after[delimiter_pos].is_hidden());
        assert_eq!(after[delimiter_pos].uri, "spotify:delimiter");
    }
}
