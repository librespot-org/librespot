use crate::{
    core::{Error, Session},
    protocol::{
        autoplay_context_request::AutoplayContextRequest, context::Context,
        transfer_state::TransferState,
    },
    state::{ConnectState, context::ContextType},
};
use std::{
    cmp::PartialEq,
    collections::{HashMap, VecDeque},
    fmt::{Display, Formatter},
    hash::Hash,
    time::Duration,
};
use thiserror::Error as ThisError;
use tokio::time::Instant;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Resolve {
    Uri(String),
    Context(Context),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(super) enum ContextAction {
    Append,
    Replace,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(super) struct ResolveContext {
    resolve: Resolve,
    fallback: Option<String>,
    update: ContextType,
    action: ContextAction,
}

impl ResolveContext {
    fn append_context(uri: impl Into<String>) -> Self {
        Self {
            resolve: Resolve::Uri(uri.into()),
            fallback: None,
            update: ContextType::Default,
            action: ContextAction::Append,
        }
    }

    pub fn from_uri(
        uri: impl Into<String>,
        fallback: impl Into<String>,
        update: ContextType,
        action: ContextAction,
    ) -> Self {
        let fallback_uri = fallback.into();
        Self {
            resolve: Resolve::Uri(uri.into()),
            fallback: (!fallback_uri.is_empty()).then_some(fallback_uri),
            update,
            action,
        }
    }

    pub fn from_context(context: Context, update: ContextType, action: ContextAction) -> Self {
        Self {
            resolve: Resolve::Context(context),
            fallback: None,
            update,
            action,
        }
    }

    /// the uri which should be used to resolve the context, might not be the context uri
    fn resolve_uri(&self) -> Option<&str> {
        // it's important to call this always, or at least for every ResolveContext
        // otherwise we might not even check if we need to fallback and just use the fallback uri
        match self.resolve {
            Resolve::Uri(ref uri) => ConnectState::valid_resolve_uri(uri),
            Resolve::Context(ref ctx) => {
                ConnectState::find_valid_uri(ctx.uri.as_deref(), ctx.pages.first())
            }
        }
        .or(self.fallback.as_deref())
    }

    /// the actual context uri
    fn context_uri(&self) -> &str {
        match self.resolve {
            Resolve::Uri(ref uri) => uri,
            Resolve::Context(ref ctx) => ctx.uri.as_deref().unwrap_or_default(),
        }
    }
}

impl Display for ResolveContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "resolve_uri: <{:?}>, context_uri: <{}>, update: <{:?}>",
            self.resolve_uri(),
            self.context_uri(),
            self.update,
        )
    }
}

#[derive(Debug, ThisError)]
enum ContextResolverError {
    #[error("no next context to resolve")]
    NoNext,
    #[error("tried appending context with {0} pages")]
    UnexpectedPagesSize(usize),
    #[error("tried resolving not allowed context: {0:?}")]
    NotAllowedContext(String),
}

impl From<ContextResolverError> for Error {
    fn from(value: ContextResolverError) -> Self {
        Error::failed_precondition(value)
    }
}

pub struct ContextResolver {
    session: Session,
    queue: VecDeque<ResolveContext>,
    unavailable_contexts: HashMap<ResolveContext, Instant>,
}

// time after which an unavailable context is retried
const RETRY_UNAVAILABLE: Duration = Duration::from_secs(3600);

impl ContextResolver {
    pub fn new(session: Session) -> Self {
        Self {
            session,
            queue: VecDeque::new(),
            unavailable_contexts: HashMap::new(),
        }
    }

    pub fn add(&mut self, resolve: ResolveContext) {
        let last_try = self
            .unavailable_contexts
            .get(&resolve)
            .map(Instant::elapsed);

        let last_try = if matches!(last_try, Some(last_try) if last_try > RETRY_UNAVAILABLE) {
            let _ = self.unavailable_contexts.remove(&resolve);
            debug!(
                "context was requested {}s ago, trying again to resolve the requested context",
                last_try.expect("checked by condition").as_secs()
            );
            None
        } else {
            last_try
        };

        if last_try.is_some() {
            debug!("tried loading unavailable context: {resolve}");
            return;
        } else if self.queue.contains(&resolve) {
            debug!("update for {resolve} is already added");
            return;
        } else {
            trace!(
                "added {} to resolver queue",
                resolve.resolve_uri().unwrap_or(resolve.context_uri())
            )
        }

        self.queue.push_back(resolve)
    }

    pub fn add_list(&mut self, resolve: Vec<ResolveContext>) {
        for resolve in resolve {
            self.add(resolve)
        }
    }

    pub fn remove_used_and_invalid(&mut self) {
        if let Some((_, _, remove)) = self.find_next() {
            let _ = self.queue.drain(0..remove); // remove invalid
        }
        self.queue.pop_front(); // remove used
    }

    pub fn clear(&mut self) {
        self.queue = VecDeque::new()
    }

    fn find_next(&self) -> Option<(&ResolveContext, &str, usize)> {
        for idx in 0..self.queue.len() {
            let next = self.queue.get(idx)?;
            match next.resolve_uri() {
                None => {
                    warn!("skipped {idx} because of invalid resolve_uri: {next}");
                    continue;
                }
                Some(uri) => return Some((next, uri, idx)),
            }
        }
        None
    }

    pub fn has_next(&self) -> bool {
        self.find_next().is_some()
    }

    /// Whether the next track has to wait for a queued resolve to add tracks.
    pub fn awaits_next_track(&self, state: &ConnectState) -> bool {
        self.has_next() && !state.has_playable_next_track()
    }

    pub async fn get_next_context(
        &self,
        recent_track_uri: impl Fn() -> Vec<String>,
    ) -> Result<Context, Error> {
        let (next, resolve_uri, _) = self.find_next().ok_or(ContextResolverError::NoNext)?;

        match next.update {
            ContextType::Default => {
                let mut ctx = self.session.spclient().get_context(resolve_uri).await;
                if let Ok(ctx) = ctx.as_mut() {
                    ctx.uri = Some(next.context_uri().to_string());
                    ctx.url = ctx.uri.as_ref().map(|s| format!("context://{s}"));
                }

                ctx
            }
            ContextType::Autoplay => {
                if resolve_uri.contains("spotify:show:") || resolve_uri.contains("spotify:episode:")
                {
                    // autoplay is not supported for podcasts
                    Err(ContextResolverError::NotAllowedContext(
                        resolve_uri.to_string(),
                    ))?
                }

                let request = AutoplayContextRequest {
                    context_uri: Some(resolve_uri.to_string()),
                    recent_track_uri: recent_track_uri(),
                    ..Default::default()
                };
                self.session.spclient().get_autoplay_context(&request).await
            }
        }
    }

    pub fn mark_next_unavailable(&mut self) {
        if let Some((next, _, _)) = self.find_next() {
            self.unavailable_contexts
                .insert(next.clone(), Instant::now());
        }
    }

    pub fn apply_next_context(
        &self,
        state: &mut ConnectState,
        mut context: Context,
    ) -> Result<Option<Vec<ResolveContext>>, Error> {
        let (next, _, _) = self.find_next().ok_or(ContextResolverError::NoNext)?;

        let remaining = match next.action {
            ContextAction::Append if context.pages.len() == 1 => state
                .fill_context_from_page(context.pages.remove(0))
                .map(|_| None),
            ContextAction::Replace => {
                let remaining = state.update_context(context, next.update);
                if let Resolve::Context(ref ctx) = next.resolve {
                    state.merge_context(ctx.pages.clone().pop());
                }

                remaining
            }
            ContextAction::Append => {
                warn!("unexpected page size: {context:#?}");
                Err(ContextResolverError::UnexpectedPagesSize(context.pages.len()).into())
            }
        }?;

        Ok(remaining.map(|remaining| {
            remaining
                .into_iter()
                .map(ResolveContext::append_context)
                .collect::<Vec<_>>()
        }))
    }

    pub fn try_finish(
        &self,
        state: &mut ConnectState,
        transfer_state: &mut Option<TransferState>,
    ) -> bool {
        let (next, _, _) = match self.find_next() {
            None => return false,
            Some(next) => next,
        };

        // when there is only one update type, we are the last of our kind, so we should update the state
        if self
            .queue
            .iter()
            .filter(|resolve| resolve.update == next.update)
            .count()
            != 1
        {
            return false;
        }

        match (next.update, state.active_context) {
            (ContextType::Default, ContextType::Default) | (ContextType::Autoplay, _) => {
                debug!(
                    "last item of type <{:?}>, finishing state setup",
                    next.update
                );
            }
            (ContextType::Default, _) => {
                debug!("skipped finishing default, because it isn't the active context");
                return false;
            }
        }

        let active_ctx = state.get_context(state.active_context);
        let res = if let Some(transfer_state) = transfer_state.take() {
            state.finish_transfer(transfer_state)
        } else if state.shuffling_context() && next.update == ContextType::Default {
            state.shuffle_new()
        } else if matches!(active_ctx, Ok(ctx) if ctx.index.track == 0) {
            // has context, and context is not touched
            // when the index is not zero, the next index was already evaluated elsewhere
            let ctx = active_ctx.expect("checked by precondition");
            let idx = ConnectState::find_index_in_context(ctx, |t| {
                state.current_track(|c| t.uri == c.uri)
            })
            .ok();

            state.reset_playback_to_position(idx)
        } else {
            state.fill_up_next_tracks()
        };

        if let Err(why) = res {
            error!("setup of state failed: {why}, last used resolve {next:#?}")
        }

        state.update_restrictions();
        state.update_queue_revision();

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        protocol::{
            context_page::ContextPage, context_track::ContextTrack, playback::Playback,
            session::Session as TransferSession,
        },
        state::context::ResetContext,
    };
    use protobuf::MessageField;

    const PLAYLIST: &str = "spotify:playlist:test";

    fn uri(i: u32) -> String {
        format!("spotify:track:{i:022}")
    }

    fn page(tracks: std::ops::Range<u32>) -> ContextPage {
        ContextPage {
            tracks: tracks
                .map(|i| ContextTrack {
                    uri: Some(uri(i)),
                    ..Default::default()
                })
                .collect(),
            ..Default::default()
        }
    }

    fn setup() -> (ContextResolver, ConnectState) {
        let session = Session::new(Default::default(), None);
        let state = ConnectState::new(Default::default(), &session);
        (ContextResolver::new(session), state)
    }

    fn handle_answer(
        resolver: &mut ContextResolver,
        state: &mut ConnectState,
        transfer: &mut Option<TransferState>,
        pages: Vec<ContextPage>,
    ) {
        let context = Context {
            uri: Some(PLAYLIST.to_string()),
            pages,
            ..Default::default()
        };
        resolver.apply_next_context(state, context).unwrap();
        resolver.try_finish(state, transfer);
        resolver.remove_used_and_invalid();
    }

    fn queue(resolver: &mut ContextResolver, update: ContextType) {
        resolver.add(ResolveContext::from_uri(
            PLAYLIST,
            "",
            update,
            ContextAction::Replace,
        ));
    }

    fn play_out(tracks: std::ops::Range<u32>) -> (ContextResolver, ConnectState) {
        let (mut resolver, mut state) = setup();
        queue(&mut resolver, ContextType::Default);
        handle_answer(&mut resolver, &mut state, &mut None, vec![page(tracks)]);
        while state.has_next_tracks(None) {
            state.next_track().unwrap();
        }
        (resolver, state)
    }

    fn current(state: &ConnectState) -> String {
        state.current_track(|t| t.uri.clone())
    }

    #[tokio::test]
    async fn an_empty_queue_without_a_resolve_does_not_wait() {
        let (resolver, state) = play_out(0..3);

        assert!(!resolver.awaits_next_track(&state));
    }

    #[tokio::test]
    async fn an_empty_queue_waits_for_the_first_autoplay_batch() {
        let (mut resolver, mut state) = play_out(0..3);
        queue(&mut resolver, ContextType::Autoplay);

        assert!(resolver.awaits_next_track(&state));

        handle_answer(&mut resolver, &mut state, &mut None, vec![page(10..13)]);

        assert!(!resolver.awaits_next_track(&state));
        state.next_track().unwrap();
        assert_eq!(current(&state), uri(10));
    }

    #[tokio::test]
    async fn an_empty_queue_waits_for_a_grown_playlist() {
        let (mut resolver, mut state) = play_out(0..3);
        queue(&mut resolver, ContextType::Default);

        assert!(resolver.awaits_next_track(&state));

        handle_answer(&mut resolver, &mut state, &mut None, vec![page(0..5)]);

        assert!(!resolver.awaits_next_track(&state));
        state.next_track().unwrap();
        assert_eq!(current(&state), uri(3));
    }

    #[tokio::test]
    async fn the_wait_ends_when_the_resolve_adds_nothing() {
        let (mut resolver, mut state) = play_out(0..3);
        queue(&mut resolver, ContextType::Default);

        handle_answer(&mut resolver, &mut state, &mut None, vec![page(0..3)]);

        assert!(!resolver.awaits_next_track(&state));
        assert!(!state.has_playable_next_track());
    }

    #[tokio::test]
    async fn the_wait_ends_when_the_resolve_fails() {
        let (mut resolver, state) = play_out(0..3);
        queue(&mut resolver, ContextType::Autoplay);

        resolver.mark_next_unavailable();
        resolver.remove_used_and_invalid();

        assert!(!resolver.awaits_next_track(&state));
    }

    #[tokio::test]
    async fn a_track_ending_before_its_transferred_context_waits_for_it() {
        let (mut resolver, mut state) = setup();
        let mut transfer = TransferState {
            playback: MessageField::some(Playback {
                current_track: MessageField::some(ContextTrack {
                    uri: Some(uri(1)),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            current_session: MessageField::some(TransferSession {
                context: MessageField::some(Context {
                    uri: Some(PLAYLIST.to_string()),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        };

        state.reset_context(ResetContext::Completely);
        let track = state.current_track_from_transfer(&transfer).unwrap();
        state.set_track(track);
        queue(&mut resolver, ContextType::Default);
        state.handle_initial_transfer(&mut transfer, Some(PLAYLIST.to_string()));

        assert!(resolver.awaits_next_track(&state));

        handle_answer(
            &mut resolver,
            &mut state,
            &mut Some(transfer),
            vec![page(0..4)],
        );

        assert!(!resolver.awaits_next_track(&state));
        state.next_track().unwrap();
        assert_eq!(current(&state), uri(2));
        let prev = state.player().prev_tracks.last().map(|t| t.uri.clone());
        assert_eq!(prev, Some(uri(1)));
    }
}
