use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::util::{from_repeated_message, impl_deref_wrapped};

use librespot_protocol as protocol;
use protocol::metadata::ExternalId as ExternalIdMessage;

#[derive(Debug, Clone)]
pub struct ExternalId {
    pub external_type: String,
    pub id: String, // this can be anything from a URL to a ISRC, EAN or UPC
}

#[derive(Debug, Clone, Default)]
pub struct ExternalIds(pub Vec<ExternalId>);

impl_deref_wrapped!(ExternalIds, Vec<ExternalId>);

impl From<&ExternalIdMessage> for ExternalId {
    fn from(external_id: &ExternalIdMessage) -> Self {
        Self {
            external_type: external_id.get_field_type().to_owned(),
            id: external_id.get_id().to_owned(),
        }
    }
}

from_repeated_message!(ExternalIdMessage, ExternalIds);
