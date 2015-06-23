use byteorder::{ByteOrder, BigEndian};
use std::cmp::min;
use std::collections::BitSet;
use std::io::{self, SeekFrom};
use std::slice::bytes::copy_memory;
use std::sync::{Arc, Condvar, Mutex};
use std::sync::mpsc::{self, TryRecvError};

use stream::{StreamRequest, StreamEvent};
use util::FileId;
use std::thread;

const CHUNK_SIZE : usize = 0x40000;

#[derive(Clone)]
pub struct AudioFile {
    file: FileId,
    size: usize,
    seek: mpsc::Sender<u64>,

    data: Arc<(Mutex<AudioFileData>, Condvar)>,
}

struct AudioFileData {
    buffer: Vec<u8>,
    bitmap: BitSet,
}

impl AudioFile {
    pub fn new(file: FileId, streams: mpsc::Sender<StreamRequest>) -> AudioFile {
        let (tx, rx) = mpsc::channel();

        streams.send(StreamRequest {
            id: file,
            offset: 0,
            size: 1,
            callback: tx
        }).unwrap();

        let size = {
            let mut size = None;
            for event in rx.iter() {
                match event {
                    StreamEvent::Header(id, data) => {
                        if id == 0x3 {
                            size = Some(BigEndian::read_u32(&data) * 4);
                            break;
                        }
                    },
                    StreamEvent::Data(_) => break
                }
            }
            size.unwrap() as usize
        };

        let bufsize = size + (CHUNK_SIZE - size % CHUNK_SIZE); 
        let (tx, rx) = mpsc::channel();
        
        let ret = AudioFile {
            file: file,
            size: size,
            seek: tx,

            data: Arc::new((Mutex::new(AudioFileData {
                buffer: vec![0u8; bufsize],
                bitmap: BitSet::with_capacity(bufsize / CHUNK_SIZE as usize)
            }), Condvar::new())),
        };

        let f = ret.clone();

        thread::spawn( move || { f.fetch(streams, rx); });

        ret
    }
    
    fn fetch_chunk(&self, streams: &mpsc::Sender<StreamRequest>, index: usize) {
        let (tx, rx) = mpsc::channel();
        streams.send(StreamRequest {
            id: self.file,
            offset: (index * CHUNK_SIZE / 4) as u32,
            size: (CHUNK_SIZE / 4) as u32,
            callback: tx
        }).unwrap();

        let mut offset = 0usize;
        for event in rx.iter() {
            match event {
                StreamEvent::Header(_,_) => (),
                StreamEvent::Data(data) => {
                    let mut handle = self.data.0.lock().unwrap();
                    copy_memory(&data, &mut handle.buffer[index * CHUNK_SIZE + offset ..]);
                    offset += data.len();

                    if offset >= CHUNK_SIZE {
                        break
                    }
                }
            }
        }
        
        {
            let mut handle = self.data.0.lock().unwrap();
            handle.bitmap.insert(index as usize);
            self.data.1.notify_all();
        }
    }

    fn fetch(&self, streams: mpsc::Sender<StreamRequest>, seek: mpsc::Receiver<u64>) {
        let mut index = 0;
        loop {
            index = if index * CHUNK_SIZE < self.size {
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
                let handle = self.data.0.lock().unwrap();
                while handle.bitmap.contains(&index) && index * CHUNK_SIZE < self.size {
                    index += 1;
                }
            }

            if index * CHUNK_SIZE < self.size {
                self.fetch_chunk(&streams, index) 
            }
        }
    }
}

pub struct AudioFileReader {
    file: AudioFile,
    position: usize,
}

impl AudioFileReader {
    pub fn new(file: AudioFile) -> AudioFileReader {
        AudioFileReader {
            file: file,
            position: 0
        }
    }
}

impl io::Read for AudioFileReader {
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        let index = self.position / CHUNK_SIZE;
        let offset = self.position % CHUNK_SIZE;
        let len = min(output.len(), CHUNK_SIZE-offset);

        let mut handle = self.file.data.0.lock().unwrap();

        while !handle.bitmap.contains(&index) {
            handle = self.file.data.1.wait(handle).unwrap();
        }

        copy_memory(&handle.buffer[self.position..self.position+len], output);
        self.position += len;

        Ok(len)
    }
}

impl io::Seek for AudioFileReader {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        let newpos = match pos {
            SeekFrom::Start(offset) => offset as i64,
            SeekFrom::End(offset) => self.file.size as i64 + offset,
            SeekFrom::Current(offset) => self.position as i64 + offset,
        };

        self.position = min(newpos as usize, self.file.size);
        self.file.seek.send(self.position as u64).unwrap();
        Ok(self.position as u64)
    }
}


