use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::{
    album::Albums,
    availability::Availabilities,
    external_id::ExternalIds,
    image::Images,
    request::RequestResult,
    restriction::Restrictions,
    sale_period::SalePeriods,
    track::Tracks,
    util::{impl_deref_wrapped, impl_from_repeated, impl_try_from_repeated},
    Metadata,
};

use librespot_core::{Error, Session, SpotifyId};

use librespot_protocol as protocol;
pub use protocol::metadata::artist_with_role::ArtistRole;

use protocol::metadata::ActivityPeriod as ActivityPeriodMessage;
use protocol::metadata::AlbumGroup as AlbumGroupMessage;
use protocol::metadata::ArtistWithRole as ArtistWithRoleMessage;
use protocol::metadata::Biography as BiographyMessage;
use protocol::metadata::TopTracks as TopTracksMessage;

#[derive(Debug, Clone)]
pub struct Artist {
    pub id: SpotifyId,
    pub name: String,
    pub popularity: i32,
    pub top_tracks: CountryTopTracks,
    pub albums: AlbumGroups,
    pub singles: AlbumGroups,
    pub compilations: AlbumGroups,
    pub appears_on_albums: AlbumGroups,
    pub genre: Vec<String>,
    pub external_ids: ExternalIds,
    pub portraits: Images,
    pub biographies: Biographies,
    pub activity_periods: ActivityPeriods,
    pub restrictions: Restrictions,
    pub related: Artists,
    pub is_portrait_album_cover: bool,
    pub portrait_group: Images,
    pub sales_periods: SalePeriods,
    pub availabilities: Availabilities,
}

#[derive(Debug, Clone, Default)]
pub struct Artists(pub Vec<Artist>);

impl_deref_wrapped!(Artists, Vec<Artist>);

#[derive(Debug, Clone)]
pub struct ArtistWithRole {
    pub id: SpotifyId,
    pub name: String,
    pub role: ArtistRole,
}

#[derive(Debug, Clone, Default)]
pub struct ArtistsWithRole(pub Vec<ArtistWithRole>);

impl_deref_wrapped!(ArtistsWithRole, Vec<ArtistWithRole>);

#[derive(Debug, Clone)]
pub struct TopTracks {
    pub country: String,
    pub tracks: Tracks,
}

#[derive(Debug, Clone, Default)]
pub struct CountryTopTracks(pub Vec<TopTracks>);

impl_deref_wrapped!(CountryTopTracks, Vec<TopTracks>);

#[derive(Debug, Clone, Default)]
pub struct AlbumGroup(pub Albums);

impl_deref_wrapped!(AlbumGroup, Albums);

/// `AlbumGroups` contains collections of album variants (different releases of the same album).
/// Ignoring the wrapping types it is structured roughly like this:
/// ```text
/// AlbumGroups [
///     [Album1], [Album2-relelease, Album2-older-release], [Album3]
/// ]
/// ```
/// In most cases only the current variant of each album is needed. A list of every album in its
/// current release variant can be obtained by using [`AlbumGroups::current_releases`]
#[derive(Debug, Clone, Default)]
pub struct AlbumGroups(pub Vec<AlbumGroup>);

impl_deref_wrapped!(AlbumGroups, Vec<AlbumGroup>);

#[derive(Debug, Clone)]
pub struct Biography {
    pub text: String,
    pub portraits: Images,
    pub portrait_group: Vec<Images>,
}

#[derive(Debug, Clone, Default)]
pub struct Biographies(pub Vec<Biography>);

impl_deref_wrapped!(Biographies, Vec<Biography>);

#[derive(Debug, Clone)]
pub enum ActivityPeriod {
    Timespan {
        start_year: u16,
        end_year: Option<u16>,
    },
    Decade(u16),
}

#[derive(Debug, Clone, Default)]
pub struct ActivityPeriods(pub Vec<ActivityPeriod>);

impl_deref_wrapped!(ActivityPeriods, Vec<ActivityPeriod>);

impl CountryTopTracks {
    pub fn for_country(&self, country: &str) -> Tracks {
        if let Some(country) = self.0.iter().find(|top_track| top_track.country == country) {
            return country.tracks.clone();
        }

        if let Some(global) = self.0.iter().find(|top_track| top_track.country.is_empty()) {
            return global.tracks.clone();
        }

        Tracks(vec![]) // none found
    }
}

impl Artist {
    /// Get the full list of albums, not containing duplicate variants of the same albums.
    ///
    /// See also [`AlbumGroups`](struct@AlbumGroups) and [`AlbumGroups::current_releases`]
    pub fn albums_current(&self) -> impl Iterator<Item = &SpotifyId> {
        self.albums.current_releases()
    }

    /// Get the full list of singles, not containing duplicate variants of the same singles.
    ///
    /// See also [`AlbumGroups`](struct@AlbumGroups) and [`AlbumGroups::current_releases`]
    pub fn singles_current(&self) -> impl Iterator<Item = &SpotifyId> {
        self.singles.current_releases()
    }

    /// Get the full list of compilations, not containing duplicate variants of the same
    /// compilations.
    ///
    /// See also [`AlbumGroups`](struct@AlbumGroups) and [`AlbumGroups::current_releases`]
    pub fn compilations_current(&self) -> impl Iterator<Item = &SpotifyId> {
        self.compilations.current_releases()
    }

    /// Get the full list of albums, not containing duplicate variants of the same albums.
    ///
    /// See also [`AlbumGroups`](struct@AlbumGroups) and [`AlbumGroups::current_releases`]
    pub fn appears_on_albums_current(&self) -> impl Iterator<Item = &SpotifyId> {
        self.appears_on_albums.current_releases()
    }
}

#[async_trait]
impl Metadata for Artist {
    type Message = protocol::metadata::Artist;

    async fn request(session: &Session, artist_id: &SpotifyId) -> RequestResult {
        session.spclient().get_artist_metadata(artist_id).await
    }

    fn parse(msg: &Self::Message, _: &SpotifyId) -> Result<Self, Error> {
        Self::try_from(msg)
    }
}

impl TryFrom<&<Self as Metadata>::Message> for Artist {
    type Error = librespot_core::Error;
    fn try_from(artist: &<Self as Metadata>::Message) -> Result<Self, Self::Error> {
        Ok(Self {
            id: artist.try_into()?,
            name: artist.name().to_owned(),
            popularity: artist.popularity(),
            top_tracks: artist.top_track.as_slice().try_into()?,
            albums: artist.album_group.as_slice().try_into()?,
            singles: artist.single_group.as_slice().try_into()?,
            compilations: artist.compilation_group.as_slice().try_into()?,
            appears_on_albums: artist.appears_on_group.as_slice().try_into()?,
            genre: artist.genre.to_vec(),
            external_ids: artist.external_id.as_slice().into(),
            portraits: artist.portrait.as_slice().into(),
            biographies: artist.biography.as_slice().into(),
            activity_periods: artist.activity_period.as_slice().try_into()?,
            restrictions: artist.restriction.as_slice().into(),
            related: artist.related.as_slice().try_into()?,
            is_portrait_album_cover: artist.is_portrait_album_cover(),
            portrait_group: artist
                .portrait_group
                .get_or_default()
                .image
                .as_slice()
                .into(),
            sales_periods: artist.sale_period.as_slice().try_into()?,
            availabilities: artist.availability.as_slice().try_into()?,
        })
    }
}

impl_try_from_repeated!(<Artist as Metadata>::Message, Artists);

impl TryFrom<&ArtistWithRoleMessage> for ArtistWithRole {
    type Error = librespot_core::Error;
    fn try_from(artist_with_role: &ArtistWithRoleMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            id: artist_with_role.try_into()?,
            name: artist_with_role.artist_name().to_owned(),
            role: artist_with_role.role(),
        })
    }
}

impl_try_from_repeated!(ArtistWithRoleMessage, ArtistsWithRole);

impl TryFrom<&TopTracksMessage> for TopTracks {
    type Error = librespot_core::Error;
    fn try_from(top_tracks: &TopTracksMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            country: top_tracks.country().to_owned(),
            tracks: top_tracks.track.as_slice().try_into()?,
        })
    }
}

impl_try_from_repeated!(TopTracksMessage, CountryTopTracks);

impl TryFrom<&AlbumGroupMessage> for AlbumGroup {
    type Error = librespot_core::Error;
    fn try_from(album_groups: &AlbumGroupMessage) -> Result<Self, Self::Error> {
        Ok(Self(album_groups.album.as_slice().try_into()?))
    }
}

impl AlbumGroups {
    /// Get the contained albums. This will only use the latest release / variant of an album if
    /// multiple variants are available. This should be used if multiple variants of the same album
    /// are not explicitely desired.
    pub fn current_releases(&self) -> impl Iterator<Item = &SpotifyId> {
        self.iter().filter_map(|agrp| agrp.first())
    }
}

impl_try_from_repeated!(AlbumGroupMessage, AlbumGroups);

impl From<&BiographyMessage> for Biography {
    fn from(biography: &BiographyMessage) -> Self {
        let portrait_group = biography
            .portrait_group
            .iter()
            .map(|it| it.image.as_slice().into())
            .collect();

        Self {
            text: biography.text().to_owned(),
            portraits: biography.portrait.as_slice().into(),
            portrait_group,
        }
    }
}

impl_from_repeated!(BiographyMessage, Biographies);

impl TryFrom<&ActivityPeriodMessage> for ActivityPeriod {
    type Error = librespot_core::Error;

    fn try_from(period: &ActivityPeriodMessage) -> Result<Self, Self::Error> {
        let activity_period = match (
            period.has_decade(),
            period.has_start_year(),
            period.has_end_year(),
        ) {
            // (decade, start_year, end_year)
            (true, false, false) => Self::Decade(period.decade().try_into()?),
            (false, true, closed_period) => Self::Timespan {
                start_year: period.start_year().try_into()?,
                end_year: closed_period
                    .then(|| period.end_year().try_into())
                    .transpose()?,
            },
            _ => {
                return Err(librespot_core::Error::failed_precondition(
                    "ActivityPeriod is expected to be either a decade or timespan",
                ))
            }
        };
        Ok(activity_period)
    }
}

impl_try_from_repeated!(ActivityPeriodMessage, ActivityPeriods);
