use std::{fmt::Debug, ops::Deref};

use time::{
    error::ComponentRange, format_description::well_known::Iso8601, Date as _Date, OffsetDateTime,
    PrimitiveDateTime, Time,
};

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
    pub fn as_timestamp_ms(&self) -> i64 {
        (self.0.unix_timestamp_nanos() / 1_000_000) as i64
    }

    pub fn from_timestamp_ms(timestamp: i64) -> Result<Self, Error> {
        let date_time = OffsetDateTime::from_unix_timestamp_nanos(timestamp as i128 * 1_000_000)?;
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

    pub fn from_iso8601(input: &str) -> Result<Self, Error> {
        let date_time = OffsetDateTime::parse(input, &Iso8601::DEFAULT)?;
        Ok(Self(date_time))
    }
}

impl TryFrom<&DateMessage> for Date {
    type Error = crate::Error;
    fn try_from(msg: &DateMessage) -> Result<Self, Self::Error> {
        // Some metadata contains a year, but no month. In that case just set January.
        let month = if msg.has_month() {
            msg.month() as u8
        } else {
            1
        };

        // Having no day will work, but may be unexpected: it will imply the last day
        // of the month before. So prevent that, and just set day 1.
        let day = if msg.has_day() { msg.day() as u8 } else { 1 };

        let date = _Date::from_calendar_date(msg.year(), month.try_into()?, day)?;
        let time = Time::from_hms(msg.hour() as u8, msg.minute() as u8, 0)?;
        Ok(Self::from_utc(PrimitiveDateTime::new(date, time)))
    }
}

impl From<OffsetDateTime> for Date {
    fn from(datetime: OffsetDateTime) -> Self {
        Self(datetime)
    }
}
