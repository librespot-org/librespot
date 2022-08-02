use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::util::{from_repeated_message, impl_deref_wrapped};

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

impl_deref_wrapped!(Copyrights, Vec<Copyright>);

impl From<&CopyrightMessage> for Copyright {
    fn from(copyright: &CopyrightMessage) -> Self {
        Self {
            copyright_type: copyright.get_field_type(),
            text: copyright.get_text().to_owned(),
        }
    }
}

from_repeated_message!(CopyrightMessage, Copyrights);
