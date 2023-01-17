use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::util::impl_deref_wrapped;
use crate::util::{impl_from_repeated, impl_from_repeated_copy};

use protocol::metadata::Restriction as RestrictionMessage;

use librespot_protocol as protocol;
pub use protocol::metadata::restriction::Catalogue as RestrictionCatalogue;
pub use protocol::metadata::restriction::Type as RestrictionType;

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

impl_deref_wrapped!(Restrictions, Vec<Restriction>);

#[derive(Debug, Clone)]
pub struct RestrictionCatalogues(pub Vec<RestrictionCatalogue>);

impl_deref_wrapped!(RestrictionCatalogues, Vec<RestrictionCatalogue>);

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
            Some(Self::parse_country_codes(restriction.countries_allowed()))
        } else {
            None
        };

        let countries_forbidden = if restriction.has_countries_forbidden() {
            Some(Self::parse_country_codes(restriction.countries_forbidden()))
        } else {
            None
        };

        Self {
            catalogues: restriction
                .catalogue
                .iter()
                .map(|c| c.enum_value_or_default())
                .collect::<Vec<RestrictionCatalogue>>()
                .as_slice()
                .into(),
            restriction_type: restriction
                .type_
                .unwrap_or_default()
                .enum_value_or_default(),
            catalogue_strs: restriction.catalogue_str.to_vec(),
            countries_allowed,
            countries_forbidden,
        }
    }
}

impl_from_repeated!(RestrictionMessage, Restrictions);
impl_from_repeated_copy!(RestrictionCatalogue, RestrictionCatalogues);

struct StrChunks<'s>(&'s str, usize);

trait StrChunksExt {
    fn chunks(&self, size: usize) -> StrChunks<'_>;
}

impl StrChunksExt for str {
    fn chunks(&self, size: usize) -> StrChunks<'_> {
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
