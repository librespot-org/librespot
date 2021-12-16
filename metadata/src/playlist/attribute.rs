use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::fmt::Debug;
use std::ops::Deref;

use crate::{error::MetadataError, image::PictureSizes, util::from_repeated_enum};

use librespot_core::date::Date;
use librespot_core::spotify_id::SpotifyId;
use librespot_protocol as protocol;

use protocol::playlist4_external::FormatListAttribute as PlaylistFormatAttributeMessage;
use protocol::playlist4_external::ItemAttributes as PlaylistItemAttributesMessage;
use protocol::playlist4_external::ItemAttributesPartialState as PlaylistPartialItemAttributesMessage;
use protocol::playlist4_external::ListAttributes as PlaylistAttributesMessage;
use protocol::playlist4_external::ListAttributesPartialState as PlaylistPartialAttributesMessage;
use protocol::playlist4_external::UpdateItemAttributes as PlaylistUpdateItemAttributesMessage;
use protocol::playlist4_external::UpdateListAttributes as PlaylistUpdateAttributesMessage;

pub use protocol::playlist4_external::ItemAttributeKind as PlaylistItemAttributeKind;
pub use protocol::playlist4_external::ListAttributeKind as PlaylistAttributeKind;

#[derive(Debug, Clone)]
pub struct PlaylistAttributes {
    pub name: String,
    pub description: String,
    pub picture: SpotifyId,
    pub is_collaborative: bool,
    pub pl3_version: String,
    pub is_deleted_by_owner: bool,
    pub client_id: String,
    pub format: String,
    pub format_attributes: PlaylistFormatAttribute,
    pub picture_sizes: PictureSizes,
}

#[derive(Debug, Clone)]
pub struct PlaylistAttributeKinds(pub Vec<PlaylistAttributeKind>);

impl Deref for PlaylistAttributeKinds {
    type Target = Vec<PlaylistAttributeKind>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

from_repeated_enum!(PlaylistAttributeKind, PlaylistAttributeKinds);

#[derive(Debug, Clone)]
pub struct PlaylistFormatAttribute(pub HashMap<String, String>);

impl Deref for PlaylistFormatAttribute {
    type Target = HashMap<String, String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct PlaylistItemAttributes {
    pub added_by: String,
    pub timestamp: Date,
    pub seen_at: Date,
    pub is_public: bool,
    pub format_attributes: PlaylistFormatAttribute,
    pub item_id: SpotifyId,
}

#[derive(Debug, Clone)]
pub struct PlaylistItemAttributeKinds(pub Vec<PlaylistItemAttributeKind>);

impl Deref for PlaylistItemAttributeKinds {
    type Target = Vec<PlaylistItemAttributeKind>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

from_repeated_enum!(PlaylistItemAttributeKind, PlaylistItemAttributeKinds);

#[derive(Debug, Clone)]
pub struct PlaylistPartialAttributes {
    #[allow(dead_code)]
    values: PlaylistAttributes,
    #[allow(dead_code)]
    no_value: PlaylistAttributeKinds,
}

#[derive(Debug, Clone)]
pub struct PlaylistPartialItemAttributes {
    #[allow(dead_code)]
    values: PlaylistItemAttributes,
    #[allow(dead_code)]
    no_value: PlaylistItemAttributeKinds,
}

#[derive(Debug, Clone)]
pub struct PlaylistUpdateAttributes {
    pub new_attributes: PlaylistPartialAttributes,
    pub old_attributes: PlaylistPartialAttributes,
}

#[derive(Debug, Clone)]
pub struct PlaylistUpdateItemAttributes {
    pub index: i32,
    pub new_attributes: PlaylistPartialItemAttributes,
    pub old_attributes: PlaylistPartialItemAttributes,
}

impl TryFrom<&PlaylistAttributesMessage> for PlaylistAttributes {
    type Error = MetadataError;
    fn try_from(attributes: &PlaylistAttributesMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            name: attributes.get_name().to_owned(),
            description: attributes.get_description().to_owned(),
            picture: attributes.get_picture().try_into()?,
            is_collaborative: attributes.get_collaborative(),
            pl3_version: attributes.get_pl3_version().to_owned(),
            is_deleted_by_owner: attributes.get_deleted_by_owner(),
            client_id: attributes.get_client_id().to_owned(),
            format: attributes.get_format().to_owned(),
            format_attributes: attributes.get_format_attributes().into(),
            picture_sizes: attributes.get_picture_size().into(),
        })
    }
}

impl From<&[PlaylistFormatAttributeMessage]> for PlaylistFormatAttribute {
    fn from(attributes: &[PlaylistFormatAttributeMessage]) -> Self {
        let format_attributes = attributes
            .iter()
            .map(|attribute| {
                (
                    attribute.get_key().to_owned(),
                    attribute.get_value().to_owned(),
                )
            })
            .collect();

        PlaylistFormatAttribute(format_attributes)
    }
}

impl TryFrom<&PlaylistItemAttributesMessage> for PlaylistItemAttributes {
    type Error = MetadataError;
    fn try_from(attributes: &PlaylistItemAttributesMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            added_by: attributes.get_added_by().to_owned(),
            timestamp: attributes.get_timestamp().try_into()?,
            seen_at: attributes.get_seen_at().try_into()?,
            is_public: attributes.get_public(),
            format_attributes: attributes.get_format_attributes().into(),
            item_id: attributes.get_item_id().try_into()?,
        })
    }
}
impl TryFrom<&PlaylistPartialAttributesMessage> for PlaylistPartialAttributes {
    type Error = MetadataError;
    fn try_from(attributes: &PlaylistPartialAttributesMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            values: attributes.get_values().try_into()?,
            no_value: attributes.get_no_value().into(),
        })
    }
}

impl TryFrom<&PlaylistPartialItemAttributesMessage> for PlaylistPartialItemAttributes {
    type Error = MetadataError;
    fn try_from(attributes: &PlaylistPartialItemAttributesMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            values: attributes.get_values().try_into()?,
            no_value: attributes.get_no_value().into(),
        })
    }
}

impl TryFrom<&PlaylistUpdateAttributesMessage> for PlaylistUpdateAttributes {
    type Error = MetadataError;
    fn try_from(update: &PlaylistUpdateAttributesMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            new_attributes: update.get_new_attributes().try_into()?,
            old_attributes: update.get_old_attributes().try_into()?,
        })
    }
}

impl TryFrom<&PlaylistUpdateItemAttributesMessage> for PlaylistUpdateItemAttributes {
    type Error = MetadataError;
    fn try_from(update: &PlaylistUpdateItemAttributesMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            index: update.get_index(),
            new_attributes: update.get_new_attributes().try_into()?,
            old_attributes: update.get_old_attributes().try_into()?,
        })
    }
}
