use librespot_protocol::{context_track::ContextTrack, player::ProvidedTrack};
use std::collections::HashMap;
use std::fmt::Display;

const CONTEXT_URI: &str = "context_uri";
const ENTITY_URI: &str = "entity_uri";
const IS_QUEUED: &str = "is_queued";
const IS_AUTOPLAY: &str = "autoplay.is_autoplay";
const HIDDEN: &str = "hidden";
const ITERATION: &str = "iteration";

const CUSTOM_CONTEXT_INDEX: &str = "context_index";

macro_rules! metadata_entry {
    ( $get:ident, $set:ident ($key:ident: $entry:ident)) => {
        metadata_entry!( $get use get, $set ($key: $entry) -> Option<&String> );
    };
    ( $get_key:ident use $get:ident, $set:ident ($key:ident: $entry:ident) -> $ty:ty ) => {
        fn $get_key (&self) -> $ty {
            self.$get($entry)
        }

        fn $set (&mut self, $key: impl Display) {
            self.metadata_mut().insert($entry.to_string(), $key.to_string());
        }
    };
}

#[allow(dead_code)]
pub trait Metadata {
    fn metadata(&self) -> &HashMap<String, String>;
    fn metadata_mut(&mut self) -> &mut HashMap<String, String>;

    fn get_bool(&self, entry: &str) -> bool {
        matches!(self.metadata().get(entry), Some(entry) if entry.eq("true"))
    }

    fn get_usize(&self, entry: &str) -> Option<usize> {
        self.metadata().get(entry)?.parse().ok()
    }

    fn get(&self, entry: &str) -> Option<&String> {
        self.metadata().get(entry)
    }

    metadata_entry!(is_from_queue use get_bool, set_from_queue (is_queued: IS_QUEUED) -> bool);
    metadata_entry!(is_from_autoplay use get_bool, set_from_autoplay (is_autoplay: IS_AUTOPLAY) -> bool);
    metadata_entry!(is_hidden use get_bool, set_hidden (is_hidden: HIDDEN) -> bool);

    metadata_entry!(get_context_index use get_usize, set_context_index (iteration: CUSTOM_CONTEXT_INDEX) -> Option<usize>);

    metadata_entry!(get_context_uri, set_context_uri (context_uri: CONTEXT_URI));
    metadata_entry!(get_entity_uri, set_entity_uri (entity_uri: ENTITY_URI));
    metadata_entry!(get_iteration, set_iteration (iteration: ITERATION));
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
