use librespot_protocol::player::{ContextTrack, ProvidedTrack};
use std::collections::HashMap;

const CONTEXT_URI: &str = "context_uri";
const ENTITY_URI: &str = "entity_uri";
const IS_QUEUED: &str = "is_queued";
const IS_AUTOPLAY: &str = "autoplay.is_autoplay";

const HIDDEN: &str = "hidden";
const ITERATION: &str = "iteration";

#[allow(dead_code)]
pub trait Metadata {
    fn metadata(&self) -> &HashMap<String, String>;
    fn metadata_mut(&mut self) -> &mut HashMap<String, String>;

    fn is_from_queue(&self) -> bool {
        matches!(self.metadata().get(IS_QUEUED), Some(is_queued) if is_queued.eq("true"))
    }

    fn is_from_autoplay(&self) -> bool {
        matches!(self.metadata().get(IS_AUTOPLAY), Some(is_autoplay) if is_autoplay.eq("true"))
    }

    fn is_hidden(&self) -> bool {
        matches!(self.metadata().get(HIDDEN), Some(is_hidden) if is_hidden.eq("true"))
    }

    fn get_context_uri(&self) -> Option<&String> {
        self.metadata().get(CONTEXT_URI)
    }

    fn get_iteration(&self) -> Option<&String> {
        self.metadata().get(ITERATION)
    }

    fn set_queued(&mut self, queued: bool) {
        self.metadata_mut()
            .insert(IS_QUEUED.to_string(), queued.to_string());
    }

    fn set_autoplay(&mut self, autoplay: bool) {
        self.metadata_mut()
            .insert(IS_AUTOPLAY.to_string(), autoplay.to_string());
    }

    fn set_hidden(&mut self, hidden: bool) {
        self.metadata_mut()
            .insert(HIDDEN.to_string(), hidden.to_string());
    }

    fn set_context_uri(&mut self, uri: String) {
        self.metadata_mut().insert(CONTEXT_URI.to_string(), uri);
    }

    fn set_entity_uri(&mut self, uri: String) {
        self.metadata_mut().insert(ENTITY_URI.to_string(), uri);
    }

    fn add_iteration(&mut self, iter: i64) {
        self.metadata_mut()
            .insert(ITERATION.to_string(), iter.to_string());
    }
}

impl Metadata for ContextTrack {
    fn metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.metadata
    }
}

impl Metadata for ProvidedTrack {
    fn metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.metadata
    }
}
