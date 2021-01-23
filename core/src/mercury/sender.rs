use futures::{Future, Sink};
use std::collections::VecDeque;

use super::*;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

pub struct MercurySender {
    mercury: MercuryManager,
    uri: String,
    pending: VecDeque<MercuryFuture<MercuryResponse>>,
}

impl MercurySender {
    // TODO: pub(super) when stable
    pub(crate) fn new(mercury: MercuryManager, uri: String) -> MercurySender {
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

type SinkItem = Vec<u8>;
impl Sink<SinkItem> for MercurySender {
    type Error = MercuryError;

    fn start_send(self: Pin<&mut Self>, item: SinkItem) -> Result<(), Self::Error> {
        let task = self.mercury.send(self.uri.clone(), item);
        self.pending.push_back(task);
        Poll::Ready(Ok(()))
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        loop {
            match self.pending.front_mut() {
                Some(task) => {
                    ready!(task.poll());
                }
                None => return Poll::Ready(Ok(())),
            }
            self.pending.pop_front();
        }
    }
}
