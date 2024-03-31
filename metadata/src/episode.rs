use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::{
    audio::file::AudioFiles,
    availability::Availabilities,
    content_rating::ContentRatings,
    image::Images,
    request::RequestResult,
    restriction::Restrictions,
    util::{impl_deref_wrapped, impl_try_from_repeated},
    video::VideoFiles,
    Metadata,
};

use librespot_core::{date::Date, Error, Session, SpotifyId};

use librespot_protocol as protocol;
pub use protocol::metadata::episode::EpisodeType;

#[derive(Debug, Clone)]
pub struct Episode {
    pub id: SpotifyId,
    pub name: String,
    pub duration: i32,
    pub audio: AudioFiles,
    pub description: String,
    pub number: i32,
    pub publish_time: Date,
    pub covers: Images,
    pub language: String,
    pub is_explicit: bool,
    pub show_name: String,
    pub videos: VideoFiles,
    pub video_previews: VideoFiles,
    pub audio_previews: AudioFiles,
    pub restrictions: Restrictions,
    pub freeze_frames: Images,
    pub keywords: Vec<String>,
    pub allow_background_playback: bool,
    pub availability: Availabilities,
    pub external_url: String,
    pub episode_type: EpisodeType,
    pub has_music_and_talk: bool,
    pub content_rating: ContentRatings,
    pub is_audiobook_chapter: bool,
}

#[derive(Debug, Clone, Default)]
pub struct Episodes(pub Vec<SpotifyId>);

impl_deref_wrapped!(Episodes, Vec<SpotifyId>);

#[async_trait]
impl Metadata for Episode {
    type Message = protocol::metadata::Episode;

    async fn request(session: &Session, episode_id: &SpotifyId) -> RequestResult {
        session.spclient().get_episode_metadata(episode_id).await
    }

    fn parse(msg: &Self::Message, _: &SpotifyId) -> Result<Self, Error> {
        Self::try_from(msg)
    }
}

impl TryFrom<&<Self as Metadata>::Message> for Episode {
    type Error = librespot_core::Error;
    fn try_from(episode: &<Self as Metadata>::Message) -> Result<Self, Self::Error> {
        Ok(Self {
            id: episode.try_into()?,
            name: episode.name().to_owned(),
            duration: episode.duration().to_owned(),
            audio: episode.audio.as_slice().into(),
            description: episode.description().to_owned(),
            number: episode.number(),
            publish_time: episode.publish_time.get_or_default().try_into()?,
            covers: episode.cover_image.image.as_slice().into(),
            language: episode.language().to_owned(),
            is_explicit: episode.explicit().to_owned(),
            show_name: episode.show.name().to_owned(),
            videos: episode.video.as_slice().into(),
            video_previews: episode.video_preview.as_slice().into(),
            audio_previews: episode.audio_preview.as_slice().into(),
            restrictions: episode.restriction.as_slice().into(),
            freeze_frames: episode.freeze_frame.image.as_slice().into(),
            keywords: episode.keyword.to_vec(),
            allow_background_playback: episode.allow_background_playback(),
            availability: episode.availability.as_slice().try_into()?,
            external_url: episode.external_url().to_owned(),
            episode_type: episode.type_(),
            has_music_and_talk: episode.music_and_talk(),
            content_rating: episode.content_rating.as_slice().into(),
            is_audiobook_chapter: episode.is_audiobook_chapter(),
        })
    }
}

impl_try_from_repeated!(<Episode as Metadata>::Message, Episodes);
