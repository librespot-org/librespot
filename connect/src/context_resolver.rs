use crate::state::ConnectState;
use librespot_protocol::player::Context;
use std::{
    fmt::{Display, Formatter},
    hash::{Hash, Hasher},
};

#[derive(Debug, Clone)]
pub(super) struct ResolveContext {
    context: Context,
    fallback: Option<String>,
    autoplay: bool,
}

impl ResolveContext {
    pub fn from_uri(uri: impl Into<String>, fallback: impl Into<String>, autoplay: bool) -> Self {
        let fallback_uri = fallback.into();
        Self {
            context: Context {
                uri: uri.into(),
                ..Default::default()
            },
            fallback: (!fallback_uri.is_empty()).then_some(fallback_uri),
            autoplay,
        }
    }

    pub fn from_context(context: Context, autoplay: bool) -> Self {
        Self {
            context,
            fallback: None,
            autoplay,
        }
    }

    /// the uri which should be used to resolve the context, might not be the context uri
    pub fn resolve_uri(&self) -> Option<&String> {
        // it's important to call this always, or at least for every ResolveContext
        // otherwise we might not even check if we need to fallback and just use the fallback uri
        ConnectState::get_context_uri_from_context(&self.context)
            .and_then(|s| (!s.is_empty()).then_some(s))
            .or(self.fallback.as_ref())
    }

    /// the actual context uri
    pub fn context_uri(&self) -> &str {
        &self.context.uri
    }

    pub fn autoplay(&self) -> bool {
        self.autoplay
    }
}

impl Display for ResolveContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "resolve_uri: <{:?}>, context_uri: <{}>, autoplay: <{}>",
            self.resolve_uri(),
            self.context.uri,
            self.autoplay,
        )
    }
}

impl PartialEq for ResolveContext {
    fn eq(&self, other: &Self) -> bool {
        let eq_context = self.context_uri() == other.context_uri();
        let eq_resolve = self.resolve_uri() == other.resolve_uri();
        let eq_autoplay = self.autoplay == other.autoplay;

        eq_context && eq_resolve && eq_autoplay
    }
}

impl Eq for ResolveContext {}

impl Hash for ResolveContext {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.context_uri().hash(state);
        self.resolve_uri().hash(state);
        self.autoplay.hash(state);
    }
}

impl From<ResolveContext> for Context {
    fn from(value: ResolveContext) -> Self {
        value.context
    }
}
