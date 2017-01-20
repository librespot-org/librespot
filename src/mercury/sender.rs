use std::collections::VecDeque;
use futures::{Async, Poll, Future, Sink, StartSend, AsyncSink};

use super::*;

pub struct MercurySender {
    mercury: MercuryManager,
    uri: String,
    pending: VecDeque<MercuryFuture<MercuryResponse>>,
}

impl MercurySender {
    // TODO: pub(super) when stable
    pub fn new(mercury: MercuryManager, uri: String) -> MercurySender {
        MercurySender {
            mercury: mercury,
            uri: uri,
            pending: VecDeque::new(),
        }
    }
}

impl Clone for MercurySender {
    fn clone(&self) -> MercurySender {
        MercurySender {
            mercury: self.mercury.clone(),
            uri: self.uri.clone(),
            pending: VecDeque::new(),
        }
    }
}

impl Sink for MercurySender {
    type SinkItem = Vec<u8>;
    type SinkError = MercuryError;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        let task = self.mercury.send(self.uri.clone(), item);
        self.pending.push_back(task);
        Ok(AsyncSink::Ready)
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        loop {
            match self.pending.front_mut() {
                Some(task) => {
                    try_ready!(task.poll());
                }
                None => {
                    return Ok(Async::Ready(()));
                }
            }
            self.pending.pop_front();
        }
    }
}
