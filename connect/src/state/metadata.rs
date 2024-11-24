use librespot_protocol::player::ProvidedTrack;

const CONTEXT_URI: &str = "context_uri";
const ENTITY_URI: &str = "entity_uri";
const IS_QUEUED: &str = "is_queued";

pub trait Metadata {
    fn is_queued(&self) -> bool;
    fn get_context_uri(&self) -> Option<&String>;

    fn add_queued(&mut self);
    fn add_context_uri(&mut self, uri: String);
    fn add_entity_uri(&mut self, uri: String);
}

impl Metadata for ProvidedTrack {
    fn is_queued(&self) -> bool {
        matches!(self.metadata.get(IS_QUEUED), Some(is_queued) if is_queued.eq("true"))
    }

    fn get_context_uri(&self) -> Option<&String> {
        self.metadata.get(CONTEXT_URI)
    }

    fn add_queued(&mut self) {
        self.metadata
            .insert(IS_QUEUED.to_string(), true.to_string());
    }

    fn add_context_uri(&mut self, uri: String) {
        self.metadata.insert(CONTEXT_URI.to_string(), uri);
    }

    fn add_entity_uri(&mut self, uri: String) {
        self.metadata.insert(ENTITY_URI.to_string(), uri);
    }
}
