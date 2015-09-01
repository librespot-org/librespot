use bit_set::BitSet;
use byteorder::{ByteOrder, BigEndian};
use std::cmp::min;
use std::sync::{Arc, Condvar, Mutex};
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::fs;
use std::io::{self, Read, Write, Seek, SeekFrom};
use std::path::PathBuf;
use tempfile::TempFile;

use util::{FileId, IgnoreExt, ZeroFile, mkdir_existing};
use session::Session;
use stream::StreamEvent;

const CHUNK_SIZE : usize = 0x20000;

pub enum AudioFile {
    Direct(fs::File),
    Loading(AudioFileLoading)
}

pub struct AudioFileLoading {
    read_file: TempFile,

    position: u64,
    seek: mpsc::Sender<u64>,

    shared: Arc<AudioFileShared>,
}

struct AudioFileShared {
    file_id: FileId,
    size: usize,
    chunk_count: usize,
    cond: Condvar,
    bitmap: Mutex<BitSet>,
}

impl AudioFileLoading {
    fn new(session: &Session, file_id: FileId) -> AudioFileLoading {
        let mut files_iter = TempFile::shared(2).unwrap().into_iter();
        let read_file = files_iter.next().unwrap();
        let mut write_file = files_iter.next().unwrap();

        let size = session.stream(file_id, 0, 1).into_iter()
            .filter_map(|event| {
                match event {
                    StreamEvent::Header(id, ref data) if id == 0x3 => {
                        Some(BigEndian::read_u32(data) as usize * 4)
                    }
                    _ => None
                }
            }).next().unwrap();

        let chunk_count = (size + CHUNK_SIZE / 2) / CHUNK_SIZE;

        let shared = Arc::new(AudioFileShared {
            file_id: file_id,
            size: size,
            chunk_count: chunk_count,
            cond: Condvar::new(),
            bitmap: Mutex::new(BitSet::with_capacity(chunk_count)),
        });

        io::copy(&mut ZeroFile::new(size as u64), &mut write_file).unwrap();

        let (seek_tx, seek_rx) = mpsc::channel();

        let _shared = shared.clone();
        let _session = session.clone();

        thread::spawn(move || {
            AudioFileLoading::fetch(&_session, _shared, write_file, seek_rx)
        });

        AudioFileLoading {
            read_file: read_file,

            position: 0,
            seek: seek_tx,

            shared: shared
        }
    }

    fn fetch(session: &Session, shared: Arc<AudioFileShared>,
             mut write_file: TempFile, seek_rx: mpsc::Receiver<u64>) {
        let mut index = 0;

        loop {
            match seek_rx.try_recv() {
                Ok(position) => {
                    index = position as usize / CHUNK_SIZE;
                }
                Err(TryRecvError::Disconnected) => break,
                Err(TryRecvError::Empty) => (),
            }

            let bitmap = shared.bitmap.lock().unwrap();
            if bitmap.len() >= shared.chunk_count {
                drop(bitmap);
                AudioFileLoading::store(session, &shared, &mut write_file);
                break;
            }

            while bitmap.contains(&index) {
                index = (index + 1) % shared.chunk_count;
            }
            drop(bitmap);

            AudioFileLoading::fetch_chunk(session, &shared, &mut write_file, index);
        }
    }

    fn fetch_chunk(session: &Session, shared: &Arc<AudioFileShared>,
                   write_file: &mut TempFile, index: usize) {

        let rx = session.stream(shared.file_id,
                     (index * CHUNK_SIZE / 4) as u32,
                     (CHUNK_SIZE / 4) as u32);

        println!("Chunk {}", index);

        write_file.seek(SeekFrom::Start((index * CHUNK_SIZE) as u64)).unwrap();

        let mut size = 0usize;
        for event in rx.iter() {
            match event {
                StreamEvent::Header(..) => (),
                StreamEvent::Data(data) => {
                    write_file.write_all(&data).unwrap();

                    size += data.len();
                    if size >= CHUNK_SIZE {
                        break
                    }
                }
            }
        }

        let mut bitmap = shared.bitmap.lock().unwrap();
        bitmap.insert(index as usize);

        shared.cond.notify_all();
    }

    fn store(session: &Session, shared: &AudioFileShared, write_file: &mut TempFile) {
        write_file.seek(SeekFrom::Start(0)).unwrap();

        mkdir_existing(&AudioFileManager::cache_dir(session, shared.file_id)).unwrap();

        let mut f = fs::File::create(AudioFileManager::cache_path(session, shared.file_id)).unwrap();
        io::copy(write_file, &mut f).unwrap();
    }
}

impl Read for AudioFileLoading {
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        let index = self.position as usize / CHUNK_SIZE;
        let offset = self.position as usize % CHUNK_SIZE;
        let len = min(output.len(), CHUNK_SIZE-offset);

        let mut bitmap = self.shared.bitmap.lock().unwrap();
        while !bitmap.contains(&index) {
            bitmap = self.shared.cond.wait(bitmap).unwrap();
        }
        drop(bitmap);

        let read_len = try!(self.read_file.read(&mut output[..len]));

        self.position += read_len as u64;

        Ok(read_len)
    }
}

impl Seek for AudioFileLoading {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.position = try!(self.read_file.seek(pos));

        /*
         * Notify the fetch thread to get the correct block
         * This can fail if fetch thread has completed, in which case the
         * block is ready. Just ignore the error.
         */
        self.seek.send(self.position).ignore();
        Ok(self.position as u64)
    }
}

impl Read for AudioFile {
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        match *self {
            AudioFile::Direct(ref mut file) => file.read(output),
            AudioFile::Loading(ref mut loading) => loading.read(output),
        }
    }
}

impl Seek for AudioFile {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        match *self {
            AudioFile::Direct(ref mut file) => file.seek(pos),
            AudioFile::Loading(ref mut loading) => loading.seek(pos),
        }
    }
}

pub struct AudioFileManager;
impl AudioFileManager {
    pub fn new() -> AudioFileManager {
        AudioFileManager
    }

    pub fn cache_dir(session: &Session, file_id: FileId) -> PathBuf {
        let name = file_id.to_base16();
        session.0.config.cache_location.join(&name[0..2])
    }

    pub fn cache_path(session: &Session, file_id: FileId) -> PathBuf {
        let name = file_id.to_base16();
        AudioFileManager::cache_dir(session, file_id).join(&name[2..])
    }

    pub fn request (&mut self, session: &Session, file_id: FileId) -> AudioFile {
        match fs::File::open(AudioFileManager::cache_path(session, file_id)) {
            Ok(f) => AudioFile::Direct(f),
            Err(..) => AudioFile::Loading(AudioFileLoading::new(session, file_id))
        }
    }
}

