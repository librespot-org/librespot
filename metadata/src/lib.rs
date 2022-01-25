#![allow(clippy::unused_io_amount)]

#[macro_use]
extern crate log;

#[macro_use]
extern crate async_trait;

pub mod cover;
use std::collections::HashMap;
use std::string::FromUtf8Error;

use librespot_core::mercury::MercuryError;
use librespot_core::session::Session;
use librespot_core::spotify_id::{FileId, SpotifyAudioType, SpotifyId, SpotifyIdError};
use librespot_protocol as protocol;
use protobuf::Message;

pub use crate::protocol::metadata::AudioFile_Format as FileFormat;

fn countrylist_contains(list: &str, country: &str) -> bool {
    list.chunks(2).any(|cc| cc == country)
}

fn parse_restrictions<'s, I>(restrictions: I, country: &str, catalogue: &str) -> bool
where
    I: IntoIterator<Item = &'s protocol::metadata::Restriction>,
{
    let mut forbidden = "".to_string();
    let mut has_forbidden = false;

    let mut allowed = "".to_string();
    let mut has_allowed = false;

    let rs = restrictions
        .into_iter()
        .filter(|r| r.get_catalogue_str().contains(&catalogue.to_owned()));

    for r in rs {
        if r.has_countries_forbidden() {
            forbidden.push_str(r.get_countries_forbidden());
            has_forbidden = true;
        }

        if r.has_countries_allowed() {
            allowed.push_str(r.get_countries_allowed());
            has_allowed = true;
        }
    }

    (has_forbidden || has_allowed)
        && (!has_forbidden || !countrylist_contains(forbidden.as_str(), country))
        && (!has_allowed || countrylist_contains(allowed.as_str(), country))
}

// A wrapper with fields the player needs
#[derive(Debug, Clone)]
pub struct AudioItem {
    pub id: SpotifyId,
    pub uri: String,
    pub files: HashMap<FileFormat, FileId>,
    pub name: String,
    pub duration: i32,
    pub available: bool,
    pub alternatives: Option<Vec<SpotifyId>>,
}

impl AudioItem {
    pub async fn get_audio_item(session: &Session, id: SpotifyId) -> Result<Self, MercuryError> {
        match id.audio_type {
            SpotifyAudioType::Track => Track::get_audio_item(session, id).await,
            SpotifyAudioType::Podcast => Episode::get_audio_item(session, id).await,
            SpotifyAudioType::NonPlayable => Err(MercuryError),
        }
    }
}

#[async_trait]
trait AudioFiles {
    async fn get_audio_item(session: &Session, id: SpotifyId) -> Result<AudioItem, MercuryError>;
}

#[async_trait]
impl AudioFiles for Track {
    async fn get_audio_item(session: &Session, id: SpotifyId) -> Result<AudioItem, MercuryError> {
        match id.to_base62() {
            Err(e) => {
                warn!("Invalid Track SpotifyId: {}", e);
                Err(MercuryError)
            }
            Ok(uri) => {
                let item = Self::get(session, id).await?;
                Ok(AudioItem {
                    id,
                    uri: format!("spotify:track:{}", uri),
                    files: item.files,
                    name: item.name,
                    duration: item.duration,
                    available: item.available,
                    alternatives: Some(item.alternatives),
                })
            }
        }
    }
}

#[async_trait]
impl AudioFiles for Episode {
    async fn get_audio_item(session: &Session, id: SpotifyId) -> Result<AudioItem, MercuryError> {
        match id.to_base62() {
            Err(e) => {
                warn!("Invalid Episode SpotifyId: {}", e);
                Err(MercuryError)
            }
            Ok(uri) => {
                let item = Self::get(session, id).await?;
                Ok(AudioItem {
                    id,
                    uri: format!("spotify:episode:{}", uri),
                    files: item.files,
                    name: item.name,
                    duration: item.duration,
                    available: item.available,
                    alternatives: None,
                })
            }
        }
    }
}

#[async_trait]
pub trait Metadata: Send + Sized + 'static {
    type Message: protobuf::Message;

    fn request_url(id: SpotifyId) -> Result<String, FromUtf8Error>;
    fn parse(msg: &Self::Message, session: &Session) -> Result<Self, SpotifyIdError>;

    async fn get(session: &Session, id: SpotifyId) -> Result<Self, MercuryError> {
        match Self::request_url(id) {
            Err(e) => {
                warn!("Invalid SpotifyId: {}", e);
                Err(MercuryError)
            }
            Ok(uri) => {
                let response = session.mercury().get(uri).await?;
                match response.payload.first() {
                    None => {
                        warn!("Empty payload");
                        Err(MercuryError)
                    }
                    Some(data) => match Self::Message::parse_from_bytes(data) {
                        Err(e) => {
                            warn!("Error parsing message from bytes: {}", e);
                            Err(MercuryError)
                        }
                        Ok(msg) => match Self::parse(&msg, session) {
                            Err(e) => {
                                warn!("Error parsing message: {:?}", e);
                                Err(MercuryError)
                            }
                            Ok(parsed_msg) => Ok(parsed_msg),
                        },
                    },
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Track {
    pub id: SpotifyId,
    pub name: String,
    pub duration: i32,
    pub album: SpotifyId,
    pub artists: Vec<SpotifyId>,
    pub files: HashMap<FileFormat, FileId>,
    pub alternatives: Vec<SpotifyId>,
    pub available: bool,
}

#[derive(Debug, Clone)]
pub struct Album {
    pub id: SpotifyId,
    pub name: String,
    pub artists: Vec<SpotifyId>,
    pub tracks: Vec<SpotifyId>,
    pub covers: Vec<FileId>,
}

#[derive(Debug, Clone)]
pub struct Episode {
    pub id: SpotifyId,
    pub name: String,
    pub external_url: String,
    pub duration: i32,
    pub language: String,
    pub show: SpotifyId,
    pub files: HashMap<FileFormat, FileId>,
    pub covers: Vec<FileId>,
    pub available: bool,
    pub explicit: bool,
}

#[derive(Debug, Clone)]
pub struct Show {
    pub id: SpotifyId,
    pub name: String,
    pub publisher: String,
    pub episodes: Vec<SpotifyId>,
    pub covers: Vec<FileId>,
}

#[derive(Debug, Clone)]
pub struct Playlist {
    pub revision: Vec<u8>,
    pub user: String,
    pub name: String,
    pub tracks: Vec<SpotifyId>,
}

#[derive(Debug, Clone)]
pub struct Artist {
    pub id: SpotifyId,
    pub name: String,
    pub top_tracks: Vec<SpotifyId>,
}

impl Metadata for Track {
    type Message = protocol::metadata::Track;

    fn request_url(id: SpotifyId) -> Result<String, FromUtf8Error> {
        let id = id.to_base16()?;
        Ok(format!("hm://metadata/3/track/{}", id))
    }

    fn parse(msg: &Self::Message, session: &Session) -> Result<Self, SpotifyIdError> {
        let country = session.country();

        let artists = msg
            .get_artist()
            .iter()
            .filter_map(|artist| {
                if artist.has_gid() {
                    SpotifyId::from_raw(artist.get_gid()).ok()
                } else {
                    None
                }
            })
            .collect();

        let files = msg
            .get_file()
            .iter()
            .filter_map(|file| {
                if file.has_file_id() {
                    let mut dst = [0u8; 20];
                    dst.clone_from_slice(file.get_file_id());
                    Some((file.get_format(), FileId(dst)))
                } else {
                    None
                }
            })
            .collect();

        Ok(Track {
            id: SpotifyId::from_raw(msg.get_gid())?,
            name: msg.get_name().to_owned(),
            duration: msg.get_duration(),
            album: SpotifyId::from_raw(msg.get_album().get_gid())?,
            artists,
            files,
            alternatives: msg
                .get_alternative()
                .iter()
                .filter_map(|alt| SpotifyId::from_raw(alt.get_gid()).ok())
                .collect(),
            available: parse_restrictions(msg.get_restriction(), &country, "premium"),
        })
    }
}

impl Metadata for Album {
    type Message = protocol::metadata::Album;

    fn request_url(id: SpotifyId) -> Result<String, FromUtf8Error> {
        let id = id.to_base16()?;
        Ok(format!("hm://metadata/3/album/{}", id))
    }

    fn parse(msg: &Self::Message, _: &Session) -> Result<Self, SpotifyIdError> {
        let artists = msg
            .get_artist()
            .iter()
            .filter_map(|artist| {
                if artist.has_gid() {
                    SpotifyId::from_raw(artist.get_gid()).ok()
                } else {
                    None
                }
            })
            .collect();

        let tracks = msg
            .get_disc()
            .iter()
            .flat_map(|disc| disc.get_track())
            .filter_map(|track| {
                if track.has_gid() {
                    SpotifyId::from_raw(track.get_gid()).ok()
                } else {
                    None
                }
            })
            .collect();

        let covers = msg
            .get_cover_group()
            .get_image()
            .iter()
            .filter_map(|image| {
                if image.has_file_id() {
                    let mut dst = [0u8; 20];
                    dst.clone_from_slice(image.get_file_id());
                    Some(FileId(dst))
                } else {
                    None
                }
            })
            .collect();

        Ok(Album {
            id: SpotifyId::from_raw(msg.get_gid())?,
            name: msg.get_name().to_owned(),
            artists,
            tracks,
            covers,
        })
    }
}

impl Metadata for Playlist {
    type Message = protocol::playlist4changes::SelectedListContent;

    fn request_url(id: SpotifyId) -> Result<String, FromUtf8Error> {
        let id = id.to_base62()?;
        Ok(format!("hm://playlist/v2/playlist/{}", id))
    }

    fn parse(msg: &Self::Message, _: &Session) -> Result<Self, SpotifyIdError> {
        let tracks = msg
            .get_contents()
            .get_items()
            .iter()
            .filter_map(|item| {
                let uri_split = item.get_uri().split(':');
                let uri_parts: Vec<&str> = uri_split.collect();
                SpotifyId::from_base62(uri_parts[2]).ok()
            })
            .collect::<Vec<_>>();

        if tracks.len() != msg.get_length() as usize {
            warn!(
                "Got {} tracks, but the playlist should contain {} tracks.",
                tracks.len(),
                msg.get_length()
            );
        }

        Ok(Playlist {
            revision: msg.get_revision().to_vec(),
            name: msg.get_attributes().get_name().to_owned(),
            tracks,
            user: msg.get_owner_username().to_string(),
        })
    }
}

impl Metadata for Artist {
    type Message = protocol::metadata::Artist;

    fn request_url(id: SpotifyId) -> Result<String, FromUtf8Error> {
        let id = id.to_base16()?;
        Ok(format!("hm://metadata/3/artist/{}", id))
    }

    fn parse(msg: &Self::Message, session: &Session) -> Result<Self, SpotifyIdError> {
        let country = session.country();

        let top_tracks: Vec<SpotifyId> = match msg
            .get_top_track()
            .iter()
            .find(|tt| !tt.has_country() || countrylist_contains(tt.get_country(), &country))
        {
            Some(tracks) => tracks
                .get_track()
                .iter()
                .filter_map(|track| {
                    if track.has_gid() {
                        SpotifyId::from_raw(track.get_gid()).ok()
                    } else {
                        None
                    }
                })
                .collect(),
            None => Vec::new(),
        };

        Ok(Artist {
            id: SpotifyId::from_raw(msg.get_gid())?,
            name: msg.get_name().to_owned(),
            top_tracks,
        })
    }
}

// Podcast
impl Metadata for Episode {
    type Message = protocol::metadata::Episode;

    fn request_url(id: SpotifyId) -> Result<String, FromUtf8Error> {
        let id = id.to_base16()?;
        Ok(format!("hm://metadata/3/episode/{}", id))
    }

    fn parse(msg: &Self::Message, session: &Session) -> Result<Self, SpotifyIdError> {
        let country = session.country();

        let files = msg
            .get_file()
            .iter()
            .filter(|file| file.has_file_id())
            .map(|file| {
                let mut dst = [0u8; 20];
                dst.clone_from_slice(file.get_file_id());
                (file.get_format(), FileId(dst))
            })
            .collect();

        let covers = msg
            .get_covers()
            .get_image()
            .iter()
            .filter(|image| image.has_file_id())
            .map(|image| {
                let mut dst = [0u8; 20];
                dst.clone_from_slice(image.get_file_id());
                FileId(dst)
            })
            .collect();

        Ok(Episode {
            id: SpotifyId::from_raw(msg.get_gid()).unwrap(),
            name: msg.get_name().to_owned(),
            external_url: msg.get_external_url().to_owned(),
            duration: msg.get_duration().to_owned(),
            language: msg.get_language().to_owned(),
            show: SpotifyId::from_raw(msg.get_show().get_gid()).unwrap(),
            covers,
            files,
            available: parse_restrictions(msg.get_restriction(), &country, "premium"),
            explicit: msg.get_explicit().to_owned(),
        })
    }
}

impl Metadata for Show {
    type Message = protocol::metadata::Show;

    fn request_url(id: SpotifyId) -> Result<String, FromUtf8Error> {
        let id = id.to_base16()?;
        Ok(format!("hm://metadata/3/show/{}", id))
    }

    fn parse(msg: &Self::Message, _: &Session) -> Result<Self, SpotifyIdError> {
        let episodes = msg
            .get_episode()
            .iter()
            .filter_map(|episode| {
                if episode.has_gid() {
                    SpotifyId::from_raw(episode.get_gid()).ok()
                } else {
                    None
                }
            })
            .collect();

        let covers = msg
            .get_covers()
            .get_image()
            .iter()
            .filter(|image| image.has_file_id())
            .map(|image| {
                let mut dst = [0u8; 20];
                dst.clone_from_slice(image.get_file_id());
                FileId(dst)
            })
            .collect();

        Ok(Show {
            id: SpotifyId::from_raw(msg.get_gid()).unwrap(),
            name: msg.get_name().to_owned(),
            publisher: msg.get_publisher().to_owned(),
            episodes,
            covers,
        })
    }
}

struct StrChunks<'s>(&'s str, usize);

trait StrChunksExt {
    fn chunks(&self, size: usize) -> StrChunks;
}

impl StrChunksExt for str {
    fn chunks(&self, size: usize) -> StrChunks {
        StrChunks(self, size)
    }
}

impl<'s> Iterator for StrChunks<'s> {
    type Item = &'s str;
    fn next(&mut self) -> Option<&'s str> {
        let &mut StrChunks(data, size) = self;
        if data.is_empty() {
            None
        } else {
            let ret = Some(&data[..size]);
            self.0 = &data[size..];
            ret
        }
    }
}
