use std::{fmt::Debug, ops::Deref};

use crate::util::from_repeated_message;

use librespot_protocol as protocol;
use protocol::metadata::Copyright as CopyrightMessage;
pub use protocol::metadata::Copyright_Type as CopyrightType;

#[derive(Debug, Clone)]
pub struct Copyright {
    pub copyright_type: CopyrightType,
    pub text: String,
}

#[derive(Debug, Clone, Default)]
pub struct Copyrights(pub Vec<Copyright>);

impl Deref for Copyrights {
    type Target = Vec<Copyright>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&CopyrightMessage> for Copyright {
    fn from(copyright: &CopyrightMessage) -> Self {
        Self {
            copyright_type: copyright.get_field_type(),
            text: copyright.get_text().to_owned(),
        }
    }
}

from_repeated_message!(CopyrightMessage, Copyrights);
