use std::{
    convert::{TryFrom, TryInto},
    fmt::Debug,
    ops::Deref,
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
    util::{from_repeated_message, try_from_repeated_message},
    Metadata,
};

use librespot_core::{Error, Session, SpotifyId};

use librespot_protocol as protocol;
pub use protocol::metadata::ArtistWithRole_ArtistRole as ArtistRole;

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
pub struct Artists(pub Vec<SpotifyId>);

impl Deref for Artists {
    type Target = Vec<SpotifyId>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct ArtistWithRole {
    pub id: SpotifyId,
    pub name: String,
    pub role: ArtistRole,
}

#[derive(Debug, Clone, Default)]
pub struct ArtistsWithRole(pub Vec<ArtistWithRole>);

impl Deref for ArtistsWithRole {
    type Target = Vec<ArtistWithRole>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct TopTracks {
    pub country: String,
    pub tracks: Tracks,
}

#[derive(Debug, Clone, Default)]
pub struct CountryTopTracks(pub Vec<TopTracks>);

impl Deref for CountryTopTracks {
    type Target = Vec<TopTracks>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Default)]
pub struct AlbumGroup(pub Albums);

impl Deref for AlbumGroup {
    type Target = Albums;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Default)]
pub struct AlbumGroups(pub Vec<AlbumGroup>);

impl Deref for AlbumGroups {
    type Target = Vec<AlbumGroup>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct Biography {
    pub text: String,
    pub portraits: Images,
    pub portrait_group: Vec<Images>,
}

#[derive(Debug, Clone, Default)]
pub struct Biographies(pub Vec<Biography>);

impl Deref for Biographies {
    type Target = Vec<Biography>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct ActivityPeriod {
    pub start_year: i32,
    pub end_year: i32,
    pub decade: i32,
}

#[derive(Debug, Clone, Default)]
pub struct ActivityPeriods(pub Vec<ActivityPeriod>);

impl Deref for ActivityPeriods {
    type Target = Vec<ActivityPeriod>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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
    pub fn albums_current(&self) -> Albums {
        self.albums.current_releases()
    }

    pub fn singles_current(&self) -> Albums {
        self.singles.current_releases()
    }

    pub fn compilations_current(&self) -> Albums {
        self.compilations.current_releases()
    }

    pub fn appears_on_albums_current(&self) -> Albums {
        self.appears_on_albums.current_releases()
    }
}

#[async_trait]
impl Metadata for Artist {
    type Message = protocol::metadata::Artist;

    async fn request(session: &Session, artist_id: SpotifyId) -> RequestResult {
        session.spclient().get_artist_metadata(artist_id).await
    }

    fn parse(msg: &Self::Message, _: SpotifyId) -> Result<Self, Error> {
        Self::try_from(msg)
    }
}

impl TryFrom<&<Self as Metadata>::Message> for Artist {
    type Error = librespot_core::Error;
    fn try_from(artist: &<Self as Metadata>::Message) -> Result<Self, Self::Error> {
        Ok(Self {
            id: artist.try_into()?,
            name: artist.get_name().to_owned(),
            popularity: artist.get_popularity(),
            top_tracks: artist.get_top_track().try_into()?,
            albums: artist.get_album_group().try_into()?,
            singles: artist.get_single_group().try_into()?,
            compilations: artist.get_compilation_group().try_into()?,
            appears_on_albums: artist.get_appears_on_group().try_into()?,
            genre: artist.get_genre().to_vec(),
            external_ids: artist.get_external_id().into(),
            portraits: artist.get_portrait().into(),
            biographies: artist.get_biography().into(),
            activity_periods: artist.get_activity_period().into(),
            restrictions: artist.get_restriction().into(),
            related: artist.get_related().try_into()?,
            is_portrait_album_cover: artist.get_is_portrait_album_cover(),
            portrait_group: artist.get_portrait_group().get_image().into(),
            sales_periods: artist.get_sale_period().try_into()?,
            availabilities: artist.get_availability().try_into()?,
        })
    }
}

try_from_repeated_message!(<Artist as Metadata>::Message, Artists);

impl TryFrom<&ArtistWithRoleMessage> for ArtistWithRole {
    type Error = librespot_core::Error;
    fn try_from(artist_with_role: &ArtistWithRoleMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            id: artist_with_role.try_into()?,
            name: artist_with_role.get_artist_name().to_owned(),
            role: artist_with_role.get_role(),
        })
    }
}

try_from_repeated_message!(ArtistWithRoleMessage, ArtistsWithRole);

impl TryFrom<&TopTracksMessage> for TopTracks {
    type Error = librespot_core::Error;
    fn try_from(top_tracks: &TopTracksMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            country: top_tracks.get_country().to_owned(),
            tracks: top_tracks.get_track().try_into()?,
        })
    }
}

try_from_repeated_message!(TopTracksMessage, CountryTopTracks);

impl TryFrom<&AlbumGroupMessage> for AlbumGroup {
    type Error = librespot_core::Error;
    fn try_from(album_groups: &AlbumGroupMessage) -> Result<Self, Self::Error> {
        Ok(Self(album_groups.get_album().try_into()?))
    }
}

impl AlbumGroups {
    pub fn current_releases(&self) -> Albums {
        let albums = self
            .iter()
            .filter_map(|agrp| agrp.first())
            .cloned()
            .collect();
        Albums(albums)
    }
}

try_from_repeated_message!(AlbumGroupMessage, AlbumGroups);

impl From<&BiographyMessage> for Biography {
    fn from(biography: &BiographyMessage) -> Self {
        let portrait_group = biography
            .get_portrait_group()
            .iter()
            .map(|it| it.get_image().into())
            .collect();

        Self {
            text: biography.get_text().to_owned(),
            portraits: biography.get_portrait().into(),
            portrait_group,
        }
    }
}

from_repeated_message!(BiographyMessage, Biographies);

impl From<&ActivityPeriodMessage> for ActivityPeriod {
    fn from(activity_periode: &ActivityPeriodMessage) -> Self {
        Self {
            start_year: activity_periode.get_start_year(),
            end_year: activity_periode.get_end_year(),
            decade: activity_periode.get_decade(),
        }
    }
}

from_repeated_message!(ActivityPeriodMessage, ActivityPeriods);
