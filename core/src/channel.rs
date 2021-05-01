use std::collections::HashMap;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Instant;

use byteorder::{BigEndian, ByteOrder};
use bytes::Bytes;
use futures_core::Stream;
use futures_util::lock::BiLock;
use futures_util::StreamExt;
use tokio::sync::mpsc;

use crate::util::SeqGenerator;

component! {
    ChannelManager : ChannelManagerInner {
        sequence: SeqGenerator<u16> = SeqGenerator::new(0),
        channels: HashMap<u16, mpsc::UnboundedSender<(u8, Bytes)>> = HashMap::new(),
        download_rate_estimate: usize = 0,
        download_measurement_start: Option<Instant> = None,
        download_measurement_bytes: usize = 0,
        invalid: bool = false,
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
        let (tx, rx) = mpsc::unbounded_channel();

        let seq = self.lock(|inner| {
            let seq = inner.sequence.get();
            if !inner.invalid {
                inner.channels.insert(seq, tx);
            }
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
                let _ = entry.get().send((cmd, data));
            }
        });
    }

    pub fn get_download_rate_estimate(&self) -> usize {
        self.lock(|inner| inner.download_rate_estimate)
    }

    pub(crate) fn shutdown(&self) {
        self.lock(|inner| {
            inner.invalid = true;
            // destroy the sending halves of the channels to signal everyone who is waiting for something.
            inner.channels.clear();
        });
    }
}

impl Channel {
    fn recv_packet(&mut self, cx: &mut Context<'_>) -> Poll<Result<Bytes, ChannelError>> {
        let (cmd, packet) = match self.receiver.poll_recv(cx) {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(o) => o.ok_or(ChannelError)?,
        };

        if cmd == 0xa {
            let code = BigEndian::read_u16(&packet.as_ref()[..2]);
            error!("channel error: {} {}", packet.len(), code);

            self.state = ChannelState::Closed;

            Poll::Ready(Err(ChannelError))
        } else {
            Poll::Ready(Ok(packet))
        }
    }

    pub fn split(self) -> (ChannelHeaders, ChannelData) {
        let (headers, data) = BiLock::new(self);

        (ChannelHeaders(headers), ChannelData(data))
    }
}

impl Stream for Channel {
    type Item = Result<ChannelEvent, ChannelError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            match self.state.clone() {
                ChannelState::Closed => panic!("Polling already terminated channel"),
                ChannelState::Header(mut data) => {
                    if data.is_empty() {
                        data = match self.recv_packet(cx) {
                            Poll::Ready(Ok(x)) => x,
                            Poll::Ready(Err(x)) => return Poll::Ready(Some(Err(x))),
                            Poll::Pending => return Poll::Pending,
                        };
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
                        return Poll::Ready(Some(Ok(event)));
                    }
                }

                ChannelState::Data => {
                    let data = match self.recv_packet(cx) {
                        Poll::Ready(Ok(x)) => x,
                        Poll::Ready(Err(x)) => return Poll::Ready(Some(Err(x))),
                        Poll::Pending => return Poll::Pending,
                    };
                    if data.is_empty() {
                        self.receiver.close();
                        self.state = ChannelState::Closed;
                        return Poll::Ready(None);
                    } else {
                        let event = ChannelEvent::Data(data);
                        return Poll::Ready(Some(Ok(event)));
                    }
                }
            }
        }
    }
}

impl Stream for ChannelData {
    type Item = Result<Bytes, ChannelError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut channel = match self.0.poll_lock(cx) {
            Poll::Ready(c) => c,
            Poll::Pending => return Poll::Pending,
        };

        loop {
            let event = match channel.poll_next_unpin(cx) {
                Poll::Ready(x) => x.transpose()?,
                Poll::Pending => return Poll::Pending,
            };

            match event {
                Some(ChannelEvent::Header(..)) => (),
                Some(ChannelEvent::Data(data)) => return Poll::Ready(Some(Ok(data))),
                None => return Poll::Ready(None),
            }
        }
    }
}

impl Stream for ChannelHeaders {
    type Item = Result<(u8, Vec<u8>), ChannelError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut channel = match self.0.poll_lock(cx) {
            Poll::Ready(c) => c,
            Poll::Pending => return Poll::Pending,
        };

        let event = match channel.poll_next_unpin(cx) {
            Poll::Ready(x) => x.transpose()?,
            Poll::Pending => return Poll::Pending,
        };

        match event {
            Some(ChannelEvent::Header(id, data)) => Poll::Ready(Some(Ok((id, data)))),
            Some(ChannelEvent::Data(..)) | None => Poll::Ready(None),
        }
    }
}
