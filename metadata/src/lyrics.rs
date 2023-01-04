use bytes::Bytes;

use librespot_core::{Error, FileId, Session, SpotifyId};

impl Lyrics {
    pub async fn get(session: &Session, id: &SpotifyId) -> Result<Self, Error> {
        let spclient = session.spclient();
        let lyrics = spclient.get_lyrics(id).await?;
        Self::try_from(&lyrics)
    }

    pub async fn get_for_image(
        session: &Session,
        id: &SpotifyId,
        image_id: &FileId,
    ) -> Result<Self, Error> {
        let spclient = session.spclient();
        let lyrics = spclient.get_lyrics_for_image(id, image_id).await?;
        Self::try_from(&lyrics)
    }
}

impl TryFrom<&Bytes> for Lyrics {
    type Error = Error;

    fn try_from(lyrics: &Bytes) -> Result<Self, Self::Error> {
        serde_json::from_slice(lyrics).map_err(|err| err.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lyrics {
    pub colors: Colors,
    pub has_vocal_removal: bool,
    pub lyrics: LyricsInner,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Colors {
    pub background: i32,
    pub highlight_text: i32,
    pub text: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LyricsInner {
    // TODO: 'alternatives' field as an array but I don't know what it's meant for
    pub fullscreen_action: String,
    pub is_dense_typeface: bool,
    pub is_rtl_language: bool,
    pub language: String,
    pub lines: Vec<Line>,
    pub provider: String,
    pub provider_display_name: String,
    pub provider_lyrics_id: String,
    pub sync_lyrics_uri: String,
    pub sync_type: SyncType,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SyncType {
    Unsynced,
    LineSynced,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Line {
    pub start_time_ms: String,
    pub end_time_ms: String,
    pub words: String,
    // TODO: 'syllables' array
}
