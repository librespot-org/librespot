use futures::{Async, Future};
use std::io;
use std::process::{Child, ExitStatus};

/// A future that resolves to a child process's exit status once it exits.
pub struct ChildWaitFuture {
    pub child: Child,
}

impl Future for ChildWaitFuture {
    type Item = ExitStatus;
    type Error = io::Error;
    
    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        match self.child.try_wait() {
            Ok(Some(status)) => Ok(Async::Ready(status)),
            Ok(None) => Ok(Async::NotReady),
            Err(e) => Err(e),
        }
    }
}
