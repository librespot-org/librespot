use std::{
    convert::{TryFrom, TryInto},
    fmt::Debug,
};

use super::operation::PlaylistOperations;

use librespot_core::SpotifyId;

use librespot_protocol as protocol;
use protocol::playlist4_external::Diff as DiffMessage;

#[derive(Debug, Clone)]
pub struct PlaylistDiff {
    pub from_revision: SpotifyId,
    pub operations: PlaylistOperations,
    pub to_revision: SpotifyId,
}

impl TryFrom<&DiffMessage> for PlaylistDiff {
    type Error = librespot_core::Error;
    fn try_from(diff: &DiffMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            from_revision: diff.get_from_revision().try_into()?,
            operations: diff.get_ops().try_into()?,
            to_revision: diff.get_to_revision().try_into()?,
        })
    }
}
