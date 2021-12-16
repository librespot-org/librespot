use std::convert::TryFrom;
use std::fmt::Debug;
use std::ops::Deref;

use chrono::{DateTime, Utc};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use thiserror::Error;

use librespot_protocol as protocol;
use protocol::metadata::Date as DateMessage;

#[derive(Debug, Error)]
pub enum DateError {
    #[error("item has invalid date")]
    InvalidTimestamp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date(pub DateTime<Utc>);

impl Deref for Date {
    type Target = DateTime<Utc>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Date {
    pub fn as_timestamp(&self) -> i64 {
        self.0.timestamp()
    }

    pub fn from_timestamp(timestamp: i64) -> Result<Self, DateError> {
        if let Some(date_time) = NaiveDateTime::from_timestamp_opt(timestamp, 0) {
            Ok(Self::from_utc(date_time))
        } else {
            Err(DateError::InvalidTimestamp)
        }
    }

    pub fn as_utc(&self) -> DateTime<Utc> {
        self.0
    }

    pub fn from_utc(date_time: NaiveDateTime) -> Self {
        Self(DateTime::<Utc>::from_utc(date_time, Utc))
    }
}

impl From<&DateMessage> for Date {
    fn from(date: &DateMessage) -> Self {
        let naive_date = NaiveDate::from_ymd(
            date.get_year() as i32,
            date.get_month() as u32,
            date.get_day() as u32,
        );
        let naive_time = NaiveTime::from_hms(date.get_hour() as u32, date.get_minute() as u32, 0);
        let naive_datetime = NaiveDateTime::new(naive_date, naive_time);
        Self(DateTime::<Utc>::from_utc(naive_datetime, Utc))
    }
}

impl From<DateTime<Utc>> for Date {
    fn from(date: DateTime<Utc>) -> Self {
        Self(date)
    }
}

impl TryFrom<i64> for Date {
    type Error = DateError;
    fn try_from(timestamp: i64) -> Result<Self, Self::Error> {
        Self::from_timestamp(timestamp)
    }
}
