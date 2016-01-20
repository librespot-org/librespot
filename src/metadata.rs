use eventual::{Async, Future};
use protobuf;

use librespot_protocol as protocol;
use mercury::{MercuryRequest, MercuryMethod};
use util::{SpotifyId, FileId, StrChunksExt};
use session::Session;

pub use librespot_protocol::metadata::AudioFile_Format as FileFormat;

fn countrylist_contains(list: &str, country: &str) -> bool {
    list.chunks(2).any(|cc| cc == country)
}

fn parse_restrictions<'s, I>(restrictions: I, country: &str, catalogue: &str) -> bool
    where I: Iterator<Item = &'s protocol::metadata::Restriction>
{
    restrictions.filter(|r| r.get_catalogue_str().contains(&catalogue.to_owned()))
                .all(|r| {
                    !countrylist_contains(r.get_countries_forbidden(), country) &&
                    (!r.has_countries_allowed() ||
                     countrylist_contains(r.get_countries_allowed(), country))
                })
}

pub trait MetadataTrait : Send + 'static {
    type Message: protobuf::MessageStatic;

    fn base_url() -> &'static str;
    fn parse(msg: &Self::Message, session: &Session) -> Self;
}

#[derive(Debug)]
pub struct Track {
    pub id: SpotifyId,
    pub name: String,
    pub album: SpotifyId,
    pub files: Vec<(FileId, FileFormat)>,
    pub alternatives: Vec<SpotifyId>,
    pub available: bool,
}

#[derive(Debug)]
pub struct Album {
    pub id: SpotifyId,
    pub name: String,
    pub artists: Vec<SpotifyId>,
    pub covers: Vec<FileId>,
}

#[derive(Debug)]
pub struct Artist {
    pub id: SpotifyId,
    pub name: String,
}

pub type MetadataRef<T> = Future<T, ()>;
pub type TrackRef = MetadataRef<Track>;
pub type AlbumRef = MetadataRef<Album>;
pub type ArtistRef = MetadataRef<Artist>;

impl MetadataTrait for Track {
    type Message = protocol::metadata::Track;

    fn base_url() -> &'static str {
        "hm://metadata/3/track"
    }

    fn parse(msg: &Self::Message, session: &Session) -> Self {
        Track {
            id: SpotifyId::from_raw(msg.get_gid()),
            name: msg.get_name().to_owned(),
            album: SpotifyId::from_raw(msg.get_album().get_gid()),
            files: msg.get_file()
                      .iter()
                      .filter(|file| file.has_file_id())
                      .map(|file| {
                          let mut dst = [0u8; 20];
                          dst.clone_from_slice(&file.get_file_id());
                          (FileId(dst), file.get_format())
                      })
                      .collect(),
            alternatives: msg.get_alternative()
                             .iter()
                             .map(|alt| SpotifyId::from_raw(alt.get_gid()))
                             .collect(),
            available: parse_restrictions(msg.get_restriction().iter(),
                                          &session.0.data.read().unwrap().country,
                                          "premium"),
        }
    }
}

impl MetadataTrait for Album {
    type Message = protocol::metadata::Album;

    fn base_url() -> &'static str {
        "hm://metadata/3/album"
    }

    fn parse(msg: &Self::Message, _: &Session) -> Self {
        Album {
            id: SpotifyId::from_raw(msg.get_gid()),
            name: msg.get_name().to_owned(),
            artists: msg.get_artist()
                        .iter()
                        .map(|a| SpotifyId::from_raw(a.get_gid()))
                        .collect(),
            covers: msg.get_cover_group()
                       .get_image()
                       .iter()
                       .filter(|image| image.has_file_id())
                       .map(|image| {
                           let mut dst = [0u8; 20];
                           dst.clone_from_slice(&image.get_file_id());
                           FileId(dst)
                       })
                       .collect(),
        }
    }
}


impl MetadataTrait for Artist {
    type Message = protocol::metadata::Artist;

    fn base_url() -> &'static str {
        "hm://metadata/3/artist"
    }

    fn parse(msg: &Self::Message, _: &Session) -> Self {
        Artist {
            id: SpotifyId::from_raw(msg.get_gid()),
            name: msg.get_name().to_owned(),
        }
    }
}

pub struct MetadataManager;

impl MetadataManager {
    pub fn new() -> MetadataManager {
        MetadataManager
    }

    pub fn get<T: MetadataTrait>(&mut self, session: &Session, id: SpotifyId) -> MetadataRef<T> {
        let session = session.clone();
        session.mercury(MercuryRequest {
                   method: MercuryMethod::GET,
                   uri: format!("{}/{}", T::base_url(), id.to_base16()),
                   content_type: None,
                   payload: Vec::new(),
               })
               .and_then(move |response| {
                   let data = response.payload.first().unwrap();
                   let msg: T::Message = protobuf::parse_from_bytes(data).unwrap();

                   Ok(T::parse(&msg, &session))
               })
    }
}
