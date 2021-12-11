use std::fmt::Debug;
use std::ops::Deref;

use thiserror::Error;

use crate::{date::Date, util::from_repeated_message};

use librespot_protocol as protocol;

use protocol::metadata::Availability as AvailabilityMessage;

pub type AudioItemAvailability = Result<(), UnavailabilityReason>;

#[derive(Debug, Clone)]
pub struct Availability {
    pub catalogue_strs: Vec<String>,
    pub start: Date,
}

#[derive(Debug, Clone)]
pub struct Availabilities(pub Vec<Availability>);

impl Deref for Availabilities {
    type Target = Vec<Availability>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Copy, Clone, Error)]
pub enum UnavailabilityReason {
    #[error("blacklist present and country on it")]
    Blacklisted,
    #[error("available date is in the future")]
    Embargo,
    #[error("required data was not present")]
    NoData,
    #[error("whitelist present and country not on it")]
    NotWhitelisted,
}

impl From<&AvailabilityMessage> for Availability {
    fn from(availability: &AvailabilityMessage) -> Self {
        Self {
            catalogue_strs: availability.get_catalogue_str().to_vec(),
            start: availability.get_start().into(),
        }
    }
}

from_repeated_message!(AvailabilityMessage, Availabilities);
