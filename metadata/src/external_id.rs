use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::util::{impl_deref_wrapped, impl_from_repeated};

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
            external_type: external_id.type_().to_owned(),
            id: external_id.id().to_owned(),
        }
    }
}

impl_from_repeated!(ExternalIdMessage, ExternalIds);
