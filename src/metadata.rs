use eventual::{Async, Future};
use protobuf;

use protocol;
use mercury::{MercuryRequest, MercuryMethod};
use util::{SpotifyId, FileId, StrChunksExt};
use session::Session;

pub use protocol::metadata::AudioFile_Format as FileFormat;

fn countrylist_contains(list: &str, country: &str) -> bool {
    list.chunks(2).any(|cc| cc == country)
}

fn parse_restrictions<'s, I>(restrictions: I, country: &str, catalogue: &str) -> bool
    where I: IntoIterator<Item = &'s protocol::metadata::Restriction>
{
    restrictions.into_iter()
                .filter(|r| r.get_catalogue_str().contains(&catalogue.to_owned()))
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

#[derive(Debug, Clone)]
pub struct Track {
    pub id: SpotifyId,
    pub name: String,
    pub album: SpotifyId,
    pub artists: Vec<SpotifyId>,
    pub files: Vec<(FileId, FileFormat)>,
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
pub struct Artist {
    pub id: SpotifyId,
    pub name: String,
    pub top_tracks: Vec<SpotifyId>,
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
        let country = session.country();

        let artists = msg.get_artist()
                         .iter()
                         .filter(|artist| artist.has_gid())
                         .map(|artist| SpotifyId::from_raw(artist.get_gid()))
                         .collect::<Vec<_>>();

        let files = msg.get_file()
                       .iter()
                       .filter(|file| file.has_file_id())
                       .map(|file| {
                           let mut dst = [0u8; 20];
                           dst.clone_from_slice(&file.get_file_id());
                           (FileId(dst), file.get_format())
                       })
                       .collect();

        Track {
            id: SpotifyId::from_raw(msg.get_gid()),
            name: msg.get_name().to_owned(),
            album: SpotifyId::from_raw(msg.get_album().get_gid()),
            artists: artists,
            files: files,
            alternatives: msg.get_alternative()
                             .iter()
                             .map(|alt| SpotifyId::from_raw(alt.get_gid()))
                             .collect(),
            available: parse_restrictions(msg.get_restriction(),
                                          &country,
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
        let artists = msg.get_artist()
                         .iter()
                         .filter(|artist| artist.has_gid())
                         .map(|artist| SpotifyId::from_raw(artist.get_gid()))
                         .collect::<Vec<_>>();

        let tracks = msg.get_disc()
                        .iter()
                        .flat_map(|disc| disc.get_track())
                        .filter(|track| track.has_gid())
                        .map(|track| SpotifyId::from_raw(track.get_gid()))
                        .collect::<Vec<_>>();

        let covers = msg.get_cover_group()
                        .get_image()
                        .iter()
                        .filter(|image| image.has_file_id())
                        .map(|image| {
                            let mut dst = [0u8; 20];
                            dst.clone_from_slice(&image.get_file_id());
                            FileId(dst)
                        })
                        .collect::<Vec<_>>();

        Album {
            id: SpotifyId::from_raw(msg.get_gid()),
            name: msg.get_name().to_owned(),
            artists: artists,
            tracks: tracks,
            covers: covers,
        }
    }
}


impl MetadataTrait for Artist {
    type Message = protocol::metadata::Artist;

    fn base_url() -> &'static str {
        "hm://metadata/3/artist"
    }

    fn parse(msg: &Self::Message, session: &Session) -> Self {
        let country = session.country();

        let top_tracks = msg.get_top_track()
                            .iter()
                            .filter(|tt| !tt.has_country() ||
                                         countrylist_contains(tt.get_country(), &country))
                            .next()
                            .unwrap()
                            .get_track()
                            .iter()
                            .filter(|track| track.has_gid())
                            .map(|track| SpotifyId::from_raw(track.get_gid()))
                            .collect::<Vec<_>>();

        Artist {
            id: SpotifyId::from_raw(msg.get_gid()),
            name: msg.get_name().to_owned(),
            top_tracks: top_tracks
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
                   let data = response.payload.first().expect("Empty payload");
                   let msg: T::Message = protobuf::parse_from_bytes(data).unwrap();

                   Ok(T::parse(&msg, &session))
               })
    }
}
