use futures::{task::Poll, Future, Sink};
use std::collections::VecDeque;

use super::*;

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

impl Sink<Vec<u8>> for MercurySender {
    type Error = MercuryError;

    fn start_send(&mut self, item: Vec<u8>) -> Result<(), Self::Error> {
        let task = self.mercury.send(self.uri.clone(), item);
        self.pending.push_back(task);
        Ok(())
    }

    fn poll_ready(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        Ok(())
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        Ok(())
    }

    fn poll_close(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        loop {
            match self.pending.front_mut() {
                Some(task) => {
                    ready!(task.poll());
                }
                None => {
                    return Poll::Ready(Ok(()));
                }
            }
            self.pending.pop_front();
        }
    }
}
