use std::{
    cmp::{max, min},
    io::{Seek, SeekFrom, Write},
    sync::{atomic, Arc},
    time::{Duration, Instant},
};

use atomic::Ordering;
use bytes::Bytes;
use futures_util::StreamExt;
use hyper::StatusCode;
use tempfile::NamedTempFile;
use tokio::sync::{mpsc, oneshot};

use librespot_core::{session::Session, Error};

use crate::range_set::{Range, RangeSet};

use super::{
    AudioFileError, AudioFileResult, AudioFileShared, DownloadStrategy, StreamLoaderCommand,
    StreamingRequest, FAST_PREFETCH_THRESHOLD_FACTOR, MAXIMUM_ASSUMED_PING_TIME,
    MAX_PREFETCH_REQUESTS, MINIMUM_DOWNLOAD_SIZE, PREFETCH_THRESHOLD_FACTOR,
};

struct PartialFileData {
    offset: usize,
    data: Bytes,
}

enum ReceivedData {
    ResponseTime(Duration),
    Data(PartialFileData),
}

async fn receive_data(
    shared: Arc<AudioFileShared>,
    file_data_tx: mpsc::UnboundedSender<ReceivedData>,
    mut request: StreamingRequest,
) -> AudioFileResult {
    let requested_offset = request.offset;
    let requested_length = request.length;

    let mut data_offset = requested_offset;
    let mut request_length = requested_length;

    let old_number_of_request = shared
        .number_of_open_requests
        .fetch_add(1, Ordering::SeqCst);

    let mut measure_ping_time = old_number_of_request == 0;

    let result: Result<_, Error> = loop {
        let response = match request.initial_response.take() {
            Some(data) => data,
            None => match request.streamer.next().await {
                Some(Ok(response)) => response,
                Some(Err(e)) => break Err(e.into()),
                None => break Ok(()),
            },
        };

        let code = response.status();
        let body = response.into_body();

        if code != StatusCode::PARTIAL_CONTENT {
            debug!("Streamer expected partial content but got: {}", code);
            break Err(AudioFileError::StatusCode(code).into());
        }

        let data = match hyper::body::to_bytes(body).await {
            Ok(bytes) => bytes,
            Err(e) => break Err(e.into()),
        };

        if measure_ping_time {
            let mut duration = Instant::now() - request.request_time;
            if duration > MAXIMUM_ASSUMED_PING_TIME {
                duration = MAXIMUM_ASSUMED_PING_TIME;
            }
            file_data_tx.send(ReceivedData::ResponseTime(duration))?;
            measure_ping_time = false;
        }

        let data_size = data.len();

        file_data_tx.send(ReceivedData::Data(PartialFileData {
            offset: data_offset,
            data,
        }))?;
        data_offset += data_size;
        if request_length < data_size {
            warn!(
                "Data receiver for range {} (+{}) received more data from server than requested ({} instead of {}).",
                requested_offset, requested_length, data_size, request_length
            );
            request_length = 0;
        } else {
            request_length -= data_size;
        }

        if request_length == 0 {
            break Ok(());
        }
    };

    drop(request.streamer);

    if request_length > 0 {
        let missing_range = Range::new(data_offset, request_length);

        let mut download_status = shared.download_status.lock();

        download_status.requested.subtract_range(&missing_range);
        shared.cond.notify_all();
    }

    shared
        .number_of_open_requests
        .fetch_sub(1, Ordering::SeqCst);

    match result {
        Ok(()) => {
            if request_length > 0 {
                warn!(
                    "Streamer for range {} (+{}) received less data from server than requested.",
                    requested_offset, requested_length
                );
            }
            Ok(())
        }
        Err(e) => {
            error!(
                "Error from streamer for range {} (+{}): {:?}",
                requested_offset, requested_length, e
            );
            Err(e)
        }
    }
}

struct AudioFileFetch {
    session: Session,
    shared: Arc<AudioFileShared>,
    output: Option<NamedTempFile>,

    file_data_tx: mpsc::UnboundedSender<ReceivedData>,
    complete_tx: Option<oneshot::Sender<NamedTempFile>>,
    network_response_times: Vec<Duration>,
}

// Might be replaced by enum from std once stable
#[derive(PartialEq, Eq)]
enum ControlFlow {
    Break,
    Continue,
}

impl AudioFileFetch {
    fn get_download_strategy(&mut self) -> DownloadStrategy {
        *(self.shared.download_strategy.lock())
    }

    fn download_range(&mut self, offset: usize, mut length: usize) -> AudioFileResult {
        if length < MINIMUM_DOWNLOAD_SIZE {
            length = MINIMUM_DOWNLOAD_SIZE;
        }

        if offset + length > self.shared.file_size {
            length = self.shared.file_size - offset;
        }

        let mut ranges_to_request = RangeSet::new();
        ranges_to_request.add_range(&Range::new(offset, length));

        let mut download_status = self.shared.download_status.lock();

        ranges_to_request.subtract_range_set(&download_status.downloaded);
        ranges_to_request.subtract_range_set(&download_status.requested);

        for range in ranges_to_request.iter() {
            let url = self.shared.cdn_url.try_get_url()?;

            let streamer = self
                .session
                .spclient()
                .stream_file(url, range.start, range.length)?;

            download_status.requested.add_range(range);

            let streaming_request = StreamingRequest {
                streamer,
                initial_response: None,
                offset: range.start,
                length: range.length,
                request_time: Instant::now(),
            };

            self.session.spawn(receive_data(
                self.shared.clone(),
                self.file_data_tx.clone(),
                streaming_request,
            ));
        }

        Ok(())
    }

    fn pre_fetch_more_data(
        &mut self,
        bytes: usize,
        max_requests_to_send: usize,
    ) -> AudioFileResult {
        let mut bytes_to_go = bytes;
        let mut requests_to_go = max_requests_to_send;

        while bytes_to_go > 0 && requests_to_go > 0 {
            // determine what is still missing
            let mut missing_data = RangeSet::new();
            missing_data.add_range(&Range::new(0, self.shared.file_size));
            {
                let download_status = self.shared.download_status.lock();

                missing_data.subtract_range_set(&download_status.downloaded);
                missing_data.subtract_range_set(&download_status.requested);
            }

            // download data from after the current read position first
            let mut tail_end = RangeSet::new();
            let read_position = self.shared.read_position.load(Ordering::Relaxed);
            tail_end.add_range(&Range::new(
                read_position,
                self.shared.file_size - read_position,
            ));
            let tail_end = tail_end.intersection(&missing_data);

            if !tail_end.is_empty() {
                let range = tail_end.get_range(0);
                let offset = range.start;
                let length = min(range.length, bytes_to_go);
                self.download_range(offset, length)?;
                requests_to_go -= 1;
                bytes_to_go -= length;
            } else if !missing_data.is_empty() {
                // ok, the tail is downloaded, download something fom the beginning.
                let range = missing_data.get_range(0);
                let offset = range.start;
                let length = min(range.length, bytes_to_go);
                self.download_range(offset, length)?;
                requests_to_go -= 1;
                bytes_to_go -= length;
            } else {
                break;
            }
        }

        Ok(())
    }

    fn handle_file_data(&mut self, data: ReceivedData) -> Result<ControlFlow, Error> {
        match data {
            ReceivedData::ResponseTime(response_time) => {
                let old_ping_time_ms = self.shared.ping_time_ms.load(Ordering::Relaxed);

                // prune old response times. Keep at most two so we can push a third.
                while self.network_response_times.len() >= 3 {
                    self.network_response_times.remove(0);
                }

                // record the response time
                self.network_response_times.push(response_time);

                // stats::median is experimental. So we calculate the median of up to three ourselves.
                let ping_time_ms = {
                    let response_time = match self.network_response_times.len() {
                        1 => self.network_response_times[0],
                        2 => (self.network_response_times[0] + self.network_response_times[1]) / 2,
                        3 => {
                            let mut times = self.network_response_times.clone();
                            times.sort_unstable();
                            times[1]
                        }
                        _ => unreachable!(),
                    };
                    response_time.as_millis() as usize
                };

                // print when the new estimate deviates by more than 10% from the last
                if f32::abs(
                    (ping_time_ms as f32 - old_ping_time_ms as f32) / old_ping_time_ms as f32,
                ) > 0.1
                {
                    debug!("Ping time now estimated as: {} ms", ping_time_ms);
                }

                // store our new estimate for everyone to see
                self.shared
                    .ping_time_ms
                    .store(ping_time_ms, Ordering::Relaxed);
            }
            ReceivedData::Data(data) => {
                match self.output.as_mut() {
                    Some(output) => {
                        output.seek(SeekFrom::Start(data.offset as u64))?;
                        output.write_all(data.data.as_ref())?;
                    }
                    None => return Err(AudioFileError::Output.into()),
                }

                let mut download_status = self.shared.download_status.lock();

                let received_range = Range::new(data.offset, data.data.len());
                download_status.downloaded.add_range(&received_range);
                self.shared.cond.notify_all();

                let full = download_status.downloaded.contained_length_from_value(0)
                    >= self.shared.file_size;

                drop(download_status);

                if full {
                    self.finish()?;
                    return Ok(ControlFlow::Break);
                }
            }
        }

        Ok(ControlFlow::Continue)
    }

    fn handle_stream_loader_command(
        &mut self,
        cmd: StreamLoaderCommand,
    ) -> Result<ControlFlow, Error> {
        match cmd {
            StreamLoaderCommand::Fetch(request) => {
                self.download_range(request.start, request.length)?;
            }
            StreamLoaderCommand::RandomAccessMode() => {
                *(self.shared.download_strategy.lock()) = DownloadStrategy::RandomAccess();
            }
            StreamLoaderCommand::StreamMode() => {
                *(self.shared.download_strategy.lock()) = DownloadStrategy::Streaming();
            }
            StreamLoaderCommand::Close() => return Ok(ControlFlow::Break),
        }

        Ok(ControlFlow::Continue)
    }

    fn finish(&mut self) -> AudioFileResult {
        let output = self.output.take();

        let complete_tx = self.complete_tx.take();

        if let Some(mut output) = output {
            output.seek(SeekFrom::Start(0))?;
            if let Some(complete_tx) = complete_tx {
                complete_tx
                    .send(output)
                    .map_err(|_| AudioFileError::Channel)?;
            }
        }

        Ok(())
    }
}

pub(super) async fn audio_file_fetch(
    session: Session,
    shared: Arc<AudioFileShared>,
    initial_request: StreamingRequest,
    output: NamedTempFile,
    mut stream_loader_command_rx: mpsc::UnboundedReceiver<StreamLoaderCommand>,
    complete_tx: oneshot::Sender<NamedTempFile>,
) -> AudioFileResult {
    let (file_data_tx, mut file_data_rx) = mpsc::unbounded_channel();

    {
        let requested_range = Range::new(
            initial_request.offset,
            initial_request.offset + initial_request.length,
        );
        let mut download_status = shared.download_status.lock();

        download_status.requested.add_range(&requested_range);
    }

    session.spawn(receive_data(
        shared.clone(),
        file_data_tx.clone(),
        initial_request,
    ));

    let mut fetch = AudioFileFetch {
        session: session.clone(),
        shared,
        output: Some(output),

        file_data_tx,
        complete_tx: Some(complete_tx),
        network_response_times: Vec::with_capacity(3),
    };

    loop {
        tokio::select! {
            cmd = stream_loader_command_rx.recv() => {
                match cmd {
                        Some(cmd) => {
                            if fetch.handle_stream_loader_command(cmd)? == ControlFlow::Break {
                                break;
                            }
                        }
                        None => break,
                    }
                }
                data = file_data_rx.recv() => {
                match data {
                    Some(data) => {
                        if fetch.handle_file_data(data)? == ControlFlow::Break {
                            break;
                        }
                    }
                    None => break,
                }
            }
        }

        if fetch.get_download_strategy() == DownloadStrategy::Streaming() {
            let number_of_open_requests =
                fetch.shared.number_of_open_requests.load(Ordering::SeqCst);
            if number_of_open_requests < MAX_PREFETCH_REQUESTS {
                let max_requests_to_send = MAX_PREFETCH_REQUESTS - number_of_open_requests;

                let bytes_pending: usize = {
                    let download_status = fetch.shared.download_status.lock();

                    download_status
                        .requested
                        .minus(&download_status.downloaded)
                        .len()
                };

                let ping_time_seconds =
                    Duration::from_millis(fetch.shared.ping_time_ms.load(Ordering::Relaxed) as u64)
                        .as_secs_f32();
                let download_rate = fetch.session.channel().get_download_rate_estimate();

                let desired_pending_bytes = max(
                    (PREFETCH_THRESHOLD_FACTOR
                        * ping_time_seconds
                        * fetch.shared.bytes_per_second as f32) as usize,
                    (FAST_PREFETCH_THRESHOLD_FACTOR * ping_time_seconds * download_rate as f32)
                        as usize,
                );

                if bytes_pending < desired_pending_bytes {
                    fetch.pre_fetch_more_data(
                        desired_pending_bytes - bytes_pending,
                        max_requests_to_send,
                    )?;
                }
            }
        }
    }

    Ok(())
}
