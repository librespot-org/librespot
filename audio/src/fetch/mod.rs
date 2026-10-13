mod receive;

use std::{
    cmp::min,
    fs,
    io::{self, Read, Seek, SeekFrom},
    sync::{
        Arc, OnceLock,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
    sync::{Condvar, Mutex},
    time::Duration,
};

use futures_util::{StreamExt, TryFutureExt, future::IntoStream};
use hyper::{Response, StatusCode, body::Incoming, header::CONTENT_RANGE};
use hyper_util::client::legacy::ResponseFuture;

use tempfile::NamedTempFile;
use thiserror::Error;
use tokio::sync::{Semaphore, mpsc, oneshot};

use librespot_core::{Error, FileId, Session, cdn_url::CdnUrl};

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

/// The CDN URLs of a file, starting with the one in use: a failed download moves on to the next.
struct CdnUrls {
    urls: Vec<String>,
    current: AtomicUsize,
}

impl CdnUrls {
    fn new(urls: Vec<String>) -> Self {
        Self {
            urls,
            current: AtomicUsize::new(0),
        }
    }

    fn current(&self) -> &str {
        self.urls
            .get(self.current.load(Ordering::Acquire))
            .map_or("", String::as_str)
    }

    /// Move on to the next URL, if there is another; returns the hosts switched from and to.
    fn switch(&self) -> Option<(&str, &str)> {
        if self.urls.len() < 2 {
            return None;
        }
        let from = self.current.load(Ordering::Acquire);
        let to = (from + 1) % self.urls.len();
        self.current.store(to, Ordering::Release);
        Some((host(&self.urls[from]), host(&self.urls[to])))
    }
}

/// The host of a URL, for logging without the URL's token.
fn host(url: &str) -> &str {
    let rest = url.split_once("://").map_or(url, |(_, rest)| rest);
    rest.split(['/', '?']).next().unwrap_or(rest)
}

struct AudioFileShared {
    cdn_urls: CdnUrls,
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
        if let Some(file) = session.cache().and_then(|cache| cache.file(file_id)) {
            debug!("File {file_id} already in cache");
            return Ok(AudioFile::Cached(file));
        }

        debug!("Downloading file {file_id}");

        let (complete_tx, complete_rx) = oneshot::channel();

        let streaming =
            AudioFileStreaming::open(session.clone(), file_id, complete_tx, bytes_per_second);

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
    ) -> Result<AudioFileStreaming, Error> {
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

        let header_value = response
            .headers()
            .get(CONTENT_RANGE)
            .ok_or(AudioFileError::Header)?;
        let str_value = header_value.to_str()?;
        let hyphen_index = str_value.find('-').unwrap_or_default();
        let slash_index = str_value.find('/').unwrap_or_default();
        let upper_bound: usize = str_value[hyphen_index + 1..slash_index].parse()?;
        let file_size = str_value[slash_index + 1..].parse()?;

        let initial_request = StreamingRequest {
            streamer,
            initial_response: Some(response),
            offset: 0,
            length: upper_bound + 1,
        };

        let shared = Arc::new(AudioFileShared {
            // the URL that answered first, then the others in their order
            cdn_urls: CdnUrls::new(
                std::iter::once(url.to_string())
                    .chain(urls.iter().filter(|u| *u != url).map(ToString::to_string))
                    .collect(),
            ),
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
        });

        let write_file = NamedTempFile::new_in(session.config().tmp_dir.clone())?;
        write_file.as_file().set_len(file_size as u64)?;

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

/// How often `read` requests a range again after its download failed, per call.
const MAX_READ_RETRIES: usize = 3;

/// The part of `wanted` that is neither downloaded nor requested.
fn missing_ranges(download_status: &AudioFileDownloadStatus, wanted: Range) -> RangeSet {
    let mut missing = RangeSet::new();
    missing.add_range(&wanted);
    missing.subtract_range_set(&download_status.downloaded);
    missing.subtract_range_set(&download_status.requested);
    missing
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

        let wanted = Range::new(offset, length_to_request);

        let mut download_status = self
            .shared
            .download_status
            .lock()
            .expect(DOWNLOAD_STATUS_POISON_MSG);

        for &range in missing_ranges(&download_status, wanted).iter() {
            self.stream_loader_command_tx
                .send(StreamLoaderCommand::Fetch(range))
                .map_err(|err| io::Error::new(io::ErrorKind::BrokenPipe, err))?;
        }

        let download_timeout = AudioFetchParams::get().download_timeout;
        let mut retries = 0;
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

            // A request that failed (e.g. a connection the CDN had closed) gives its range
            // back as neither downloaded nor requested and wakes us up: request it again,
            // as `fetch_blocking` does, instead of waiting for data that will never come.
            if retries < MAX_READ_RETRIES {
                let missing = missing_ranges(&download_status, wanted);
                if !missing.is_empty() {
                    retries += 1;
                    // the CDN that failed may keep failing for this range: try another one
                    match self.shared.cdn_urls.switch() {
                        Some((from, to)) => warn!(
                            "Re-requesting {missing} from {to} after a failed download from {from} (retry {retries})"
                        ),
                        None => warn!(
                            "Re-requesting {missing} after a failed download (retry {retries})"
                        ),
                    }
                    for &range in missing.iter() {
                        self.stream_loader_command_tx
                            .send(StreamLoaderCommand::Fetch(range))
                            .map_err(|err| io::Error::new(io::ErrorKind::BrokenPipe, err))?;
                    }
                }
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

    fn status(
        requested: &[(usize, usize)],
        downloaded: &[(usize, usize)],
    ) -> AudioFileDownloadStatus {
        let mut s = AudioFileDownloadStatus {
            requested: RangeSet::new(),
            downloaded: RangeSet::new(),
        };
        for &(start, length) in requested {
            s.requested.add_range(&Range::new(start, length));
        }
        for &(start, length) in downloaded {
            s.downloaded.add_range(&Range::new(start, length));
        }
        s
    }

    /// A stream loader that behaves like `audio_file_fetch` + `receive_data`, except that the
    /// first request fails the way a dropped CDN connection does: its range is marked
    /// requested, then given back as not requested, and readers are woken up.  Every later
    /// request is served.
    fn flaky_loader(
        shared: Arc<AudioFileShared>,
        mut commands: mpsc::UnboundedReceiver<StreamLoaderCommand>,
    ) -> std::thread::JoinHandle<usize> {
        std::thread::spawn(move || {
            let mut fetches = 0;
            while let Some(StreamLoaderCommand::Fetch(range)) = commands.blocking_recv() {
                fetches += 1;
                let mut status = shared.download_status.lock().unwrap();
                status.requested.add_range(&range);
                if fetches == 1 {
                    // receive_data's error path
                    status.requested.subtract_range(&range);
                } else {
                    status.downloaded.add_range(&range);
                }
                shared.cond.notify_all();
            }
            fetches
        })
    }

    /// A loader for which every request to host `a` fails and every other one is served; it
    /// returns the host of each request.
    fn loader_with_a_failing_host(
        shared: Arc<AudioFileShared>,
        mut commands: mpsc::UnboundedReceiver<StreamLoaderCommand>,
    ) -> std::thread::JoinHandle<Vec<String>> {
        std::thread::spawn(move || {
            let mut hosts = Vec::new();
            while let Some(StreamLoaderCommand::Fetch(range)) = commands.blocking_recv() {
                let h = host(shared.cdn_urls.current()).to_string();
                let mut status = shared.download_status.lock().unwrap();
                if h == "a" {
                    // receive_data's error path: the range is handed back
                    status.requested.add_range(&range);
                    status.requested.subtract_range(&range);
                } else {
                    status.downloaded.add_range(&range);
                }
                hosts.push(h);
                shared.cond.notify_all();
            }
            hosts
        })
    }

    #[test]
    fn a_failed_download_moves_on_to_the_next_cdn() {
        let _ = AudioFetchParams::set(AudioFetchParams {
            download_timeout: Duration::from_secs(2),
            ..Default::default()
        });
        let data: Vec<u8> = (0..256 * 1024).map(|i| (i % 251) as u8).collect();
        let mut file = NamedTempFile::new().unwrap();
        std::io::Write::write_all(&mut file, &data).unwrap();
        let shared = Arc::new(AudioFileShared {
            cdn_urls: CdnUrls::new(vec![
                "https://a/audio/x?token".into(),
                "https://b/audio/x?token".into(),
            ]),
            file_size: data.len(),
            bytes_per_second: 40 * 1024,
            cond: Condvar::new(),
            download_status: Mutex::new(AudioFileDownloadStatus {
                requested: RangeSet::new(),
                downloaded: RangeSet::new(),
            }),
            download_streaming: AtomicBool::new(true),
            download_slots: Semaphore::new(1),
            ping_time_ms: AtomicUsize::new(0),
            read_position: AtomicUsize::new(0),
            throughput: AtomicUsize::new(0),
        });
        let (tx, rx) = mpsc::unbounded_channel();
        let loader = loader_with_a_failing_host(shared.clone(), rx);
        let mut audio = AudioFileStreaming {
            read_file: file.reopen().unwrap(),
            position: 0,
            stream_loader_command_tx: tx,
            shared,
        };

        let mut buf = vec![0u8; 4096];
        let n = audio
            .read(&mut buf)
            .expect("read() should get the range from the other CDN");
        assert_eq!(&buf[..n], &data[..n]);

        drop(audio);
        assert_eq!(loader.join().unwrap(), ["a", "b"]);
    }

    #[test]
    fn switching_cdns_cycles_and_logs_only_hosts() {
        let urls = CdnUrls::new(vec![
            "https://audio-fa.scdn.co/audio/x?token".into(),
            "https://audio-ak.scdn.co/audio/x?token".into(),
        ]);
        assert_eq!(
            urls.switch(),
            Some(("audio-fa.scdn.co", "audio-ak.scdn.co"))
        );
        assert_eq!(urls.current(), "https://audio-ak.scdn.co/audio/x?token");
        assert_eq!(
            urls.switch(),
            Some(("audio-ak.scdn.co", "audio-fa.scdn.co"))
        );
        assert_eq!(CdnUrls::new(vec!["https://a/x".into()]).switch(), None);
    }

    #[test]
    fn read_requests_a_failed_range_again() {
        // fail fast instead of after the default 8s
        let _ = AudioFetchParams::set(AudioFetchParams {
            download_timeout: Duration::from_secs(2),
            ..Default::default()
        });
        let data: Vec<u8> = (0..256 * 1024).map(|i| (i % 251) as u8).collect();
        let mut file = NamedTempFile::new().unwrap();
        std::io::Write::write_all(&mut file, &data).unwrap();
        let shared = Arc::new(AudioFileShared {
            cdn_urls: CdnUrls::new(Vec::new()),
            file_size: data.len(),
            bytes_per_second: 40 * 1024,
            cond: Condvar::new(),
            download_status: Mutex::new(AudioFileDownloadStatus {
                requested: RangeSet::new(),
                downloaded: RangeSet::new(),
            }),
            download_streaming: AtomicBool::new(true),
            download_slots: Semaphore::new(1),
            ping_time_ms: AtomicUsize::new(0),
            read_position: AtomicUsize::new(0),
            throughput: AtomicUsize::new(0),
        });
        let (tx, rx) = mpsc::unbounded_channel();
        let loader = flaky_loader(shared.clone(), rx);
        let mut audio = AudioFileStreaming {
            read_file: file.reopen().unwrap(),
            position: 0,
            stream_loader_command_tx: tx,
            shared,
        };

        let mut buf = vec![0u8; 4096];
        let n = audio
            .read(&mut buf)
            .expect("read() should request the failed range again, not time out");
        assert!(n > 0);
        assert_eq!(&buf[..n], &data[..n]);

        drop(audio);
        assert!(loader.join().unwrap() >= 2, "the range was requested again");
    }

    #[test]
    fn nothing_missing_while_a_request_is_in_flight() {
        let s = status(&[(0, 100)], &[(0, 40)]);
        assert!(missing_ranges(&s, Range::new(0, 100)).is_empty());
    }

    #[test]
    fn a_failed_request_leaves_its_rest_missing() {
        // receive_data failed after 40 of 100 bytes: the rest is given back
        let mut s = status(&[(0, 100)], &[(0, 40)]);
        s.requested.subtract_range(&Range::new(40, 60));
        let missing = missing_ranges(&s, Range::new(0, 100));
        assert_eq!(missing.len(), 60);
        assert!(missing.contains(40) && missing.contains(99) && !missing.contains(39));
    }

    #[test]
    fn only_the_wanted_range_is_requested_again() {
        let s = status(&[], &[(0, 10)]);
        let missing = missing_ranges(&s, Range::new(5, 20));
        assert_eq!(missing.len(), 15);
        assert!(missing.contains(10) && missing.contains(24) && !missing.contains(25));
    }
}
