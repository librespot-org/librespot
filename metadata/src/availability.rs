use std::{
    convert::TryFrom,
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use thiserror::Error;

use crate::util::{impl_deref_wrapped, impl_try_from_repeated};

use librespot_core::date::Date;

use librespot_protocol as protocol;
use protocol::metadata::Availability as AvailabilityMessage;

pub type AudioItemAvailability = Result<(), UnavailabilityReason>;

#[derive(Debug, Clone)]
pub struct Availability {
    pub catalogue_strs: Vec<String>,
    pub start: Date,
}

#[derive(Debug, Clone, Default)]
pub struct Availabilities(pub Vec<Availability>);

impl_deref_wrapped!(Availabilities, Vec<Availability>);

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
            catalogue_strs: availability.catalogue_str.to_vec(),
            start: availability.start.get_or_default().try_into()?,
        })
    }
}

impl_try_from_repeated!(AvailabilityMessage, Availabilities);
