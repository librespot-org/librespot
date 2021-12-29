use std::{fmt::Debug, ops::Deref};

use crate::util::from_repeated_message;

use librespot_protocol as protocol;
use protocol::metadata::ContentRating as ContentRatingMessage;

#[derive(Debug, Clone)]
pub struct ContentRating {
    pub country: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ContentRatings(pub Vec<ContentRating>);

impl Deref for ContentRatings {
    type Target = Vec<ContentRating>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&ContentRatingMessage> for ContentRating {
    fn from(content_rating: &ContentRatingMessage) -> Self {
        Self {
            country: content_rating.get_country().to_owned(),
            tags: content_rating.get_tag().to_vec(),
        }
    }
}

from_repeated_message!(ContentRatingMessage, ContentRatings);
