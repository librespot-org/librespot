use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::{
    artist::Artists,
    availability::Availabilities,
    copyright::Copyrights,
    external_id::ExternalIds,
    image::Images,
    request::RequestResult,
    restriction::Restrictions,
    sale_period::SalePeriods,
    track::Tracks,
    util::{impl_deref_wrapped, impl_try_from_repeated},
    Metadata,
};

use librespot_core::{date::Date, Error, Session, SpotifyId};

use librespot_protocol as protocol;
pub use protocol::metadata::album::Type as AlbumType;
use protocol::metadata::Disc as DiscMessage;

#[derive(Debug, Clone)]
pub struct Album {
    pub id: SpotifyId,
    pub name: String,
    pub artists: Artists,
    pub album_type: AlbumType,
    pub label: String,
    pub date: Date,
    pub popularity: i32,
    pub genres: Vec<String>,
    pub covers: Images,
    pub external_ids: ExternalIds,
    pub discs: Discs,
    pub reviews: Vec<String>,
    pub copyrights: Copyrights,
    pub restrictions: Restrictions,
    pub related: Albums,
    pub sale_periods: SalePeriods,
    pub cover_group: Images,
    pub original_title: String,
    pub version_title: String,
    pub type_str: String,
    pub availability: Availabilities,
}

#[derive(Debug, Clone, Default)]
pub struct Albums(pub Vec<SpotifyId>);

impl_deref_wrapped!(Albums, Vec<SpotifyId>);

#[derive(Debug, Clone)]
pub struct Disc {
    pub number: i32,
    pub name: String,
    pub tracks: Tracks,
}

#[derive(Debug, Clone, Default)]
pub struct Discs(pub Vec<Disc>);

impl_deref_wrapped!(Discs, Vec<Disc>);

impl Album {
    pub fn tracks(&self) -> impl Iterator<Item = &SpotifyId> {
        self.discs.iter().flat_map(|disc| disc.tracks.iter())
    }
}

#[async_trait]
impl Metadata for Album {
    type Message = protocol::metadata::Album;

    async fn request(session: &Session, album_id: &SpotifyId) -> RequestResult {
        session.spclient().get_album_metadata(album_id).await
    }

    fn parse(msg: &Self::Message, _: &SpotifyId) -> Result<Self, Error> {
        Self::try_from(msg)
    }
}

impl TryFrom<&<Self as Metadata>::Message> for Album {
    type Error = librespot_core::Error;
    fn try_from(album: &<Self as Metadata>::Message) -> Result<Self, Self::Error> {
        Ok(Self {
            id: album.try_into()?,
            name: album.name().to_owned(),
            artists: album.artist.as_slice().try_into()?,
            album_type: album.type_(),
            label: album.label().to_owned(),
            date: album.date.get_or_default().try_into()?,
            popularity: album.popularity(),
            genres: album.genre.to_vec(),
            covers: album.cover_group.get_or_default().into(),
            external_ids: album.external_id.as_slice().into(),
            discs: album.disc.as_slice().try_into()?,
            reviews: album.review.to_vec(),
            copyrights: album.copyright.as_slice().into(),
            restrictions: album.restriction.as_slice().into(),
            related: album.related.as_slice().try_into()?,
            sale_periods: album.sale_period.as_slice().try_into()?,
            cover_group: album.cover_group.image.as_slice().into(),
            original_title: album.original_title().to_owned(),
            version_title: album.version_title().to_owned(),
            type_str: album.type_str().to_owned(),
            availability: album.availability.as_slice().try_into()?,
        })
    }
}

impl_try_from_repeated!(<Album as Metadata>::Message, Albums);

impl TryFrom<&DiscMessage> for Disc {
    type Error = librespot_core::Error;
    fn try_from(disc: &DiscMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            number: disc.number(),
            name: disc.name().to_owned(),
            tracks: disc.track.as_slice().try_into()?,
        })
    }
}

impl_try_from_repeated!(DiscMessage, Discs);
