use bit_set::BitSet;
use byteorder::{ByteOrder, BigEndian};
use eventual;
use std::cmp::min;
use std::sync::{Arc, Condvar, Mutex};
use std::sync::mpsc::{self, TryRecvError};
use std::fs;
use std::io::{self, Read, Write, Seek, SeekFrom};
use tempfile::NamedTempFile;

use util::{FileId, IgnoreExt};
use session::Session;
use audio_file2;

const CHUNK_SIZE: usize = 0x20000;

pub struct AudioFile {
    read_file: fs::File,

    position: u64,
    seek: mpsc::Sender<u64>,

    shared: Arc<AudioFileShared>,
}

struct AudioFileInternal {
    partial_tx: Option<eventual::Complete<fs::File, ()>>,
    complete_tx: eventual::Complete<NamedTempFile, ()>,
    write_file: NamedTempFile,
    seek_rx: mpsc::Receiver<u64>,
    shared: Arc<AudioFileShared>,
    chunk_count: usize,
}

struct AudioFileShared {
    cond: Condvar,
    bitmap: Mutex<BitSet>,
}

impl AudioFile {
    pub fn new(session: &Session, file_id: FileId)
        -> (eventual::Future<AudioFile, ()>, eventual::Future<NamedTempFile, ()>) {

        let shared = Arc::new(AudioFileShared {
            cond: Condvar::new(),
            bitmap: Mutex::new(BitSet::new()),
        });

        let (seek_tx, seek_rx) = mpsc::channel();
        let (partial_tx, partial_rx) = eventual::Future::pair();
        let (complete_tx, complete_rx) = eventual::Future::pair();

        let internal = AudioFileInternal {
            shared: shared.clone(),
            write_file: NamedTempFile::new().unwrap(),
            seek_rx: seek_rx,
            partial_tx: Some(partial_tx),
            complete_tx: complete_tx,
            chunk_count: 0,
        };

        audio_file2::AudioFile::new(file_id, 0, internal, session);

        let file_rx = partial_rx.map(|read_file| {
            AudioFile {
                read_file: read_file,

                position: 0,
                seek: seek_tx,

                shared: shared,
            }
        });
        
        (file_rx, complete_rx)
    }
}

impl audio_file2::Handler for AudioFileInternal {
    fn on_header(mut self, header_id: u8, header_data: &[u8], _session: &Session) -> audio_file2::Response<Self> {
        if header_id == 0x3 {
            if let Some(tx) = self.partial_tx.take() {
                let size = BigEndian::read_u32(header_data) as usize * 4;
                self.write_file.set_len(size as u64).unwrap();
                let read_file = self.write_file.reopen().unwrap();

                self.chunk_count = (size + CHUNK_SIZE - 1) / CHUNK_SIZE;
                self.shared.bitmap.lock().unwrap().reserve_len(self.chunk_count);

                tx.complete(read_file)
            }
        }

        audio_file2::Response::Continue(self)
    }

    fn on_data(mut self, offset: usize, data: &[u8], _session: &Session) -> audio_file2::Response<Self> {
        self.write_file.seek(SeekFrom::Start(offset as u64)).unwrap();
        self.write_file.write_all(&data).unwrap();

        // We've crossed a chunk boundary
        // Mark the previous one as complete in the bitmap and notify the reader
        let seek = if (offset + data.len()) % CHUNK_SIZE < data.len() {
            let mut index = offset / CHUNK_SIZE;
            let mut bitmap = self.shared.bitmap.lock().unwrap();
            bitmap.insert(index);
            self.shared.cond.notify_all();

            println!("{}/{} {:?}", bitmap.len(), self.chunk_count, *bitmap);

            // If all blocks are complete when can stop
            if bitmap.len() >= self.chunk_count {
                println!("All good");
                drop(bitmap);
                self.write_file.seek(SeekFrom::Start(0)).unwrap();
                self.complete_tx.complete(self.write_file);
                return audio_file2::Response::Close;
            }

            // Find the next undownloaded block
            index = (index + 1) % self.chunk_count;
            while bitmap.contains(index) {
                index = (index + 1) % self.chunk_count;
            }

            Some(index)
        } else {
            None
        };

        match self.seek_rx.try_recv() {
            Ok(seek_offset) => audio_file2::Response::Seek(self, seek_offset as usize / CHUNK_SIZE * CHUNK_SIZE),
            Err(TryRecvError::Disconnected) => audio_file2::Response::Close,
            Err(TryRecvError::Empty) => match seek {
                Some(index) => audio_file2::Response::Seek(self, index * CHUNK_SIZE),
                None => audio_file2::Response::Continue(self),
            },
        }
    }

    fn on_eof(mut self, _session: &Session) -> audio_file2::Response<Self> {
        let index = {
            let mut index = self.chunk_count - 1;
            let mut bitmap = self.shared.bitmap.lock().unwrap();
            bitmap.insert(index);
            self.shared.cond.notify_all();

            println!("{:?}", *bitmap);

            println!("{} {}", bitmap.len(), self.chunk_count);

            // If all blocks are complete when can stop
            if bitmap.len() >= self.chunk_count {
                drop(bitmap);
                self.write_file.seek(SeekFrom::Start(0)).unwrap();
                self.complete_tx.complete(self.write_file);
                return audio_file2::Response::Close;
            }

            // Find the next undownloaded block
            index = (index + 1) % self.chunk_count;
            while bitmap.contains(index) {
                index = (index + 1) % self.chunk_count;
            }
            index
        };

        audio_file2::Response::Seek(self, index * CHUNK_SIZE)
    }

    fn on_error(self, _session: &Session) {
    }
}

impl Read for AudioFile {
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        let index = self.position as usize / CHUNK_SIZE;
        let offset = self.position as usize % CHUNK_SIZE;
        let len = min(output.len(), CHUNK_SIZE - offset);

        let mut bitmap = self.shared.bitmap.lock().unwrap();
        while !bitmap.contains(index) {
            bitmap = self.shared.cond.wait(bitmap).unwrap();
        }
        drop(bitmap);

        let read_len = try!(self.read_file.read(&mut output[..len]));

        self.position += read_len as u64;

        Ok(read_len)
    }
}

impl Seek for AudioFile {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.position = try!(self.read_file.seek(pos));

        // Notify the fetch thread to get the correct block
        // This can fail if fetch thread has completed, in which case the
        // block is ready. Just ignore the error.
        self.seek.send(self.position).ignore();
        Ok(self.position as u64)
    }
}
