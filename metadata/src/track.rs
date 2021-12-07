use std::convert::{TryFrom, TryInto};
use std::fmt::Debug;
use std::ops::Deref;

use chrono::Local;
use uuid::Uuid;

use crate::{
    artist::{Artists, ArtistsWithRole},
    audio::{
        file::AudioFiles,
        item::{AudioItem, AudioItemResult, InnerAudioItem},
    },
    availability::{Availabilities, UnavailabilityReason},
    content_rating::ContentRatings,
    date::Date,
    error::RequestError,
    external_id::ExternalIds,
    restriction::Restrictions,
    sale_period::SalePeriods,
    util::try_from_repeated_message,
    Metadata, MetadataError, RequestResult,
};

use librespot_core::session::Session;
use librespot_core::spotify_id::SpotifyId;
use librespot_protocol as protocol;

#[derive(Debug, Clone)]
pub struct Track {
    pub id: SpotifyId,
    pub name: String,
    pub album: SpotifyId,
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

#[derive(Debug, Clone)]
pub struct Tracks(pub Vec<SpotifyId>);

impl Deref for Tracks {
    type Target = Vec<SpotifyId>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait]
impl InnerAudioItem for Track {
    async fn get_audio_item(session: &Session, id: SpotifyId) -> AudioItemResult {
        let track = Self::get(session, id).await?;
        let alternatives = {
            if track.alternatives.is_empty() {
                None
            } else {
                Some(track.alternatives.clone())
            }
        };

        // TODO: check meaning of earliest_live_timestamp in
        let availability = if Local::now() < track.earliest_live_timestamp.as_utc() {
            Err(UnavailabilityReason::Embargo)
        } else {
            Self::available_in_country(&track.availability, &track.restrictions, &session.country())
        };

        Ok(AudioItem {
            id,
            spotify_uri: id.to_uri(),
            files: track.files,
            name: track.name,
            duration: track.duration,
            availability,
            alternatives,
        })
    }
}

#[async_trait]
impl Metadata for Track {
    type Message = protocol::metadata::Track;

    async fn request(session: &Session, track_id: SpotifyId) -> RequestResult {
        session
            .spclient()
            .get_track_metadata(track_id)
            .await
            .map_err(RequestError::Http)
    }

    fn parse(msg: &Self::Message, _: SpotifyId) -> Result<Self, MetadataError> {
        Self::try_from(msg)
    }
}

impl TryFrom<&<Self as Metadata>::Message> for Track {
    type Error = MetadataError;
    fn try_from(track: &<Self as Metadata>::Message) -> Result<Self, Self::Error> {
        Ok(Self {
            id: track.try_into()?,
            name: track.get_name().to_owned(),
            album: track.get_album().try_into()?,
            artists: track.get_artist().try_into()?,
            number: track.get_number(),
            disc_number: track.get_disc_number(),
            duration: track.get_duration(),
            popularity: track.get_popularity(),
            is_explicit: track.get_explicit(),
            external_ids: track.get_external_id().into(),
            restrictions: track.get_restriction().into(),
            files: track.get_file().into(),
            alternatives: track.get_alternative().try_into()?,
            sale_periods: track.get_sale_period().into(),
            previews: track.get_preview().into(),
            tags: track.get_tags().to_vec(),
            earliest_live_timestamp: track.get_earliest_live_timestamp().try_into()?,
            has_lyrics: track.get_has_lyrics(),
            availability: track.get_availability().into(),
            licensor: Uuid::from_slice(track.get_licensor().get_uuid())
                .unwrap_or_else(|_| Uuid::nil()),
            language_of_performance: track.get_language_of_performance().to_vec(),
            content_ratings: track.get_content_rating().into(),
            original_title: track.get_original_title().to_owned(),
            version_title: track.get_version_title().to_owned(),
            artists_with_role: track.get_artist_with_role().try_into()?,
        })
    }
}

try_from_repeated_message!(<Track as Metadata>::Message, Tracks);
