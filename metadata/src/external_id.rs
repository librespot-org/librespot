use std::fmt::Debug;
use std::ops::Deref;

use crate::util::from_repeated_message;

use librespot_protocol as protocol;

use protocol::metadata::ExternalId as ExternalIdMessage;

#[derive(Debug, Clone)]
pub struct ExternalId {
    pub external_type: String,
    pub id: String, // this can be anything from a URL to a ISRC, EAN or UPC
}

#[derive(Debug, Clone)]
pub struct ExternalIds(pub Vec<ExternalId>);

impl Deref for ExternalIds {
    type Target = Vec<ExternalId>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&ExternalIdMessage> for ExternalId {
    fn from(external_id: &ExternalIdMessage) -> Self {
        Self {
            external_type: external_id.get_field_type().to_owned(),
            id: external_id.get_id().to_owned(),
        }
    }
}

from_repeated_message!(ExternalIdMessage, ExternalIds);
