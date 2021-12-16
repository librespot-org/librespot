use std::convert::{TryFrom, TryInto};
use std::fmt::Debug;
use std::ops::Deref;

use protobuf::Message;

use crate::{
    error::MetadataError,
    request::{MercuryRequest, RequestResult},
    util::{from_repeated_enum, try_from_repeated_message},
    Metadata,
};

use super::{
    attribute::PlaylistAttributes, diff::PlaylistDiff, item::PlaylistItemList,
    permission::Capabilities,
};

use librespot_core::date::Date;
use librespot_core::session::Session;
use librespot_core::spotify_id::{NamedSpotifyId, SpotifyId};
use librespot_protocol as protocol;

use protocol::playlist4_external::GeoblockBlockingType as Geoblock;

#[derive(Debug, Clone)]
pub struct Geoblocks(Vec<Geoblock>);

impl Deref for Geoblocks {
    type Target = Vec<Geoblock>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct Playlist {
    pub id: NamedSpotifyId,
    pub revision: SpotifyId,
    pub length: i32,
    pub attributes: PlaylistAttributes,
    pub contents: PlaylistItemList,
    pub diff: PlaylistDiff,
    pub sync_result: PlaylistDiff,
    pub resulting_revisions: Playlists,
    pub has_multiple_heads: bool,
    pub is_up_to_date: bool,
    pub nonces: Vec<i64>,
    pub timestamp: Date,
    pub has_abuse_reporting: bool,
    pub capabilities: Capabilities,
    pub geoblocks: Geoblocks,
}

#[derive(Debug, Clone)]
pub struct Playlists(pub Vec<SpotifyId>);

impl Deref for Playlists {
    type Target = Vec<SpotifyId>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct RootPlaylist(pub SelectedListContent);

impl Deref for RootPlaylist {
    type Target = SelectedListContent;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct SelectedListContent {
    pub revision: SpotifyId,
    pub length: i32,
    pub attributes: PlaylistAttributes,
    pub contents: PlaylistItemList,
    pub diff: PlaylistDiff,
    pub sync_result: PlaylistDiff,
    pub resulting_revisions: Playlists,
    pub has_multiple_heads: bool,
    pub is_up_to_date: bool,
    pub nonces: Vec<i64>,
    pub timestamp: Date,
    pub owner_username: String,
    pub has_abuse_reporting: bool,
    pub capabilities: Capabilities,
    pub geoblocks: Geoblocks,
}

impl Playlist {
    #[allow(dead_code)]
    async fn request_for_user(
        session: &Session,
        username: &str,
        playlist_id: SpotifyId,
    ) -> RequestResult {
        let uri = format!(
            "hm://playlist/user/{}/playlist/{}",
            username,
            playlist_id.to_base62()
        );
        <Self as MercuryRequest>::request(session, &uri).await
    }

    #[allow(dead_code)]
    pub async fn get_for_user(
        session: &Session,
        username: &str,
        playlist_id: SpotifyId,
    ) -> Result<Self, MetadataError> {
        let response = Self::request_for_user(session, username, playlist_id).await?;
        let msg = <Self as Metadata>::Message::parse_from_bytes(&response)?;
        Self::parse(&msg, playlist_id)
    }

    pub fn tracks(&self) -> Vec<SpotifyId> {
        let tracks = self
            .contents
            .items
            .iter()
            .map(|item| item.id)
            .collect::<Vec<_>>();

        let length = tracks.len();
        let expected_length = self.length as usize;
        if length != expected_length {
            warn!(
                "Got {} tracks, but the list should contain {} tracks.",
                length, expected_length,
            );
        }

        tracks
    }

    pub fn name(&self) -> &str {
        &self.attributes.name
    }
}

impl MercuryRequest for Playlist {}

#[async_trait]
impl Metadata for Playlist {
    type Message = protocol::playlist4_external::SelectedListContent;

    async fn request(session: &Session, playlist_id: SpotifyId) -> RequestResult {
        let uri = format!("hm://playlist/v2/playlist/{}", playlist_id.to_base62());
        <Self as MercuryRequest>::request(session, &uri).await
    }

    fn parse(msg: &Self::Message, id: SpotifyId) -> Result<Self, MetadataError> {
        // the playlist proto doesn't contain the id so we decorate it
        let playlist = SelectedListContent::try_from(msg)?;
        let id = NamedSpotifyId::from_spotify_id(id, playlist.owner_username);

        Ok(Self {
            id,
            revision: playlist.revision,
            length: playlist.length,
            attributes: playlist.attributes,
            contents: playlist.contents,
            diff: playlist.diff,
            sync_result: playlist.sync_result,
            resulting_revisions: playlist.resulting_revisions,
            has_multiple_heads: playlist.has_multiple_heads,
            is_up_to_date: playlist.is_up_to_date,
            nonces: playlist.nonces,
            timestamp: playlist.timestamp,
            has_abuse_reporting: playlist.has_abuse_reporting,
            capabilities: playlist.capabilities,
            geoblocks: playlist.geoblocks,
        })
    }
}

impl MercuryRequest for RootPlaylist {}

impl RootPlaylist {
    #[allow(dead_code)]
    async fn request_for_user(session: &Session, username: &str) -> RequestResult {
        let uri = format!("hm://playlist/user/{}/rootlist", username,);
        <Self as MercuryRequest>::request(session, &uri).await
    }

    #[allow(dead_code)]
    pub async fn get_root_for_user(
        session: &Session,
        username: &str,
    ) -> Result<Self, MetadataError> {
        let response = Self::request_for_user(session, username).await?;
        let msg = protocol::playlist4_external::SelectedListContent::parse_from_bytes(&response)?;
        Ok(Self(SelectedListContent::try_from(&msg)?))
    }
}

impl TryFrom<&<Playlist as Metadata>::Message> for SelectedListContent {
    type Error = MetadataError;
    fn try_from(playlist: &<Playlist as Metadata>::Message) -> Result<Self, Self::Error> {
        Ok(Self {
            revision: playlist.get_revision().try_into()?,
            length: playlist.get_length(),
            attributes: playlist.get_attributes().try_into()?,
            contents: playlist.get_contents().try_into()?,
            diff: playlist.get_diff().try_into()?,
            sync_result: playlist.get_sync_result().try_into()?,
            resulting_revisions: playlist.get_resulting_revisions().try_into()?,
            has_multiple_heads: playlist.get_multiple_heads(),
            is_up_to_date: playlist.get_up_to_date(),
            nonces: playlist.get_nonces().into(),
            timestamp: playlist.get_timestamp().try_into()?,
            owner_username: playlist.get_owner_username().to_owned(),
            has_abuse_reporting: playlist.get_abuse_reporting_enabled(),
            capabilities: playlist.get_capabilities().into(),
            geoblocks: playlist.get_geoblock().into(),
        })
    }
}

from_repeated_enum!(Geoblock, Geoblocks);
try_from_repeated_message!(Vec<u8>, Playlists);
