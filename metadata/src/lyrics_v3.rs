//! Color-lyrics v3 protobuf responses, separate from the existing v2 JSON types.
//!
//! Times are milliseconds. Preview lines and translations remain separate from
//! full lyrics. Callers should inspect `cap_status` and `sync_type`, including
//! unknown enum values, before presenting lyrics as complete or synchronized.

use bytes::Bytes;
use protobuf::Message;

use librespot_core::{Error, Session, SpotifyId};
use librespot_protocol::color_lyrics::LyricsWrapperResponse;

pub use librespot_protocol::color_lyrics::{
    LyricsV3Response,
    lyrics_v3response::{
        CapStatus, Language, LyricsLine, Provider, SyncType, Translation, lyrics_line::SyncData,
    },
};

/// A v3 response with its full/preview lyrics, provider, language and cap status.
///
/// ```no_run
/// # async fn example(session: &librespot_core::Session, id: &librespot_core::SpotifyId)
/// #     -> Result<(), librespot_core::Error> {
/// use librespot_metadata::lyrics_v3::{CapStatus, LyricsV3};
///
/// let response = LyricsV3::get(session, id, "en").await?;
/// if response.lyrics.cap_status.enum_value() == Ok(CapStatus::UNCAPPED) {
///     for line in &response.lyrics.lines {
///         println!("{}: {}", line.start_time_ms, line.words);
///     }
/// }
/// # Ok(())
/// # }
/// ```
///
/// This does not replace or implicitly convert to the v2 `Lyrics` model: doing
/// so would lose rich sync, preview/translation boundaries and cap information.
#[derive(Debug, Clone, PartialEq)]
pub struct LyricsV3 {
    pub lyrics: LyricsV3Response,
    pub has_vocal_removal: bool,
}

impl LyricsV3 {
    /// Request full lyrics using the session's existing authentication.
    ///
    /// This does not create a player or change playback. HTTP failures are
    /// returned to the caller; there is no automatic v2 fallback.
    pub async fn get(
        session: &Session,
        id: &SpotifyId,
        client_language: &str,
    ) -> Result<Self, Error> {
        let response = session
            .spclient()
            .get_lyrics_v3(id, client_language)
            .await?;
        Self::try_from(&response)
    }
}

impl TryFrom<&Bytes> for LyricsV3 {
    type Error = Error;

    fn try_from(bytes: &Bytes) -> Result<Self, Self::Error> {
        // Bound parsing work for callers supplying their own response bytes.
        // This is not a transport download limit.
        if bytes.len() > 256 * 1024 {
            return Err(Error::resource_exhausted("lyrics response exceeds 256 KiB"));
        }
        let response = LyricsWrapperResponse::parse_from_bytes(bytes)?;
        let lyrics = response
            .lyrics
            .into_option()
            .ok_or_else(|| Error::not_found("lyrics response has no lyrics"))?;
        Ok(Self {
            lyrics,
            has_vocal_removal: response.has_vocal_removal,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use librespot_core::error::ErrorKind;

    // Synthetic wire data only: no account data or captured song lyrics.
    fn varint(mut value: u64) -> Vec<u8> {
        let mut bytes = Vec::new();
        while value >= 128 {
            bytes.push((value as u8 & 127) | 128);
            value >>= 7;
        }
        bytes.push(value as u8);
        bytes
    }

    fn number(field: u64, value: u64) -> Vec<u8> {
        [varint(field << 3), varint(value)].concat()
    }

    fn message(field: u64, bytes: &[u8]) -> Vec<u8> {
        [
            varint((field << 3) | 2),
            varint(bytes.len() as u64),
            bytes.to_vec(),
        ]
        .concat()
    }

    fn parse(lyrics: &[u8]) -> LyricsV3 {
        LyricsV3::try_from(&Bytes::from(message(1, lyrics))).unwrap()
    }

    #[test]
    fn line_synced_milliseconds_and_provider() {
        // Line: start=1000, end=2500, words="Test".
        let line = [8, 232, 7, 16, 196, 19, 26, 4, b'T', b'e', b's', b't'];
        let provider = [
            message(1, b"SyntheticProvider"),
            message(2, b"synthetic-id"),
            message(3, b"Synthetic Provider"),
        ]
        .concat();
        let body = [
            number(1, 1),
            message(2, &line),
            message(4, &provider),
            message(6, &message(1, b"en")),
            number(7, 1),
        ]
        .concat();
        let bytes = Bytes::from([message(1, &body), number(2, 1)].concat());
        let parsed = LyricsV3::try_from(&bytes).unwrap();
        assert!(parsed.has_vocal_removal);
        assert_eq!(
            parsed.lyrics.sync_type.enum_value(),
            Ok(SyncType::LINE_SYNCED)
        );
        assert_eq!(
            parsed.lyrics.cap_status.enum_value(),
            Ok(CapStatus::UNCAPPED)
        );
        let line = &parsed.lyrics.lines[0];
        assert_eq!((line.start_time_ms, line.end_time_ms), (1000, 2500));
        assert_eq!(line.words, "Test");
        let provider = parsed.lyrics.provider.as_ref().unwrap();
        assert_eq!(provider.name, "SyntheticProvider");
        assert_eq!(provider.lyrics_id, "synthetic-id");
        assert_eq!(provider.display_name, "Synthetic Provider");
        assert_eq!(parsed.lyrics.language.as_ref().unwrap().code, "en");
    }

    #[test]
    fn rich_sync_and_rtl_are_preserved() {
        let sync = [number(1, 1000), number(2, 1700), number(3, 4)].concat();
        let line = [
            number(1, 1000),
            number(2, 2500),
            message(3, "اختبار".as_bytes()),
            message(4, &sync),
        ]
        .concat();
        let language = [message(1, b"ar"), number(2, 1)].concat();
        let parsed = parse(&[number(1, 2), message(2, &line), message(6, &language)].concat());
        assert_eq!(
            parsed.lyrics.sync_type.enum_value(),
            Ok(SyncType::RICH_SYNCED)
        );
        assert!(parsed.lyrics.language.as_ref().unwrap().is_rtl);
        assert_eq!(parsed.lyrics.lines[0].words, "اختبار");
        let sync = &parsed.lyrics.lines[0].sync_data[0];
        assert_eq!(
            (sync.start_time_ms, sync.end_time_ms, sync.num_chars),
            (1000, 1700, 4)
        );
    }

    #[test]
    fn previews_translations_and_capped_status_do_not_become_full_lyrics() {
        let preview = message(3, b"Synthetic preview");
        let translation = [
            message(1, &message(1, b"es")),
            message(2, b"Synthetic translation"),
        ]
        .concat();
        let parsed =
            parse(&[message(3, &preview), message(5, &translation), number(7, 3)].concat());
        assert!(parsed.lyrics.lines.is_empty());
        assert_eq!(parsed.lyrics.preview_lines[0].words, "Synthetic preview");
        assert_eq!(parsed.lyrics.cap_status.enum_value(), Ok(CapStatus::CAPPED));
        let translation = parsed.lyrics.translation.as_ref().unwrap();
        assert_eq!(translation.language.as_ref().unwrap().code, "es");
        assert_eq!(translation.lines, ["Synthetic translation"]);
    }

    #[test]
    fn unsynced_text_does_not_invent_timestamps() {
        let parsed = parse(&message(2, &message(3, b"Synthetic unsynced text")));
        assert_eq!(parsed.lyrics.sync_type.enum_value(), Ok(SyncType::UNSYNCED));
        assert_eq!(parsed.lyrics.lines[0].start_time_ms, 0);
        assert!(parsed.lyrics.lines[0].sync_data.is_empty());
    }

    #[test]
    fn future_fields_and_enum_values_are_preserved() {
        let body = [number(1, 99), number(7, 2), message(90, b"future")].concat();
        let parsed = parse(&body);
        assert_eq!(parsed.lyrics.sync_type.enum_value(), Err(99));
        assert_eq!(parsed.lyrics.cap_status.enum_value(), Err(2));
        assert!(
            parsed
                .lyrics
                .special_fields
                .unknown_fields()
                .get(90)
                .is_some()
        );
        let decoded =
            LyricsV3Response::parse_from_bytes(&parsed.lyrics.write_to_bytes().unwrap()).unwrap();
        assert_eq!(decoded, parsed.lyrics);
    }

    #[test]
    fn missing_lyrics_is_distinct_from_an_empty_lyrics_message() {
        assert_eq!(
            LyricsV3::try_from(&Bytes::new()).unwrap_err().kind,
            ErrorKind::NotFound
        );
        assert!(parse(&[]).lyrics.lines.is_empty());
    }

    #[test]
    fn malformed_protobuf_and_invalid_utf8_are_rejected() {
        for bytes in [
            vec![10, 255],
            vec![0],
            message(1, &message(2, &message(3, &[255]))),
        ] {
            assert!(LyricsV3::try_from(&Bytes::from(bytes)).is_err());
        }
    }

    #[test]
    fn oversized_payload_is_rejected_before_parsing() {
        assert_eq!(
            LyricsV3::try_from(&Bytes::from(vec![0; 256 * 1024 + 1]))
                .unwrap_err()
                .kind,
            ErrorKind::ResourceExhausted
        );
    }
}
