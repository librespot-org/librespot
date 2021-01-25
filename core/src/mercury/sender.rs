use futures::Sink;
use std::{collections::VecDeque, pin::Pin, task::Context};

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

    fn poll_ready(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.poll_flush(cx)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        loop {
            match self.pending.front_mut() {
                Some(task) => {
                    match Pin::new(task).poll(cx) {
                        Poll::Ready(Err(x)) => return Poll::Ready(Err(x)),
                        Poll::Pending => return Poll::Pending,
                        _ => (),
                    };
                }
                None => {
                    return Poll::Ready(Ok(()));
                }
            }
            self.pending.pop_front();
        }
    }

    fn start_send(mut self: Pin<&mut Self>, item: Vec<u8>) -> Result<(), Self::Error> {
        let task = self.mercury.send(self.uri.clone(), item);
        self.pending.push_back(task);
        Ok(())
    }
}
