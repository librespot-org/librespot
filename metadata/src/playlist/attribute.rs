use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::{
    image::PictureSizes,
    util::{impl_deref_wrapped, impl_from_repeated_copy},
};

use librespot_core::date::Date;

use librespot_protocol as protocol;
use protocol::playlist4_external::FormatListAttribute as PlaylistFormatAttributeMessage;
pub use protocol::playlist4_external::ItemAttributeKind as PlaylistItemAttributeKind;
use protocol::playlist4_external::ItemAttributes as PlaylistItemAttributesMessage;
use protocol::playlist4_external::ItemAttributesPartialState as PlaylistPartialItemAttributesMessage;
pub use protocol::playlist4_external::ListAttributeKind as PlaylistAttributeKind;
use protocol::playlist4_external::ListAttributes as PlaylistAttributesMessage;
use protocol::playlist4_external::ListAttributesPartialState as PlaylistPartialAttributesMessage;
use protocol::playlist4_external::UpdateItemAttributes as PlaylistUpdateItemAttributesMessage;
use protocol::playlist4_external::UpdateListAttributes as PlaylistUpdateAttributesMessage;

#[derive(Debug, Clone)]
pub struct PlaylistAttributes {
    pub name: String,
    pub description: String,
    pub picture: Vec<u8>,
    pub is_collaborative: bool,
    pub pl3_version: String,
    pub is_deleted_by_owner: bool,
    pub client_id: String,
    pub format: String,
    pub format_attributes: PlaylistFormatAttribute,
    pub picture_sizes: PictureSizes,
}

#[derive(Debug, Clone, Default)]
pub struct PlaylistAttributeKinds(pub Vec<PlaylistAttributeKind>);

impl_deref_wrapped!(PlaylistAttributeKinds, Vec<PlaylistAttributeKind>);

impl_from_repeated_copy!(PlaylistAttributeKind, PlaylistAttributeKinds);

#[derive(Debug, Clone, Default)]
pub struct PlaylistFormatAttribute(pub HashMap<String, String>);

impl_deref_wrapped!(PlaylistFormatAttribute, HashMap<String, String>);

#[derive(Debug, Clone)]
pub struct PlaylistItemAttributes {
    pub added_by: String,
    pub timestamp: Date,
    pub seen_at: Date,
    pub is_public: bool,
    pub format_attributes: PlaylistFormatAttribute,
    pub item_id: Vec<u8>,
}

#[derive(Debug, Clone, Default)]
pub struct PlaylistItemAttributeKinds(pub Vec<PlaylistItemAttributeKind>);

impl_deref_wrapped!(PlaylistItemAttributeKinds, Vec<PlaylistItemAttributeKind>);

impl_from_repeated_copy!(PlaylistItemAttributeKind, PlaylistItemAttributeKinds);

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
    type Error = librespot_core::Error;
    fn try_from(attributes: &PlaylistAttributesMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            name: attributes.get_name().to_owned(),
            description: attributes.get_description().to_owned(),
            picture: attributes.get_picture().to_owned(),
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
    type Error = librespot_core::Error;
    fn try_from(attributes: &PlaylistItemAttributesMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            added_by: attributes.get_added_by().to_owned(),
            timestamp: Date::from_timestamp_ms(attributes.get_timestamp())?,
            seen_at: Date::from_timestamp_ms(attributes.get_seen_at())?,
            is_public: attributes.get_public(),
            format_attributes: attributes.get_format_attributes().into(),
            item_id: attributes.get_item_id().to_owned(),
        })
    }
}
impl TryFrom<&PlaylistPartialAttributesMessage> for PlaylistPartialAttributes {
    type Error = librespot_core::Error;
    fn try_from(attributes: &PlaylistPartialAttributesMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            values: attributes.get_values().try_into()?,
            no_value: attributes.get_no_value().into(),
        })
    }
}

impl TryFrom<&PlaylistPartialItemAttributesMessage> for PlaylistPartialItemAttributes {
    type Error = librespot_core::Error;
    fn try_from(attributes: &PlaylistPartialItemAttributesMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            values: attributes.get_values().try_into()?,
            no_value: attributes.get_no_value().into(),
        })
    }
}

impl TryFrom<&PlaylistUpdateAttributesMessage> for PlaylistUpdateAttributes {
    type Error = librespot_core::Error;
    fn try_from(update: &PlaylistUpdateAttributesMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            new_attributes: update.get_new_attributes().try_into()?,
            old_attributes: update.get_old_attributes().try_into()?,
        })
    }
}

impl TryFrom<&PlaylistUpdateItemAttributesMessage> for PlaylistUpdateItemAttributes {
    type Error = librespot_core::Error;
    fn try_from(update: &PlaylistUpdateItemAttributesMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            index: update.get_index(),
            new_attributes: update.get_new_attributes().try_into()?,
            old_attributes: update.get_old_attributes().try_into()?,
        })
    }
}
