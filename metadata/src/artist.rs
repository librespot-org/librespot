use std::convert::{TryFrom, TryInto};
use std::fmt::Debug;
use std::ops::Deref;

use crate::{
    error::{MetadataError, RequestError},
    request::RequestResult,
    track::Tracks,
    util::try_from_repeated_message,
    Metadata,
};

use librespot_core::session::Session;
use librespot_core::spotify_id::SpotifyId;
use librespot_protocol as protocol;

use protocol::metadata::ArtistWithRole as ArtistWithRoleMessage;
use protocol::metadata::TopTracks as TopTracksMessage;

pub use protocol::metadata::ArtistWithRole_ArtistRole as ArtistRole;

#[derive(Debug, Clone)]
pub struct Artist {
    pub id: SpotifyId,
    pub name: String,
    pub top_tracks: CountryTopTracks,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct CountryTopTracks(pub Vec<TopTracks>);

impl Deref for CountryTopTracks {
    type Target = Vec<TopTracks>;
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

#[async_trait]
impl Metadata for Artist {
    type Message = protocol::metadata::Artist;

    async fn request(session: &Session, artist_id: SpotifyId) -> RequestResult {
        session
            .spclient()
            .get_artist_metadata(artist_id)
            .await
            .map_err(RequestError::Http)
    }

    fn parse(msg: &Self::Message, _: SpotifyId) -> Result<Self, MetadataError> {
        Self::try_from(msg)
    }
}

impl TryFrom<&<Self as Metadata>::Message> for Artist {
    type Error = MetadataError;
    fn try_from(artist: &<Self as Metadata>::Message) -> Result<Self, Self::Error> {
        Ok(Self {
            id: artist.try_into()?,
            name: artist.get_name().to_owned(),
            top_tracks: artist.get_top_track().try_into()?,
        })
    }
}

try_from_repeated_message!(<Artist as Metadata>::Message, Artists);

impl TryFrom<&ArtistWithRoleMessage> for ArtistWithRole {
    type Error = MetadataError;
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
    type Error = MetadataError;
    fn try_from(top_tracks: &TopTracksMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            country: top_tracks.get_country().to_owned(),
            tracks: top_tracks.get_track().try_into()?,
        })
    }
}

try_from_repeated_message!(TopTracksMessage, CountryTopTracks);
