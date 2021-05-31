mod receive;

use std::cmp::{max, min};
use std::fs;
use std::io::{self, Read, Seek, SeekFrom};
use std::sync::atomic::{self, AtomicUsize};
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

use byteorder::{BigEndian, ByteOrder};
use futures_util::{future, StreamExt, TryFutureExt, TryStreamExt};
use librespot_core::channel::{ChannelData, ChannelError, ChannelHeaders};
use librespot_core::session::Session;
use librespot_core::spotify_id::FileId;
use tempfile::NamedTempFile;
use tokio::sync::{mpsc, oneshot};

use self::receive::{audio_file_fetch, request_range};
use crate::range_set::{Range, RangeSet};

/// The minimum size of a block that is requested from the Spotify servers in one request.
/// This is the block size that is typically requested while doing a `seek()` on a file.
/// Note: smaller requests can happen if part of the block is downloaded already.
const MINIMUM_DOWNLOAD_SIZE: usize = 1024 * 16;

/// The amount of data that is requested when initially opening a file.
/// Note: if the file is opened to play from the beginning, the amount of data to
/// read ahead is requested in addition to this amount. If the file is opened to seek to
/// another position, then only this amount is requested on the first request.
const INITIAL_DOWNLOAD_SIZE: usize = 1024 * 16;

/// The ping time that is used for calculations before a ping time was actually measured.
const INITIAL_PING_TIME_ESTIMATE: Duration = Duration::from_millis(500);

/// If the measured ping time to the Spotify server is larger than this value, it is capped
/// to avoid run-away block sizes and pre-fetching.
const MAXIMUM_ASSUMED_PING_TIME: Duration = Duration::from_millis(1500);

/// Before playback starts, this many seconds of data must be present.
/// Note: the calculations are done using the nominal bitrate of the file. The actual amount
/// of audio data may be larger or smaller.
pub const READ_AHEAD_BEFORE_PLAYBACK: Duration = Duration::from_secs(1);

/// Same as `READ_AHEAD_BEFORE_PLAYBACK`, but the time is taken as a factor of the ping
/// time to the Spotify server. Both `READ_AHEAD_BEFORE_PLAYBACK` and
/// `READ_AHEAD_BEFORE_PLAYBACK_ROUNDTRIPS` are obeyed.
/// Note: the calculations are done using the nominal bitrate of the file. The actual amount
/// of audio data may be larger or smaller.
pub const READ_AHEAD_BEFORE_PLAYBACK_ROUNDTRIPS: f32 = 2.0;

/// While playing back, this many seconds of data ahead of the current read position are
/// requested.
/// Note: the calculations are done using the nominal bitrate of the file. The actual amount
/// of audio data may be larger or smaller.
pub const READ_AHEAD_DURING_PLAYBACK: Duration = Duration::from_secs(5);

/// Same as `READ_AHEAD_DURING_PLAYBACK`, but the time is taken as a factor of the ping
/// time to the Spotify server.
/// Note: the calculations are done using the nominal bitrate of the file. The actual amount
/// of audio data may be larger or smaller.
pub const READ_AHEAD_DURING_PLAYBACK_ROUNDTRIPS: f32 = 10.0;

/// If the amount of data that is pending (requested but not received) is less than a certain amount,
/// data is pre-fetched in addition to the read ahead settings above. The threshold for requesting more
/// data is calculated as `<pending bytes> < PREFETCH_THRESHOLD_FACTOR * <ping time> * <nominal data rate>`
const PREFETCH_THRESHOLD_FACTOR: f32 = 4.0;

/// Similar to `PREFETCH_THRESHOLD_FACTOR`, but it also takes the current download rate into account.
/// The formula used is `<pending bytes> < FAST_PREFETCH_THRESHOLD_FACTOR * <ping time> * <measured download rate>`
/// This mechanism allows for fast downloading of the remainder of the file. The number should be larger
/// than `1.0` so the download rate ramps up until the bandwidth is saturated. The larger the value, the faster
/// the download rate ramps up. However, this comes at the cost that it might hurt ping time if a seek is
/// performed while downloading. Values smaller than `1.0` cause the download rate to collapse and effectively
/// only `PREFETCH_THRESHOLD_FACTOR` is in effect. Thus, set to `0.0` if bandwidth saturation is not wanted.
const FAST_PREFETCH_THRESHOLD_FACTOR: f32 = 1.5;

/// Limit the number of requests that are pending simultaneously before pre-fetching data. Pending
/// requests share bandwidth. Thus, havint too many requests can lead to the one that is needed next
/// for playback to be delayed leading to a buffer underrun. This limit has the effect that a new
/// pre-fetch request is only sent if less than `MAX_PREFETCH_REQUESTS` are pending.
const MAX_PREFETCH_REQUESTS: usize = 4;

/// The time we will wait to obtain status updates on downloading.
const DOWNLOAD_TIMEOUT: Duration = Duration::from_secs(1);

pub enum AudioFile {
    Cached(fs::File),
    Streaming(AudioFileStreaming),
}

#[derive(Debug)]
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

    pub fn ping_time(&self) -> Duration {
        Duration::from_millis(self.stream_shared.as_ref().map_or(0, |shared| {
            shared.ping_time_ms.load(atomic::Ordering::Relaxed) as u64
        }))
    }

    fn send_stream_loader_command(&self, command: StreamLoaderCommand) {
        if let Some(ref channel) = self.channel_tx {
            // ignore the error in case the channel has been closed already.
            let _ = channel.send(command);
        }
    }

    pub fn fetch(&self, range: Range) {
        // signal the stream loader to fetch a range of the file
        self.send_stream_loader_command(StreamLoaderCommand::Fetch(range));
    }

    pub fn fetch_blocking(&self, mut range: Range) {
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
                    .wait_timeout(download_status, DOWNLOAD_TIMEOUT)
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
                    self.fetch(range);
                }
            }
        }
    }

    pub fn fetch_next(&self, length: usize) {
        if let Some(ref shared) = self.stream_shared {
            let range = Range {
                start: shared.read_position.load(atomic::Ordering::Relaxed),
                length,
            };
            self.fetch(range)
        }
    }

    pub fn fetch_next_blocking(&self, length: usize) {
        if let Some(ref shared) = self.stream_shared {
            let range = Range {
                start: shared.read_position.load(atomic::Ordering::Relaxed),
                length,
            };
            self.fetch_blocking(range);
        }
    }

    pub fn set_random_access_mode(&self) {
        // optimise download strategy for random access
        self.send_stream_loader_command(StreamLoaderCommand::RandomAccessMode());
    }

    pub fn set_stream_mode(&self) {
        // optimise download strategy for streaming
        self.send_stream_loader_command(StreamLoaderCommand::StreamMode());
    }

    pub fn close(&self) {
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

#[derive(Copy, Clone, PartialEq, Eq)]
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
                    (READ_AHEAD_DURING_PLAYBACK.as_secs_f32() * bytes_per_second as f32) as usize,
                    (INITIAL_PING_TIME_ESTIMATE.as_secs_f32()
                        * READ_AHEAD_DURING_PLAYBACK_ROUNDTRIPS
                        * bytes_per_second as f32) as usize,
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
                debug!("File {} complete, saving to cache", file_id);
                cache.save_file(file_id, &mut file);
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

    pub fn is_cached(&self) -> bool {
        matches!(self, AudioFile::Cached { .. })
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
            file_id,
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

        // let (seek_tx, seek_rx) = mpsc::unbounded();
        let (stream_loader_command_tx, stream_loader_command_rx) =
            mpsc::unbounded_channel::<StreamLoaderCommand>();

        session.spawn(audio_file_fetch(
            session.clone(),
            shared.clone(),
            initial_data_rx,
            initial_request_sent_time,
            initial_data_length,
            write_file,
            stream_loader_command_rx,
            complete_tx,
        ));

        Ok(AudioFileStreaming {
            read_file,
            position: 0,
            stream_loader_command_tx,
            shared,
        })
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
                // Due to the read-ahead stuff, we potentially request more than the actual request demanded.
                let ping_time_seconds = Duration::from_millis(
                    self.shared.ping_time_ms.load(atomic::Ordering::Relaxed) as u64,
                )
                .as_secs_f32();

                let length_to_request = length
                    + max(
                        (READ_AHEAD_DURING_PLAYBACK.as_secs_f32()
                            * self.shared.stream_data_rate as f32) as usize,
                        (READ_AHEAD_DURING_PLAYBACK_ROUNDTRIPS
                            * ping_time_seconds
                            * self.shared.stream_data_rate as f32) as usize,
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
                .send(StreamLoaderCommand::Fetch(range))
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
                .wait_timeout(download_status, DOWNLOAD_TIMEOUT)
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
