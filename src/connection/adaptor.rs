use futures::future::ok;
use futures::sync::mpsc;
use futures::{Future, Sink, Stream, BoxFuture, IntoFuture};
use std::thread;
use tokio_core::reactor::Core;
use tokio_core::reactor::Handle;

pub struct SinkAdaptor<T>(Option<mpsc::Sender<T>>);
pub struct StreamAdaptor<T, E>(Option<mpsc::Receiver<Result<T, E>>>);

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

fn adapt_sink<S>(sink: S, rx: mpsc::Receiver<S::SinkItem>) -> BoxFuture<(), ()>
    where S: Sink + Send + 'static,
          S::SinkItem: Send,
          S::SinkError: Send,
{
    rx.map_err(|_| -> S::SinkError { panic!("") })
      .forward(sink)
      .map(|_| ()).map_err(|_| ())
      .boxed()
}

fn adapt_stream<S>(stream: S, tx: mpsc::Sender<Result<S::Item, S::Error>>) -> BoxFuture<(), ()>
    where S: Stream + Send + 'static,
          S::Item: Send,
          S::Error: Send,
{
    stream.then(ok::<_, mpsc::SendError<_>>)
        .forward(tx)
        .map(|_| ()).map_err(|_| ())
        .boxed()
}

pub fn adapt<F, U, S>(f: F) -> (SinkAdaptor<S::SinkItem>, StreamAdaptor<S::Item, S::Error>)
    where F: FnOnce(&Handle) -> U + Send + 'static,
          U: IntoFuture<Item=S>,
          S: Sink + Stream + Send + 'static,
          S::Item: Send + 'static,
          S::Error: Send + 'static,
          S::SinkItem: Send + 'static,
          S::SinkError: Send + 'static,
{

    let (receiver_tx, receiver_rx) = mpsc::channel(0);
    let (sender_tx, sender_rx) = mpsc::channel(0);


    thread::spawn(move || {
        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let task =
            f(&handle).into_future()
            .map(|connection| connection.split())
            .map_err(|_| ())
            .and_then(|(sink, stream)| {
                (adapt_sink(sink, sender_rx),
                 adapt_stream(stream, receiver_tx))
            });

        core.run(task).unwrap();
    });

    (SinkAdaptor(Some(sender_tx)),
     StreamAdaptor(Some(receiver_rx)))
}
