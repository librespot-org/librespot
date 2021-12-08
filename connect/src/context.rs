use crate::core::spotify_id::SpotifyId;
use crate::protocol::spirc::TrackRef;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct StationContext {
    pub uri: Option<String>,
    pub next_page_url: String,
    #[serde(deserialize_with = "deserialize_protobuf_TrackRef")]
    pub tracks: Vec<TrackRef>,
    // Not required for core functionality
    // pub seeds: Vec<String>,
    // #[serde(rename = "imageUri")]
    // pub image_uri: String,
    // pub subtitle: Option<String>,
    // pub subtitles: Vec<String>,
    // #[serde(rename = "subtitleUri")]
    // pub subtitle_uri: Option<String>,
    // pub title: String,
    // #[serde(rename = "titleUri")]
    // pub title_uri: String,
    // pub related_artists: Vec<ArtistContext>,
}

#[derive(Deserialize, Debug)]
pub struct PageContext {
    pub uri: String,
    pub next_page_url: String,
    #[serde(deserialize_with = "deserialize_protobuf_TrackRef")]
    pub tracks: Vec<TrackRef>,
    // Not required for core functionality
    // pub url: String,
    // // pub restrictions:
}

#[derive(Deserialize, Debug)]
pub struct TrackContext {
    #[serde(rename = "original_gid")]
    pub gid: String,
    pub uri: String,
    pub uid: String,
    // Not required for core functionality
    // pub album_uri: String,
    // pub artist_uri: String,
    // pub metadata: MetadataContext,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ArtistContext {
    artist_name: String,
    artist_uri: String,
    image_uri: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct MetadataContext {
    album_title: String,
    artist_name: String,
    artist_uri: String,
    image_url: String,
    title: String,
    uid: String,
}

#[allow(non_snake_case)]
fn deserialize_protobuf_TrackRef<'d, D>(de: D) -> Result<Vec<TrackRef>, D::Error>
where
    D: serde::Deserializer<'d>,
{
    let v: Vec<TrackContext> = serde::Deserialize::deserialize(de)?;
    let track_vec = v
        .iter()
        .map(|v| {
            let mut t = TrackRef::new();
            //  This has got to be the most round about way of doing this.
            t.set_gid(SpotifyId::from_base62(&v.gid).unwrap().to_raw().to_vec());
            t.set_uri(v.uri.to_owned());

            t
        })
        .collect::<Vec<TrackRef>>();

    Ok(track_vec)
}
