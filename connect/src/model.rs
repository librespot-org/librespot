use librespot_protocol::player::Context;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct ResolveContext {
    context: Context,
    autoplay: bool,
}

impl ResolveContext {
    pub fn from_uri(uri: impl Into<String>, autoplay: bool) -> Self {
        Self {
            context: Context {
                uri: uri.into(),
                ..Default::default()
            },
            autoplay,
        }
    }

    pub fn from_context(context: Context, autoplay: bool) -> Self {
        Self { context, autoplay }
    }

    pub fn uri(&self) -> &str {
        &self.context.uri
    }

    pub fn autoplay(&self) -> bool {
        self.autoplay
    }
}

impl Display for ResolveContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "uri: {}, autoplay: {}", self.context.uri, self.autoplay)
    }
}

impl PartialEq for ResolveContext {
    fn eq(&self, other: &Self) -> bool {
        let eq_autoplay = self.autoplay == other.autoplay;
        let eq_context = self.context.uri == other.context.uri;

        eq_autoplay && eq_context
    }
}

impl From<ResolveContext> for Context {
    fn from(value: ResolveContext) -> Self {
        value.context
    }
}
