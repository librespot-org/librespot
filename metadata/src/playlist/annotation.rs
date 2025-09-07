use std::fmt::Debug;

use protobuf::Message;

use crate::{
    Metadata,
    image::TranscodedPictures,
    request::{MercuryRequest, RequestResult},
};

use librespot_core::{Error, Session, SpotifyId, SpotifyUri};
use librespot_protocol as protocol;
pub use protocol::playlist_annotate3::AbuseReportState;

#[derive(Debug, Clone)]
pub struct PlaylistAnnotation {
    pub description: String,
    pub picture: String,
    pub transcoded_pictures: TranscodedPictures,
    pub has_abuse_reporting: bool,
    pub abuse_report_state: AbuseReportState,
}

#[async_trait]
impl Metadata for PlaylistAnnotation {
    type Message = protocol::playlist_annotate3::PlaylistAnnotation;

    async fn request(session: &Session, playlist_uri: &SpotifyUri) -> RequestResult {
        let current_user = session.username();

        let SpotifyUri::Playlist {
            id: playlist_id, ..
        } = playlist_uri
        else {
            return Err(Error::invalid_argument("playlist_uri"));
        };

        Self::request_for_user(session, &current_user, playlist_id).await
    }

    fn parse(msg: &Self::Message, _: &SpotifyUri) -> Result<Self, Error> {
        Ok(Self {
            description: msg.description().to_owned(),
            picture: msg.picture().to_owned(), // TODO: is this a URL or Spotify URI?
            transcoded_pictures: msg.transcoded_picture.as_slice().try_into()?,
            has_abuse_reporting: msg.is_abuse_reporting_enabled(),
            abuse_report_state: msg.abuse_report_state(),
        })
    }
}

impl PlaylistAnnotation {
    async fn request_for_user(
        session: &Session,
        username: &str,
        playlist_id: &SpotifyId,
    ) -> RequestResult {
        let uri = format!(
            "hm://playlist-annotate/v1/annotation/user/{}/playlist/{}",
            username,
            playlist_id.to_base62()?
        );
        <Self as MercuryRequest>::request(session, &uri).await
    }

    #[allow(dead_code)]
    async fn get_for_user(
        session: &Session,
        username: &str,
        playlist_uri: &SpotifyUri,
    ) -> Result<Self, Error> {
        let SpotifyUri::Playlist {
            id: playlist_id, ..
        } = playlist_uri
        else {
            return Err(Error::invalid_argument("playlist_uri"));
        };

        let response = Self::request_for_user(session, username, playlist_id).await?;
        let msg = <Self as Metadata>::Message::parse_from_bytes(&response)?;
        Self::parse(&msg, playlist_uri)
    }
}

impl MercuryRequest for PlaylistAnnotation {}

impl TryFrom<&<PlaylistAnnotation as Metadata>::Message> for PlaylistAnnotation {
    type Error = librespot_core::Error;
    fn try_from(
        annotation: &<PlaylistAnnotation as Metadata>::Message,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            description: annotation.description().to_owned(),
            picture: annotation.picture().to_owned(),
            transcoded_pictures: annotation.transcoded_picture.as_slice().try_into()?,
            has_abuse_reporting: annotation.is_abuse_reporting_enabled(),
            abuse_report_state: annotation.abuse_report_state(),
        })
    }
}
