use std::fmt::Debug;

use crate::{
    availability::Availabilities, copyright::Copyrights, episode::Episodes, image::Images,
    restriction::Restrictions, Metadata, RequestResult,
};

use librespot_core::{Error, Session, SpotifyId};

use librespot_protocol as protocol;
pub use protocol::metadata::show::ConsumptionOrder as ShowConsumptionOrder;
pub use protocol::metadata::show::MediaType as ShowMediaType;

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
    pub is_audiobook: bool,
}

#[async_trait]
impl Metadata for Show {
    type Message = protocol::metadata::Show;

    async fn request(session: &Session, show_id: &SpotifyId) -> RequestResult {
        session.spclient().get_show_metadata(show_id).await
    }

    fn parse(msg: &Self::Message, _: &SpotifyId) -> Result<Self, Error> {
        Self::try_from(msg)
    }
}

impl TryFrom<&<Self as Metadata>::Message> for Show {
    type Error = librespot_core::Error;
    fn try_from(show: &<Self as Metadata>::Message) -> Result<Self, Self::Error> {
        Ok(Self {
            id: show.try_into()?,
            name: show.name().to_owned(),
            description: show.description().to_owned(),
            publisher: show.publisher().to_owned(),
            language: show.language().to_owned(),
            is_explicit: show.explicit(),
            covers: show.cover_image.image.as_slice().into(),
            episodes: show.episode.as_slice().try_into()?,
            copyrights: show.copyright.as_slice().into(),
            restrictions: show.restriction.as_slice().into(),
            keywords: show.keyword.to_vec(),
            media_type: show.media_type(),
            consumption_order: show.consumption_order(),
            availability: show.availability.as_slice().try_into()?,
            trailer_uri: SpotifyId::from_uri(show.trailer_uri())?,
            has_music_and_talk: show.music_and_talk(),
            is_audiobook: show.is_audiobook(),
        })
    }
}
