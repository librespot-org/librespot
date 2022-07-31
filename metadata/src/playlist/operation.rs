use std::{
    convert::{TryFrom, TryInto},
    fmt::Debug,
    ops::Deref,
};

use crate::{
    playlist::{
        attribute::{PlaylistUpdateAttributes, PlaylistUpdateItemAttributes},
        item::PlaylistItems,
    },
    util::try_from_repeated_message,
};

use librespot_protocol as protocol;
use protocol::playlist4_external::Add as PlaylistAddMessage;
use protocol::playlist4_external::Mov as PlaylistMoveMessage;
use protocol::playlist4_external::Op as PlaylistOperationMessage;
pub use protocol::playlist4_external::Op_Kind as PlaylistOperationKind;
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

impl Deref for PlaylistOperations {
    type Target = Vec<PlaylistOperation>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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
            kind: operation.get_kind(),
            add: operation.get_add().try_into()?,
            rem: operation.get_rem().try_into()?,
            mov: operation.get_mov().into(),
            update_item_attributes: operation.get_update_item_attributes().try_into()?,
            update_list_attributes: operation.get_update_list_attributes().try_into()?,
        })
    }
}

try_from_repeated_message!(PlaylistOperationMessage, PlaylistOperations);

impl TryFrom<&PlaylistAddMessage> for PlaylistOperationAdd {
    type Error = librespot_core::Error;
    fn try_from(add: &PlaylistAddMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            from_index: add.get_from_index(),
            items: add.get_items().try_into()?,
            add_last: add.get_add_last(),
            add_first: add.get_add_first(),
        })
    }
}

impl From<&PlaylistMoveMessage> for PlaylistOperationMove {
    fn from(mov: &PlaylistMoveMessage) -> Self {
        Self {
            from_index: mov.get_from_index(),
            length: mov.get_length(),
            to_index: mov.get_to_index(),
        }
    }
}

impl TryFrom<&PlaylistRemoveMessage> for PlaylistOperationRemove {
    type Error = librespot_core::Error;
    fn try_from(remove: &PlaylistRemoveMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            from_index: remove.get_from_index(),
            length: remove.get_length(),
            items: remove.get_items().try_into()?,
            has_items_as_key: remove.get_items_as_key(),
        })
    }
}
