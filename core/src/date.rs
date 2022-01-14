use std::{
    convert::{TryFrom, TryInto},
    fmt::Debug,
    ops::Deref,
};

use time::{error::ComponentRange, Date as _Date, OffsetDateTime, PrimitiveDateTime, Time};

use crate::Error;

use librespot_protocol as protocol;
use protocol::metadata::Date as DateMessage;

impl From<ComponentRange> for Error {
    fn from(err: ComponentRange) -> Self {
        Error::out_of_range(err)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date(pub OffsetDateTime);

impl Deref for Date {
    type Target = OffsetDateTime;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Date {
    pub fn as_timestamp(&self) -> i64 {
        self.0.unix_timestamp()
    }

    pub fn from_timestamp(timestamp: i64) -> Result<Self, Error> {
        let date_time = OffsetDateTime::from_unix_timestamp(timestamp)?;
        Ok(Self(date_time))
    }

    pub fn as_utc(&self) -> OffsetDateTime {
        self.0
    }

    pub fn from_utc(date_time: PrimitiveDateTime) -> Self {
        Self(date_time.assume_utc())
    }

    pub fn now_utc() -> Self {
        Self(OffsetDateTime::now_utc())
    }
}

impl TryFrom<&DateMessage> for Date {
    type Error = crate::Error;
    fn try_from(msg: &DateMessage) -> Result<Self, Self::Error> {
        let date = _Date::from_calendar_date(
            msg.get_year(),
            (msg.get_month() as u8).try_into()?,
            msg.get_day() as u8,
        )?;
        let time = Time::from_hms(msg.get_hour() as u8, msg.get_minute() as u8, 0)?;
        Ok(Self::from_utc(PrimitiveDateTime::new(date, time)))
    }
}

impl From<OffsetDateTime> for Date {
    fn from(datetime: OffsetDateTime) -> Self {
        Self(datetime)
    }
}

impl TryFrom<i64> for Date {
    type Error = crate::Error;
    fn try_from(timestamp: i64) -> Result<Self, Self::Error> {
        Self::from_timestamp(timestamp)
    }
}
