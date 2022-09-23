use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MetadataError {
    #[error("empty response")]
    Empty,
    #[error("audio item is non-playable when it should be")]
    NonPlayable,
    #[error("audio item duration can not be: {0}")]
    InvalidDuration(i32),
    #[error("track is marked as explicit, which client setting forbids")]
    ExplicitContentFiltered,
}
