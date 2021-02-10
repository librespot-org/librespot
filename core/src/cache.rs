use std::fs;
use std::fs::File;
use std::io::{self, Error, ErrorKind, Read, Write};
use std::path::{Path, PathBuf};

use crate::authentication::Credentials;
use crate::spotify_id::FileId;

/// A cache for volume, credentials and audio files.
#[derive(Clone)]
pub struct Cache {
    credentials_location: Option<PathBuf>,
    volume_location: Option<PathBuf>,
    audio_location: Option<PathBuf>,
}

impl Cache {
    pub fn new<P: AsRef<Path>>(
        system_location: Option<P>,
        audio_location: Option<P>,
    ) -> io::Result<Self> {
        if let Some(location) = &system_location {
            fs::create_dir_all(location)?;
        }

        if let Some(location) = &audio_location {
            fs::create_dir_all(location)?;
        }

        let audio_location = audio_location.map(|p| p.as_ref().to_owned());
        let volume_location = system_location.as_ref().map(|p| p.as_ref().join("volume"));
        let credentials_location = system_location
            .as_ref()
            .map(|p| p.as_ref().join("credentials.json"));

        let cache = Cache {
            credentials_location,
            volume_location,
            audio_location,
        };

        Ok(cache)
    }

    pub fn credentials(&self) -> Option<Credentials> {
        let location = self.credentials_location.as_ref()?;

        // This closure is just convencience to enable the question mark operator
        let read = || {
            let mut file = File::open(location)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            serde_json::from_str(&contents).map_err(|e| Error::new(ErrorKind::InvalidData, e))
        };

        match read() {
            Ok(c) => Some(c),
            Err(e) => {
                // If the file did not exist, the file was probably not written
                // before. Otherwise, log the error.
                if e.kind() != ErrorKind::NotFound {
                    warn!("Error reading credentials from cache: {}", e);
                }
                None
            }
        }
    }

    pub fn save_credentials(&self, cred: &Credentials) {
        if let Some(location) = &self.credentials_location {
            let result = File::create(location).and_then(|mut file| {
                let data = serde_json::to_string(cred)?;
                write!(file, "{}", data)
            });

            if let Err(e) = result {
                warn!("Cannot save credentials to cache: {}", e)
            }
        }
    }

    pub fn volume(&self) -> Option<u16> {
        let location = self.volume_location.as_ref()?;

        let read = || {
            let mut file = File::open(location)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            contents
                .parse()
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))
        };

        match read() {
            Ok(v) => Some(v),
            Err(e) => {
                if e.kind() != ErrorKind::NotFound {
                    warn!("Error reading volume from cache: {}", e);
                }
                None
            }
        }
    }

    pub fn save_volume(&self, volume: u16) {
        if let Some(ref location) = self.volume_location {
            let result = File::create(location).and_then(|mut file| write!(file, "{}", volume));
            if let Err(e) = result {
                warn!("Cannot save volume to cache: {}", e);
            }
        }
    }

    fn file_path(&self, file: FileId) -> Option<PathBuf> {
        self.audio_location.as_ref().map(|location| {
            let name = file.to_base16();
            let mut path = location.join(&name[0..2]);
            path.push(&name[2..]);
            path
        })
    }

    pub fn file(&self, file: FileId) -> Option<File> {
        File::open(self.file_path(file)?)
            .map_err(|e| {
                if e.kind() != ErrorKind::NotFound {
                    warn!("Error reading file from cache: {}", e)
                }
            })
            .ok()
    }

    pub fn save_file<F: Read>(&self, file: FileId, contents: &mut F) {
        let path = if let Some(path) = self.file_path(file) {
            path
        } else {
            return;
        };
        let parent = path.parent().unwrap();

        let result = fs::create_dir_all(parent)
            .and_then(|_| File::create(&path))
            .and_then(|mut file| io::copy(contents, &mut file));

        if let Err(e) = result {
            if e.kind() == ErrorKind::Other {
                // Perhaps there's no space left in the cache
                // TODO: try to narrow down the error (platform-dependently)
                info!("An error occured while writing to cache, trying to flush the cache");

                if fs::remove_dir_all(self.audio_location.as_ref().unwrap())
                    .and_then(|_| fs::create_dir_all(parent))
                    .and_then(|_| File::create(&path))
                    .and_then(|mut file| io::copy(contents, &mut file))
                    .is_ok()
                {
                    // It worked, there's no need to print a warning
                    return;
                }
            }

            warn!("Cannot save file to cache: {}", e)
        }
    }

    pub fn remove_file(&self, file: FileId) -> bool {
        if let Some(path) = self.file_path(file) {
            if let Err(err) = fs::remove_file(path) {
                warn!("Unable to remove file from cache: {}", err);
                false
            } else {
                true
            }
        } else {
            false
        }
    }
}
