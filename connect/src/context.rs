// TODO : move to metadata

use crate::core::spotify_id::SpotifyId;
use crate::protocol::spirc::TrackRef;

use serde::{
    de::{Error, Unexpected},
    Deserialize,
};

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
    #[serde(deserialize_with = "deserialize_protobuf_TrackRef")]
    pub tracks: Vec<TrackRef>,
    pub next_page_url: String,
    pub correlation_id: String,
    pub related_artists: Vec<ArtistContext>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct PageContext {
    #[serde(deserialize_with = "deserialize_protobuf_TrackRef")]
    pub tracks: Vec<TrackRef>,
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

fn bool_from_string<'de, D>(de: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    match String::deserialize(de)?.as_ref() {
        "true" => Ok(true),
        "false" => Ok(false),
        other => Err(D::Error::invalid_value(
            Unexpected::Str(other),
            &"true or false",
        )),
    }
}

#[allow(non_snake_case)]
fn deserialize_protobuf_TrackRef<'d, D>(de: D) -> Result<Vec<TrackRef>, D::Error>
where
    D: serde::Deserializer<'d>,
{
    let v: Vec<TrackContext> = serde::Deserialize::deserialize(de)?;
    v.iter()
        .map(|v| {
            let mut t = TrackRef::new();
            //  This has got to be the most round about way of doing this.
            t.set_gid(
                SpotifyId::from_base62(&v.gid)
                    .map_err(|_| {
                        D::Error::invalid_value(
                            Unexpected::Str(&v.gid),
                            &"a Base-62 encoded Spotify ID",
                        )
                    })?
                    .to_raw()
                    .to_vec(),
            );
            t.set_uri(v.uri.to_owned());
            Ok(t)
        })
        .collect::<Result<Vec<TrackRef>, D::Error>>()
}
