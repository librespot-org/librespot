use std::{fmt::Debug, ops::Deref};

use crate::{restriction::Restrictions, util::from_repeated_message};

use librespot_core::date::Date;

use librespot_protocol as protocol;
use protocol::metadata::SalePeriod as SalePeriodMessage;

#[derive(Debug, Clone)]
pub struct SalePeriod {
    pub restrictions: Restrictions,
    pub start: Date,
    pub end: Date,
}

#[derive(Debug, Clone)]
pub struct SalePeriods(pub Vec<SalePeriod>);

impl Deref for SalePeriods {
    type Target = Vec<SalePeriod>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&SalePeriodMessage> for SalePeriod {
    fn from(sale_period: &SalePeriodMessage) -> Self {
        Self {
            restrictions: sale_period.get_restriction().into(),
            start: sale_period.get_start().into(),
            end: sale_period.get_end().into(),
        }
    }
}

from_repeated_message!(SalePeriodMessage, SalePeriods);
