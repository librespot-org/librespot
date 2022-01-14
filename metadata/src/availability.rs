use std::{
    convert::{TryFrom, TryInto},
    fmt::Debug,
    ops::Deref,
};

use thiserror::Error;

use crate::util::try_from_repeated_message;

use librespot_core::date::Date;

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

impl TryFrom<&AvailabilityMessage> for Availability {
    type Error = librespot_core::Error;
    fn try_from(availability: &AvailabilityMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            catalogue_strs: availability.get_catalogue_str().to_vec(),
            start: availability.get_start().try_into()?,
        })
    }
}

try_from_repeated_message!(AvailabilityMessage, Availabilities);
