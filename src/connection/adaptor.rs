use futures::future::ok;
use futures::sync::mpsc;
use futures::sync::oneshot;
use futures::{Future, Sink, Stream, BoxFuture, IntoFuture};
use std::thread;
use tokio_core::reactor::Core;
use tokio_core::reactor::Handle;

pub struct SinkAdaptor<T>(pub Option<mpsc::Sender<T>>);
pub struct StreamAdaptor<T, E>(pub Option<mpsc::Receiver<Result<T, E>>>);

impl <T> SinkAdaptor<T> {
    pub fn send(&mut self, item: T) {
        let sender = self.0.take().unwrap();
        let sending = sender.send(item);
        self.0 = Some(sending.wait().unwrap());
    }
}

impl <T, E> StreamAdaptor<T, E> {
    pub fn recv(&mut self) -> Result<T, E> {
        let receiver = self.0.take().unwrap();
        let receiving = receiver.into_future();

        let (packet, receiver) = receiving.wait().map_err(|(e, _)| e).unwrap();

        self.0 = Some(receiver);

        packet.unwrap()
    }
}

pub fn adapt<S, E>(transport: S) -> (SinkAdaptor<S::SinkItem>,
                                     StreamAdaptor<S::Item, E>,
                                     BoxFuture<(), E>)
    where S: Sink<SinkError=E> + Stream<Error=E> + Send + 'static,
          S::Item: Send + 'static,
          S::SinkItem: Send + 'static,
          E: Send + 'static,
{
    let (receiver_tx, receiver_rx) = mpsc::channel(0);
    let (sender_tx, sender_rx) = mpsc::channel(0);

    let (sink, stream) = transport.split();

    let receiver_task = stream
        .then(ok::<_, mpsc::SendError<_>>)
        .forward(receiver_tx).map(|_| ())
        .map_err(|e| -> E { panic!(e) });

    let sender_task = sender_rx
        .map_err(|e| -> E { panic!(e) })
        .forward(sink).map(|_| ());

    let task = (receiver_task, sender_task).into_future()
        .map(|((), ())| ()).boxed();

    (SinkAdaptor(Some(sender_tx)),
     StreamAdaptor(Some(receiver_rx)), task)
}

pub fn adapt_future<F, U>(f: F) -> oneshot::Receiver<Result<U::Item, U::Error>>
    where F: FnOnce(Handle) -> U + Send + 'static,
          U: IntoFuture,
          U::Item: Send + 'static,
          U::Error: Send + 'static,
{
    let (tx, rx) = oneshot::channel();

    thread::spawn(move || {
        let mut core = Core::new().unwrap();
        let handle = core.handle();

        let task = f(handle).into_future();
        let result = core.run(task);

        tx.complete(result);
    });

    rx
}
