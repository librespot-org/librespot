use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::{
    Metadata,
    request::RequestResult,
    util::{impl_deref_wrapped, impl_from_repeated_copy, impl_try_from_repeated},
};

use super::{
    attribute::PlaylistAttributes, diff::PlaylistDiff, item::PlaylistItemList,
    permission::Capabilities,
};

use librespot_core::{Error, Session, SpotifyUri, date::Date, spotify_id::SpotifyId};
use librespot_protocol as protocol;
use protobuf::Message as _;
use protocol::playlist4_external::GeoblockBlockingType as Geoblock;

#[derive(Debug, Clone, Default)]
pub struct Geoblocks(Vec<Geoblock>);

impl_deref_wrapped!(Geoblocks, Vec<Geoblock>);

#[derive(Debug, Clone)]
pub struct Playlist {
    pub id: SpotifyUri,
    pub revision: Vec<u8>,
    pub length: i32,
    pub attributes: PlaylistAttributes,
    pub contents: PlaylistItemList,
    pub diff: Option<PlaylistDiff>,
    pub sync_result: Option<PlaylistDiff>,
    pub resulting_revisions: Playlists,
    pub has_multiple_heads: bool,
    pub is_up_to_date: bool,
    pub nonces: Vec<i64>,
    pub timestamp: Date,
    pub has_abuse_reporting: bool,
    pub capabilities: Capabilities,
    pub geoblocks: Geoblocks,
}

#[derive(Debug, Clone, Default)]
pub struct Playlists(pub Vec<SpotifyId>);

impl_deref_wrapped!(Playlists, Vec<SpotifyId>);

#[derive(Debug, Clone)]
pub struct SelectedListContent {
    pub revision: Vec<u8>,
    pub length: i32,
    pub attributes: PlaylistAttributes,
    pub contents: PlaylistItemList,
    pub diff: Option<PlaylistDiff>,
    pub sync_result: Option<PlaylistDiff>,
    pub resulting_revisions: Playlists,
    pub has_multiple_heads: bool,
    pub is_up_to_date: bool,
    pub nonces: Vec<i64>,
    pub timestamp: Date,
    pub owner_username: String,
    pub has_abuse_reporting: bool,
    pub capabilities: Capabilities,
    pub geoblocks: Geoblocks,
}

impl Playlist {
    pub fn tracks(&self) -> impl ExactSizeIterator<Item = &SpotifyUri> {
        let tracks = self.contents.items.iter().map(|item| &item.id);

        let length = tracks.len();
        let expected_length = self.length as usize;
        if length != expected_length {
            warn!("Got {length} tracks, but the list should contain {expected_length} tracks.",);
        }

        tracks
    }

    pub fn name(&self) -> &str {
        &self.attributes.name
    }

    /// `length` items starting at `from`, with the list's header.
    ///
    /// Unlike [`Metadata::get`], this does not download the whole list:
    /// `contents.items` holds only the requested window and
    /// `contents.position` says where it starts, while `length` still counts
    /// the whole list. A `length` of zero fetches the header alone.
    pub async fn get_range(
        session: &Session,
        playlist_uri: &SpotifyUri,
        from: usize,
        length: usize,
    ) -> Result<Self, Error> {
        let SpotifyUri::Playlist {
            id: playlist_id, ..
        } = playlist_uri
        else {
            return Err(Error::invalid_argument("playlist_uri"));
        };

        let response = session
            .spclient()
            .get_playlist_range(playlist_id, from, length)
            .await?;
        let msg = <Self as Metadata>::Message::parse_from_bytes(&response)?;
        Self::parse(&msg, playlist_uri)
    }
}

#[async_trait]
impl Metadata for Playlist {
    type Message = protocol::playlist4_external::SelectedListContent;

    async fn request(session: &Session, playlist_uri: &SpotifyUri) -> RequestResult {
        let SpotifyUri::Playlist {
            id: playlist_id, ..
        } = playlist_uri
        else {
            return Err(Error::invalid_argument("playlist_uri"));
        };

        session.spclient().get_playlist(playlist_id).await
    }

    fn parse(msg: &Self::Message, uri: &SpotifyUri) -> Result<Self, Error> {
        let SpotifyUri::Playlist {
            id: playlist_id, ..
        } = uri
        else {
            return Err(Error::invalid_argument("playlist_uri"));
        };

        // the playlist proto doesn't contain the id so we decorate it
        let playlist = SelectedListContent::try_from(msg)?;

        let new_uri = SpotifyUri::Playlist {
            id: *playlist_id,
            user: Some(playlist.owner_username),
        };

        Ok(Self {
            id: new_uri,
            revision: playlist.revision,
            length: playlist.length,
            attributes: playlist.attributes,
            contents: playlist.contents,
            diff: playlist.diff,
            sync_result: playlist.sync_result,
            resulting_revisions: playlist.resulting_revisions,
            has_multiple_heads: playlist.has_multiple_heads,
            is_up_to_date: playlist.is_up_to_date,
            nonces: playlist.nonces,
            timestamp: playlist.timestamp,
            has_abuse_reporting: playlist.has_abuse_reporting,
            capabilities: playlist.capabilities,
            geoblocks: playlist.geoblocks,
        })
    }
}

impl TryFrom<&<Playlist as Metadata>::Message> for SelectedListContent {
    type Error = librespot_core::Error;
    fn try_from(playlist: &<Playlist as Metadata>::Message) -> Result<Self, Self::Error> {
        let timestamp = playlist.timestamp();
        let timestamp = if timestamp > 9295169800000 {
            // timestamp is way out of range for milliseconds. Some seem to be in microseconds?
            // Observed on playlists where:
            //   format: "artist-mix-reader"
            //   format_attributes {
            //     key: "mediaListConfig"
            //     value: "spotify:medialistconfig:artist-seed-mix:default_v18"
            //   }
            warn!("timestamp is very large; assuming it's in microseconds");
            timestamp / 1000
        } else {
            timestamp
        };
        let timestamp = Date::from_timestamp_ms(timestamp)?;

        Ok(Self {
            revision: playlist.revision().to_owned(),
            length: playlist.length(),
            attributes: playlist.attributes.get_or_default().try_into()?,
            contents: playlist.contents.get_or_default().try_into()?,
            diff: playlist.diff.as_ref().map(TryInto::try_into).transpose()?,
            sync_result: playlist
                .sync_result
                .as_ref()
                .map(TryInto::try_into)
                .transpose()?,
            resulting_revisions: Playlists(
                playlist
                    .resulting_revisions
                    .iter()
                    .map(std::convert::TryInto::try_into)
                    .collect::<Result<Vec<SpotifyId>, Error>>()?,
            ),
            has_multiple_heads: playlist.multiple_heads(),
            is_up_to_date: playlist.up_to_date(),
            nonces: playlist.nonces.clone(),
            timestamp,
            owner_username: playlist.owner_username().to_owned(),
            has_abuse_reporting: playlist.abuse_reporting_enabled(),
            capabilities: playlist.capabilities.get_or_default().into(),
            geoblocks: Geoblocks(
                playlist
                    .geoblock
                    .iter()
                    .map(protobuf::EnumOrUnknown::enum_value_or_default)
                    .collect(),
            ),
        })
    }
}

impl_from_repeated_copy!(Geoblock, Geoblocks);
impl_try_from_repeated!(Vec<u8>, Playlists);

#[cfg(test)]
mod tests {
    use super::*;

    /// A windowed answer parses like a whole one: the rows are the window,
    /// `position` says where it starts, and `length` still counts the list.
    #[test]
    fn a_window_keeps_its_place_in_the_whole_list() {
        let mut msg = protocol::playlist4_external::SelectedListContent::new();
        msg.set_revision(vec![0, 0, 0, 7]);
        msg.set_length(500);
        msg.set_owner_username("someone".into());
        let contents = msg.contents.mut_or_insert_default();
        contents.set_pos(100);
        contents.set_truncated(true);
        for track in ["4uLU6hMCjMI75M1A2tKUQC", "7GhIk7Il098yCjg4BQjzvb"] {
            let mut item = protocol::playlist4_external::Item::new();
            item.set_uri(format!("spotify:track:{track}"));
            contents.items.push(item);
        }

        let uri = SpotifyUri::Playlist {
            id: SpotifyId::from_base62("37i9dQZF1DXbIbVYph0Zr5").unwrap(),
            user: None,
        };
        let playlist = Playlist::parse(&msg, &uri).unwrap();

        assert_eq!(playlist.contents.position, 100);
        assert_eq!(playlist.contents.items.len(), 2);
        assert!(playlist.contents.is_truncated);
        assert_eq!(playlist.length, 500, "the whole list, not the window");
        assert_eq!(playlist.revision, vec![0, 0, 0, 7]);
        assert!(
            matches!(&playlist.id, SpotifyUri::Playlist { user: Some(owner), .. } if owner == "someone"),
            "the owner rides on the id"
        );
    }
}
