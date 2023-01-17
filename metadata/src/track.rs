use std::{
    convert::{TryFrom, TryInto},
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use uuid::Uuid;

use crate::{
    artist::{Artists, ArtistsWithRole},
    audio::file::AudioFiles,
    availability::Availabilities,
    content_rating::ContentRatings,
    external_id::ExternalIds,
    restriction::Restrictions,
    sale_period::SalePeriods,
    util::{impl_deref_wrapped, impl_try_from_repeated},
    Album, Metadata, RequestResult,
};

use librespot_core::{date::Date, Error, Session, SpotifyId};
use librespot_protocol as protocol;

#[derive(Debug, Clone)]
pub struct Track {
    pub id: SpotifyId,
    pub name: String,
    pub album: Album,
    pub artists: Artists,
    pub number: i32,
    pub disc_number: i32,
    pub duration: i32,
    pub popularity: i32,
    pub is_explicit: bool,
    pub external_ids: ExternalIds,
    pub restrictions: Restrictions,
    pub files: AudioFiles,
    pub alternatives: Tracks,
    pub sale_periods: SalePeriods,
    pub previews: AudioFiles,
    pub tags: Vec<String>,
    pub earliest_live_timestamp: Date,
    pub has_lyrics: bool,
    pub availability: Availabilities,
    pub licensor: Uuid,
    pub language_of_performance: Vec<String>,
    pub content_ratings: ContentRatings,
    pub original_title: String,
    pub version_title: String,
    pub artists_with_role: ArtistsWithRole,
}

#[derive(Debug, Clone, Default)]
pub struct Tracks(pub Vec<SpotifyId>);

impl_deref_wrapped!(Tracks, Vec<SpotifyId>);

#[async_trait]
impl Metadata for Track {
    type Message = protocol::metadata::Track;

    async fn request(session: &Session, track_id: &SpotifyId) -> RequestResult {
        session.spclient().get_track_metadata(track_id).await
    }

    fn parse(msg: &Self::Message, _: &SpotifyId) -> Result<Self, Error> {
        Self::try_from(msg)
    }
}

impl TryFrom<&<Self as Metadata>::Message> for Track {
    type Error = librespot_core::Error;
    fn try_from(track: &<Self as Metadata>::Message) -> Result<Self, Self::Error> {
        Ok(Self {
            id: track.try_into()?,
            name: track.name().to_owned(),
            album: track.album.get_or_default().try_into()?,
            artists: track.artist.as_slice().try_into()?,
            number: track.number(),
            disc_number: track.disc_number(),
            duration: track.duration(),
            popularity: track.popularity(),
            is_explicit: track.explicit(),
            external_ids: track.external_id.as_slice().into(),
            restrictions: track.restriction.as_slice().into(),
            files: track.file.as_slice().into(),
            alternatives: track.alternative.as_slice().try_into()?,
            sale_periods: track.sale_period.as_slice().try_into()?,
            previews: track.preview.as_slice().into(),
            tags: track.tags.to_vec(),
            earliest_live_timestamp: Date::from_timestamp_ms(track.earliest_live_timestamp())?,
            has_lyrics: track.has_lyrics(),
            availability: track.availability.as_slice().try_into()?,
            licensor: Uuid::from_slice(track.licensor.uuid()).unwrap_or_else(|_| Uuid::nil()),
            language_of_performance: track.language_of_performance.to_vec(),
            content_ratings: track.content_rating.as_slice().into(),
            original_title: track.original_title().to_owned(),
            version_title: track.version_title().to_owned(),
            artists_with_role: track.artist_with_role.as_slice().try_into()?,
        })
    }
}

impl_try_from_repeated!(<Track as Metadata>::Message, Tracks);
