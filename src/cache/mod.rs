use util::{SpotifyId, FileId, ReadSeek};
use audio_key::AudioKey;
use authentication::Credentials;
use std::io::Read;

pub trait Cache {
    fn get_audio_key(&self, _track: SpotifyId, _file: FileId) -> Option<AudioKey> {
        None
    }
    fn put_audio_key(&self, _track: SpotifyId, _file: FileId, _audio_key: AudioKey) { }

    fn get_credentials(&self) -> Option<Credentials> {
        None
    }
    fn put_credentials(&self, _cred: &Credentials) { }

    fn get_file(&self, _file: FileId) -> Option<Box<ReadSeek>> {
        None
    }
    fn put_file(&self, _file: FileId, _contents: &mut Read) { }
}

pub struct NoCache;
impl Cache for NoCache { }

mod default_cache;
pub use self::default_cache::DefaultCache;
