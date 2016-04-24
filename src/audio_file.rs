use bit_set::BitSet;
use byteorder::{ByteOrder, BigEndian};
use eventual;
use std::cmp::min;
use std::sync::{Arc, Condvar, Mutex};
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::fs;
use std::io::{self, Read, Write, Seek, SeekFrom};
use tempfile::NamedTempFile;

use util::{FileId, IgnoreExt};
use session::Session;
use stream::StreamEvent;

const CHUNK_SIZE: usize = 0x20000;

pub struct AudioFile {
    read_file: fs::File,

    position: u64,
    seek: mpsc::Sender<u64>,

    shared: Arc<AudioFileShared>,
}

struct AudioFileShared {
    file_id: FileId,
    chunk_count: usize,
    cond: Condvar,
    bitmap: Mutex<BitSet>,
}

impl AudioFile {
    pub fn new(session: &Session, file_id: FileId)
        -> (AudioFile, eventual::Future<NamedTempFile, ()>) {

        let size = session.stream(file_id, 0, 1)
                          .iter()
                          .filter_map(|event| {
                              match event {
                                  StreamEvent::Header(id, ref data) if id == 0x3 => {
                                      Some(BigEndian::read_u32(data) as usize * 4)
                                  }
                                  _ => None,
                              }
                          })
                          .next()
                          .unwrap();

        let chunk_count = (size + CHUNK_SIZE - 1) / CHUNK_SIZE;

        let shared = Arc::new(AudioFileShared {
            file_id: file_id,
            chunk_count: chunk_count,
            cond: Condvar::new(),
            bitmap: Mutex::new(BitSet::with_capacity(chunk_count)),
        });

        let write_file = NamedTempFile::new().unwrap();
        write_file.set_len(size as u64).unwrap();
        let read_file = write_file.reopen().unwrap();

        let (seek_tx, seek_rx) = mpsc::channel();
        let (complete_tx, complete_rx) = eventual::Future::pair();

        {
            let shared = shared.clone();
            let session = session.clone();
            thread::spawn(move || AudioFile::fetch(&session, shared, write_file, seek_rx, complete_tx));
        }

        (AudioFile {
            read_file: read_file,

            position: 0,
            seek: seek_tx,

            shared: shared,
        }, complete_rx)
    }

    fn fetch(session: &Session,
             shared: Arc<AudioFileShared>,
             mut write_file: NamedTempFile,
             seek_rx: mpsc::Receiver<u64>,
             complete_tx: eventual::Complete<NamedTempFile, ()>) {
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
                write_file.seek(SeekFrom::Start(0)).unwrap();
                complete_tx.complete(write_file);
                break;
            }

            while bitmap.contains(&index) {
                index = (index + 1) % shared.chunk_count;
            }
            drop(bitmap);

            AudioFile::fetch_chunk(session, &shared, &mut write_file, index);
        }
    }

    fn fetch_chunk(session: &Session,
                   shared: &Arc<AudioFileShared>,
                   write_file: &mut NamedTempFile,
                   index: usize) {

        let rx = session.stream(shared.file_id,
                                (index * CHUNK_SIZE / 4) as u32,
                                (CHUNK_SIZE / 4) as u32);

        debug!("Fetch chunk {} / {}", index + 1, shared.chunk_count);

        write_file.seek(SeekFrom::Start((index * CHUNK_SIZE) as u64)).unwrap();

        let mut size = 0usize;
        for event in rx.iter() {
            match event {
                StreamEvent::Header(..) => (),
                StreamEvent::Data(data) => {
                    write_file.write_all(&data).unwrap();

                    size += data.len();
                    if size >= CHUNK_SIZE {
                        break;
                    }
                }
            }
        }

        let mut bitmap = shared.bitmap.lock().unwrap();
        bitmap.insert(index as usize);

        shared.cond.notify_all();
    }
}

impl Read for AudioFile {
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        let index = self.position as usize / CHUNK_SIZE;
        let offset = self.position as usize % CHUNK_SIZE;
        let len = min(output.len(), CHUNK_SIZE - offset);

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
