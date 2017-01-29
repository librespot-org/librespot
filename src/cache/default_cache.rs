use std::path::PathBuf;
use std::io::Read;
use std::fs::File;

use util::{SpotifyId, FileId, ReadSeek, mkdir_existing};
use authentication::Credentials;
use audio_key::AudioKey;

use super::Cache;

pub struct DefaultCache {
    environment: lmdb::Environment,
    root: PathBuf,
}

impl DefaultCache {
    pub fn new(location: PathBuf) -> Result<DefaultCache, ()> {
        let env = lmdb::EnvBuilder::new().max_dbs(5).open(&location.join("db"), 0o755).unwrap();

        mkdir_existing(&location).unwrap();
        mkdir_existing(&location.join("files")).unwrap();

        Ok(DefaultCache {
            environment: env,
            root: location
        })
    }

    fn audio_keys(&self) -> MdbResult<lmdb::DbHandle> {
        self.environment.create_db("audio-keys", lmdb::DbFlags::empty())
    }

    fn file_path(&self, file: FileId) -> PathBuf {
        let name = file.to_base16();
        self.root.join("files").join(&name[0..2]).join(&name[2..])
    }

    fn credentials_path(&self) -> PathBuf {
        self.root.join("credentials.json")
    }
}

impl Cache for DefaultCache {
    fn get_audio_key(&self, track: SpotifyId, file: FileId) -> Option<AudioKey> {
        let reader = self.environment.get_reader().unwrap();
        let handle = self.audio_keys().unwrap();
        let db = reader.bind(&handle);

        let mut key = Vec::new();
        key.extend_from_slice(&track.to_raw());
        key.extend_from_slice(&file.0);

        let value : Option<Vec<_>> = db.get(&key).ok();
        value.and_then(|value| if value.len() == 16 {
            let mut result = [0u8; 16];
            result.clone_from_slice(&value);
            Some(AudioKey(result))
        } else {
            None
        })
    }

    fn put_audio_key(&self, track: SpotifyId, file: FileId, audio_key: AudioKey) {
        let xact = self.environment.new_transaction().unwrap();
        let handle = self.audio_keys().unwrap();

        {
            let db = xact.bind(&handle);

            let mut key = Vec::new();
            key.extend_from_slice(&track.to_raw());
            key.extend_from_slice(&file.0);

            db.set(&key, &audio_key.0.as_ref()).unwrap();
        }

        xact.commit().unwrap();
    }

    fn get_credentials(&self) -> Option<Credentials> {
        let path = self.credentials_path();
        Credentials::from_file(path)
    }
    fn put_credentials(&self, cred: &Credentials) {
        let path = self.credentials_path();
        cred.save_to_file(&path);
    }

    fn get_file(&self, file: FileId) -> Option<Box<ReadSeek>> {
        File::open(self.file_path(file)).ok().map(|f| Box::new(f) as Box<ReadSeek>)
    }

    fn put_file(&self, file: FileId, contents: &mut Read) {
        let path = self.file_path(file);

        mkdir_existing(path.parent().unwrap()).unwrap();

        let mut cache_file = File::create(path).unwrap();
        ::std::io::copy(contents, &mut cache_file).unwrap();
    }
}
