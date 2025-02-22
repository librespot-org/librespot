use crate::{context::Context, context_page::ContextPage, context_track::ContextTrack};
use protobuf::Message;
use std::hash::{Hash, Hasher};

impl Hash for Context {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Ok(ctx) = self.write_to_bytes() {
            ctx.hash(state)
        }
    }
}

impl Eq for Context {}

impl From<Vec<String>> for ContextPage {
    fn from(value: Vec<String>) -> Self {
        ContextPage {
            tracks: value
                .into_iter()
                .map(|uri| ContextTrack {
                    uri: Some(uri),
                    ..Default::default()
                })
                .collect(),
            ..Default::default()
        }
    }
}

impl From<Vec<ContextTrack>> for ContextPage {
    fn from(tracks: Vec<ContextTrack>) -> Self {
        ContextPage {
            tracks,
            ..Default::default()
        }
    }
}
