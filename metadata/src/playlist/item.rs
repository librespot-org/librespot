use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::util::{impl_deref_wrapped, impl_try_from_repeated};

use super::{
    attribute::{PlaylistAttributes, PlaylistItemAttributes},
    permission::Capabilities,
};

use librespot_core::{date::Date, SpotifyId};

use librespot_protocol as protocol;
use protocol::playlist4_external::Item as PlaylistItemMessage;
use protocol::playlist4_external::ListItems as PlaylistItemsMessage;
use protocol::playlist4_external::MetaItem as PlaylistMetaItemMessage;

#[derive(Debug, Clone)]
pub struct PlaylistItem {
    pub id: SpotifyId,
    pub attributes: PlaylistItemAttributes,
}

#[derive(Debug, Clone, Default)]
pub struct PlaylistItems(pub Vec<PlaylistItem>);

impl_deref_wrapped!(PlaylistItems, Vec<PlaylistItem>);

#[derive(Debug, Clone)]
pub struct PlaylistItemList {
    pub position: i32,
    pub is_truncated: bool,
    pub items: PlaylistItems,
    pub meta_items: PlaylistMetaItems,
}

#[derive(Debug, Clone)]
pub struct PlaylistMetaItem {
    pub revision: SpotifyId,
    pub attributes: PlaylistAttributes,
    pub length: i32,
    pub timestamp: Date,
    pub owner_username: String,
    pub has_abuse_reporting: bool,
    pub capabilities: Capabilities,
}

#[derive(Debug, Clone, Default)]
pub struct PlaylistMetaItems(pub Vec<PlaylistMetaItem>);

impl_deref_wrapped!(PlaylistMetaItems, Vec<PlaylistMetaItem>);

impl TryFrom<&PlaylistItemMessage> for PlaylistItem {
    type Error = librespot_core::Error;
    fn try_from(item: &PlaylistItemMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            id: item.try_into()?,
            attributes: item.attributes.get_or_default().try_into()?,
        })
    }
}

impl_try_from_repeated!(PlaylistItemMessage, PlaylistItems);

impl TryFrom<&PlaylistItemsMessage> for PlaylistItemList {
    type Error = librespot_core::Error;
    fn try_from(list_items: &PlaylistItemsMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            position: list_items.pos(),
            is_truncated: list_items.truncated(),
            items: list_items.items.as_slice().try_into()?,
            meta_items: list_items.meta_items.as_slice().try_into()?,
        })
    }
}

impl TryFrom<&PlaylistMetaItemMessage> for PlaylistMetaItem {
    type Error = librespot_core::Error;
    fn try_from(item: &PlaylistMetaItemMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            revision: item.try_into()?,
            attributes: item.attributes.get_or_default().try_into()?,
            length: item.length(),
            timestamp: Date::from_timestamp_ms(item.timestamp())?,
            owner_username: item.owner_username().to_owned(),
            has_abuse_reporting: item.abuse_reporting_enabled(),
            capabilities: item.capabilities.get_or_default().into(),
        })
    }
}

impl_try_from_repeated!(PlaylistMetaItemMessage, PlaylistMetaItems);
