use std::convert::{TryFrom, TryInto};
use std::fmt::Debug;

use crate::{
    availability::Availabilities, copyright::Copyrights, episode::Episodes, error::RequestError,
    image::Images, restriction::Restrictions, Metadata, MetadataError, RequestResult,
};

use librespot_core::session::Session;
use librespot_core::spotify_id::SpotifyId;
use librespot_protocol as protocol;

pub use protocol::metadata::Show_ConsumptionOrder as ShowConsumptionOrder;
pub use protocol::metadata::Show_MediaType as ShowMediaType;

#[derive(Debug, Clone)]
pub struct Show {
    pub id: SpotifyId,
    pub name: String,
    pub description: String,
    pub publisher: String,
    pub language: String,
    pub is_explicit: bool,
    pub covers: Images,
    pub episodes: Episodes,
    pub copyrights: Copyrights,
    pub restrictions: Restrictions,
    pub keywords: Vec<String>,
    pub media_type: ShowMediaType,
    pub consumption_order: ShowConsumptionOrder,
    pub availability: Availabilities,
    pub trailer_uri: SpotifyId,
    pub has_music_and_talk: bool,
}

#[async_trait]
impl Metadata for Show {
    type Message = protocol::metadata::Show;

    async fn request(session: &Session, show_id: SpotifyId) -> RequestResult {
        session
            .spclient()
            .get_show_metadata(show_id)
            .await
            .map_err(RequestError::Http)
    }

    fn parse(msg: &Self::Message, _: SpotifyId) -> Result<Self, MetadataError> {
        Self::try_from(msg)
    }
}

impl TryFrom<&<Self as Metadata>::Message> for Show {
    type Error = MetadataError;
    fn try_from(show: &<Self as Metadata>::Message) -> Result<Self, Self::Error> {
        Ok(Self {
            id: show.try_into()?,
            name: show.get_name().to_owned(),
            description: show.get_description().to_owned(),
            publisher: show.get_publisher().to_owned(),
            language: show.get_language().to_owned(),
            is_explicit: show.get_explicit(),
            covers: show.get_cover_image().get_image().into(),
            episodes: show.get_episode().try_into()?,
            copyrights: show.get_copyright().into(),
            restrictions: show.get_restriction().into(),
            keywords: show.get_keyword().to_vec(),
            media_type: show.get_media_type(),
            consumption_order: show.get_consumption_order(),
            availability: show.get_availability().into(),
            trailer_uri: SpotifyId::from_uri(show.get_trailer_uri())?,
            has_music_and_talk: show.get_music_and_talk(),
        })
    }
}
