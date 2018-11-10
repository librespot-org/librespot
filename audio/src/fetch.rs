use bit_set::BitSet;
use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use futures::sync::{mpsc, oneshot};
use futures::Stream;
use futures::{Async, Future, Poll};
use std::cmp::min;
use std::fs;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::sync::{Arc, Condvar, Mutex};
use tempfile::NamedTempFile;

use core::channel::{Channel, ChannelData, ChannelError, ChannelHeaders};
use core::session::Session;
use core::spotify_id::FileId;

const CHUNK_SIZE: usize = 0x20000;

pub enum AudioFile {
    Cached(fs::File),
    Streaming(AudioFileStreaming),
}

pub enum AudioFileOpen {
    Cached(Option<fs::File>),
    Streaming(AudioFileOpenStreaming),
}

pub struct AudioFileOpenStreaming {
    session: Session,
    data_rx: Option<ChannelData>,
    headers: ChannelHeaders,
    file_id: FileId,
    complete_tx: Option<oneshot::Sender<NamedTempFile>>,
}

pub struct AudioFileStreaming {
    read_file: fs::File,

    position: u64,
    seek: mpsc::UnboundedSender<u64>,

    shared: Arc<AudioFileShared>,
}

struct AudioFileShared {
    file_id: FileId,
    chunk_count: usize,
    cond: Condvar,
    bitmap: Mutex<BitSet>,
}

impl AudioFileOpenStreaming {
    fn finish(&mut self, size: usize) -> AudioFileStreaming {
        let chunk_count = (size + CHUNK_SIZE - 1) / CHUNK_SIZE;

        let shared = Arc::new(AudioFileShared {
            file_id: self.file_id,
            chunk_count: chunk_count,
            cond: Condvar::new(),
            bitmap: Mutex::new(BitSet::with_capacity(chunk_count)),
        });

        let mut write_file = NamedTempFile::new().unwrap();
        write_file.set_len(size as u64).unwrap();
        write_file.seek(SeekFrom::Start(0)).unwrap();

        let read_file = write_file.reopen().unwrap();

        let data_rx = self.data_rx.take().unwrap();
        let complete_tx = self.complete_tx.take().unwrap();
        let (seek_tx, seek_rx) = mpsc::unbounded();

        let fetcher = AudioFileFetch::new(
            self.session.clone(),
            shared.clone(),
            data_rx,
            write_file,
            seek_rx,
            complete_tx,
        );
        self.session.spawn(move |_| fetcher);

        AudioFileStreaming {
            read_file: read_file,

            position: 0,
            seek: seek_tx,

            shared: shared,
        }
    }
}

impl Future for AudioFileOpen {
    type Item = AudioFile;
    type Error = ChannelError;

    fn poll(&mut self) -> Poll<AudioFile, ChannelError> {
        match *self {
            AudioFileOpen::Streaming(ref mut open) => {
                let file = try_ready!(open.poll());
                Ok(Async::Ready(AudioFile::Streaming(file)))
            }
            AudioFileOpen::Cached(ref mut file) => {
                let file = file.take().unwrap();
                Ok(Async::Ready(AudioFile::Cached(file)))
            }
        }
    }
}

impl Future for AudioFileOpenStreaming {
    type Item = AudioFileStreaming;
    type Error = ChannelError;

    fn poll(&mut self) -> Poll<AudioFileStreaming, ChannelError> {
        loop {
            let (id, data) = try_ready!(self.headers.poll()).unwrap();

            if id == 0x3 {
                let size = BigEndian::read_u32(&data) as usize * 4;
                let file = self.finish(size);

                return Ok(Async::Ready(file));
            }
        }
    }
}

impl AudioFile {
    pub fn open(session: &Session, file_id: FileId) -> AudioFileOpen {
        let cache = session.cache().cloned();

        if let Some(file) = cache.as_ref().and_then(|cache| cache.file(file_id)) {
            debug!("File {} already in cache", file_id);
            return AudioFileOpen::Cached(Some(file));
        }

        debug!("Downloading file {}", file_id);

        let (complete_tx, complete_rx) = oneshot::channel();
        let (headers, data) = request_chunk(session, file_id, 0).split();

        let open = AudioFileOpenStreaming {
            session: session.clone(),
            file_id: file_id,

            headers: headers,
            data_rx: Some(data),

            complete_tx: Some(complete_tx),
        };

        let session_ = session.clone();
        session.spawn(move |_| {
            complete_rx
                .map(move |mut file| {
                    if let Some(cache) = session_.cache() {
                        cache.save_file(file_id, &mut file);
                        debug!("File {} complete, saving to cache", file_id);
                    } else {
                        debug!("File {} complete", file_id);
                    }
                })
                .or_else(|oneshot::Canceled| Ok(()))
        });

        AudioFileOpen::Streaming(open)
    }
}

fn request_chunk(session: &Session, file: FileId, index: usize) -> Channel {
    trace!("requesting chunk {}", index);

    let start = (index * CHUNK_SIZE / 4) as u32;
    let end = ((index + 1) * CHUNK_SIZE / 4) as u32;

    let (id, channel) = session.channel().allocate();

    let mut data: Vec<u8> = Vec::new();
    data.write_u16::<BigEndian>(id).unwrap();
    data.write_u8(0).unwrap();
    data.write_u8(1).unwrap();
    data.write_u16::<BigEndian>(0x0000).unwrap();
    data.write_u32::<BigEndian>(0x00000000).unwrap();
    data.write_u32::<BigEndian>(0x00009C40).unwrap();
    data.write_u32::<BigEndian>(0x00020000).unwrap();
    data.write(&file.0).unwrap();
    data.write_u32::<BigEndian>(start).unwrap();
    data.write_u32::<BigEndian>(end).unwrap();

    session.send_packet(0x8, data);

    channel
}

struct AudioFileFetch {
    session: Session,
    shared: Arc<AudioFileShared>,
    output: Option<NamedTempFile>,

    index: usize,
    data_rx: ChannelData,

    seek_rx: mpsc::UnboundedReceiver<u64>,
    complete_tx: Option<oneshot::Sender<NamedTempFile>>,
}

impl AudioFileFetch {
    fn new(
        session: Session,
        shared: Arc<AudioFileShared>,
        data_rx: ChannelData,
        output: NamedTempFile,
        seek_rx: mpsc::UnboundedReceiver<u64>,
        complete_tx: oneshot::Sender<NamedTempFile>,
    ) -> AudioFileFetch {
        AudioFileFetch {
            session: session,
            shared: shared,
            output: Some(output),

            index: 0,
            data_rx: data_rx,

            seek_rx: seek_rx,
            complete_tx: Some(complete_tx),
        }
    }

    fn download(&mut self, mut new_index: usize) {
        assert!(new_index < self.shared.chunk_count);

        {
            let bitmap = self.shared.bitmap.lock().unwrap();
            while bitmap.contains(new_index) {
                new_index = (new_index + 1) % self.shared.chunk_count;
            }
        }

        if self.index != new_index {
            self.index = new_index;

            let offset = self.index * CHUNK_SIZE;

            self.output
                .as_mut()
                .unwrap()
                .seek(SeekFrom::Start(offset as u64))
                .unwrap();

            let (_headers, data) = request_chunk(&self.session, self.shared.file_id, self.index).split();
            self.data_rx = data;
        }
    }

    fn finish(&mut self) {
        let mut output = self.output.take().unwrap();
        let complete_tx = self.complete_tx.take().unwrap();

        output.seek(SeekFrom::Start(0)).unwrap();
        let _ = complete_tx.send(output);
    }
}

impl Future for AudioFileFetch {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<(), ()> {
        loop {
            let mut progress = false;

            match self.seek_rx.poll() {
                Ok(Async::Ready(None)) => {
                    return Ok(Async::Ready(()));
                }
                Ok(Async::Ready(Some(offset))) => {
                    progress = true;
                    let index = offset as usize / CHUNK_SIZE;
                    self.download(index);
                }
                Ok(Async::NotReady) => (),
                Err(()) => unreachable!(),
            }

            match self.data_rx.poll() {
                Ok(Async::Ready(Some(data))) => {
                    progress = true;

                    self.output.as_mut().unwrap().write_all(data.as_ref()).unwrap();
                }
                Ok(Async::Ready(None)) => {
                    progress = true;

                    trace!("chunk {} / {} complete", self.index, self.shared.chunk_count);

                    let full = {
                        let mut bitmap = self.shared.bitmap.lock().unwrap();
                        bitmap.insert(self.index as usize);
                        self.shared.cond.notify_all();

                        bitmap.len() >= self.shared.chunk_count
                    };

                    if full {
                        self.finish();
                        return Ok(Async::Ready(()));
                    }

                    let new_index = (self.index + 1) % self.shared.chunk_count;
                    self.download(new_index);
                }
                Ok(Async::NotReady) => (),
                Err(ChannelError) => {
                    warn!("error from channel");
                    return Ok(Async::Ready(()));
                }
            }

            if !progress {
                return Ok(Async::NotReady);
            }
        }
    }
}

impl Read for AudioFileStreaming {
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

impl Seek for AudioFileStreaming {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.position = try!(self.read_file.seek(pos));
        // Do not seek past EOF
        if (self.position as usize % CHUNK_SIZE) != 0  {
            // Notify the fetch thread to get the correct block
            // This can fail if fetch thread has completed, in which case the
            // block is ready. Just ignore the error.
            let _ = self.seek.unbounded_send(self.position);
        } else {
            warn!("Trying to seek past EOF");
        }

        Ok(self.position)
    }
}

impl Read for AudioFile {
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        match *self {
            AudioFile::Cached(ref mut file) => file.read(output),
            AudioFile::Streaming(ref mut file) => file.read(output),
        }
    }
}

impl Seek for AudioFile {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        match *self {
            AudioFile::Cached(ref mut file) => file.seek(pos),
            AudioFile::Streaming(ref mut file) => file.seek(pos),
        }
    }
}
