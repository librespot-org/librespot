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

    pub fn is_flushed(&self) -> bool {
        self.pending.is_empty()
    }

    pub fn send(&mut self, item: Vec<u8>) {
        let task = self.mercury.send(self.uri.clone(), item);
        self.pending.push_back(task);
    }

    pub async fn flush(&mut self) -> Result<(), MercuryError> {
        for fut in self.pending.drain(..) {
            fut.await?;
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
        }
    }
}
