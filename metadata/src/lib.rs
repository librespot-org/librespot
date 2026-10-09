#[macro_use]
extern crate log;

use protobuf::Message;
use std::future::Future;

use librespot_core::{Error, Session, SpotifyUri};

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

pub trait Metadata: Send + Sized + 'static {
    type Message: Message + std::fmt::Debug;

    // Request a protobuf
    fn request(
        session: &Session,
        id: &SpotifyUri,
    ) -> impl Future<Output = RequestResult> + Send + Sized;

    // Request a metadata struct
    fn get(
        session: &Session,
        id: &SpotifyUri,
    ) -> impl Future<Output = Result<Self, Error>> + Send + Sized {
        map_request_to_message(Self::request(session, id), id)
    }

    fn parse(msg: &Self::Message, _: &SpotifyUri) -> Result<Self, Error>;
}

async fn map_request_to_message<M, P, F>(response: F, uri: &SpotifyUri) -> Result<M, Error>
where
    P: Message + std::fmt::Debug,
    M: Metadata<Message = P>,
    F: Future<Output = RequestResult> + Send + Sized,
{
    let response = response.await?;
    let msg = P::parse_from_bytes(&response)?;
    trace!("Received metadata: {msg:#?}");
    M::parse(&msg, uri)
}
