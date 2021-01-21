use crate::range_set::{Range, RangeSet};
use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use bytes::Bytes;
use futures::{
    channel::{mpsc, oneshot},
    future,
};
use futures::{Future, Stream, StreamExt, TryFutureExt, TryStreamExt};

use std::fs;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::sync::{Arc, Condvar, Mutex};
use std::task::Poll;
use std::time::{Duration, Instant};
use std::{
    cmp::{max, min},
    pin::Pin,
    task::Context,
};
use tempfile::NamedTempFile;

use futures::channel::mpsc::unbounded;
use librespot_core::channel::{Channel, ChannelData, ChannelError, ChannelHeaders};
use librespot_core::session::Session;
use librespot_core::spotify_id::FileId;
use std::sync::atomic;
use std::sync::atomic::AtomicUsize;

const MINIMUM_DOWNLOAD_SIZE: usize = 1024 * 16;
// The minimum size of a block that is requested from the Spotify servers in one request.
// This is the block size that is typically requested while doing a seek() on a file.
// Note: smaller requests can happen if part of the block is downloaded already.

const INITIAL_DOWNLOAD_SIZE: usize = 1024 * 16;
// The amount of data that is requested when initially opening a file.
// Note: if the file is opened to play from the beginning, the amount of data to
// read ahead is requested in addition to this amount. If the file is opened to seek to
// another position, then only this amount is requested on the first request.

const INITIAL_PING_TIME_ESTIMATE_SECONDS: f64 = 0.5;
// The pig time that is used for calculations before a ping time was actually measured.

const MAXIMUM_ASSUMED_PING_TIME_SECONDS: f64 = 1.5;
// If the measured ping time to the Spotify server is larger than this value, it is capped
// to avoid run-away block sizes and pre-fetching.

pub const READ_AHEAD_BEFORE_PLAYBACK_SECONDS: f64 = 1.0;
// Before playback starts, this many seconds of data must be present.
// Note: the calculations are done using the nominal bitrate of the file. The actual amount
// of audio data may be larger or smaller.

pub const READ_AHEAD_BEFORE_PLAYBACK_ROUNDTRIPS: f64 = 2.0;
// Same as READ_AHEAD_BEFORE_PLAYBACK_SECONDS, but the time is taken as a factor of the ping
// time to the Spotify server.
// Both, READ_AHEAD_BEFORE_PLAYBACK_SECONDS and READ_AHEAD_BEFORE_PLAYBACK_ROUNDTRIPS are
// obeyed.
// Note: the calculations are done using the nominal bitrate of the file. The actual amount
// of audio data may be larger or smaller.

pub const READ_AHEAD_DURING_PLAYBACK_SECONDS: f64 = 5.0;
// While playing back, this many seconds of data ahead of the current read position are
// requested.
// Note: the calculations are done using the nominal bitrate of the file. The actual amount
// of audio data may be larger or smaller.

pub const READ_AHEAD_DURING_PLAYBACK_ROUNDTRIPS: f64 = 10.0;
// Same as READ_AHEAD_DURING_PLAYBACK_SECONDS, but the time is taken as a factor of the ping
// time to the Spotify server.
// Note: the calculations are done using the nominal bitrate of the file. The actual amount
// of audio data may be larger or smaller.

const PREFETCH_THRESHOLD_FACTOR: f64 = 4.0;
// If the amount of data that is pending (requested but not received) is less than a certain amount,
// data is pre-fetched in addition to the read ahead settings above. The threshold for requesting more
// data is calculated as
// <pending bytes> < PREFETCH_THRESHOLD_FACTOR * <ping time> * <nominal data rate>

const FAST_PREFETCH_THRESHOLD_FACTOR: f64 = 1.5;
// Similar to PREFETCH_THRESHOLD_FACTOR, but it also takes the current download rate into account.
// The formula used is
// <pending bytes> < FAST_PREFETCH_THRESHOLD_FACTOR * <ping time> * <measured download rate>
// This mechanism allows for fast downloading of the remainder of the file. The number should be larger
// than 1 so the download rate ramps up until the bandwidth is saturated. The larger the value, the faster
// the download rate ramps up. However, this comes at the cost that it might hurt ping-time if a seek is
// performed while downloading. Values smaller than 1 cause the download rate to collapse and effectively
// only PREFETCH_THRESHOLD_FACTOR is in effect. Thus, set to zero if bandwidth saturation is not wanted.

const MAX_PREFETCH_REQUESTS: usize = 4;
// Limit the number of requests that are pending simultaneously before pre-fetching data. Pending
// requests share bandwidth. Thus, havint too many requests can lead to the one that is needed next
// for playback to be delayed leading to a buffer underrun. This limit has the effect that a new
// pre-fetch request is only sent if less than MAX_PREFETCH_REQUESTS are pending.

pub enum AudioFile {
    Cached(fs::File),
    Streaming(AudioFileStreaming),
}

enum StreamLoaderCommand {
    Fetch(Range),       // signal the stream loader to fetch a range of the file
    RandomAccessMode(), // optimise download strategy for random access
    StreamMode(),       // optimise download strategy for streaming
    Close(),            // terminate and don't load any more data
}

#[derive(Clone)]
pub struct StreamLoaderController {
    channel_tx: Option<mpsc::UnboundedSender<StreamLoaderCommand>>,
    stream_shared: Option<Arc<AudioFileShared>>,
    file_size: usize,
}

impl StreamLoaderController {
    pub fn len(&self) -> usize {
        self.file_size
    }

    pub fn is_empty(&self) -> bool {
        self.file_size == 0
    }

    pub fn range_available(&self, range: Range) -> bool {
        if let Some(ref shared) = self.stream_shared {
            let download_status = shared.download_status.lock().unwrap();
            range.length
                <= download_status
                    .downloaded
                    .contained_length_from_value(range.start)
        } else {
            range.length <= self.len() - range.start
        }
    }

    pub fn range_to_end_available(&self) -> bool {
        self.stream_shared.as_ref().map_or(true, |shared| {
            let read_position = shared.read_position.load(atomic::Ordering::Relaxed);
            self.range_available(Range::new(read_position, self.len() - read_position))
        })
    }

    pub fn ping_time_ms(&self) -> usize {
        self.stream_shared.as_ref().map_or(0, |shared| {
            shared.ping_time_ms.load(atomic::Ordering::Relaxed)
        })
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
            while range.length
                > download_status
                    .downloaded
                    .contained_length_from_value(range.start)
            {
                download_status = shared
                    .cond
                    .wait_timeout(download_status, Duration::from_millis(1000))
                    .unwrap()
                    .0;
                if range.length
                    > (download_status
                        .downloaded
                        .union(&download_status.requested)
                        .contained_length_from_value(range.start))
                {
                    // For some reason, the requested range is neither downloaded nor requested.
                    // This could be due to a network error. Request it again.
                    // We can't use self.fetch here because self can't be borrowed mutably, so we access the channel directly.
                    if let Some(ref mut channel) = self.channel_tx {
                        // ignore the error in case the channel has been closed already.
                        let _ = channel.unbounded_send(StreamLoaderCommand::Fetch(range));
                    }
                }
            }
        }
    }

    pub fn fetch_next(&mut self, length: usize) {
        if let Some(ref shared) = self.stream_shared {
            let range = Range {
                start: shared.read_position.load(atomic::Ordering::Relaxed),
                length: length,
            };
            self.fetch(range)
        }
    }

    pub fn fetch_next_blocking(&mut self, length: usize) {
        if let Some(ref shared) = self.stream_shared {
            let range = Range {
                start: shared.read_position.load(atomic::Ordering::Relaxed),
                length: length,
            };
            self.fetch_blocking(range);
        }
    }

    pub fn set_random_access_mode(&mut self) {
        // optimise download strategy for random access
        self.send_stream_loader_command(StreamLoaderCommand::RandomAccessMode());
    }

    pub fn set_stream_mode(&mut self) {
        // optimise download strategy for streaming
        self.send_stream_loader_command(StreamLoaderCommand::StreamMode());
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

#[derive(Copy, Clone)]
enum DownloadStrategy {
    RandomAccess(),
    Streaming(),
}

struct AudioFileShared {
    file_id: FileId,
    file_size: usize,
    stream_data_rate: usize,
    cond: Condvar,
    download_status: Mutex<AudioFileDownloadStatus>,
    download_strategy: Mutex<DownloadStrategy>,
    number_of_open_requests: AtomicUsize,
    ping_time_ms: AtomicUsize,
    read_position: AtomicUsize,
}

impl AudioFile {
    pub async fn open(
        session: &Session,
        file_id: FileId,
        bytes_per_second: usize,
        play_from_beginning: bool,
    ) -> Result<AudioFile, ChannelError> {
        if let Some(file) = session.cache().and_then(|cache| cache.file(file_id)) {
            debug!("File {} already in cache", file_id);
            return Ok(AudioFile::Cached(file));
        }

        debug!("Downloading file {}", file_id);

        let (complete_tx, complete_rx) = oneshot::channel();
        let mut initial_data_length = if play_from_beginning {
            INITIAL_DOWNLOAD_SIZE
                + max(
                    (READ_AHEAD_DURING_PLAYBACK_SECONDS * bytes_per_second as f64) as usize,
                    (INITIAL_PING_TIME_ESTIMATE_SECONDS
                        * READ_AHEAD_DURING_PLAYBACK_ROUNDTRIPS
                        * bytes_per_second as f64) as usize,
                )
        } else {
            INITIAL_DOWNLOAD_SIZE
        };
        if initial_data_length % 4 != 0 {
            initial_data_length += 4 - (initial_data_length % 4);
        }
        let (headers, data) = request_range(session, file_id, 0, initial_data_length).split();

        let streaming = AudioFileStreaming::open(
            session.clone(),
            data,
            initial_data_length,
            Instant::now(),
            headers,
            file_id,
            complete_tx,
            bytes_per_second,
        );

        let session_ = session.clone();
        session.spawn(complete_rx.map_ok(move |mut file| {
            if let Some(cache) = session_.cache() {
                cache.save_file(file_id, &mut file);
                debug!("File {} complete, saving to cache", file_id);
            } else {
                debug!("File {} complete", file_id);
            }
        }));

        Ok(AudioFile::Streaming(streaming.await?))
    }

    pub fn get_stream_loader_controller(&self) -> StreamLoaderController {
        match self {
            AudioFile::Streaming(ref stream) => StreamLoaderController {
                channel_tx: Some(stream.stream_loader_command_tx.clone()),
                stream_shared: Some(stream.shared.clone()),
                file_size: stream.shared.file_size,
            },
            AudioFile::Cached(ref file) => StreamLoaderController {
                channel_tx: None,
                stream_shared: None,
                file_size: file.metadata().unwrap().len() as usize,
            },
        }
    }
}

impl AudioFileStreaming {
    pub async fn open(
        session: Session,
        initial_data_rx: ChannelData,
        initial_data_length: usize,
        initial_request_sent_time: Instant,
        headers: ChannelHeaders,
        file_id: FileId,
        complete_tx: oneshot::Sender<NamedTempFile>,
        streaming_data_rate: usize,
    ) -> Result<AudioFileStreaming, ChannelError> {
        let (_, data) = headers
            .try_filter(|(id, _)| future::ready(*id == 0x3))
            .next()
            .await
            .unwrap()?;

        let size = BigEndian::read_u32(&data) as usize * 4;

        let shared = Arc::new(AudioFileShared {
            file_id: file_id,
            file_size: size,
            stream_data_rate: streaming_data_rate,
            cond: Condvar::new(),
            download_status: Mutex::new(AudioFileDownloadStatus {
                requested: RangeSet::new(),
                downloaded: RangeSet::new(),
            }),
            download_strategy: Mutex::new(DownloadStrategy::RandomAccess()), // start with random access mode until someone tells us otherwise
            number_of_open_requests: AtomicUsize::new(0),
            ping_time_ms: AtomicUsize::new(0),
            read_position: AtomicUsize::new(0),
        });

        let mut write_file = NamedTempFile::new().unwrap();
        write_file.as_file().set_len(size as u64).unwrap();
        write_file.seek(SeekFrom::Start(0)).unwrap();

        let read_file = write_file.reopen().unwrap();

        //let (seek_tx, seek_rx) = mpsc::unbounded();
        let (stream_loader_command_tx, stream_loader_command_rx) =
            mpsc::unbounded::<StreamLoaderCommand>();

        let fetcher = AudioFileFetch::new(
            session.clone(),
            shared.clone(),
            initial_data_rx,
            initial_request_sent_time,
            initial_data_length,
            write_file,
            stream_loader_command_rx,
            complete_tx,
        );

        session.spawn(fetcher);
        Ok(AudioFileStreaming {
            read_file: read_file,
            position: 0,
            //seek: seek_tx,
            stream_loader_command_tx: stream_loader_command_tx,
            shared: shared,
        })
    }
}

fn request_range(session: &Session, file: FileId, offset: usize, length: usize) -> Channel {
    assert!(
        offset % 4 == 0,
        "Range request start positions must be aligned by 4 bytes."
    );
    assert!(
        length % 4 == 0,
        "Range request range lengths must be aligned by 4 bytes."
    );
    let start = offset / 4;
    let end = (offset + length) / 4;

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

async fn audio_file_fetch_receive_data(
    shared: Arc<AudioFileShared>,
    file_data_tx: mpsc::UnboundedSender<ReceivedData>,
    data_rx: ChannelData,
    initial_data_offset: usize,
    initial_request_length: usize,
    request_sent_time: Instant,
) {
    let mut data_offset = initial_data_offset;
    let mut request_length = initial_request_length;
    let mut measure_ping_time = shared
        .number_of_open_requests
        .load(atomic::Ordering::SeqCst)
        == 0;

    shared
        .number_of_open_requests
        .fetch_add(1, atomic::Ordering::SeqCst);

    enum TryFoldErr {
        ChannelError,
        FinishEarly,
    }

    let result = data_rx
        .map_err(|_| TryFoldErr::ChannelError)
        .try_for_each(|data| {
            if measure_ping_time {
                let duration = Instant::now() - request_sent_time;
                let duration_ms: u64;
                if 0.001 * (duration.as_millis() as f64)
                    > MAXIMUM_ASSUMED_PING_TIME_SECONDS
                {
                    duration_ms = (MAXIMUM_ASSUMED_PING_TIME_SECONDS * 1000.0) as u64;
                } else {
                    duration_ms = duration.as_millis() as u64;
                }
                let _ = file_data_tx
                    .unbounded_send(ReceivedData::ResponseTimeMs(duration_ms as usize));
                measure_ping_time = false;
            }
            let data_size = data.len();
            let _ = file_data_tx
                .unbounded_send(ReceivedData::Data(PartialFileData {
                    offset: data_offset,
                    data: data,
                }));
            data_offset += data_size;
            if request_length < data_size {
                warn!("Data receiver for range {} (+{}) received more data from server than requested.", initial_data_offset, initial_request_length);
                request_length = 0;
            } else {
                request_length -= data_size;
            }

            future::ready(if request_length == 0 {
                Err(TryFoldErr::FinishEarly)
            } else { 
                Ok(()) 
            })
        })
        .await;

    if request_length > 0 {
        let missing_range = Range::new(data_offset, request_length);

        let mut download_status = shared.download_status.lock().unwrap();
        download_status.requested.subtract_range(&missing_range);
        shared.cond.notify_all();
    }

    shared
        .number_of_open_requests
        .fetch_sub(1, atomic::Ordering::SeqCst);

    if let Err(TryFoldErr::ChannelError) = result {
        warn!(
            "Error from channel for data receiver for range {} (+{}).",
            initial_data_offset, initial_request_length
        );
    } else if request_length > 0 {
        warn!(
            "Data receiver for range {} (+{}) received less data from server than requested.",
            initial_data_offset, initial_request_length
        );
    }
}
/* 
async fn audio_file_fetch(
    session: Session,
    shared: Arc<AudioFileShared>,
    initial_data_rx: ChannelData,
    initial_request_sent_time: Instant,
    initial_data_length: usize,

    output: NamedTempFile,
    stream_loader_command_rx: mpsc::UnboundedReceiver<StreamLoaderCommand>,
    complete_tx: oneshot::Sender<NamedTempFile>,
) {
    let (file_data_tx, file_data_rx) = unbounded::<ReceivedData>();

    let requested_range = Range::new(0, initial_data_length);
    let mut download_status = shared.download_status.lock().unwrap();
    download_status.requested.add_range(&requested_range);

    session.spawn(audio_file_fetch_receive_data(
        shared.clone(),
        file_data_tx.clone(),
        initial_data_rx,
        0,
        initial_data_length,
        initial_request_sent_time,
    ));

    let mut network_response_times_ms: Vec::new();

    let f1 = file_data_rx.map(|x| Ok::<_, ()>(x)).try_for_each(|x| {
        match x {
            ReceivedData::ResponseTimeMs(response_time_ms) => {
                trace!("Ping time estimated as: {} ms.", response_time_ms);

                // record the response time
                network_response_times_ms.push(response_time_ms);

                // prune old response times. Keep at most three.
                while network_response_times_ms.len() > 3 {
                    network_response_times_ms.remove(0);
                }

                // stats::median is experimental. So we calculate the median of up to three ourselves.
                let ping_time_ms: usize = match network_response_times_ms.len() {
                    1 => network_response_times_ms[0] as usize,
                    2 => {
                        ((network_response_times_ms[0] + network_response_times_ms[1]) / 2) as usize
                    }
                    3 => {
                        let mut times = network_response_times_ms.clone();
                        times.sort();
                        times[1]
                    }
                    _ => unreachable!(),
                };

                // store our new estimate for everyone to see
                shared
                    .ping_time_ms
                    .store(ping_time_ms, atomic::Ordering::Relaxed);
            }
            ReceivedData::Data(data) => {
                output
                    .as_mut()
                    .unwrap()
                    .seek(SeekFrom::Start(data.offset as u64))
                    .unwrap();
                output
                    .as_mut()
                    .unwrap()
                    .write_all(data.data.as_ref())
                    .unwrap();

                let mut full = false;

                {
                    let mut download_status = shared.download_status.lock().unwrap();

                    let received_range = Range::new(data.offset, data.data.len());
                    download_status.downloaded.add_range(&received_range);
                    shared.cond.notify_all();

                    if download_status.downloaded.contained_length_from_value(0)
                        >= shared.file_size
                    {
                        full = true;
                    }

                    drop(download_status);
                }

                if full {
                    self.finish();
                    return future::ready(Err(()));
                }
            }
        }
        future::ready(Ok(()))
    });

    let f2 = stream_loader_command_rx.map(Ok::<_, ()>).try_for_each(|x| {
        match cmd {
            StreamLoaderCommand::Fetch(request) => {
                self.download_range(request.start, request.length);
            }
            StreamLoaderCommand::RandomAccessMode() => {
                *(shared.download_strategy.lock().unwrap()) = DownloadStrategy::RandomAccess();
            }
            StreamLoaderCommand::StreamMode() => {
                *(shared.download_strategy.lock().unwrap()) = DownloadStrategy::Streaming();
            }
            StreamLoaderCommand::Close() => return future::ready(Err(())),
        }
        Ok(())
    });

    let f3 = future::poll_fn(|_| {
        if let DownloadStrategy::Streaming() = self.get_download_strategy() {
            let number_of_open_requests = shared
                .number_of_open_requests
                .load(atomic::Ordering::SeqCst);
            let max_requests_to_send =
                MAX_PREFETCH_REQUESTS - min(MAX_PREFETCH_REQUESTS, number_of_open_requests);

            if max_requests_to_send > 0 {
                let bytes_pending: usize = {
                    let download_status = shared.download_status.lock().unwrap();
                    download_status
                        .requested
                        .minus(&download_status.downloaded)
                        .len()
                };

                let ping_time_seconds =
                    0.001 * shared.ping_time_ms.load(atomic::Ordering::Relaxed) as f64;
                let download_rate = session.channel().get_download_rate_estimate();

                let desired_pending_bytes = max(
                    (PREFETCH_THRESHOLD_FACTOR * ping_time_seconds * shared.stream_data_rate as f64)
                        as usize,
                    (FAST_PREFETCH_THRESHOLD_FACTOR * ping_time_seconds * download_rate as f64)
                        as usize,
                );

                if bytes_pending < desired_pending_bytes {
                    self.pre_fetch_more_data(
                        desired_pending_bytes - bytes_pending,
                        max_requests_to_send,
                    );
                }
            }
        }
        Poll::Pending
    });
    future::select_all(vec![f1, f2, f3]).await
}*/

#[pin_project]
struct AudioFileFetch {
    session: Session,
    shared: Arc<AudioFileShared>,
    output: Option<NamedTempFile>,

    file_data_tx: mpsc::UnboundedSender<ReceivedData>,
    #[pin]
    file_data_rx: mpsc::UnboundedReceiver<ReceivedData>,

    #[pin]
    stream_loader_command_rx: mpsc::UnboundedReceiver<StreamLoaderCommand>,
    complete_tx: Option<oneshot::Sender<NamedTempFile>>,
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

        session.spawn(audio_file_fetch_receive_data(
            shared.clone(),
            file_data_tx.clone(),
            initial_data_rx,
            0,
            initial_data_length,
            initial_request_sent_time,
        ));

        AudioFileFetch {
            session: session,
            shared: shared,
            output: Some(output),

            file_data_tx: file_data_tx,
            file_data_rx: file_data_rx,

            stream_loader_command_rx: stream_loader_command_rx,
            complete_tx: Some(complete_tx),
            network_response_times_ms: Vec::new(),
        }
    }

    fn get_download_strategy(&mut self) -> DownloadStrategy {
        *(self.shared.download_strategy.lock().unwrap())
    }

    fn download_range(&mut self, mut offset: usize, mut length: usize) {
        if length < MINIMUM_DOWNLOAD_SIZE {
            length = MINIMUM_DOWNLOAD_SIZE;
        }

        // ensure the values are within the bounds and align them by 4 for the spotify protocol.
        if offset >= self.shared.file_size {
            return;
        }

        if length == 0 {
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
            let (_headers, data) = request_range(
                &self.session,
                self.shared.file_id,
                range.start,
                range.length,
            )
            .split();

            download_status.requested.add_range(range);

            self.session.spawn(audio_file_fetch_receive_data(
                self.shared.clone(),
                self.file_data_tx.clone(),
                data,
                range.start,
                range.length,
                Instant::now(),
            ));
        }
    }

    fn pre_fetch_more_data(&mut self, bytes: usize, max_requests_to_send: usize) {
        let mut bytes_to_go = bytes;
        let mut requests_to_go = max_requests_to_send;

        while bytes_to_go > 0 && requests_to_go > 0 {
            // determine what is still missing
            let mut missing_data = RangeSet::new();
            missing_data.add_range(&Range::new(0, self.shared.file_size));
            {
                let download_status = self.shared.download_status.lock().unwrap();
                missing_data.subtract_range_set(&download_status.downloaded);
                missing_data.subtract_range_set(&download_status.requested);
            }

            // download data from after the current read position first
            let mut tail_end = RangeSet::new();
            let read_position = self.shared.read_position.load(atomic::Ordering::Relaxed);
            tail_end.add_range(&Range::new(
                read_position,
                self.shared.file_size - read_position,
            ));
            let tail_end = tail_end.intersection(&missing_data);

            if !tail_end.is_empty() {
                let range = tail_end.get_range(0);
                let offset = range.start;
                let length = min(range.length, bytes_to_go);
                self.download_range(offset, length);
                requests_to_go -= 1;
                bytes_to_go -= length;
            } else if !missing_data.is_empty() {
                // ok, the tail is downloaded, download something fom the beginning.
                let range = missing_data.get_range(0);
                let offset = range.start;
                let length = min(range.length, bytes_to_go);
                self.download_range(offset, length);
                requests_to_go -= 1;
                bytes_to_go -= length;
            } else {
                return;
            }
        }
    }



    fn poll_file_data_rx(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        loop {
            match Pin::new(&mut self.file_data_rx).poll_next(cx) {
                Poll::Ready(None) => return Poll::Ready(()),
                Poll::Ready(Some(ReceivedData::ResponseTimeMs(response_time_ms))) => {
                    trace!("Ping time estimated as: {} ms.", response_time_ms);

                    // record the response time
                    self.network_response_times_ms.push(response_time_ms);

                    // prune old response times. Keep at most three.
                    while self.network_response_times_ms.len() > 3 {
                        self.network_response_times_ms.remove(0);
                    }

                    // stats::median is experimental. So we calculate the median of up to three ourselves.
                    let ping_time_ms: usize = match self.network_response_times_ms.len() {
                        1 => self.network_response_times_ms[0] as usize,
                        2 => {
                            ((self.network_response_times_ms[0]
                                + self.network_response_times_ms[1])
                                / 2) as usize
                        }
                        3 => {
                            let mut times = self.network_response_times_ms.clone();
                            times.sort_unstable();
                            times[1]
                        }
                        _ => unreachable!(),
                    };

                    // store our new estimate for everyone to see
                    self.shared
                        .ping_time_ms
                        .store(ping_time_ms, atomic::Ordering::Relaxed);
                }
                Poll::Ready(Some(ReceivedData::Data(data))) => {
                    self.output
                        .as_mut()
                        .unwrap()
                        .seek(SeekFrom::Start(data.offset as u64))
                        .unwrap();
                    self.output
                        .as_mut()
                        .unwrap()
                        .write_all(data.data.as_ref())
                        .unwrap();

                    let mut full = false;

                    {
                        let mut download_status = self.shared.download_status.lock().unwrap();

                        let received_range = Range::new(data.offset, data.data.len());
                        download_status.downloaded.add_range(&received_range);
                        self.shared.cond.notify_all();

                        if download_status.downloaded.contained_length_from_value(0)
                            >= self.shared.file_size
                        {
                            full = true;
                        }

                        drop(download_status);
                    }

                    if full {
                        self.finish();
                        return Poll::Ready(())
                    }
                }
                Poll::Pending => {
                    return Poll::Pending
                }
            }
        }
    }

    fn poll_stream_loader_command_rx(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        loop {
            match Pin::new(&mut self.stream_loader_command_rx).poll_next(cx) {
                Poll::Ready(None) => 
                    return Poll::Ready(()),
                Poll::Ready(Some(cmd)) => {
                    match cmd {
                        StreamLoaderCommand::Fetch(request) => {
                            self.download_range(request.start, request.length);
                        }
                        StreamLoaderCommand::RandomAccessMode() => {
                            *(self.shared.download_strategy.lock().unwrap()) =
                            DownloadStrategy::RandomAccess();
                        }
                        StreamLoaderCommand::StreamMode() => {

                            *(self.shared.download_strategy.lock().unwrap()) =
                            DownloadStrategy::Streaming();
                        }
                        StreamLoaderCommand::Close() => return Poll::Ready(())

                    }
                }
                Poll::Pending => return Poll::Pending
            }
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
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if let Poll::Ready(()) = self.poll_stream_loader_command_rx(cx) {
            return Poll::Ready(())
        }

        if let Poll::Ready(()) = self.poll_file_data_rx(cx) {
            return Poll::Ready(())
        }

        if let DownloadStrategy::Streaming() = self.get_download_strategy() {
            let number_of_open_requests = self
                .shared
                .number_of_open_requests
                .load(atomic::Ordering::SeqCst);
            let max_requests_to_send =
                MAX_PREFETCH_REQUESTS - min(MAX_PREFETCH_REQUESTS, number_of_open_requests);

            if max_requests_to_send > 0 {
                let bytes_pending: usize = {
                    let download_status = self.shared.download_status.lock().unwrap();
                    download_status
                        .requested
                        .minus(&download_status.downloaded)
                        .len()
                };

                let ping_time_seconds =
                    0.001 * self.shared.ping_time_ms.load(atomic::Ordering::Relaxed) as f64;
                let download_rate = self.session.channel().get_download_rate_estimate();

                let desired_pending_bytes = max(
                    (PREFETCH_THRESHOLD_FACTOR
                        * ping_time_seconds
                        * self.shared.stream_data_rate as f64) as usize,
                    (FAST_PREFETCH_THRESHOLD_FACTOR * ping_time_seconds * download_rate as f64)
                        as usize,
                );

                if bytes_pending < desired_pending_bytes {
                    self.pre_fetch_more_data(
                        desired_pending_bytes - bytes_pending,
                        max_requests_to_send,
                    );
                }
            }
        }
        Poll::Pending
    }
}

impl Read for AudioFileStreaming {
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        let offset = self.position as usize;

        if offset >= self.shared.file_size {
            return Ok(0);
        }

        let length = min(output.len(), self.shared.file_size - offset);

        let length_to_request = match *(self.shared.download_strategy.lock().unwrap()) {
            DownloadStrategy::RandomAccess() => length,
            DownloadStrategy::Streaming() => {
                // Due to the read-ahead stuff, we potentially request more than the actual reqeust demanded.
                let ping_time_seconds =
                    0.0001 * self.shared.ping_time_ms.load(atomic::Ordering::Relaxed) as f64;

                let length_to_request = length
                    + max(
                        (READ_AHEAD_DURING_PLAYBACK_SECONDS * self.shared.stream_data_rate as f64)
                            as usize,
                        (READ_AHEAD_DURING_PLAYBACK_ROUNDTRIPS
                            * ping_time_seconds
                            * self.shared.stream_data_rate as f64) as usize,
                    );
                min(length_to_request, self.shared.file_size - offset)
            }
        };

        let mut ranges_to_request = RangeSet::new();
        ranges_to_request.add_range(&Range::new(offset, length_to_request));

        let mut download_status = self.shared.download_status.lock().unwrap();
        ranges_to_request.subtract_range_set(&download_status.downloaded);
        ranges_to_request.subtract_range_set(&download_status.requested);

        for &range in ranges_to_request.iter() {
            self.stream_loader_command_tx
                .unbounded_send(StreamLoaderCommand::Fetch(range))
                .unwrap();
        }

        if length == 0 {
            return Ok(0);
        }

        let mut download_message_printed = false;
        while !download_status.downloaded.contains(offset) {
            if let DownloadStrategy::Streaming() = *self.shared.download_strategy.lock().unwrap() {
                if !download_message_printed {
                    debug!("Stream waiting for download of file position {}. Downloaded ranges: {}. Pending ranges: {}", offset, download_status.downloaded, download_status.requested.minus(&download_status.downloaded));
                    download_message_printed = true;
                }
            }
            download_status = self
                .shared
                .cond
                .wait_timeout(download_status, Duration::from_millis(1000))
                .unwrap()
                .0;
        }
        let available_length = download_status
            .downloaded
            .contained_length_from_value(offset);
        assert!(available_length > 0);
        drop(download_status);

        self.position = self.read_file.seek(SeekFrom::Start(offset as u64)).unwrap();
        let read_len = min(length, available_length);
        let read_len = self.read_file.read(&mut output[..read_len])?;

        if download_message_printed {
            debug!(
                "Read at postion {} completed. {} bytes returned, {} bytes were requested.",
                offset,
                read_len,
                output.len()
            );
        }

        self.position += read_len as u64;
        self.shared
            .read_position
            .store(self.position as usize, atomic::Ordering::Relaxed);

        Ok(read_len)
    }
}

impl Seek for AudioFileStreaming {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.position = self.read_file.seek(pos)?;
        // Do not seek past EOF
        self.shared
            .read_position
            .store(self.position as usize, atomic::Ordering::Relaxed);
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
