// TODO : move to metadata

use crate::core::deserialize_with::{bool_from_string, vec_json_proto};

use librespot_protocol::player::{ContextPage, ContextTrack, ProvidedTrack};
use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct StationContext {
    pub uri: String,
    pub title: String,
    #[serde(rename = "titleUri")]
    pub title_uri: String,
    pub subtitles: Vec<SubtitleContext>,
    #[serde(rename = "imageUri")]
    pub image_uri: String,
    pub seeds: Vec<String>,
    #[serde(deserialize_with = "vec_json_proto")]
    pub tracks: Vec<ProvidedTrack>,
    pub next_page_url: String,
    pub correlation_id: String,
    pub related_artists: Vec<ArtistContext>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct PageContext {
    #[serde(deserialize_with = "vec_json_proto")]
    pub tracks: Vec<ProvidedTrack>,
    pub next_page_url: String,
    pub correlation_id: String,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct TrackContext {
    pub uri: String,
    pub uid: String,
    pub artist_uri: String,
    pub album_uri: String,
    #[serde(rename = "original_gid")]
    pub gid: String,
    pub metadata: MetadataContext,
    pub name: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ArtistContext {
    #[serde(rename = "artistName")]
    artist_name: String,
    #[serde(rename = "imageUri")]
    image_uri: String,
    #[serde(rename = "artistUri")]
    artist_uri: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Default, Clone)]
pub struct MetadataContext {
    album_title: String,
    artist_name: String,
    artist_uri: String,
    image_url: String,
    title: String,
    #[serde(deserialize_with = "bool_from_string")]
    is_explicit: bool,
    #[serde(deserialize_with = "bool_from_string")]
    is_promotional: bool,
    decision_id: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Default, Clone)]
pub struct SubtitleContext {
    name: String,
    uri: String,
}

impl From<PageContext> for ContextPage {
    fn from(value: PageContext) -> Self {
        Self {
            next_page_url: value.next_page_url,
            tracks: value
                .tracks
                .into_iter()
                .map(|track| ContextTrack {
                    uri: track.uri,
                    metadata: track.metadata,
                    ..Default::default()
                })
                .collect(),
            loading: false,
            ..Default::default()
        }
    }
}
