use std::fs;
use std::fs::File;
use std::io::{self, Error, ErrorKind, Read, Write};
use std::path::{Path, PathBuf};

use crate::authentication::Credentials;
use crate::spotify_id::FileId;
use crate::volume::Volume;

#[derive(Clone)]
pub struct Cache {
    audio_root: PathBuf,
    system_root: PathBuf,
    use_audio_cache: bool,
}

fn mkdir_existing(path: &Path) -> io::Result<()> {
    fs::create_dir(path).or_else(|err| {
        if err.kind() == io::ErrorKind::AlreadyExists {
            Ok(())
        } else {
            Err(err)
        }
    })
}

impl Cache {
    pub fn new(
        audio_location: PathBuf,
        system_location: PathBuf,
        use_audio_cache: bool,
    ) -> io::Result<Cache> {
        if use_audio_cache {
            mkdir_existing(&audio_location)?;
            mkdir_existing(&audio_location.join("files"))?;
        }
        mkdir_existing(&system_location)?;

        Ok(Cache {
            audio_root: audio_location,
            system_root: system_location,
            use_audio_cache,
        })
    }
}

impl Cache {
    fn open_credentials_file(&self) -> io::Result<File> {
        File::open(self.system_root.join("credentials.json"))
    }

    fn read_credentials(&self) -> io::Result<Credentials> {
        let mut file = self.open_credentials_file()?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        serde_json::from_str(&contents).map_err(|e| Error::new(ErrorKind::InvalidData, e))
    }

    pub fn credentials(&self) -> Option<Credentials> {
        match self.read_credentials() {
            Ok(c) => Some(c),
            Err(e) => {
                if e.kind() != ErrorKind::NotFound {
                    warn!("Error reading credentials from cache: {}", e);
                }
                None
            }
        }
    }

    pub fn save_credentials(&self, cred: &Credentials) {
        let result = self
            .open_credentials_file()
            .and_then(|mut file| write!(file, "{}", serde_json::to_string(cred)?));
        if let Err(e) = result {
            warn!("Cannot save credentials to cache: {}", e);
        }
    }
}

// cache volume to system_root/volume
impl Cache {
    fn open_volume_file(&self) -> io::Result<File> {
        File::open(self.system_root.join("volume"))
    }

    fn read_volume(&self) -> io::Result<Volume> {
        let mut file = self.open_volume_file()?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        contents
            .parse()
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))
    }

    pub fn volume(&self) -> Option<Volume> {
        match self.read_volume() {
            Ok(v) => Some(v),
            Err(e) => {
                if e.kind() != ErrorKind::NotFound {
                    warn!("Error reading volume from cache: {}", e);
                }
                None
            }
        }
    }

    pub fn save_volume(&self, volume: Volume) {
        let result = self
            .open_volume_file()
            .and_then(|mut file| write!(file, "{}", volume));
        if let Err(e) = result {
            warn!("Cannot save volume to cache: {}", e);
        }
    }
}

impl Cache {
    fn file_path(&self, file: FileId) -> PathBuf {
        let name = file.to_base16();
        self.audio_root
            .join("files")
            .join(&name[0..2])
            .join(&name[2..])
    }

    pub fn file(&self, file: FileId) -> Option<File> {
        File::open(self.file_path(file))
            .map_err(|e| {
                if e.kind() != ErrorKind::NotFound {
                    warn!("Error reading file from cache: {}", e)
                }
            })
            .ok()
    }

    pub fn save_file<F: Read>(&self, file: FileId, contents: &mut F) -> io::Result<()> {
        if self.use_audio_cache {
            let path = self.file_path(file);
            mkdir_existing(path.parent().unwrap())?;

            let mut cache_file = File::create(path).or_else(|_| {
                fs::remove_dir_all(&self.audio_root.join("files"))?;
                mkdir_existing(&self.audio_root.join("files"))?;

                let path = self.file_path(file);
                mkdir_existing(path.parent().unwrap())?;
                File::create(path)
            })?;

            io::copy(contents, &mut cache_file).or_else(|_| {
                fs::remove_dir_all(&self.audio_root.join("files"))?;
                mkdir_existing(&self.audio_root.join("files"))?;

                let path = self.file_path(file);
                mkdir_existing(path.parent().unwrap())?;
                let mut file = File::create(path)?;
                io::copy(contents, &mut file)
            })?;
        }
        Ok(())
    }
}
