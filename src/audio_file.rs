use byteorder::{ByteOrder, BigEndian};
use std::cmp::min;
use std::collections::BitSet;
use std::io;
use std::slice::bytes::copy_memory;
use std::sync::{mpsc, Arc, Condvar, Mutex};

use stream::{StreamRequest, StreamEvent};
use util::FileId;

const CHUNK_SIZE: usize = 0x40000;
#[derive(Clone)]
pub struct AudioFileRef(Arc<AudioFile>);

struct AudioFile {
    file: FileId,
    size: usize,

    data: Mutex<AudioFileData>,
    cond: Condvar
}

struct AudioFileData {
    buffer: Vec<u8>,
    bitmap: BitSet,
}

impl AudioFileRef {
    pub fn new(file: FileId, streams: mpsc::Sender<StreamRequest>) -> AudioFileRef {
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
        
        AudioFileRef(Arc::new(AudioFile {
            file: file,
            size: size,

            data: Mutex::new(AudioFileData {
                buffer: vec![0u8; size + (CHUNK_SIZE - size % CHUNK_SIZE)],
                bitmap: BitSet::with_capacity(size / CHUNK_SIZE)
            }),
            cond: Condvar::new(),
        }))
    }
    
    pub fn fetch(&self, streams: mpsc::Sender<StreamRequest>) {
        let &AudioFileRef(ref inner) = self;

        let mut index : usize = 0;

        while index * CHUNK_SIZE < inner.size {
            let (tx, rx) = mpsc::channel();

            streams.send(StreamRequest {
                id: inner.file,
                offset: (index * CHUNK_SIZE / 4) as u32,
                size: (CHUNK_SIZE / 4) as u32,
                callback: tx
            }).unwrap();

            let mut offset = 0;
            for event in rx.iter() {
                match event {
                    StreamEvent::Header(_,_) => (),
                    StreamEvent::Data(data) => {
                        let mut handle = inner.data.lock().unwrap();
                        copy_memory(&data, &mut handle.buffer[index * CHUNK_SIZE + offset..]);
                        offset += data.len();

                        if offset >= CHUNK_SIZE {
                            break
                        }
                    }
                }
            }
            
            {
                let mut handle = inner.data.lock().unwrap();
                handle.bitmap.insert(index);
                inner.cond.notify_all();
            }

            index += 1;
        }
    }
}

pub struct AudioFileReader {
    file: AudioFileRef,
    position: usize
}

impl AudioFileReader {
    pub fn new(file: &AudioFileRef) -> AudioFileReader {
        AudioFileReader {
            file: file.clone(),
            position: 0
        }
    }
}

impl io::Read for AudioFileReader {
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        let index = self.position / CHUNK_SIZE;
        let offset = self.position % CHUNK_SIZE;
        let len = min(output.len(), CHUNK_SIZE-offset);

        let &AudioFileRef(ref inner) = &self.file;
        let mut handle = inner.data.lock().unwrap();

        while !handle.bitmap.contains(&index) {
            handle = inner.cond.wait(handle).unwrap();
        }

        copy_memory(&handle.buffer[self.position..self.position+len], output);
        self.position += len;

        Ok(len)
    }
}

impl io::Seek for AudioFileReader {
    fn seek(&mut self, _pos: io::SeekFrom) -> io::Result<u64> {
        Err(io::Error::new(io::ErrorKind::Other, "Cannot seek"))
    }
}


