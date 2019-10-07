extern crate byteorder;
extern crate futures;
extern crate linear_map;
extern crate protobuf;

extern crate librespot_core;
extern crate librespot_protocol as protocol;

pub mod cover;

use futures::Future;
use linear_map::LinearMap;

use librespot_core::mercury::MercuryError;
use librespot_core::session::Session;
use librespot_core::spotify_id::{FileId, SpotifyAudioType, SpotifyId};

pub use protocol::metadata::AudioFile_Format as FileFormat;

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
    pub files: LinearMap<FileFormat, FileId>,
    pub name: String,
    pub available: bool,
    pub alternatives: Option<Vec<SpotifyId>>,
}

impl AudioItem {
    pub fn get_audio_item(
        session: &Session,
        id: SpotifyId,
    ) -> Box<dyn Future<Item = AudioItem, Error = MercuryError>> {
        match id.audio_type {
            SpotifyAudioType::Track => Track::get_audio_item(session, id),
            SpotifyAudioType::Podcast => Episode::get_audio_item(session, id),
        }
    }
}

trait AudioFiles {
    fn get_audio_item(
        session: &Session,
        id: SpotifyId,
    ) -> Box<dyn Future<Item = AudioItem, Error = MercuryError>>;
}

impl AudioFiles for Track {
    fn get_audio_item(
        session: &Session,
        id: SpotifyId,
    ) -> Box<dyn Future<Item = AudioItem, Error = MercuryError>> {
        Box::new(Self::get(session, id).and_then(move |item| {
            Ok(AudioItem {
                id: id,
                uri: format!("spotify:track:{}", id.to_base62()),
                files: item.files,
                name: item.name,
                available: item.available,
                alternatives: Some(item.alternatives),
            })
        }))
    }
}

impl AudioFiles for Episode {
    fn get_audio_item(
        session: &Session,
        id: SpotifyId,
    ) -> Box<dyn Future<Item = AudioItem, Error = MercuryError>> {
        Box::new(Self::get(session, id).and_then(move |item| {
            Ok(AudioItem {
                id: id,
                uri: format!("spotify:episode:{}", id.to_base62()),
                files: item.files,
                name: item.name,
                available: item.available,
                alternatives: None,
            })
        }))
    }
}
pub trait Metadata: Send + Sized + 'static {
    type Message: protobuf::Message;

    fn base_url() -> &'static str;
    fn parse(msg: &Self::Message, session: &Session) -> Self;

    fn get(session: &Session, id: SpotifyId) -> Box<dyn Future<Item = Self, Error = MercuryError>> {
        let uri = format!("{}/{}", Self::base_url(), id.to_base16());
        let request = session.mercury().get(uri);

        let session = session.clone();
        Box::new(request.and_then(move |response| {
            let data = response.payload.first().expect("Empty payload");
            let msg: Self::Message = protobuf::parse_from_bytes(data).unwrap();

            Ok(Self::parse(&msg, &session))
        }))
    }
}

#[derive(Debug, Clone)]
pub struct Track {
    pub id: SpotifyId,
    pub name: String,
    pub duration: i32,
    pub album: SpotifyId,
    pub artists: Vec<SpotifyId>,
    pub files: LinearMap<FileFormat, FileId>,
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
    pub files: LinearMap<FileFormat, FileId>,
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
pub struct Artist {
    pub id: SpotifyId,
    pub name: String,
    pub top_tracks: Vec<SpotifyId>,
}

impl Metadata for Track {
    type Message = protocol::metadata::Track;

    fn base_url() -> &'static str {
        "hm://metadata/3/track"
    }

    fn parse(msg: &Self::Message, session: &Session) -> Self {
        let country = session.country();

        let artists = msg
            .get_artist()
            .iter()
            .filter(|artist| artist.has_gid())
            .map(|artist| SpotifyId::from_raw(artist.get_gid()).unwrap())
            .collect::<Vec<_>>();

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

        Track {
            id: SpotifyId::from_raw(msg.get_gid()).unwrap(),
            name: msg.get_name().to_owned(),
            duration: msg.get_duration(),
            album: SpotifyId::from_raw(msg.get_album().get_gid()).unwrap(),
            artists: artists,
            files: files,
            alternatives: msg
                .get_alternative()
                .iter()
                .map(|alt| SpotifyId::from_raw(alt.get_gid()).unwrap())
                .collect(),
            available: parse_restrictions(msg.get_restriction(), &country, "premium"),
        }
    }
}

impl Metadata for Album {
    type Message = protocol::metadata::Album;

    fn base_url() -> &'static str {
        "hm://metadata/3/album"
    }

    fn parse(msg: &Self::Message, _: &Session) -> Self {
        let artists = msg
            .get_artist()
            .iter()
            .filter(|artist| artist.has_gid())
            .map(|artist| SpotifyId::from_raw(artist.get_gid()).unwrap())
            .collect::<Vec<_>>();

        let tracks = msg
            .get_disc()
            .iter()
            .flat_map(|disc| disc.get_track())
            .filter(|track| track.has_gid())
            .map(|track| SpotifyId::from_raw(track.get_gid()).unwrap())
            .collect::<Vec<_>>();

        let covers = msg
            .get_cover_group()
            .get_image()
            .iter()
            .filter(|image| image.has_file_id())
            .map(|image| {
                let mut dst = [0u8; 20];
                dst.clone_from_slice(image.get_file_id());
                FileId(dst)
            })
            .collect::<Vec<_>>();

        Album {
            id: SpotifyId::from_raw(msg.get_gid()).unwrap(),
            name: msg.get_name().to_owned(),
            artists: artists,
            tracks: tracks,
            covers: covers,
        }
    }
}

impl Metadata for Artist {
    type Message = protocol::metadata::Artist;

    fn base_url() -> &'static str {
        "hm://metadata/3/artist"
    }

    fn parse(msg: &Self::Message, session: &Session) -> Self {
        let country = session.country();

        let top_tracks: Vec<SpotifyId> = match msg
            .get_top_track()
            .iter()
            .find(|tt| !tt.has_country() || countrylist_contains(tt.get_country(), &country))
        {
            Some(tracks) => tracks
                .get_track()
                .iter()
                .filter(|track| track.has_gid())
                .map(|track| SpotifyId::from_raw(track.get_gid()).unwrap())
                .collect::<Vec<_>>(),
            None => Vec::new(),
        };

        Artist {
            id: SpotifyId::from_raw(msg.get_gid()).unwrap(),
            name: msg.get_name().to_owned(),
            top_tracks: top_tracks,
        }
    }
}

// Podcast
impl Metadata for Episode {
    type Message = protocol::metadata::Episode;

    fn base_url() -> &'static str {
        "hm://metadata/3/episode"
    }

    fn parse(msg: &Self::Message, session: &Session) -> Self {
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
            .collect::<Vec<_>>();

        Episode {
            id: SpotifyId::from_raw(msg.get_gid()).unwrap(),
            name: msg.get_name().to_owned(),
            external_url: msg.get_external_url().to_owned(),
            duration: msg.get_duration().to_owned(),
            language: msg.get_language().to_owned(),
            show: SpotifyId::from_raw(msg.get_show().get_gid()).unwrap(),
            covers: covers,
            files: files,
            available: parse_restrictions(msg.get_restriction(), &country, "premium"),
            explicit: msg.get_explicit().to_owned(),
        }
    }
}

impl Metadata for Show {
    type Message = protocol::metadata::Show;

    fn base_url() -> &'static str {
        "hm://metadata/3/show"
    }

    fn parse(msg: &Self::Message, _: &Session) -> Self {
        let episodes = msg
            .get_episode()
            .iter()
            .filter(|episode| episode.has_gid())
            .map(|episode| SpotifyId::from_raw(episode.get_gid()).unwrap())
            .collect::<Vec<_>>();

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
            .collect::<Vec<_>>();

        Show {
            id: SpotifyId::from_raw(msg.get_gid()).unwrap(),
            name: msg.get_name().to_owned(),
            publisher: msg.get_publisher().to_owned(),
            episodes: episodes,
            covers: covers,
        }
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
