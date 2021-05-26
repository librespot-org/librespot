use std::collections::VecDeque;

use super::*;

pub struct MercurySender {
    mercury: MercuryManager,
    uri: String,
    pending: VecDeque<MercuryFuture<MercuryResponse>>,
    buffered_future: Option<MercuryFuture<MercuryResponse>>,
}

impl MercurySender {
    pub(crate) fn new(mercury: MercuryManager, uri: String) -> MercurySender {
        MercurySender {
            mercury,
            uri,
            pending: VecDeque::new(),
            buffered_future: None,
        }
    }

    pub fn is_flushed(&self) -> bool {
        self.buffered_future.is_none() && self.pending.is_empty()
    }

    pub fn send(&mut self, item: Vec<u8>) {
        let task = self.mercury.send(self.uri.clone(), item);
        self.pending.push_back(task);
    }

    pub async fn flush(&mut self) -> Result<(), MercuryError> {
        if self.buffered_future.is_none() {
            self.buffered_future = self.pending.pop_front();
        }

        while let Some(fut) = self.buffered_future.as_mut() {
            fut.await?;
            self.buffered_future = self.pending.pop_front();
        }
        Ok(())
    }
}

impl Clone for MercurySender {
    fn clone(&self) -> MercurySender {
        MercurySender {
            mercury: self.mercury.clone(),
            uri: self.uri.clone(),
            pending: VecDeque::new(),
            buffered_future: None,
        }
    }
}
