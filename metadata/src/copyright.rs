use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::util::{impl_deref_wrapped, impl_from_repeated};

use librespot_protocol as protocol;
pub use protocol::metadata::copyright::Type as CopyrightType;
use protocol::metadata::Copyright as CopyrightMessage;

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
            copyright_type: copyright.type_(),
            text: copyright.text().to_owned(),
        }
    }
}

impl_from_repeated!(CopyrightMessage, Copyrights);
