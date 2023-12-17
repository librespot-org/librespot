use std::{
    cmp::{max, min},
    io::{Seek, SeekFrom, Write},
    sync::Arc,
    time::{Duration, Instant},
};

use bytes::Bytes;
use futures_util::StreamExt;
use hyper::StatusCode;
use tempfile::NamedTempFile;
use tokio::sync::{mpsc, oneshot};

use librespot_core::{http_client::HttpClient, session::Session, Error};

use crate::range_set::{Range, RangeSet};

use super::{
    AudioFetchParams, AudioFileError, AudioFileResult, AudioFileShared, StreamLoaderCommand,
    StreamingRequest,
};

struct PartialFileData {
    offset: usize,
    data: Bytes,
}

enum ReceivedData {
    Throughput(usize),
    ResponseTime(Duration),
    Data(PartialFileData),
}

const ONE_SECOND: Duration = Duration::from_secs(1);

async fn receive_data(
    shared: Arc<AudioFileShared>,
    file_data_tx: mpsc::UnboundedSender<ReceivedData>,
    mut request: StreamingRequest,
) -> AudioFileResult {
    let mut offset = request.offset;
    let mut actual_length = 0;

    let permit = shared.download_slots.acquire().await?;

    let request_time = Instant::now();
    let mut measure_ping_time = true;
    let mut measure_throughput = true;

    let result: Result<_, Error> = loop {
        let response = match request.initial_response.take() {
            Some(data) => {
                // the request was already made outside of this function
                measure_ping_time = false;
                measure_throughput = false;

                data
            }
            None => match request.streamer.next().await {
                Some(Ok(response)) => response,
                Some(Err(e)) => break Err(e.into()),
                None => {
                    if actual_length != request.length {
                        let msg = format!("did not expect body to contain {actual_length} bytes");
                        break Err(Error::data_loss(msg));
                    }

                    break Ok(());
                }
            },
        };

        if measure_ping_time {
            let duration = Instant::now().duration_since(request_time);
            // may be zero if we are handling an initial response
            if duration.as_millis() > 0 {
                file_data_tx.send(ReceivedData::ResponseTime(duration))?;
                measure_ping_time = false;
            }
        }

        let code = response.status();
        if code != StatusCode::PARTIAL_CONTENT {
            if code == StatusCode::TOO_MANY_REQUESTS {
                if let Some(duration) = HttpClient::get_retry_after(response.headers()) {
                    warn!(
                        "Rate limiting, retrying in {} seconds...",
                        duration.as_secs()
                    );
                    // sleeping here means we hold onto this streamer "slot"
                    // (we don't decrease the number of open requests)
                    tokio::time::sleep(duration).await;
                }
            }

            break Err(AudioFileError::StatusCode(code).into());
        }

        let body = response.into_body();
        let data = match hyper::body::to_bytes(body).await {
            Ok(bytes) => bytes,
            Err(e) => break Err(e.into()),
        };

        let data_size = data.len();
        file_data_tx.send(ReceivedData::Data(PartialFileData { offset, data }))?;

        actual_length += data_size;
        offset += data_size;
    };

    drop(request.streamer);

    if measure_throughput {
        let duration = Instant::now().duration_since(request_time).as_millis();
        if actual_length > 0 && duration > 0 {
            let throughput = ONE_SECOND.as_millis() as usize * actual_length / duration as usize;
            file_data_tx.send(ReceivedData::Throughput(throughput))?;
        }
    }

    let bytes_remaining = request.length - actual_length;
    if bytes_remaining > 0 {
        {
            let missing_range = Range::new(offset, bytes_remaining);
            let mut download_status = shared.download_status.lock();
            download_status.requested.subtract_range(&missing_range);
            shared.cond.notify_all();
        }
    }

    drop(permit);

    if let Err(e) = result {
        error!(
            "Streamer error requesting range {} +{}: {:?}",
            request.offset, request.length, e
        );
        return Err(e);
    }

    Ok(())
}

struct AudioFileFetch {
    session: Session,
    shared: Arc<AudioFileShared>,
    output: Option<NamedTempFile>,

    file_data_tx: mpsc::UnboundedSender<ReceivedData>,
    complete_tx: Option<oneshot::Sender<NamedTempFile>>,
    network_response_times: Vec<Duration>,

    params: AudioFetchParams,
}

// Might be replaced by enum from std once stable
#[derive(PartialEq, Eq)]
enum ControlFlow {
    Break,
    Continue,
}

impl AudioFileFetch {
    fn has_download_slots_available(&self) -> bool {
        self.shared.download_slots.available_permits() > 0
    }

    fn download_range(&mut self, offset: usize, mut length: usize) -> AudioFileResult {
        if length < self.params.minimum_download_size {
            length = self.params.minimum_download_size;
        }

        // If we are in streaming mode (so not seeking) then start downloading as large
        // of chunks as possible for better throughput and improved CPU usage, while
        // still being reasonably responsive (~1 second) in case we want to seek.
        if self.shared.is_download_streaming() {
            let throughput = self.shared.throughput();
            length = max(length, throughput);
        }

        if offset + length > self.shared.file_size {
            length = self.shared.file_size - offset;
        }
        let mut ranges_to_request = RangeSet::new();
        ranges_to_request.add_range(&Range::new(offset, length));

        // The iteration that follows spawns streamers fast, without awaiting them,
        // so holding the lock for the entire scope of this function should be faster
        // then locking and unlocking multiple times.
        let mut download_status = self.shared.download_status.lock();

        ranges_to_request.subtract_range_set(&download_status.downloaded);
        ranges_to_request.subtract_range_set(&download_status.requested);

        // TODO : refresh cdn_url when the token expired

        for range in ranges_to_request.iter() {
            let streamer = self.session.spclient().stream_from_cdn(
                &self.shared.cdn_url,
                range.start,
                range.length,
            )?;

            download_status.requested.add_range(range);

            let streaming_request = StreamingRequest {
                streamer,
                initial_response: None,
                offset: range.start,
                length: range.length,
            };

            self.session.spawn(receive_data(
                self.shared.clone(),
                self.file_data_tx.clone(),
                streaming_request,
            ));
        }

        Ok(())
    }

    fn pre_fetch_more_data(&mut self, bytes: usize) -> AudioFileResult {
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
        let read_position = self.shared.read_position();
        tail_end.add_range(&Range::new(
            read_position,
            self.shared.file_size - read_position,
        ));
        let tail_end = tail_end.intersection(&missing_data);

        if !tail_end.is_empty() {
            let range = tail_end.get_range(0);
            let offset = range.start;
            let length = min(range.length, bytes);
            self.download_range(offset, length)?;
        } else if !missing_data.is_empty() {
            // ok, the tail is downloaded, download something fom the beginning.
            let range = missing_data.get_range(0);
            let offset = range.start;
            let length = min(range.length, bytes);
            self.download_range(offset, length)?;
        }

        Ok(())
    }

    fn handle_file_data(&mut self, data: ReceivedData) -> Result<ControlFlow, Error> {
        match data {
            ReceivedData::Throughput(mut throughput) => {
                if throughput < self.params.minimum_throughput {
                    warn!(
                        "Throughput {} kbps lower than minimum {}, setting to minimum",
                        throughput / 1000,
                        self.params.minimum_throughput / 1000,
                    );
                    throughput = self.params.minimum_throughput;
                }

                let old_throughput = self.shared.throughput();
                let avg_throughput = if old_throughput > 0 {
                    (old_throughput + throughput) / 2
                } else {
                    throughput
                };

                // print when the new estimate deviates by more than 10% from the last
                if f32::abs((avg_throughput as f32 - old_throughput as f32) / old_throughput as f32)
                    > 0.1
                {
                    trace!(
                        "Throughput now estimated as: {} kbps",
                        avg_throughput / 1000
                    );
                }

                self.shared.set_throughput(avg_throughput);
            }
            ReceivedData::ResponseTime(mut response_time) => {
                if response_time > self.params.maximum_assumed_ping_time {
                    warn!(
                        "Time to first byte {} ms exceeds maximum {}, setting to maximum",
                        response_time.as_millis(),
                        self.params.maximum_assumed_ping_time.as_millis()
                    );
                    response_time = self.params.maximum_assumed_ping_time;
                }

                let old_ping_time_ms = self.shared.ping_time().as_millis();

                // prune old response times. Keep at most two so we can push a third.
                while self.network_response_times.len() >= 3 {
                    self.network_response_times.remove(0);
                }

                // record the response time
                self.network_response_times.push(response_time);

                // stats::median is experimental. So we calculate the median of up to three ourselves.
                let ping_time = {
                    match self.network_response_times.len() {
                        1 => self.network_response_times[0],
                        2 => (self.network_response_times[0] + self.network_response_times[1]) / 2,
                        3 => {
                            let mut times = self.network_response_times.clone();
                            times.sort_unstable();
                            times[1]
                        }
                        _ => unreachable!(),
                    }
                };

                // print when the new estimate deviates by more than 10% from the last
                if f32::abs(
                    (ping_time.as_millis() as f32 - old_ping_time_ms as f32)
                        / old_ping_time_ms as f32,
                ) > 0.1
                {
                    trace!(
                        "Time to first byte now estimated as: {} ms",
                        ping_time.as_millis()
                    );
                }

                // store our new estimate for everyone to see
                self.shared.set_ping_time(ping_time);
            }
            ReceivedData::Data(data) => {
                match self.output.as_mut() {
                    Some(output) => {
                        output.seek(SeekFrom::Start(data.offset as u64))?;
                        output.write_all(data.data.as_ref())?;
                    }
                    None => return Err(AudioFileError::Output.into()),
                }

                let received_range = Range::new(data.offset, data.data.len());

                let full = {
                    let mut download_status = self.shared.download_status.lock();
                    download_status.downloaded.add_range(&received_range);
                    self.shared.cond.notify_all();

                    download_status.downloaded.contained_length_from_value(0)
                        >= self.shared.file_size
                };

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
                self.download_range(request.start, request.length)?
            }
            StreamLoaderCommand::Close => return Ok(ControlFlow::Break),
        }

        Ok(ControlFlow::Continue)
    }

    fn finish(&mut self) -> AudioFileResult {
        let output = self.output.take();

        let complete_tx = self.complete_tx.take();

        if let Some(mut output) = output {
            output.rewind()?;
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

    let params = AudioFetchParams::get();

    let mut fetch = AudioFileFetch {
        session: session.clone(),
        shared,
        output: Some(output),

        file_data_tx,
        complete_tx: Some(complete_tx),
        network_response_times: Vec::with_capacity(3),

        params: params.clone(),
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
            },
            else => (),
        }

        if fetch.shared.is_download_streaming() && fetch.has_download_slots_available() {
            let bytes_pending: usize = {
                let download_status = fetch.shared.download_status.lock();

                download_status
                    .requested
                    .minus(&download_status.downloaded)
                    .len()
            };

            let ping_time_seconds = fetch.shared.ping_time().as_secs_f32();
            let throughput = fetch.shared.throughput();

            let desired_pending_bytes = max(
                (params.prefetch_threshold_factor
                    * ping_time_seconds
                    * fetch.shared.bytes_per_second as f32) as usize,
                (ping_time_seconds * throughput as f32) as usize,
            );

            if bytes_pending < desired_pending_bytes {
                fetch.pre_fetch_more_data(desired_pending_bytes - bytes_pending)?;
            }
        }
    }

    Ok(())
}
