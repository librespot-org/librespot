use crate::state::context::ContextType;
use crate::{
    core::{Error, Session},
    protocol::{
        autoplay_context_request::AutoplayContextRequest,
        player::{Context, TransferState},
    },
    state::{context::UpdateContext, ConnectState},
};
use std::cmp::PartialEq;
use std::{
    collections::{HashMap, VecDeque},
    fmt::{Display, Formatter},
    hash::{Hash, Hasher},
    time::Duration,
};
use thiserror::Error as ThisError;
use tokio::time::Instant;

#[derive(Debug, Clone)]
enum Resolve {
    Uri(String),
    Context(Context),
}

#[derive(Debug, Clone)]
pub(super) enum ContextAction {
    Append,
    Replace,
}

#[derive(Debug, Clone)]
pub(super) struct ResolveContext {
    resolve: Resolve,
    fallback: Option<String>,
    update: UpdateContext,
    action: ContextAction,
}

impl ResolveContext {
    fn append_context(uri: impl Into<String>) -> Self {
        Self {
            resolve: Resolve::Uri(uri.into()),
            fallback: None,
            update: UpdateContext::Default,
            action: ContextAction::Append,
        }
    }

    pub fn from_uri(
        uri: impl Into<String>,
        fallback: impl Into<String>,
        update: UpdateContext,
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

    pub fn from_context(context: Context, update: UpdateContext, action: ContextAction) -> Self {
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
            Resolve::Context(ref ctx) => ConnectState::get_context_uri_from_context(ctx),
        }
        .or(self.fallback.as_deref())
    }

    /// the actual context uri
    fn context_uri(&self) -> &str {
        match self.resolve {
            Resolve::Uri(ref uri) => uri,
            Resolve::Context(ref ctx) => &ctx.uri,
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

impl PartialEq for ResolveContext {
    fn eq(&self, other: &Self) -> bool {
        let eq_context = self.context_uri() == other.context_uri();
        let eq_resolve = self.resolve_uri() == other.resolve_uri();
        let eq_autoplay = self.update == other.update;

        eq_context && eq_resolve && eq_autoplay
    }
}

impl Eq for ResolveContext {}

impl Hash for ResolveContext {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.context_uri().hash(state);
        self.resolve_uri().hash(state);
        self.update.hash(state);
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
            .map(|i| i.duration_since(Instant::now()));

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
            for _ in 0..remove {
                let _ = self.queue.pop_front();
            }
        }
        self.queue.pop_front();
    }

    pub fn clear(&mut self) {
        self.queue = VecDeque::new()
    }

    fn find_next(&self) -> Option<(&ResolveContext, &str, usize)> {
        let mut idx = 0;
        loop {
            let next = self.queue.get(idx)?;
            match next.resolve_uri() {
                None if idx < self.queue.len() => {
                    warn!("skipped {idx} because of no valid resolve_uri: {next}");
                    idx += 1;
                    continue;
                }
                value => break value.map(|uri| (next, uri, idx)),
            }
        }
    }

    pub fn has_next(&self) -> bool {
        self.find_next().is_some()
    }

    pub async fn get_next_context(
        &self,
        recent_track_uri: impl Fn() -> Vec<String>,
    ) -> Result<Context, Error> {
        let (next, resolve_uri, _) = self.find_next().ok_or(ContextResolverError::NoNext)?;

        match next.update {
            UpdateContext::Default => {
                let mut ctx = self.session.spclient().get_context(resolve_uri).await;
                if let Ok(ctx) = ctx.as_mut() {
                    ctx.uri = next.context_uri().to_string();
                    ctx.url = format!("context://{}", ctx.uri);
                }

                ctx
            }
            UpdateContext::Autoplay => {
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
                    state.merge_context(Some(ctx.clone()));
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
            (UpdateContext::Default, ContextType::Default) | (UpdateContext::Autoplay, _) => {
                debug!(
                    "last item of type <{:?}>, finishing state setup",
                    next.update
                );
            }
            (UpdateContext::Default, _) => {
                debug!("skipped finishing default, because it isn't the active context");
                return false;
            }
        }

        if let Some(transfer_state) = transfer_state.take() {
            if let Err(why) = state.finish_transfer(transfer_state) {
                error!("finishing setup of transfer failed: {why}")
            }
        }

        state.update_restrictions();
        state.update_queue_revision();

        true
    }
}
