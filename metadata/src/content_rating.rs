use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::util::{from_repeated_message, impl_deref_wrapped};

use librespot_protocol as protocol;
use protocol::metadata::ContentRating as ContentRatingMessage;

#[derive(Debug, Clone)]
pub struct ContentRating {
    pub country: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ContentRatings(pub Vec<ContentRating>);

impl_deref_wrapped!(ContentRatings, Vec<ContentRating>);

impl From<&ContentRatingMessage> for ContentRating {
    fn from(content_rating: &ContentRatingMessage) -> Self {
        Self {
            country: content_rating.get_country().to_owned(),
            tags: content_rating.get_tag().to_vec(),
        }
    }
}

from_repeated_message!(ContentRatingMessage, ContentRatings);
