use byteorder::{BigEndian, ByteOrder};
use futures::sync::{BiLock, mpsc};
use futures::{Poll, Async, Stream};
use std::collections::HashMap;
use tokio_core::io::EasyBuf;

use util::SeqGenerator;

component! {
    ChannelManager : ChannelManagerInner {
        sequence: SeqGenerator<u16> = SeqGenerator::new(0),
        channels: HashMap<u16, mpsc::UnboundedSender<(u8, Vec<u8>)>> = HashMap::new(),
    }
}

#[derive(Debug,Hash,PartialEq,Eq,Copy,Clone)]
pub struct ChannelError;

pub struct Channel {
    receiver: mpsc::UnboundedReceiver<(u8, Vec<u8>)>,
    state: ChannelState,
}

pub struct ChannelHeaders(BiLock<Channel>);
pub struct ChannelData(BiLock<Channel>);

pub enum ChannelEvent {
    Header(u8, Vec<u8>),
    Data(Vec<u8>),
}

#[derive(Clone)]
enum ChannelState {
    Header(EasyBuf),
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
            state: ChannelState::Header(EasyBuf::new()),
        };

        (seq, channel)
    }

    pub fn dispatch(&self, cmd: u8, data: Vec<u8>) {
        use std::collections::hash_map::Entry;

        let id: u16 = BigEndian::read_u16(&data[..2]);

        self.lock(|inner| {
            if let Entry::Occupied(entry) = inner.channels.entry(id) {
                let _ = entry.get().send((cmd, data[2..].to_owned()));
            }
        });
    }
}

impl Channel {
    fn recv_packet(&mut self) -> Poll<Vec<u8>, ChannelError> {
        let (cmd, packet) = match self.receiver.poll() {
            Ok(Async::Ready(t)) => t.expect("channel closed"),
            Ok(Async::NotReady) => return Ok(Async::NotReady),
            Err(()) => unreachable!(),
        };

        if cmd == 0xa {
            let code = BigEndian::read_u16(&packet[..2]);
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
                        data = EasyBuf::from(try_ready!(self.recv_packet()));
                    }

                    let length = BigEndian::read_u16(data.drain_to(2).as_ref()) as usize;
                    if length == 0 {
                        assert_eq!(data.len(), 0);
                        self.state = ChannelState::Data;
                    } else {
                        let header_id = data.drain_to(1).as_ref()[0];
                        let header_data = data.drain_to(length - 1).as_ref().to_owned();

                        self.state = ChannelState::Header(data);

                        return Ok(Async::Ready(Some(ChannelEvent::Header(header_id, header_data))));
                    }
                }

                ChannelState::Data => {
                    let data = try_ready!(self.recv_packet());
                    if data.is_empty() {
                        self.receiver.close();
                        self.state = ChannelState::Closed;
                        return Ok(Async::Ready(None));
                    } else {
                        return Ok(Async::Ready(Some(ChannelEvent::Data(data))));
                    }
                }
            }
        }
    }
}

impl Stream for ChannelData {
    type Item = Vec<u8>;
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
