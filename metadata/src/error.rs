use std::fmt::Debug;
use thiserror::Error;

use protobuf::ProtobufError;

use librespot_core::date::DateError;
use librespot_core::mercury::MercuryError;
use librespot_core::spclient::SpClientError;
use librespot_core::spotify_id::SpotifyIdError;

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("could not get metadata over HTTP: {0}")]
    Http(#[from] SpClientError),
    #[error("could not get metadata over Mercury: {0}")]
    Mercury(#[from] MercuryError),
    #[error("response was empty")]
    Empty,
}

#[derive(Debug, Error)]
pub enum MetadataError {
    #[error("{0}")]
    InvalidSpotifyId(#[from] SpotifyIdError),
    #[error("item has invalid date")]
    InvalidTimestamp(#[from] DateError),
    #[error("audio item is non-playable")]
    NonPlayable,
    #[error("could not parse protobuf: {0}")]
    Protobuf(#[from] ProtobufError),
    #[error("error executing request: {0}")]
    Request(#[from] RequestError),
    #[error("could not parse repeated fields")]
    InvalidRepeated,
}
