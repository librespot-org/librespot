#[macro_use]
extern crate log;

#[macro_use]
extern crate async_trait;

use protobuf::Message;

use librespot_core::{Error, Session, SpotifyId};

pub mod album;
pub mod artist;
pub mod audio;
pub mod availability;
pub mod content_rating;
pub mod copyright;
pub mod episode;
pub mod error;
pub mod external_id;
pub mod image;
pub mod lyrics;
pub mod playlist;
mod request;
pub mod restriction;
pub mod sale_period;
pub mod show;
pub mod track;
mod util;
pub mod video;

pub use error::MetadataError;
use request::RequestResult;

pub use album::Album;
pub use artist::Artist;
pub use episode::Episode;
pub use lyrics::Lyrics;
pub use playlist::Playlist;
pub use show::Show;
pub use track::Track;

#[async_trait]
pub trait Metadata: Send + Sized + 'static {
    type Message: protobuf::Message + std::fmt::Debug;

    // Request a protobuf
    async fn request(session: &Session, id: &SpotifyId) -> RequestResult;

    // Request a metadata struct
    async fn get(session: &Session, id: &SpotifyId) -> Result<Self, Error> {
        let response = Self::request(session, id).await?;
        let msg = Self::Message::parse_from_bytes(&response)?;
        trace!("Received metadata: {:#?}", msg);
        Self::parse(&msg, id)
    }

    fn parse(msg: &Self::Message, _: &SpotifyId) -> Result<Self, Error>;
}
