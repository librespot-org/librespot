use byteorder::{BigEndian, ByteOrder};
use bytes::Bytes;
use futures::sync::{mpsc, BiLock};
use futures::{Async, Poll, Stream};
use std::collections::HashMap;
use std::time::Instant;

use util::SeqGenerator;

component! {
    ChannelManager : ChannelManagerInner {
        sequence: SeqGenerator<u16> = SeqGenerator::new(0),
        channels: HashMap<u16, mpsc::UnboundedSender<(u8, Bytes)>> = HashMap::new(),
        download_rate_estimate: usize = 0,
        download_measurement_start: Option<Instant> = None,
        download_measurement_bytes: usize = 0,
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct ChannelError;

pub struct Channel {
    receiver: mpsc::UnboundedReceiver<(u8, Bytes)>,
    state: ChannelState,
}

pub struct ChannelHeaders(BiLock<Channel>);
pub struct ChannelData(BiLock<Channel>);

pub enum ChannelEvent {
    Header(u8, Vec<u8>),
    Data(Bytes),
}

#[derive(Clone)]
enum ChannelState {
    Header(Bytes),
    Data,
    Closed,
}

impl ChannelManager {
    pub fn allocate(&self) -> (u16, Channel) {
        let (tx, rx) = mpsc::unbounded();

        let seq = self.lock(|inner| {
            let seq = inner.sequence.get();
            inner.channels.insert(seq, tx);
            seq
        });

        let channel = Channel {
            receiver: rx,
            state: ChannelState::Header(Bytes::new()),
        };

        (seq, channel)
    }

    pub(crate) fn dispatch(&self, cmd: u8, mut data: Bytes) {
        use std::collections::hash_map::Entry;

        let id: u16 = BigEndian::read_u16(data.split_to(2).as_ref());

        self.lock(|inner| {
            let current_time = Instant::now();
            if let Some(download_measurement_start) = inner.download_measurement_start {
                if (current_time - download_measurement_start).as_millis() > 1000 {
                    inner.download_rate_estimate = 1000 * inner.download_measurement_bytes
                        / (current_time - download_measurement_start).as_millis() as usize;
                    inner.download_measurement_start = Some(current_time);
                    inner.download_measurement_bytes = 0;
                }
            } else {
                inner.download_measurement_start = Some(current_time);
            }

            inner.download_measurement_bytes += data.len();

            if let Entry::Occupied(entry) = inner.channels.entry(id) {
                let _ = entry.get().unbounded_send((cmd, data));
            }
        });
    }

    pub fn get_download_rate_estimate(&self) -> usize {
        return self.lock(|inner| inner.download_rate_estimate);
    }
}

impl Channel {
    fn recv_packet(&mut self) -> Poll<Bytes, ChannelError> {
        let (cmd, packet) = match self.receiver.poll() {
            Ok(Async::Ready(Some(t))) => t,
            Ok(Async::Ready(None)) => return Err(ChannelError), // The channel has been closed.
            Ok(Async::NotReady) => return Ok(Async::NotReady),
            Err(()) => unreachable!(),
        };

        if cmd == 0xa {
            let code = BigEndian::read_u16(&packet.as_ref()[..2]);
            error!("channel error: {} {}", packet.len(), code);

            self.state = ChannelState::Closed;

            Err(ChannelError)
        } else {
            Ok(Async::Ready(packet))
        }
    }

    pub fn split(self) -> (ChannelHeaders, ChannelData) {
        let (headers, data) = BiLock::new(self);

        (ChannelHeaders(headers), ChannelData(data))
    }
}

impl Stream for Channel {
    type Item = ChannelEvent;
    type Error = ChannelError;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        loop {
            match self.state.clone() {
                ChannelState::Closed => panic!("Polling already terminated channel"),
                ChannelState::Header(mut data) => {
                    if data.len() == 0 {
                        data = try_ready!(self.recv_packet());
                    }

                    let length = BigEndian::read_u16(data.split_to(2).as_ref()) as usize;
                    if length == 0 {
                        assert_eq!(data.len(), 0);
                        self.state = ChannelState::Data;
                    } else {
                        let header_id = data.split_to(1).as_ref()[0];
                        let header_data = data.split_to(length - 1).as_ref().to_owned();

                        self.state = ChannelState::Header(data);

                        let event = ChannelEvent::Header(header_id, header_data);
                        return Ok(Async::Ready(Some(event)));
                    }
                }

                ChannelState::Data => {
                    let data = try_ready!(self.recv_packet());
                    if data.len() == 0 {
                        self.receiver.close();
                        self.state = ChannelState::Closed;
                        return Ok(Async::Ready(None));
                    } else {
                        let event = ChannelEvent::Data(data);
                        return Ok(Async::Ready(Some(event)));
                    }
                }
            }
        }
    }
}

impl Stream for ChannelData {
    type Item = Bytes;
    type Error = ChannelError;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let mut channel = match self.0.poll_lock() {
            Async::Ready(c) => c,
            Async::NotReady => return Ok(Async::NotReady),
        };

        loop {
            match try_ready!(channel.poll()) {
                Some(ChannelEvent::Header(..)) => (),
                Some(ChannelEvent::Data(data)) => return Ok(Async::Ready(Some(data))),
                None => return Ok(Async::Ready(None)),
            }
        }
    }
}

impl Stream for ChannelHeaders {
    type Item = (u8, Vec<u8>);
    type Error = ChannelError;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let mut channel = match self.0.poll_lock() {
            Async::Ready(c) => c,
            Async::NotReady => return Ok(Async::NotReady),
        };

        match try_ready!(channel.poll()) {
            Some(ChannelEvent::Header(id, data)) => Ok(Async::Ready(Some((id, data)))),
            Some(ChannelEvent::Data(..)) | None => Ok(Async::Ready(None)),
        }
    }
}
