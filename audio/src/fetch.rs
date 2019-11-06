use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use bytes::Bytes;
use futures::sync::{mpsc, oneshot};
use futures::Stream;
use futures::{Async, Future, Poll};
use std::cmp::min;
use std::fs;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};
use tempfile::NamedTempFile;
use range_set::{Range, RangeSet};

use librespot_core::channel::{Channel, ChannelData, ChannelError, ChannelHeaders};
use librespot_core::session::Session;
use librespot_core::spotify_id::FileId;
use futures::sync::mpsc::unbounded;
use std::sync::atomic;
use std::sync::atomic::AtomicUsize;

const MINIMUM_CHUNK_SIZE: usize = 1024 * 16; // This number MUST be divisible by 4.
const MAXIMUM_CHUNK_SIZE: usize = 1024 * 128;
const MAXIMUM_ASSUMED_PING_TIME_SECONDS: u64 = 5;

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
    initial_data_rx: Option<ChannelData>,
    initial_data_length: Option<usize>,
    initial_request_sent_time: Instant,
    headers: ChannelHeaders,
    file_id: FileId,
    complete_tx: Option<oneshot::Sender<NamedTempFile>>,
}


enum StreamLoaderCommand{
    Fetch(Range), // signal the stream loader to fetch a range of the file
    RandomAccessMode(), // optimise download strategy for random access
    StreamMode(), // optimise download strategy for streaming
    StreamDataRate(usize), // when optimising for streaming, assume a streaming rate of this many bytes per second.
    Close(), // terminate and don't load any more data
}


#[derive(Clone)]
pub struct StreamLoaderController {
    channel_tx: Option<mpsc::UnboundedSender<StreamLoaderCommand>>,
    stream_shared: Option<Arc<AudioFileShared>>,
    file_size: usize,
    bytes_per_second: usize,
}


impl StreamLoaderController {
    pub fn len(&self) -> usize {
        return self.file_size;
    }

    pub fn data_rate(&self) -> usize { return self.bytes_per_second; }

    pub fn range_available(&self, range: Range) -> bool {
        if let Some(ref shared) = self.stream_shared {
            let download_status = shared.download_status.lock().unwrap();
            if range.length <= download_status.downloaded.contained_length_from_value(range.start) {
                return true;
            } else {
                return false;
            }
        } else {
            if range.length <= self.len() - range.start {
                return true;
            } else {
                return false;
            }
        }
    }

    pub fn ping_time_ms(&self) -> usize {
        if let Some(ref shared) = self.stream_shared {
            return shared.ping_time_ms.load(atomic::Ordering::Relaxed);
        } else {
            return 0;
        }
    }

    fn send_stream_loader_command(&mut self, command: StreamLoaderCommand) {
        if let Some(ref mut channel) = self.channel_tx {
            // ignore the error in case the channel has been closed already.
            let _ = channel.unbounded_send(command);
        }
    }

    pub fn fetch(&mut self, range: Range) {
        // signal the stream loader to fetch a range of the file
        self.send_stream_loader_command(StreamLoaderCommand::Fetch(range));
    }

    pub fn fetch_blocking(&mut self, mut range: Range) {
        // signal the stream loader to tech a range of the file and block until it is loaded.

        // ensure the range is within the file's bounds.
        if range.start >= self.len() {
            range.length = 0;
        } else if range.end() > self.len() {
            range.length = self.len() - range.start;
        }

        self.fetch(range);

        if let Some(ref shared) = self.stream_shared {
            let mut download_status = shared.download_status.lock().unwrap();
            while range.length > download_status.downloaded.contained_length_from_value(range.start) {
                download_status = shared.cond.wait_timeout(download_status, Duration::from_millis(1000)).unwrap().0;
                if range.length > (download_status.downloaded.union(&download_status.requested).contained_length_from_value(range.start)) {
                    // For some reason, the requested range is neither downloaded nor requested.
                    // This could be due to a network error. Request it again.
                    // We can't use self.fetch here because self can't borrowed mutably, so we access the channel directly.
                    if let Some(ref mut channel) = self.channel_tx {
                        // ignore the error in case the channel has been closed already.
                        let _ = channel.unbounded_send(StreamLoaderCommand::Fetch(range));
                    }
                }
            }
        }

    }

    pub fn fetch_next(&mut self, length: usize) {
        let range:Range = if let Some(ref shared) = self.stream_shared {
            Range {
                start: shared.read_position.load(atomic::Ordering::Relaxed),
                length: length,
            }
        } else {
            return;
        };
        self.fetch(range);
    }

    pub fn fetch_next_blocking(&mut self, length: usize) {
        let range:Range = if let Some(ref shared) = self.stream_shared {
            Range {
                start: shared.read_position.load(atomic::Ordering::Relaxed),
                length: length,
            }
        } else {
            return;
        };
        self.fetch_blocking(range);
    }

    pub fn set_random_access_mode(&mut self) {
        // optimise download strategy for random access
        self.send_stream_loader_command(StreamLoaderCommand::RandomAccessMode());
    }

    pub fn set_stream_mode(&mut self) {
        // optimise download strategy for streaming
        self.send_stream_loader_command(StreamLoaderCommand::StreamMode());
    }

    pub fn set_stream_data_rate(&mut self, bytes_per_second: usize) {
        // when optimising for streaming, assume a streaming rate of this many bytes per second.
        self.bytes_per_second = bytes_per_second;
        self.send_stream_loader_command(StreamLoaderCommand::StreamDataRate(bytes_per_second));
    }

    pub fn close(&mut self) {
        // terminate stream loading and don't load any more data for this file.
        self.send_stream_loader_command(StreamLoaderCommand::Close());
    }


}


pub struct AudioFileStreaming {
    read_file: fs::File,

    position: u64,

    stream_loader_command_tx: mpsc::UnboundedSender<StreamLoaderCommand>,

    shared: Arc<AudioFileShared>,
}


struct AudioFileDownloadStatus {
    requested: RangeSet,
    downloaded: RangeSet,
}

struct AudioFileShared {
    file_id: FileId,
    file_size: usize,
    cond: Condvar,
    download_status: Mutex<AudioFileDownloadStatus>,
    number_of_open_requests: AtomicUsize,
    ping_time_ms: AtomicUsize,
    read_position: AtomicUsize,
}

impl AudioFileOpenStreaming {
    fn finish(&mut self, size: usize) -> AudioFileStreaming {

        let shared = Arc::new(AudioFileShared {
            file_id: self.file_id,
            file_size: size,
            cond: Condvar::new(),
            download_status: Mutex::new(AudioFileDownloadStatus {requested: RangeSet::new(), downloaded: RangeSet::new()}),
            number_of_open_requests: AtomicUsize::new(0),
            ping_time_ms: AtomicUsize::new(0),
            read_position: AtomicUsize::new(0),
        });

        let mut write_file = NamedTempFile::new().unwrap();
        write_file.as_file().set_len(size as u64).unwrap();
        write_file.seek(SeekFrom::Start(0)).unwrap();

        let read_file = write_file.reopen().unwrap();

        let initial_data_rx = self.initial_data_rx.take().unwrap();
        let initial_data_length = self.initial_data_length.take().unwrap();
        let complete_tx = self.complete_tx.take().unwrap();
        //let (seek_tx, seek_rx) = mpsc::unbounded();
        let (stream_loader_command_tx, stream_loader_command_rx) = mpsc::unbounded::<StreamLoaderCommand>();

        let fetcher = AudioFileFetch::new(
            self.session.clone(),
            shared.clone(),
            initial_data_rx,
            self.initial_request_sent_time,
            initial_data_length,
            write_file,
            stream_loader_command_rx,
            complete_tx,
        );
        self.session.spawn(move |_| fetcher);

        AudioFileStreaming {
            read_file: read_file,

            position: 0,
            //seek: seek_tx,
            stream_loader_command_tx: stream_loader_command_tx,

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
        let initial_data_length = MINIMUM_CHUNK_SIZE;
        let (headers, data) = request_range(session, file_id, 0, initial_data_length).split();

        let open = AudioFileOpenStreaming {
            session: session.clone(),
            file_id: file_id,

            headers: headers,
            initial_data_rx: Some(data),
            initial_data_length: Some(initial_data_length),
            initial_request_sent_time: Instant::now(),

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

    pub fn get_stream_loader_controller(&self, bytes_per_second: usize) -> StreamLoaderController {
        match self {
            AudioFile::Streaming(stream) => {
                let mut result = StreamLoaderController {
                    channel_tx: Some(stream.stream_loader_command_tx.clone()),
                    stream_shared: Some(stream.shared.clone()),
                    file_size: stream.shared.file_size,
                    bytes_per_second: bytes_per_second,
                };
                result.set_stream_data_rate(bytes_per_second);
                return result;
            }
            AudioFile::Cached(ref file) => {
                return StreamLoaderController {
                    channel_tx: None,
                    stream_shared: None,
                    file_size: file.metadata().unwrap().len() as usize,
                    bytes_per_second: bytes_per_second,
                }
            }
        }
    }
}


fn request_range(session: &Session, file: FileId, offset: usize, length: usize) -> Channel {

    assert!(offset % 4 == 0, "Range request start positions must be aligned by 4 bytes.");
    assert!(length % 4 == 0, "Range request range lengths must be aligned by 4 bytes.");
    let start = offset / 4;
    let end = (offset+length) / 4;

    let (id, channel) = session.channel().allocate();

    trace!("requesting range starting at {} of length {} on channel {}.", offset, length, id);

    let mut data: Vec<u8> = Vec::new();
    data.write_u16::<BigEndian>(id).unwrap();
    data.write_u8(0).unwrap();
    data.write_u8(1).unwrap();
    data.write_u16::<BigEndian>(0x0000).unwrap();
    data.write_u32::<BigEndian>(0x00000000).unwrap();
    data.write_u32::<BigEndian>(0x00009C40).unwrap();
    data.write_u32::<BigEndian>(0x00020000).unwrap();
    data.write(&file.0).unwrap();
    data.write_u32::<BigEndian>(start as u32).unwrap();
    data.write_u32::<BigEndian>(end as u32).unwrap();

    session.send_packet(0x8, data);

    channel
}



struct PartialFileData {
    offset: usize,
    data: Bytes,
}

enum ReceivedData {
    ResponseTimeMs(usize),
    Data(PartialFileData),
}

struct AudioFileFetchDataReceiver {
    shared: Arc<AudioFileShared>,
    file_data_tx: mpsc::UnboundedSender<ReceivedData>,
    data_rx: ChannelData,
    initial_data_offset: usize,
    initial_request_length: usize,
    data_offset: usize,
    request_length: usize,
    request_sent_time: Option<Instant>,
    measure_ping_time: bool,
}

impl AudioFileFetchDataReceiver {
    fn new(
        shared: Arc<AudioFileShared>,
        file_data_tx: mpsc::UnboundedSender<ReceivedData>,
        data_rx: ChannelData,
        data_offset: usize,
        request_length: usize,
        request_sent_time: Instant,
    ) -> AudioFileFetchDataReceiver {

        let measure_ping_time = shared.number_of_open_requests.load(atomic::Ordering::SeqCst) == 0;

        shared.number_of_open_requests.fetch_add(1, atomic::Ordering::SeqCst);

        AudioFileFetchDataReceiver {
            shared: shared,
            data_rx: data_rx,
            file_data_tx: file_data_tx,
            initial_data_offset: data_offset,
            initial_request_length: request_length,
            data_offset: data_offset,
            request_length: request_length,
            request_sent_time: Some(request_sent_time),
            measure_ping_time: measure_ping_time,
        }
    }
}



impl AudioFileFetchDataReceiver {
    fn finish(&mut self) {
        if self.request_length > 0 {

            let missing_range = Range::new(self.data_offset, self.request_length);

            let mut download_status = self.shared.download_status.lock().unwrap();
            download_status.requested.subtract_range(&missing_range);
            self.shared.cond.notify_all();
        }

        self.shared.number_of_open_requests.fetch_sub(1, atomic::Ordering::SeqCst);

    }
}

impl Future for AudioFileFetchDataReceiver {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<(), ()> {
        loop {
            match self.data_rx.poll() {
                Ok(Async::Ready(Some(data))) => {
                    if self.measure_ping_time {
                        if let Some(request_sent_time) = self.request_sent_time {
                            let duration = Instant::now() - request_sent_time;
                            let duration_ms: u64;
                            if duration.as_secs() > MAXIMUM_ASSUMED_PING_TIME_SECONDS {
                                duration_ms = MAXIMUM_ASSUMED_PING_TIME_SECONDS * 1000;
                            } else {
                                duration_ms = duration.as_secs() * 1000 + duration.subsec_millis() as u64;
                            }
                            let _ = self.file_data_tx.unbounded_send(ReceivedData::ResponseTimeMs(duration_ms as usize));
                            self.measure_ping_time = false;
                        }
                    }
                    let data_size = data.len();
                    trace!("data_receiver for range {} (+{}) got {} bytes of data starting at {}. ({} bytes pending).", self.initial_data_offset, self.initial_request_length, data_size, self.data_offset, self.request_length - data_size);
                    let _ = self.file_data_tx.unbounded_send(ReceivedData::Data(PartialFileData { offset: self.data_offset, data: data, }));
                    self.data_offset += data_size;
                    if self.request_length < data_size {
                        warn!("Data receiver for range {} (+{}) received more data from server than requested.", self.initial_data_offset, self.initial_request_length);
                        self.request_length = 0;
                    } else {
                        self.request_length -= data_size;
                    }
                    if self.request_length == 0 {
                        trace!("Data receiver for range {} (+{}) completed.", self.initial_data_offset, self.initial_request_length);
                        self.finish();
                        return Ok(Async::Ready(()));
                    }
                }
                Ok(Async::Ready(None)) => {
                    if self.request_length > 0 {
                        warn!("Data receiver for range {} (+{}) received less data from server than requested.", self.initial_data_offset, self.initial_request_length);
                    }
                    self.finish();
                    return Ok(Async::Ready(()));
                }
                Ok(Async::NotReady) => {
                    return Ok(Async::NotReady);
                }
                Err(ChannelError) => {
                    warn!("Error from channel for data receiver for range {} (+{}).", self.initial_data_offset, self.initial_request_length);
                    self.finish();
                    return Ok(Async::Ready(()));
                }
            }
        }
    }
}


enum DownloadStrategy {
    RandomAccess(),
    Streaming(),
}

struct AudioFileFetch {
    session: Session,
    shared: Arc<AudioFileShared>,
    output: Option<NamedTempFile>,

    file_data_tx: mpsc::UnboundedSender<ReceivedData>,
    file_data_rx: mpsc::UnboundedReceiver<ReceivedData>,

    //seek_rx: mpsc::UnboundedReceiver<u64>,
    stream_loader_command_rx: mpsc::UnboundedReceiver<StreamLoaderCommand>,
    complete_tx: Option<oneshot::Sender<NamedTempFile>>,
    download_strategy: DownloadStrategy,
    streaming_data_rate: usize,
    network_response_times_ms: Vec<usize>,
}

impl AudioFileFetch {
    fn new(
        session: Session,
        shared: Arc<AudioFileShared>,
        initial_data_rx: ChannelData,
        initial_request_sent_time: Instant,
        initial_data_length: usize,

        output: NamedTempFile,
        stream_loader_command_rx: mpsc::UnboundedReceiver<StreamLoaderCommand>,
        complete_tx: oneshot::Sender<NamedTempFile>,
    ) -> AudioFileFetch {

        let (file_data_tx, file_data_rx) = unbounded::<ReceivedData>();

        {
            let requested_range = Range::new(0, initial_data_length);
            let mut download_status = shared.download_status.lock().unwrap();
            download_status.requested.add_range(&requested_range);
        }


        let initial_data_receiver = AudioFileFetchDataReceiver::new(
            shared.clone(),
            file_data_tx.clone(),
            initial_data_rx,
            0,
            initial_data_length,
            initial_request_sent_time,
        );

        session.spawn(move |_| initial_data_receiver);

        AudioFileFetch {
            session: session,
            shared: shared,
            output: Some(output),

            file_data_tx: file_data_tx,
            file_data_rx: file_data_rx,

            stream_loader_command_rx: stream_loader_command_rx,
            complete_tx: Some(complete_tx),
            download_strategy: DownloadStrategy::RandomAccess(), // start with random access mode until someone tells us otherwise
            streaming_data_rate: 40, // assume 360 kbit per second unless someone tells us otherwise.
            network_response_times_ms: Vec::new(),
        }
    }

    fn download_range(&mut self, mut offset: usize, mut length: usize) {

        if length < MINIMUM_CHUNK_SIZE {
            length = MINIMUM_CHUNK_SIZE;
        }

        // ensure the values are within the bounds and align them by 4 for the spotify protocol.
        if offset >= self.shared.file_size {
            return;
        }

        if length <= 0 {
            return;
        }

        if offset + length > self.shared.file_size {
            length = self.shared.file_size - offset;
        }

        if offset % 4 != 0 {
            length += offset % 4;
            offset -= offset % 4;
        }

        if length % 4 != 0 {
            length += 4 - (length % 4);
        }

        let mut ranges_to_request = RangeSet::new();
        ranges_to_request.add_range(&Range::new(offset, length));

        let mut download_status = self.shared.download_status.lock().unwrap();

        ranges_to_request.subtract_range_set(&download_status.downloaded);
        ranges_to_request.subtract_range_set(&download_status.requested);


        for range in ranges_to_request.iter() {
            let (_headers, data) = request_range(&self.session, self.shared.file_id, range.start, range.length).split();

            download_status.requested.add_range(range);


            let receiver = AudioFileFetchDataReceiver::new(
                self.shared.clone(),
                self.file_data_tx.clone(),
                data,
                range.start,
                range.length,
                Instant::now(),
            );

            self.session.spawn(move |_| receiver);
        }

    }

    fn pre_fetch_more_data(&mut self) {

        // determine what is still missing
        let mut missing_data = RangeSet::new();
        missing_data.add_range(&Range::new(0,self.shared.file_size));
        {
            let download_status = self.shared.download_status.lock().unwrap();
            missing_data.subtract_range_set(&download_status.downloaded);
            missing_data.subtract_range_set(&download_status.requested);
        }

        // download data from after the current read position first
        let mut tail_end = RangeSet::new();
        let read_position = self.shared.read_position.load(atomic::Ordering::Relaxed);
        tail_end.add_range(&Range::new(read_position, self.shared.file_size - read_position));
        let tail_end = tail_end.intersection(&missing_data);

        if ! tail_end.is_empty() {
            let range = tail_end.get_range(0);
            let offset = range.start;
            let length = min(range.length, MAXIMUM_CHUNK_SIZE);
            self.download_range(offset, length);

        } else if ! missing_data.is_empty() {
            // ok, the tail is downloaded, download something fom the beginning.
            let range = missing_data.get_range(0);
            let offset = range.start;
            let length = min(range.length, MAXIMUM_CHUNK_SIZE);
            self.download_range(offset, length);
        }

    }


    fn poll_file_data_rx(&mut self) -> Poll<(), ()> {

        loop {
            match self.file_data_rx.poll() {
                Ok(Async::Ready(None)) => {
                    return Ok(Async::Ready(()));
                }
                Ok(Async::Ready(Some(ReceivedData::ResponseTimeMs(response_time_ms)))) => {
                    trace!("Received ping time estimate: {} ms.", response_time_ms);

                    // record the response time
                    self.network_response_times_ms.push(response_time_ms);

                    // prune old response times. Keep at most three.
                    while self.network_response_times_ms.len() > 3 {
                        self.network_response_times_ms.remove(0);
                    }

                    // stats::median is experimental. So we calculate the median of up to three ourselves.
                    let ping_time_ms: usize = match self.network_response_times_ms.len() {
                        1 => self.network_response_times_ms[0] as usize,
                        2 => ((self.network_response_times_ms[0] + self.network_response_times_ms[1]) / 2) as usize,
                        3 => {
                            let mut times = self.network_response_times_ms.clone();
                            times.sort();
                            times[1]
                        }
                        _ => unreachable!(),
                    };

                    // store our new estimate for everyone to see
                    self.shared.ping_time_ms.store(ping_time_ms, atomic::Ordering::Relaxed);

                },
                Ok(Async::Ready(Some(ReceivedData::Data(data)))) => {


                    self.output
                        .as_mut()
                        .unwrap()
                        .seek(SeekFrom::Start(data.offset as u64))
                        .unwrap();
                    self.output.as_mut().unwrap().write_all(data.data.as_ref()).unwrap();



                    let mut full = false;

                    {
                        let mut download_status = self.shared.download_status.lock().unwrap();

                        let received_range = Range::new(data.offset, data.data.len());
                        download_status.downloaded.add_range(&received_range);
                        self.shared.cond.notify_all();

                        if download_status.downloaded.contained_length_from_value(0) >= self.shared.file_size {
                            full = true;
                        }

                        trace!("Downloaded: {} Requested: {}", download_status.downloaded, download_status.requested.minus(&download_status.downloaded));

                        drop(download_status);
                    }

                    if full {
                        self.finish();
                        return Ok(Async::Ready(()));
                    }


                }
                Ok(Async::NotReady) => {
                    return Ok(Async::NotReady);
                },
                Err(()) => unreachable!(),
            }

        }

    }


    fn poll_stream_loader_command_rx(&mut self) -> Poll<(), ()> {

        loop {
            match self.stream_loader_command_rx.poll() {
                Ok(Async::Ready(None)) => {
                    return Ok(Async::Ready(()));
                }
                Ok(Async::Ready(Some(StreamLoaderCommand::Fetch(request)))) => {
                    self.download_range(request.start, request.length);
                }
                Ok(Async::Ready(Some(StreamLoaderCommand::RandomAccessMode()))) => {
                    self.download_strategy = DownloadStrategy::RandomAccess();
                }
                Ok(Async::Ready(Some(StreamLoaderCommand::StreamMode()))) => {
                    self.download_strategy = DownloadStrategy::Streaming();
                }
                Ok(Async::Ready(Some(StreamLoaderCommand::StreamDataRate(rate)))) => {
                    self.streaming_data_rate = rate;
                }
                Ok(Async::Ready(Some(StreamLoaderCommand::Close()))) => {
                    return Ok(Async::Ready(()));
                }
                Ok(Async::NotReady) => {
                    return Ok(Async::NotReady)
                },
                Err(()) => unreachable!(),
            }
        }

    }

    fn finish(&mut self) {
        trace!("====== FINISHED DOWNLOADING FILE! ======");
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

        match self.poll_stream_loader_command_rx() {
            Ok(Async::NotReady) => (),
            Ok(Async::Ready(_)) => {
                return Ok(Async::Ready(()));
            }
            Err(()) => unreachable!(),
        }

        match self.poll_file_data_rx() {
            Ok(Async::NotReady) => (),
            Ok(Async::Ready(_)) => {
                return Ok(Async::Ready(()));
            }
            Err(()) => unreachable!(),
        }


        if let DownloadStrategy::Streaming() = self.download_strategy {
            let bytes_pending: usize = {
                let download_status = self.shared.download_status.lock().unwrap();
                download_status.requested.minus(&download_status.downloaded).len()
            };

            let ping_time = self.shared.ping_time_ms.load(atomic::Ordering::Relaxed);

            if bytes_pending < 2 * ping_time * self.streaming_data_rate / 1000 {
                trace!("Prefetching more data. pending bytes({}) < 2 * ping time ({}) * data rate({}) / 1000.",bytes_pending, ping_time, self.streaming_data_rate);
                self.pre_fetch_more_data();
            }
        }


        return Ok(Async::NotReady)
    }
}

impl Read for AudioFileStreaming {
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        let offset = self.position as usize;

        if offset >= self.shared.file_size {
            return Ok(0);
        }

        let length = min(output.len(), self.shared.file_size - offset);

        if length == 0 {
            return Ok(0);
        }



        let mut ranges_to_request = RangeSet::new();
        ranges_to_request.add_range(&Range::new(offset, length));

        trace!("reading at postion {} (length : {})", offset, length);

        let mut download_status = self.shared.download_status.lock().unwrap();
        ranges_to_request.subtract_range_set(&download_status.downloaded);
        ranges_to_request.subtract_range_set(&download_status.requested);


        for range in ranges_to_request.iter() {
            trace!("requesting data at position {} (length : {})", range.start, range.length);
            self.stream_loader_command_tx.unbounded_send(StreamLoaderCommand::Fetch(range.clone())).unwrap();
        }

        while !download_status.downloaded.contains(offset) {
            trace!("waiting for download");
            download_status = self.shared.cond.wait_timeout(download_status, Duration::from_millis(1000)).unwrap().0;
            trace!("re-checking data availability at offset {}.", offset);
        }
        let available_length = download_status.downloaded.contained_length_from_value(offset);
        assert!(available_length > 0);
        drop(download_status);


        self.position = self.read_file.seek(SeekFrom::Start(offset as u64)).unwrap();
        let read_len = min(length, available_length);
        let read_len = try!(self.read_file.read(&mut output[..read_len]));

        trace!("read successfully at postion {} (length : {})", offset, read_len);

        self.position += read_len as u64;
        self.shared.read_position.store(self.position as usize, atomic::Ordering::Relaxed);


        return Ok(read_len);
    }
}

impl Seek for AudioFileStreaming {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.position = try!(self.read_file.seek(pos));
        // Do not seek past EOF
        self.shared.read_position.store(self.position as usize, atomic::Ordering::Relaxed);
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
