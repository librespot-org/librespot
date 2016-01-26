use util::SpotifyId;
use session::Session;
use metadata::{MetadataRef, Album, Artist, Track};

#[derive(Debug,Clone)]
pub enum Link {
    Track {
        id: SpotifyId,
        offset: u32,
    },

    Album {
        id: SpotifyId,
    },

    Artist {
        id: SpotifyId,
    },

    /*
    Search,
    Playlist,
    Profile,
    Starred,
    LocalTrack,
    Image,
    */
}

impl Link {
    pub fn from_str(uri: &str) -> Result<Link, ()> {
        let mut parts = uri.split(':');

        if parts.next() != Some("spotify") {
            return Err(())
        }

        match parts.next() {
            Some("track") => parts.next()
                                  .map(SpotifyId::from_base62)
                                  .map(|id| Link::Track {
                                      id: id,
                                      offset: 0,
                                  })
                                  .ok_or(()),
            Some("album") => parts.next()
                                  .map(SpotifyId::from_base62)
                                  .map(|id| Link::Album {
                                      id: id,
                                  })
                                  .ok_or(()),
            Some("artist") => parts.next()
                                   .map(SpotifyId::from_base62)
                                   .map(|id| Link::Artist {
                                       id: id,
                                   })
                                   .ok_or(()),
            _ => Err(())
        }
    }

    pub fn as_track(&self, session: &Session) -> Option<MetadataRef<Track>> {
        match *self {
            Link::Track { id, .. } => Some(session.metadata::<Track>(id)),
            _ => None,
        }
    }

    pub fn as_album(&self, session: &Session) -> Option<MetadataRef<Album>> {
        match *self {
            Link::Album { id, .. } => Some(session.metadata::<Album>(id)),
            _ => None,
        }
    }

    pub fn as_artist(&self, session: &Session) -> Option<MetadataRef<Artist>> {
        match *self {
            Link::Artist { id, .. } => Some(session.metadata::<Artist>(id)),
            _ => None,
        }
    }
}

