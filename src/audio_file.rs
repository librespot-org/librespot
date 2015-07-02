use byteorder::{ByteOrder, BigEndian};
use std::cmp::min;
use std::collections::BitSet;
use std::io::{self, SeekFrom};
use std::slice::bytes::copy_memory;
use std::sync::{Arc, Condvar, Mutex};
use std::sync::mpsc::{self, TryRecvError};
use std::thread;

use stream::StreamEvent;
use util::FileId;
use session::Session;

const CHUNK_SIZE : usize = 0x10000;

pub struct AudioFile<'s> {
    position: usize,
    seek: mpsc::Sender<u64>,
    shared: Arc<AudioFileShared>,

    #[allow(dead_code)]
    thread: thread::JoinGuard<'s, ()>,
}

struct AudioFileShared {
    file_id: FileId,
    size: usize,
    data: Mutex<AudioFileData>,
    cond: Condvar
}

struct AudioFileData {
    buffer: Vec<u8>,
    bitmap: BitSet,
}

impl <'s> AudioFile <'s> {
    pub fn new(session: &Session, file_id: FileId) -> AudioFile {
        let mut it = session.stream(file_id, 0, 1).into_iter()
            .filter_map(|event| {
                match event {
                    StreamEvent::Header(id, ref data) if id == 0x3 => {
                        Some(BigEndian::read_u32(data) as usize * 4)
                    }
                    _ => None
                }
            });
        
        let size = it.next().unwrap();

        let bufsize = size + (CHUNK_SIZE - size % CHUNK_SIZE); 

        let shared = Arc::new(AudioFileShared {
            file_id: file_id,
            size: size,
            data: Mutex::new(AudioFileData {
                buffer: vec![0u8; bufsize],
                bitmap: BitSet::with_capacity(bufsize / CHUNK_SIZE as usize)
            }),
            cond: Condvar::new(),
        });
        
        let shared_ = shared.clone();
        let (seek_tx, seek_rx) = mpsc::channel();

        let file = AudioFile {
            thread: thread::scoped( move || { AudioFile::fetch(session, shared_, seek_rx); }),
            position: 0,
            seek: seek_tx,
            shared: shared,
        };

        file
    }

    fn fetch_chunk(session: &Session, shared: &Arc<AudioFileShared>, index: usize) {
        let rx = session.stream(shared.file_id,
                     (index * CHUNK_SIZE / 4) as u32,
                     (CHUNK_SIZE / 4) as u32);

        let mut offset = 0usize;
        for event in rx.iter() {
            match event {
                StreamEvent::Header(_,_) => (),
                StreamEvent::Data(data) => {
                    let mut handle = shared.data.lock().unwrap();
                    copy_memory(&data, &mut handle.buffer[index * CHUNK_SIZE + offset ..]);
                    offset += data.len();

                    if offset >= CHUNK_SIZE {
                        break
                    }
                }
            }
        }

        {
            let mut handle = shared.data.lock().unwrap();
            handle.bitmap.insert(index as usize);
            shared.cond.notify_all();
        }
    }

    fn fetch(session: &Session, shared: Arc<AudioFileShared>, seek: mpsc::Receiver<u64>) {
        let mut index = 0;
        loop {
            index = if index * CHUNK_SIZE < shared.size {
                match seek.try_recv() {
                    Ok(position) => position as usize / CHUNK_SIZE,
                    Err(TryRecvError::Empty) => index,
                    Err(TryRecvError::Disconnected) => break
                }
            } else {
                match seek.recv() {
                    Ok(position) => position as usize / CHUNK_SIZE,
                    Err(_) => break
                }
            };

            {
                let handle = shared.data.lock().unwrap();
                while handle.bitmap.contains(&index) && index * CHUNK_SIZE < shared.size {
                    index += 1;
                }
            }

            if index * CHUNK_SIZE < shared.size {
                AudioFile::fetch_chunk(session, &shared, index) 
            }
        }
    }
}

impl <'s> io::Read for AudioFile <'s> {
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        let index = self.position / CHUNK_SIZE;
        let offset = self.position % CHUNK_SIZE;
        let len = min(output.len(), CHUNK_SIZE-offset);

        let mut handle = self.shared.data.lock().unwrap();

        while !handle.bitmap.contains(&index) {
            handle = self.shared.cond.wait(handle).unwrap();
        }

        copy_memory(&handle.buffer[self.position..self.position+len], output);
        self.position += len;

        Ok(len)
    }
}

impl <'s> io::Seek for AudioFile <'s> {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        let newpos = match pos {
            SeekFrom::Start(offset) => offset as i64,
            SeekFrom::End(offset) => self.shared.size as i64 + offset,
            SeekFrom::Current(offset) => self.position as i64 + offset,
        };

        self.position = min(newpos as usize, self.shared.size);
        self.seek.send(self.position as u64).unwrap();
        Ok(self.position as u64)
    }
}


