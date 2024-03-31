use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::{
    playlist::{
        attribute::{PlaylistUpdateAttributes, PlaylistUpdateItemAttributes},
        item::PlaylistItems,
    },
    util::{impl_deref_wrapped, impl_try_from_repeated},
};

use librespot_protocol as protocol;
pub use protocol::playlist4_external::op::Kind as PlaylistOperationKind;
use protocol::playlist4_external::Add as PlaylistAddMessage;
use protocol::playlist4_external::Mov as PlaylistMoveMessage;
use protocol::playlist4_external::Op as PlaylistOperationMessage;
use protocol::playlist4_external::Rem as PlaylistRemoveMessage;

#[derive(Debug, Clone)]
pub struct PlaylistOperation {
    pub kind: PlaylistOperationKind,
    pub add: PlaylistOperationAdd,
    pub rem: PlaylistOperationRemove,
    pub mov: PlaylistOperationMove,
    pub update_item_attributes: PlaylistUpdateItemAttributes,
    pub update_list_attributes: PlaylistUpdateAttributes,
}

#[derive(Debug, Clone, Default)]
pub struct PlaylistOperations(pub Vec<PlaylistOperation>);

impl_deref_wrapped!(PlaylistOperations, Vec<PlaylistOperation>);

#[derive(Debug, Clone)]
pub struct PlaylistOperationAdd {
    pub from_index: i32,
    pub items: PlaylistItems,
    pub add_last: bool,
    pub add_first: bool,
}

#[derive(Debug, Clone)]
pub struct PlaylistOperationMove {
    pub from_index: i32,
    pub length: i32,
    pub to_index: i32,
}

#[derive(Debug, Clone)]
pub struct PlaylistOperationRemove {
    pub from_index: i32,
    pub length: i32,
    pub items: PlaylistItems,
    pub has_items_as_key: bool,
}

impl TryFrom<&PlaylistOperationMessage> for PlaylistOperation {
    type Error = librespot_core::Error;
    fn try_from(operation: &PlaylistOperationMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            kind: operation.kind(),
            add: operation.add.get_or_default().try_into()?,
            rem: operation.rem.get_or_default().try_into()?,
            mov: operation.mov.get_or_default().into(),
            update_item_attributes: operation
                .update_item_attributes
                .get_or_default()
                .try_into()?,
            update_list_attributes: operation
                .update_list_attributes
                .get_or_default()
                .try_into()?,
        })
    }
}

impl_try_from_repeated!(PlaylistOperationMessage, PlaylistOperations);

impl TryFrom<&PlaylistAddMessage> for PlaylistOperationAdd {
    type Error = librespot_core::Error;
    fn try_from(add: &PlaylistAddMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            from_index: add.from_index(),
            items: add.items.as_slice().try_into()?,
            add_last: add.add_last(),
            add_first: add.add_first(),
        })
    }
}

impl From<&PlaylistMoveMessage> for PlaylistOperationMove {
    fn from(mov: &PlaylistMoveMessage) -> Self {
        Self {
            from_index: mov.from_index(),
            length: mov.length(),
            to_index: mov.to_index(),
        }
    }
}

impl TryFrom<&PlaylistRemoveMessage> for PlaylistOperationRemove {
    type Error = librespot_core::Error;
    fn try_from(remove: &PlaylistRemoveMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            from_index: remove.from_index(),
            length: remove.length(),
            items: remove.items.as_slice().try_into()?,
            has_items_as_key: remove.items_as_key(),
        })
    }
}
