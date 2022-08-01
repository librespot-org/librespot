use std::{fmt::Debug, ops::Deref};

use crate::util::{from_repeated_enum, from_repeated_message};

use protocol::metadata::Restriction as RestrictionMessage;

use librespot_protocol as protocol;
pub use protocol::metadata::Restriction_Catalogue as RestrictionCatalogue;
pub use protocol::metadata::Restriction_Type as RestrictionType;

#[derive(Debug, Clone)]
pub struct Restriction {
    pub catalogues: RestrictionCatalogues,
    pub restriction_type: RestrictionType,
    pub catalogue_strs: Vec<String>,
    pub countries_allowed: Option<Vec<String>>,
    pub countries_forbidden: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default)]
pub struct Restrictions(pub Vec<Restriction>);

impl Deref for Restrictions {
    type Target = Vec<Restriction>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct RestrictionCatalogues(pub Vec<RestrictionCatalogue>);

impl Deref for RestrictionCatalogues {
    type Target = Vec<RestrictionCatalogue>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Restriction {
    fn parse_country_codes(country_codes: &str) -> Vec<String> {
        country_codes
            .chunks(2)
            .map(|country_code| country_code.to_owned())
            .collect()
    }
}

impl From<&RestrictionMessage> for Restriction {
    fn from(restriction: &RestrictionMessage) -> Self {
        let countries_allowed = if restriction.has_countries_allowed() {
            Some(Self::parse_country_codes(
                restriction.get_countries_allowed(),
            ))
        } else {
            None
        };

        let countries_forbidden = if restriction.has_countries_forbidden() {
            Some(Self::parse_country_codes(
                restriction.get_countries_forbidden(),
            ))
        } else {
            None
        };

        Self {
            catalogues: restriction.get_catalogue().into(),
            restriction_type: restriction.get_field_type(),
            catalogue_strs: restriction.get_catalogue_str().to_vec(),
            countries_allowed,
            countries_forbidden,
        }
    }
}

from_repeated_message!(RestrictionMessage, Restrictions);
from_repeated_enum!(RestrictionCatalogue, RestrictionCatalogues);

struct StrChunks<'s>(&'s str, usize);

trait StrChunksExt {
    fn chunks(&self, size: usize) -> StrChunks;
}

impl StrChunksExt for str {
    fn chunks(&self, size: usize) -> StrChunks {
        StrChunks(self, size)
    }
}

impl<'s> Iterator for StrChunks<'s> {
    type Item = &'s str;
    fn next(&mut self) -> Option<&'s str> {
        let &mut StrChunks(data, size) = self;
        if data.is_empty() {
            None
        } else {
            let ret = Some(&data[..size]);
            self.0 = &data[size..];
            ret
        }
    }
}
