use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

use authentication::Credentials;
use spotify_id::FileId;
use volume::Volume;

#[derive(Clone)]
pub struct Cache {
    root: PathBuf,
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
    pub fn new(location: PathBuf, use_audio_cache: bool) -> Cache {
        mkdir_existing(&location).unwrap();
        mkdir_existing(&location.join("files")).unwrap();

        Cache {
            root: location,
            use_audio_cache: use_audio_cache,
        }
    }
}

impl Cache {
    fn credentials_path(&self) -> PathBuf {
        self.root.join("credentials.json")
    }

    pub fn credentials(&self) -> Option<Credentials> {
        let path = self.credentials_path();
        Credentials::from_file(path)
    }

    pub fn save_credentials(&self, cred: &Credentials) {
        let path = self.credentials_path();
        cred.save_to_file(&path);
    }
}

// cache volume to root/volume
impl Cache {
    fn volume_path(&self) -> PathBuf {
        self.root.join("volume")
    }

    pub fn volume(&self) -> Option<u16> {
        let path = self.volume_path();
        Volume::from_file(path)
    }

    pub fn save_volume(&self, volume: Volume) {
        let path = self.volume_path();
        volume.save_to_file(&path);
    }
}

impl Cache {
    fn file_path(&self, file: FileId) -> PathBuf {
        let name = file.to_base16();
        self.root.join("files").join(&name[0..2]).join(&name[2..])
    }

    pub fn file(&self, file: FileId) -> Option<File> {
        File::open(self.file_path(file)).ok()
    }

    pub fn save_file(&self, file: FileId, contents: &mut Read) {
        if self.use_audio_cache {
            let path = self.file_path(file);

            mkdir_existing(path.parent().unwrap()).unwrap();

            let mut cache_file = File::create(path).unwrap();
            ::std::io::copy(contents, &mut cache_file).unwrap();
        }
    }
}
