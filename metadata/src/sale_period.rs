use std::{
    convert::TryFrom,
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::{
    restriction::Restrictions,
    util::{impl_deref_wrapped, impl_try_from_repeated},
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
            restrictions: sale_period.restriction.as_slice().into(),
            start: sale_period.start.get_or_default().try_into()?,
            end: sale_period.end.get_or_default().try_into()?,
        })
    }
}

impl_try_from_repeated!(SalePeriodMessage, SalePeriods);
