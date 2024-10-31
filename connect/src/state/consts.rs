// providers used by spotify
pub const PROVIDER_CONTEXT: &str = "context";
pub const PROVIDER_QUEUE: &str = "queue";
pub const PROVIDER_AUTOPLAY: &str = "autoplay";

// custom providers, used to identify certain states that we can't handle preemptively, yet
// todo: we might just need to remove tracks that are unavailable to play, will have to see how the official clients handle this provider
//  it seems like spotify just knows that the track isn't available, currently i didn't found
//  a solution to do the same, so we stay with the old solution for now
pub const UNAVAILABLE_PROVIDER: &str = "unavailable";

// identifier used as part of the uid
pub const IDENTIFIER_DELIMITER: &str = "delimiter";

// metadata entries
pub const METADATA_CONTEXT_URI: &str = "context_uri";
pub const METADATA_ENTITY_URI: &str = "entity_uri";
pub const METADATA_IS_QUEUED: &str = "is_queued";
