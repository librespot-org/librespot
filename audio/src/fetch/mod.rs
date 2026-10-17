mod receive;

use std::{
    cmp::min,
    fs,
    io::{self, Read, Seek, SeekFrom, Write},
    sync::{
        Arc, OnceLock,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
    sync::{Condvar, Mutex},
    time::Duration,
};

use bytes::Bytes;
use futures_util::{StreamExt, TryFutureExt, future::IntoStream};
use http_body_util::BodyExt;
use hyper::{Response, StatusCode, body::Incoming, header::CONTENT_RANGE};
use hyper_util::client::legacy::ResponseFuture;

use tempfile::NamedTempFile;
use thiserror::Error;
use tokio::sync::{Semaphore, mpsc, oneshot};

use librespot_core::{
    Error, FileId, Session,
    cdn_url::{CdnUrl, CdnUrlError},
};

use self::receive::audio_file_fetch;

use crate::range_set::{Range, RangeSet};

pub type AudioFileResult = Result<(), librespot_core::Error>;

const DOWNLOAD_STATUS_POISON_MSG: &str = "audio download status mutex should not be poisoned";

#[derive(Error, Debug)]
pub enum AudioFileError {
    #[error("other end of channel disconnected")]
    Channel,
    #[error("required header not found")]
    Header,
    #[error("streamer received no data")]
    NoData,
    #[error("no output available")]
    Output,
    #[error("invalid status code {0}")]
    StatusCode(StatusCode),
    #[error("wait timeout exceeded")]
    WaitTimeout,
}

impl From<AudioFileError> for Error {
    fn from(err: AudioFileError) -> Self {
        match err {
            AudioFileError::Channel => Error::aborted(err),
            AudioFileError::Header => Error::unavailable(err),
            AudioFileError::NoData => Error::unavailable(err),
            AudioFileError::Output => Error::aborted(err),
            AudioFileError::StatusCode(_) => Error::failed_precondition(err),
            AudioFileError::WaitTimeout => Error::deadline_exceeded(err),
        }
    }
}

#[derive(Clone)]
pub struct AudioFetchParams {
    /// The minimum size of a block that is requested from the Spotify servers in one request.
    /// This is the block size that is typically requested while doing a `seek()` on a file.
    /// The Symphonia decoder requires this to be a power of 2 and > 32 kB.
    /// Note: smaller requests can happen if part of the block is downloaded already.
    pub minimum_download_size: usize,

    /// The minimum network throughput that we expect. Together with the minimum download size,
    /// this will determine the time we will wait for a response.
    pub minimum_throughput: usize,

    /// The ping time that is used for calculations before a ping time was actually measured.
    pub initial_ping_time_estimate: Duration,

    /// If the measured ping time to the Spotify server is larger than this value, it is capped
    /// to avoid run-away block sizes and pre-fetching.
    pub maximum_assumed_ping_time: Duration,

    /// Before playback starts, this many seconds of data must be present.
    /// Note: the calculations are done using the nominal bitrate of the file. The actual amount
    /// of audio data may be larger or smaller.
    pub read_ahead_before_playback: Duration,

    /// While playing back, this many seconds of data ahead of the current read position are
    /// requested.
    /// Note: the calculations are done using the nominal bitrate of the file. The actual amount
    /// of audio data may be larger or smaller.
    pub read_ahead_during_playback: Duration,

    /// If the amount of data that is pending (requested but not received) is less than a certain amount,
    /// data is pre-fetched in addition to the read ahead settings above. The threshold for requesting more
    /// data is calculated as `<pending bytes> < PREFETCH_THRESHOLD_FACTOR * <ping time> * <nominal data rate>`
    pub prefetch_threshold_factor: f32,

    /// The time we will wait to obtain status updates on downloading.
    pub download_timeout: Duration,
}

impl Default for AudioFetchParams {
    fn default() -> Self {
        let minimum_download_size = 64 * 1024;
        let minimum_throughput = 8 * 1024;
        Self {
            minimum_download_size,
            minimum_throughput,
            initial_ping_time_estimate: Duration::from_millis(500),
            maximum_assumed_ping_time: Duration::from_millis(1500),
            read_ahead_before_playback: Duration::from_secs(1),
            read_ahead_during_playback: Duration::from_secs(5),
            prefetch_threshold_factor: 4.0,
            download_timeout: Duration::from_secs(
                (minimum_download_size / minimum_throughput) as u64,
            ),
        }
    }
}

static AUDIO_FETCH_PARAMS: OnceLock<AudioFetchParams> = OnceLock::new();

impl AudioFetchParams {
    pub fn set(params: AudioFetchParams) -> Result<(), AudioFetchParams> {
        AUDIO_FETCH_PARAMS.set(params)
    }

    pub fn get() -> &'static AudioFetchParams {
        AUDIO_FETCH_PARAMS.get_or_init(AudioFetchParams::default)
    }
}

pub enum AudioFile {
    Cached(fs::File),
    Streaming(AudioFileStreaming),
}

/// The first bytes of an audio file, fetched ahead of time so that opening
/// the file later needs no round trip to the CDN before decoding can start.
///
/// The bytes are the file as the CDN serves it, still encrypted. A head only
/// helps while the CDN URL it came from is valid; see [`AudioFileHead::is_fresh`].
#[derive(Clone)]
pub struct AudioFileHead {
    file_id: FileId,
    cdn_url: CdnUrl,
    url: String,
    file_size: usize,
    data: Bytes,
}

impl std::fmt::Debug for AudioFileHead {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudioFileHead")
            .field("file_id", &self.file_id)
            .field("file_size", &self.file_size)
            .field("length", &self.data.len())
            .finish_non_exhaustive()
    }
}

impl AudioFileHead {
    /// Resolves the file's CDN URL and fetches its first `length` bytes.
    pub async fn fetch(session: &Session, file_id: FileId, length: usize) -> Result<Self, Error> {
        let cdn_url = CdnUrl::new(file_id).resolve_audio(session).await?;
        let urls = cdn_url.try_get_urls()?;
        let mut last_error = None;
        for url in &urls {
            match Self::fetch_from(session, url, length).await {
                Ok((file_size, data)) => {
                    return Ok(Self {
                        file_id,
                        url: url.to_string(),
                        cdn_url: cdn_url.clone(),
                        file_size,
                        data,
                    });
                }
                Err(e) => {
                    warn!("Fetching the head of {file_id} from {url} failed with error {e:?}");
                    last_error = Some(e);
                }
            }
        }
        Err(last_error.unwrap_or_else(|| CdnUrlError::Unresolved.into()))
    }

    async fn fetch_from(
        session: &Session,
        url: &str,
        length: usize,
    ) -> Result<(usize, Bytes), Error> {
        let mut streamer = session.spclient().stream_from_cdn(url, 0, length)?;
        let response = tokio::time::timeout(Duration::from_secs(10), streamer.next())
            .await
            .map_err(|_| Error::from(AudioFileError::WaitTimeout))?
            .ok_or(AudioFileError::NoData)??;
        if response.status() != StatusCode::PARTIAL_CONTENT {
            return Err(AudioFileError::StatusCode(response.status()).into());
        }
        let (_, file_size) = content_range(&response)?;
        let data = response
            .into_body()
            .collect()
            .await
            .map(http_body_util::Collected::to_bytes)?;
        if data.is_empty() || data.len() > file_size {
            return Err(AudioFileError::NoData.into());
        }
        Ok((file_size, data))
    }

    pub fn file_id(&self) -> FileId {
        self.file_id
    }

    /// The number of bytes held.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Whether the CDN URL the rest of the file would come from is still
    /// valid, so that the head can be used to open the file.
    pub fn is_fresh(&self) -> bool {
        self.cdn_url
            .try_get_urls()
            .is_ok_and(|urls| urls.contains(&self.url.as_str()))
    }
}

/// The last byte a `206 Partial Content` response holds and the size of the
/// whole file, from its `Content-Range` header.
fn content_range(response: &Response<Incoming>) -> Result<(usize, usize), Error> {
    let header_value = response
        .headers()
        .get(CONTENT_RANGE)
        .ok_or(AudioFileError::Header)?;
    let str_value = header_value.to_str()?;
    let hyphen_index = str_value.find('-').unwrap_or_default();
    let slash_index = str_value.find('/').unwrap_or_default();
    let upper_bound: usize = str_value[hyphen_index + 1..slash_index].parse()?;
    let file_size = str_value[slash_index + 1..].parse()?;
    Ok((upper_bound, file_size))
}

#[derive(Debug)]
pub struct StreamingRequest {
    streamer: IntoStream<ResponseFuture>,
    initial_response: Option<Response<Incoming>>,
    offset: usize,
    length: usize,
}

#[derive(Debug)]
pub enum StreamLoaderCommand {
    Fetch(Range), // signal the stream loader to fetch a range of the file
    Close,        // terminate and don't load any more data
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
            let download_status = shared
                .download_status
                .lock()
                .expect(DOWNLOAD_STATUS_POISON_MSG);

            range.length
                <= download_status
                    .downloaded
                    .contained_length_from_value(range.start)
        } else {
            range.length <= self.len() - range.start
        }
    }

    pub fn range_to_end_available(&self) -> bool {
        match self.stream_shared {
            Some(ref shared) => {
                let read_position = shared.read_position();
                self.range_available(Range::new(read_position, self.len() - read_position))
            }
            None => true,
        }
    }

    pub fn ping_time(&self) -> Option<Duration> {
        self.stream_shared.as_ref().map(|shared| shared.ping_time())
    }

    fn send_stream_loader_command(&self, command: StreamLoaderCommand) {
        if let Some(ref channel) = self.channel_tx {
            // Ignore the error in case the channel has been closed already.
            // This means that the file was completely downloaded.
            let _ = channel.send(command);
        }
    }

    pub fn fetch(&self, range: Range) {
        // signal the stream loader to fetch a range of the file
        self.send_stream_loader_command(StreamLoaderCommand::Fetch(range));
    }

    pub fn fetch_blocking(&self, mut range: Range) -> AudioFileResult {
        // signal the stream loader to tech a range of the file and block until it is loaded.

        // ensure the range is within the file's bounds.
        if range.start >= self.len() {
            range.length = 0;
        } else if range.end() > self.len() {
            range.length = self.len() - range.start;
        }

        self.fetch(range);

        if let Some(ref shared) = self.stream_shared {
            let mut download_status = shared
                .download_status
                .lock()
                .expect(DOWNLOAD_STATUS_POISON_MSG);
            let download_timeout = AudioFetchParams::get().download_timeout;

            while range.length
                > download_status
                    .downloaded
                    .contained_length_from_value(range.start)
            {
                let (new_download_status, wait_result) = shared
                    .cond
                    .wait_timeout(download_status, download_timeout)
                    .expect(DOWNLOAD_STATUS_POISON_MSG);

                download_status = new_download_status;
                if wait_result.timed_out() {
                    return Err(AudioFileError::WaitTimeout.into());
                }

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

        Ok(())
    }

    pub fn fetch_next_and_wait(
        &self,
        request_length: usize,
        wait_length: usize,
    ) -> AudioFileResult {
        match self.stream_shared {
            Some(ref shared) => {
                let start = shared.read_position();

                let request_range = Range {
                    start,
                    length: request_length,
                };
                self.fetch(request_range);

                let wait_range = Range {
                    start,
                    length: wait_length,
                };
                self.fetch_blocking(wait_range)
            }
            None => Ok(()),
        }
    }

    pub fn set_random_access_mode(&self) {
        // optimise download strategy for random access
        if let Some(ref shared) = self.stream_shared {
            shared.set_download_streaming(false)
        }
    }

    pub fn set_stream_mode(&self) {
        // optimise download strategy for streaming
        if let Some(ref shared) = self.stream_shared {
            shared.set_download_streaming(true)
        }
    }

    pub fn close(&self) {
        // terminate stream loading and don't load any more data for this file.
        self.send_stream_loader_command(StreamLoaderCommand::Close);
    }

    pub fn from_local_file(file_size: u64) -> Self {
        Self {
            channel_tx: None,
            stream_shared: None,
            file_size: file_size as usize,
        }
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
    cdn_url: String,
    file_size: usize,
    bytes_per_second: usize,
    cond: Condvar,
    download_status: Mutex<AudioFileDownloadStatus>,
    download_streaming: AtomicBool,
    download_slots: Semaphore,
    ping_time_ms: AtomicUsize,
    read_position: AtomicUsize,
    throughput: AtomicUsize,
}

impl AudioFileShared {
    fn new(cdn_url: String, file_size: usize, bytes_per_second: usize) -> Self {
        Self {
            cdn_url,
            file_size,
            bytes_per_second,
            cond: Condvar::new(),
            download_status: Mutex::new(AudioFileDownloadStatus {
                requested: RangeSet::new(),
                downloaded: RangeSet::new(),
            }),
            download_streaming: AtomicBool::new(false),
            download_slots: Semaphore::new(1),
            ping_time_ms: AtomicUsize::new(0),
            read_position: AtomicUsize::new(0),
            throughput: AtomicUsize::new(0),
        }
    }

    fn is_download_streaming(&self) -> bool {
        self.download_streaming.load(Ordering::Acquire)
    }

    fn set_download_streaming(&self, streaming: bool) {
        self.download_streaming.store(streaming, Ordering::Release)
    }

    fn ping_time(&self) -> Duration {
        let ping_time_ms = self.ping_time_ms.load(Ordering::Acquire);
        if ping_time_ms > 0 {
            Duration::from_millis(ping_time_ms as u64)
        } else {
            AudioFetchParams::get().initial_ping_time_estimate
        }
    }

    fn set_ping_time(&self, duration: Duration) {
        self.ping_time_ms
            .store(duration.as_millis() as usize, Ordering::Release)
    }

    fn throughput(&self) -> usize {
        self.throughput.load(Ordering::Acquire)
    }

    fn set_throughput(&self, throughput: usize) {
        self.throughput.store(throughput, Ordering::Release)
    }

    fn read_position(&self) -> usize {
        self.read_position.load(Ordering::Acquire)
    }

    fn set_read_position(&self, position: u64) {
        self.read_position
            .store(position as usize, Ordering::Release)
    }
}

impl AudioFile {
    pub async fn open(
        session: &Session,
        file_id: FileId,
        bytes_per_second: usize,
    ) -> Result<AudioFile, Error> {
        Self::open_with_head(session, file_id, bytes_per_second, None).await
    }

    /// Opens the file like [`AudioFile::open`], starting from a head fetched
    /// earlier when it belongs to this file and its CDN URL is still valid.
    /// The download then continues after the head, so the file can be read
    /// from the start without waiting for the CDN.
    pub async fn open_with_head(
        session: &Session,
        file_id: FileId,
        bytes_per_second: usize,
        head: Option<AudioFileHead>,
    ) -> Result<AudioFile, Error> {
        if let Some(file) = session.cache().and_then(|cache| cache.file(file_id)) {
            debug!("File {file_id} already in cache");
            return Ok(AudioFile::Cached(file));
        }

        debug!("Downloading file {file_id}");

        let (complete_tx, complete_rx) = oneshot::channel();

        let head = head.filter(|head| {
            // A head that is the whole file leaves nothing to download, and
            // finishing the download is what writes the file to the cache.
            head.file_id == file_id && head.len() < head.file_size && head.is_fresh()
        });
        let streaming = AudioFileStreaming::open(
            session.clone(),
            file_id,
            complete_tx,
            bytes_per_second,
            head,
        );

        let session_ = session.clone();
        session.spawn(complete_rx.map_ok(move |mut file| {
            debug!("Downloading file {file_id} complete");

            if let Some(cache) = session_.cache() {
                if let Some(cache_id) = cache.file_path(file_id) {
                    if let Err(e) = cache.save_file(file_id, &mut file) {
                        error!("Error caching file {file_id} to {cache_id:?}: {e}");
                    } else {
                        debug!("File {file_id} cached to {cache_id:?}");
                    }
                }
            }
        }));

        Ok(AudioFile::Streaming(streaming.await?))
    }

    pub fn get_stream_loader_controller(&self) -> Result<StreamLoaderController, Error> {
        let controller = match self {
            AudioFile::Streaming(stream) => StreamLoaderController {
                channel_tx: Some(stream.stream_loader_command_tx.clone()),
                stream_shared: Some(stream.shared.clone()),
                file_size: stream.shared.file_size,
            },
            AudioFile::Cached(file) => StreamLoaderController {
                channel_tx: None,
                stream_shared: None,
                file_size: file.metadata()?.len() as usize,
            },
        };

        Ok(controller)
    }

    pub fn is_cached(&self) -> bool {
        matches!(self, AudioFile::Cached { .. })
    }
}

impl AudioFileStreaming {
    pub async fn open(
        session: Session,
        file_id: FileId,
        complete_tx: oneshot::Sender<NamedTempFile>,
        bytes_per_second: usize,
        head: Option<AudioFileHead>,
    ) -> Result<AudioFileStreaming, Error> {
        if let Some(head) = head {
            debug!(
                "Starting file {file_id} from a prefetched head of {} bytes",
                head.len()
            );
            return Self::open_after_head(session, complete_tx, bytes_per_second, head);
        }

        let cdn_url = CdnUrl::new(file_id).resolve_audio(&session).await?;

        let minimum_download_size = AudioFetchParams::get().minimum_download_size;

        let mut response_streamer_url = None;
        let urls = cdn_url.try_get_urls()?;
        for url in &urls {
            // When the audio file is really small, this `download_size` may turn out to be
            // larger than the audio file we're going to stream later on. This is OK; requesting
            // `Content-Range` > `Content-Length` will return the complete file with status code
            // 206 Partial Content.
            let mut streamer =
                session
                    .spclient()
                    .stream_from_cdn(*url, 0, minimum_download_size)?;

            // Get the first chunk with the headers to get the file size.
            // The remainder of that chunk with possibly also a response body is then
            // further processed in `audio_file_fetch`.
            let streamer_result = tokio::time::timeout(Duration::from_secs(10), streamer.next())
                .await
                .map_err(|_| AudioFileError::WaitTimeout.into())
                .and_then(|x| x.ok_or_else(|| AudioFileError::NoData.into()))
                .and_then(|x| x.map_err(Error::from));

            match streamer_result {
                Ok(r) if r.status() == StatusCode::PARTIAL_CONTENT => {
                    response_streamer_url = Some((r, streamer, url));
                    break;
                }
                Ok(r) => warn!(
                    "Fetching {url} returned {} (expected 206 Partial Content), trying next",
                    r.status()
                ),
                Err(e) => warn!("Fetching {url} failed with error {e:?}, trying next"),
            }
        }

        let Some((response, streamer, url)) = response_streamer_url else {
            return Err(Error::unavailable(format!(
                "{} URLs failed, none left to try",
                urls.len()
            )));
        };

        trace!("Streaming from {url}");

        let (upper_bound, file_size) = content_range(&response)?;

        let initial_request = StreamingRequest {
            streamer,
            initial_response: Some(response),
            offset: 0,
            length: upper_bound + 1,
        };

        let shared = Arc::new(AudioFileShared::new(
            url.to_string(),
            file_size,
            bytes_per_second,
        ));

        let write_file = NamedTempFile::new_in(session.config().tmp_dir.clone())?;
        write_file.as_file().set_len(file_size as u64)?;

        Self::start(session, shared, initial_request, write_file, complete_tx)
    }

    /// Opens the file from a head held in memory: the head is written to the
    /// file and counted as downloaded, and the download continues after it.
    fn open_after_head(
        session: Session,
        complete_tx: oneshot::Sender<NamedTempFile>,
        bytes_per_second: usize,
        head: AudioFileHead,
    ) -> Result<AudioFileStreaming, Error> {
        let AudioFileHead {
            url,
            file_size,
            data,
            ..
        } = head;
        let shared = Arc::new(AudioFileShared::new(url, file_size, bytes_per_second));

        let mut write_file = NamedTempFile::new_in(session.config().tmp_dir.clone())?;
        write_file.as_file().set_len(file_size as u64)?;
        write_file.write_all(&data)?;
        shared
            .download_status
            .lock()
            .expect(DOWNLOAD_STATUS_POISON_MSG)
            .downloaded
            .add_range(&Range::new(0, data.len()));

        let offset = data.len();
        let length = min(
            AudioFetchParams::get().minimum_download_size,
            file_size - offset,
        );
        let initial_request = StreamingRequest {
            streamer: session
                .spclient()
                .stream_from_cdn(&shared.cdn_url, offset, length)?,
            initial_response: None,
            offset,
            length,
        };

        Self::start(session, shared, initial_request, write_file, complete_tx)
    }

    fn start(
        session: Session,
        shared: Arc<AudioFileShared>,
        initial_request: StreamingRequest,
        write_file: NamedTempFile,
        complete_tx: oneshot::Sender<NamedTempFile>,
    ) -> Result<AudioFileStreaming, Error> {
        let read_file = write_file.reopen()?;

        let (stream_loader_command_tx, stream_loader_command_rx) =
            mpsc::unbounded_channel::<StreamLoaderCommand>();

        session.spawn(audio_file_fetch(
            session.clone(),
            shared.clone(),
            initial_request,
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
        if length == 0 {
            return Ok(0);
        }

        let read_ahead_during_playback = AudioFetchParams::get().read_ahead_during_playback;
        let length_to_request = if self.shared.is_download_streaming() {
            let length_to_request = length
                + (read_ahead_during_playback.as_secs_f32() * self.shared.bytes_per_second as f32)
                    as usize;

            // Due to the read-ahead stuff, we potentially request more than the actual request demanded.
            min(length_to_request, self.shared.file_size - offset)
        } else {
            length
        };

        let mut ranges_to_request = RangeSet::new();
        ranges_to_request.add_range(&Range::new(offset, length_to_request));

        let mut download_status = self
            .shared
            .download_status
            .lock()
            .expect(DOWNLOAD_STATUS_POISON_MSG);

        ranges_to_request.subtract_range_set(&download_status.downloaded);
        ranges_to_request.subtract_range_set(&download_status.requested);

        for &range in ranges_to_request.iter() {
            self.stream_loader_command_tx
                .send(StreamLoaderCommand::Fetch(range))
                .map_err(|err| io::Error::new(io::ErrorKind::BrokenPipe, err))?;
        }

        let download_timeout = AudioFetchParams::get().download_timeout;
        while !download_status.downloaded.contains(offset) {
            let (new_download_status, wait_result) = self
                .shared
                .cond
                .wait_timeout(download_status, download_timeout)
                .expect(DOWNLOAD_STATUS_POISON_MSG);

            download_status = new_download_status;
            if wait_result.timed_out() {
                return Err(io::Error::new(
                    io::ErrorKind::TimedOut,
                    Error::deadline_exceeded(AudioFileError::WaitTimeout),
                ));
            }
        }
        let available_length = download_status
            .downloaded
            .contained_length_from_value(offset);

        drop(download_status);

        self.position = self.read_file.seek(SeekFrom::Start(offset as u64))?;
        let read_len = min(length, available_length);
        let read_len = self.read_file.read(&mut output[..read_len])?;

        self.position += read_len as u64;
        self.shared.set_read_position(self.position);

        Ok(read_len)
    }
}

impl Seek for AudioFileStreaming {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        // If we are already at this position, we don't need to switch download mode.
        // These checks and locks are less expensive than interrupting streaming.
        let current_position = self.position as i64;
        let requested_pos = match pos {
            SeekFrom::Start(pos) => pos as i64,
            SeekFrom::End(pos) => self.shared.file_size as i64 - pos - 1,
            SeekFrom::Current(pos) => current_position + pos,
        };
        if requested_pos == current_position {
            return Ok(current_position as u64);
        }

        // Again if we have already downloaded this part.
        let available = self
            .shared
            .download_status
            .lock()
            .expect(DOWNLOAD_STATUS_POISON_MSG)
            .downloaded
            .contains(requested_pos as usize);

        let mut was_streaming = false;
        if !available {
            // Ensure random access mode if we need to download this part.
            // Checking whether we are streaming now is a micro-optimization
            // to save an atomic load.
            was_streaming = self.shared.is_download_streaming();
            if was_streaming {
                self.shared.set_download_streaming(false);
            }
        }

        self.position = self.read_file.seek(pos)?;
        self.shared.set_read_position(self.position);

        if !available && was_streaming {
            self.shared.set_download_streaming(true);
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

#[cfg(test)]
mod tests {
    use super::*;
    use librespot_core::SessionConfig;

    #[tokio::test]
    async fn a_prefetched_head_reads_without_waiting_for_the_cdn() {
        let session = Session::new(SessionConfig::default(), None);
        let file_id = FileId::from_raw(&[7; 20]);
        let data: Vec<u8> = (0..64 * 1024).map(|i| (i % 251) as u8).collect();
        let head = AudioFileHead {
            file_id,
            cdn_url: CdnUrl::new(file_id),
            // Nothing listens here, so the rest of the file never arrives.
            url: "http://127.0.0.1:9/audio".into(),
            file_size: 256 * 1024,
            data: Bytes::from(data.clone()),
        };
        let (complete_tx, _complete_rx) = oneshot::channel();
        let mut file =
            AudioFileStreaming::open_after_head(session, complete_tx, 40 * 1024, head).unwrap();

        let mut read = vec![0; data.len()];
        let mut filled = 0;
        while filled < read.len() {
            let n = file.read(&mut read[filled..]).unwrap();
            assert!(n > 0);
            filled += n;
        }
        assert_eq!(read, data);

        let status = file.shared.download_status.lock().unwrap();
        assert_eq!(status.downloaded.contained_length_from_value(0), data.len());
    }

    #[test]
    fn an_unresolved_head_is_not_fresh() {
        let file_id = FileId::from_raw(&[7; 20]);
        let head = AudioFileHead {
            file_id,
            cdn_url: CdnUrl::new(file_id),
            url: "https://audio.example/file".into(),
            file_size: 2,
            data: Bytes::from_static(b"x"),
        };
        assert!(!head.is_fresh());
    }
}
