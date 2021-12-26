use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MetadataError {
    #[error("empty response")]
    Empty,
    #[error("audio item is non-playable when it should be")]
    NonPlayable,
}
