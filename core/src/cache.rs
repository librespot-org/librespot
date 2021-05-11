use std::cmp::Reverse;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Error, ErrorKind, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

use priority_queue::PriorityQueue;

use crate::authentication::Credentials;
use crate::spotify_id::FileId;

/// Some kind of data structure that holds some paths, the size of these files and a timestamp.
/// It keeps track of the file sizes and is able to pop the path with the oldest timestamp if
/// a given limit is exceeded.
struct SizeLimiter {
    queue: PriorityQueue<PathBuf, Reverse<SystemTime>>,
    sizes: HashMap<PathBuf, u64>,
    size_limit: u64,
    in_use: u64,
}

impl SizeLimiter {
    /// Creates a new instance with the given size limit.
    fn new(limit: u64) -> Self {
        Self {
            queue: PriorityQueue::new(),
            sizes: HashMap::new(),
            size_limit: limit,
            in_use: 0,
        }
    }

    /// Adds an entry to this data structure.
    ///
    /// If this file is already contained, it will be updated accordingly.
    fn add(&mut self, file: &Path, size: u64, accessed: SystemTime) {
        self.in_use += size;
        self.queue.push(file.to_owned(), Reverse(accessed));
        if let Some(old_size) = self.sizes.insert(file.to_owned(), size) {
            // It's important that decreasing happens after
            // increasing the size, to prevent an overflow.
            self.in_use -= old_size;
        }
    }

    /// Returns true if the limit is exceeded.
    fn exceeds_limit(&self) -> bool {
        self.in_use > self.size_limit
    }

    /// Returns the least recently accessed file if the size of the cache exceeds
    /// the limit.
    ///
    /// The entry is removed from the data structure, but the caller is responsible
    /// to delete the file in the file system.
    fn pop(&mut self) -> Option<PathBuf> {
        if self.exceeds_limit() {
            let (next, _) = self
                .queue
                .pop()
                .expect("in_use was > 0, so the queue should have contained an item.");
            let size = self
                .sizes
                .remove(&next)
                .expect("`queue` and `sizes` should have the same keys.");
            self.in_use -= size;
            Some(next)
        } else {
            None
        }
    }

    /// Updates the timestamp of an existing element. Returns `true` if the item did exist.
    fn update(&mut self, file: &Path, access_time: SystemTime) -> bool {
        self.queue
            .change_priority(file, Reverse(access_time))
            .is_some()
    }

    /// Removes an element with the specified path. Returns `true` if the item did exist.
    fn remove(&mut self, file: &Path) -> bool {
        if self.queue.remove(file).is_none() {
            return false;
        }

        let size = self
            .sizes
            .remove(file)
            .expect("`queue` and `sizes` should have the same keys.");
        self.in_use -= size;

        true
    }
}

struct FsSizeLimiter {
    limiter: Mutex<SizeLimiter>,
}

impl FsSizeLimiter {
    /// Returns access time and file size of a given path.
    fn get_metadata(file: &Path) -> io::Result<(SystemTime, u64)> {
        let metadata = file.metadata()?;

        // The first of the following timestamps which is available will be chosen as access time:
        // 1. Access time
        // 2. Modification time
        // 3. Creation time
        // 4. Current time
        let access_time = metadata
            .accessed()
            .or_else(|_| metadata.modified())
            .or_else(|_| metadata.created())
            .unwrap_or_else(|_| SystemTime::now());

        let size = metadata.len();

        Ok((access_time, size))
    }

    /// Recursively search a directory for files and add them to the `limiter` struct.
    fn init_dir(limiter: &mut SizeLimiter, path: &Path) {
        let list_dir = match fs::read_dir(path) {
            Ok(list_dir) => list_dir,
            Err(e) => {
                warn!("Could not read directory {:?} in cache dir: {}", path, e);
                return;
            }
        };

        for entry in list_dir {
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => {
                    warn!("Could not directory {:?} in cache dir: {}", path, e);
                    return;
                }
            };

            match entry.file_type() {
                Ok(file_type) if file_type.is_dir() || file_type.is_symlink() => {
                    Self::init_dir(limiter, &entry.path())
                }
                Ok(file_type) if file_type.is_file() => {
                    let path = entry.path();
                    match Self::get_metadata(&path) {
                        Ok((access_time, size)) => {
                            limiter.add(&path, size, access_time);
                        }
                        Err(e) => {
                            warn!("Could not read file {:?} in cache dir: {}", path, e)
                        }
                    }
                }
                Ok(ft) => {
                    warn!(
                        "File {:?} in cache dir has unsupported type {:?}",
                        entry.path(),
                        ft
                    )
                }
                Err(e) => {
                    warn!(
                        "Could not get type of file {:?} in cache dir: {}",
                        entry.path(),
                        e
                    )
                }
            };
        }
    }

    fn add(&self, file: &Path, size: u64) {
        self.limiter
            .lock()
            .unwrap()
            .add(file, size, SystemTime::now());
    }

    fn touch(&self, file: &Path) -> bool {
        self.limiter.lock().unwrap().update(file, SystemTime::now())
    }

    fn remove(&self, file: &Path) {
        self.limiter.lock().unwrap().remove(file);
    }

    fn prune_internal<F: FnMut() -> Option<PathBuf>>(mut pop: F) {
        let mut first = true;
        let mut count = 0;

        while let Some(file) = pop() {
            if first {
                debug!("Cache dir exceeds limit, removing least recently used files.");
                first = false;
            }

            if let Err(e) = fs::remove_file(&file) {
                warn!("Could not remove file {:?} from cache dir: {}", file, e);
            } else {
                count += 1;
            }
        }

        if count > 0 {
            info!("Removed {} cache files.", count);
        }
    }

    fn prune(&self) {
        Self::prune_internal(|| self.limiter.lock().unwrap().pop())
    }

    fn new(path: &Path, limit: u64) -> Self {
        let mut limiter = SizeLimiter::new(limit);

        Self::init_dir(&mut limiter, path);
        Self::prune_internal(|| limiter.pop());

        Self {
            limiter: Mutex::new(limiter),
        }
    }
}

/// A cache for volume, credentials and audio files.
#[derive(Clone)]
pub struct Cache {
    credentials_location: Option<PathBuf>,
    volume_location: Option<PathBuf>,
    audio_location: Option<PathBuf>,
    size_limiter: Option<Arc<FsSizeLimiter>>,
}

pub struct RemoveFileError(());

impl Cache {
    pub fn new<P: AsRef<Path>>(
        system_location: Option<P>,
        audio_location: Option<P>,
        size_limit: Option<u64>,
    ) -> io::Result<Self> {
        if let Some(location) = &system_location {
            fs::create_dir_all(location)?;
        }

        let mut size_limiter = None;

        if let Some(location) = &audio_location {
            fs::create_dir_all(location)?;
            if let Some(limit) = size_limit {
                let limiter = FsSizeLimiter::new(location.as_ref(), limit);
                size_limiter = Some(Arc::new(limiter));
            }
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
            size_limiter,
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
        let path = self.file_path(file)?;
        match File::open(&path) {
            Ok(file) => {
                if let Some(limiter) = self.size_limiter.as_deref() {
                    limiter.touch(&path);
                }
                Some(file)
            }
            Err(e) => {
                if e.kind() != ErrorKind::NotFound {
                    warn!("Error reading file from cache: {}", e)
                }
                None
            }
        }
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

        if let Ok(size) = result {
            if let Some(limiter) = self.size_limiter.as_deref() {
                limiter.add(&path, size);
                limiter.prune();
            }
        }
    }

    pub fn remove_file(&self, file: FileId) -> Result<(), RemoveFileError> {
        let path = self.file_path(file).ok_or(RemoveFileError(()))?;

        if let Err(err) = fs::remove_file(&path) {
            warn!("Unable to remove file from cache: {}", err);
            Err(RemoveFileError(()))
        } else {
            if let Some(limiter) = self.size_limiter.as_deref() {
                limiter.remove(&path);
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::time::Duration;

    fn ordered_time(v: u64) -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::from_secs(v)
    }

    #[test]
    fn test_size_limiter() {
        let mut limiter = SizeLimiter::new(1000);

        limiter.add(Path::new("a"), 500, ordered_time(2));
        limiter.add(Path::new("b"), 500, ordered_time(1));

        // b (500) -> a (500)  => sum: 1000 <= 1000
        assert!(!limiter.exceeds_limit());
        assert_eq!(limiter.pop(), None);

        limiter.add(Path::new("c"), 1000, ordered_time(3));

        // b (500) -> a (500) -> c (1000)  => sum: 2000 > 1000
        assert!(limiter.exceeds_limit());
        assert_eq!(limiter.pop().as_deref(), Some(Path::new("b")));
        // a (500) -> c (1000)  => sum: 1500 > 1000
        assert_eq!(limiter.pop().as_deref(), Some(Path::new("a")));
        // c (1000)   => sum: 1000 <= 1000
        assert_eq!(limiter.pop().as_deref(), None);

        limiter.add(Path::new("d"), 5, ordered_time(2));
        // d (5) -> c (1000) => sum: 1005 > 1000
        assert_eq!(limiter.pop().as_deref(), Some(Path::new("d")));
        // c (1000)   => sum: 1000 <= 1000
        assert_eq!(limiter.pop().as_deref(), None);

        // Test updating

        limiter.add(Path::new("e"), 500, ordered_time(3));
        //  c (1000) -> e (500)  => sum: 1500 > 1000
        assert!(limiter.update(Path::new("c"), ordered_time(4)));
        // e (500) -> c (1000)  => sum: 1500 > 1000
        assert_eq!(limiter.pop().as_deref(), Some(Path::new("e")));
        // c (1000)  => sum: 1000 <= 1000

        // Test removing
        limiter.add(Path::new("f"), 500, ordered_time(2));
        assert!(limiter.remove(Path::new("c")));
        assert!(!limiter.exceeds_limit());
    }
}
