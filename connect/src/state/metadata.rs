use librespot_protocol::player::{ContextTrack, ProvidedTrack};
use std::collections::HashMap;

const CONTEXT_URI: &str = "context_uri";
const ENTITY_URI: &str = "entity_uri";
const IS_QUEUED: &str = "is_queued";

pub trait Metadata {
    fn metadata(&self) -> &HashMap<String, String>;
    fn metadata_mut(&mut self) -> &mut HashMap<String, String>;

    fn is_queued(&self) -> bool {
        matches!(self.metadata().get(IS_QUEUED), Some(is_queued) if is_queued.eq("true"))
    }

    fn get_context_uri(&self) -> Option<&String> {
        self.metadata().get(CONTEXT_URI)
    }

    fn set_queued(&mut self) {
        self.metadata_mut()
            .insert(IS_QUEUED.to_string(), true.to_string());
    }

    fn add_context_uri(&mut self, uri: String) {
        self.metadata_mut().insert(CONTEXT_URI.to_string(), uri);
    }

    fn add_entity_uri(&mut self, uri: String) {
        self.metadata_mut().insert(ENTITY_URI.to_string(), uri);
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
