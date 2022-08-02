use std::{
    convert::{TryFrom, TryInto},
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::{
    restriction::Restrictions,
    util::{impl_deref_wrapped, try_from_repeated_message},
};

use librespot_core::date::Date;

use librespot_protocol as protocol;
use protocol::metadata::SalePeriod as SalePeriodMessage;

#[derive(Debug, Clone)]
pub struct SalePeriod {
    pub restrictions: Restrictions,
    pub start: Date,
    pub end: Date,
}

#[derive(Debug, Clone, Default)]
pub struct SalePeriods(pub Vec<SalePeriod>);

impl_deref_wrapped!(SalePeriods, Vec<SalePeriod>);

impl TryFrom<&SalePeriodMessage> for SalePeriod {
    type Error = librespot_core::Error;
    fn try_from(sale_period: &SalePeriodMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            restrictions: sale_period.get_restriction().into(),
            start: sale_period.get_start().try_into()?,
            end: sale_period.get_end().try_into()?,
        })
    }
}

try_from_repeated_message!(SalePeriodMessage, SalePeriods);
