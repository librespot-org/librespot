use std::convert::{TryFrom, TryInto};
use std::fmt::Debug;
use std::ops::Deref;

use crate::{error::MetadataError, util::try_from_repeated_message};

use super::attribute::{PlaylistAttributes, PlaylistItemAttributes};

use librespot_core::date::Date;
use librespot_core::spotify_id::SpotifyId;
use librespot_protocol as protocol;

use super::permission::Capabilities;

use protocol::playlist4_external::Item as PlaylistItemMessage;
use protocol::playlist4_external::ListItems as PlaylistItemsMessage;
use protocol::playlist4_external::MetaItem as PlaylistMetaItemMessage;

#[derive(Debug, Clone)]
pub struct PlaylistItem {
    pub id: SpotifyId,
    pub attributes: PlaylistItemAttributes,
}

#[derive(Debug, Clone)]
pub struct PlaylistItems(pub Vec<PlaylistItem>);

impl Deref for PlaylistItems {
    type Target = Vec<PlaylistItem>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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

#[derive(Debug, Clone)]
pub struct PlaylistMetaItems(pub Vec<PlaylistMetaItem>);

impl Deref for PlaylistMetaItems {
    type Target = Vec<PlaylistMetaItem>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<&PlaylistItemMessage> for PlaylistItem {
    type Error = MetadataError;
    fn try_from(item: &PlaylistItemMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            id: item.try_into()?,
            attributes: item.get_attributes().try_into()?,
        })
    }
}

try_from_repeated_message!(PlaylistItemMessage, PlaylistItems);

impl TryFrom<&PlaylistItemsMessage> for PlaylistItemList {
    type Error = MetadataError;
    fn try_from(list_items: &PlaylistItemsMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            position: list_items.get_pos(),
            is_truncated: list_items.get_truncated(),
            items: list_items.get_items().try_into()?,
            meta_items: list_items.get_meta_items().try_into()?,
        })
    }
}

impl TryFrom<&PlaylistMetaItemMessage> for PlaylistMetaItem {
    type Error = MetadataError;
    fn try_from(item: &PlaylistMetaItemMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            revision: item.try_into()?,
            attributes: item.get_attributes().try_into()?,
            length: item.get_length(),
            timestamp: item.get_timestamp().try_into()?,
            owner_username: item.get_owner_username().to_owned(),
            has_abuse_reporting: item.get_abuse_reporting_enabled(),
            capabilities: item.get_capabilities().into(),
        })
    }
}

try_from_repeated_message!(PlaylistMetaItemMessage, PlaylistMetaItems);
