use std::fmt::Debug;

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
            from_revision: diff
                .from_revision
                .clone()
                .unwrap_or_default()
                .as_slice()
                .try_into()?,
            operations: diff.ops.as_slice().try_into()?,
            to_revision: diff
                .to_revision
                .clone()
                .unwrap_or_default()
                .as_slice()
                .try_into()?,
        })
    }
}
