mod maps;
pub mod protocol;

use std::{
    iter,
    pin::Pin,
    sync::{
        atomic::{self, AtomicBool},
        Arc,
    },
    task::Poll,
    time::Duration,
};

use futures_core::{Future, Stream};
use futures_util::{future::join_all, SinkExt, StreamExt};
use parking_lot::Mutex;
use thiserror::Error;
use tokio::{
    select,
    sync::{
        mpsc::{self, UnboundedReceiver},
        Semaphore,
    },
    task::JoinHandle,
};
use tokio_tungstenite::tungstenite;
use tungstenite::error::UrlError;
use url::Url;

use self::maps::*;
use self::protocol::*;

use crate::{
    socket,
    util::{keep_flushing, CancelOnDrop, TimeoutOnDrop},
    Error,
};

type WsMessage = tungstenite::Message;
type WsError = tungstenite::Error;
type WsResult<T> = Result<T, tungstenite::Error>;

const WEBSOCKET_CLOSE_TIMEOUT: Duration = Duration::from_secs(3);

const PING_INTERVAL: Duration = Duration::from_secs(30);
const PING_TIMEOUT: Duration = Duration::from_secs(3);

const RECONNECT_INTERVAL: Duration = Duration::from_secs(10);

pub struct Response {
    pub success: bool,
}

pub struct Responder {
    key: String,
    tx: mpsc::UnboundedSender<WsMessage>,
    sent: bool,
}

impl Responder {
    fn new(key: String, tx: mpsc::UnboundedSender<WsMessage>) -> Self {
        Self {
            key,
            tx,
            sent: false,
        }
    }

    // Should only be called once
    fn send_internal(&mut self, response: Response) {
        let response = serde_json::json!({
            "type": "reply",
            "key": &self.key,
            "payload": {
                "success": response.success,
            }
        })
        .to_string();

        if let Err(e) = self.tx.send(WsMessage::Text(response)) {
            warn!("Wasn't able to reply to dealer request: {}", e);
        }
    }

    pub fn send(mut self, response: Response) {
        self.send_internal(response);
        self.sent = true;
    }

    pub fn force_unanswered(mut self) {
        self.sent = true;
    }
}

impl Drop for Responder {
    fn drop(&mut self) {
        if !self.sent {
            self.send_internal(Response { success: false });
        }
    }
}

pub trait IntoResponse {
    fn respond(self, responder: Responder);
}

impl IntoResponse for Response {
    fn respond(self, responder: Responder) {
        responder.send(self)
    }
}

impl<F> IntoResponse for F
where
    F: Future<Output = Response> + Send + 'static,
{
    fn respond(self, responder: Responder) {
        tokio::spawn(async move {
            responder.send(self.await);
        });
    }
}

impl<F, R> RequestHandler for F
where
    F: (Fn(Request) -> R) + Send + 'static,
    R: IntoResponse,
{
    fn handle_request(&self, request: Request, responder: Responder) {
        self(request).respond(responder);
    }
}

pub trait RequestHandler: Send + 'static {
    fn handle_request(&self, request: Request, responder: Responder);
}

type MessageHandler = mpsc::UnboundedSender<Message>;

// TODO: Maybe it's possible to unregister subscription directly when they
//       are dropped instead of on next failed attempt.
pub struct Subscription(UnboundedReceiver<Message>);

impl Stream for Subscription {
    type Item = Message;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        self.0.poll_recv(cx)
    }
}

fn split_uri(s: &str) -> Option<impl Iterator<Item = &'_ str>> {
    let (scheme, sep, rest) = if let Some(rest) = s.strip_prefix("hm://") {
        ("hm", '/', rest)
    } else if let Some(rest) = s.strip_suffix("spotify:") {
        ("spotify", ':', rest)
    } else {
        return None;
    };

    let rest = rest.trim_end_matches(sep);
    let split = rest.split(sep);

    Some(iter::once(scheme).chain(split))
}

#[derive(Debug, Clone, Error)]
pub enum AddHandlerError {
    #[error("There is already a handler for the given uri")]
    AlreadyHandled,
    #[error("The specified uri {0} is invalid")]
    InvalidUri(String),
}

impl From<AddHandlerError> for Error {
    fn from(err: AddHandlerError) -> Self {
        match err {
            AddHandlerError::AlreadyHandled => Error::aborted(err),
            AddHandlerError::InvalidUri(_) => Error::invalid_argument(err),
        }
    }
}

#[derive(Debug, Clone, Error)]
pub enum SubscriptionError {
    #[error("The specified uri is invalid")]
    InvalidUri(String),
}

impl From<SubscriptionError> for Error {
    fn from(err: SubscriptionError) -> Self {
        Error::invalid_argument(err)
    }
}

fn add_handler(
    map: &mut HandlerMap<Box<dyn RequestHandler>>,
    uri: &str,
    handler: impl RequestHandler,
) -> Result<(), Error> {
    let split = split_uri(uri).ok_or_else(|| AddHandlerError::InvalidUri(uri.to_string()))?;
    map.insert(split, Box::new(handler))
}

fn remove_handler<T>(map: &mut HandlerMap<T>, uri: &str) -> Option<T> {
    map.remove(split_uri(uri)?)
}

fn subscribe(
    map: &mut SubscriberMap<MessageHandler>,
    uris: &[&str],
) -> Result<Subscription, Error> {
    let (tx, rx) = mpsc::unbounded_channel();

    for &uri in uris {
        let split = split_uri(uri).ok_or_else(|| SubscriptionError::InvalidUri(uri.to_string()))?;
        map.insert(split, tx.clone());
    }

    Ok(Subscription(rx))
}

#[derive(Default)]
pub struct Builder {
    message_handlers: SubscriberMap<MessageHandler>,
    request_handlers: HandlerMap<Box<dyn RequestHandler>>,
}

macro_rules! create_dealer {
    ($builder:expr, $shared:ident -> $body:expr) => {
        match $builder {
            builder => {
                let shared = Arc::new(DealerShared {
                    message_handlers: Mutex::new(builder.message_handlers),
                    request_handlers: Mutex::new(builder.request_handlers),
                    notify_drop: Semaphore::new(0),
                });

                let handle = {
                    let $shared = Arc::clone(&shared);
                    tokio::spawn($body)
                };

                Dealer {
                    shared,
                    handle: TimeoutOnDrop::new(handle, WEBSOCKET_CLOSE_TIMEOUT),
                }
            }
        }
    };
}

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_handler(&mut self, uri: &str, handler: impl RequestHandler) -> Result<(), Error> {
        add_handler(&mut self.request_handlers, uri, handler)
    }

    pub fn subscribe(&mut self, uris: &[&str]) -> Result<Subscription, Error> {
        subscribe(&mut self.message_handlers, uris)
    }

    pub fn launch_in_background<Fut, F>(self, get_url: F, proxy: Option<Url>) -> Dealer
    where
        Fut: Future<Output = Url> + Send + 'static,
        F: (FnMut() -> Fut) + Send + 'static,
    {
        create_dealer!(self, shared -> run(shared, None, get_url, proxy))
    }

    pub async fn launch<Fut, F>(self, mut get_url: F, proxy: Option<Url>) -> WsResult<Dealer>
    where
        Fut: Future<Output = Url> + Send + 'static,
        F: (FnMut() -> Fut) + Send + 'static,
    {
        let dealer = create_dealer!(self, shared -> {
            // Try to connect.
            let url = get_url().await;
            let tasks = connect(&url, proxy.as_ref(), &shared).await?;

            // If a connection is established, continue in a background task.
            run(shared, Some(tasks), get_url, proxy)
        });

        Ok(dealer)
    }
}

struct DealerShared {
    message_handlers: Mutex<SubscriberMap<MessageHandler>>,
    request_handlers: Mutex<HandlerMap<Box<dyn RequestHandler>>>,

    // Semaphore with 0 permits. By closing this semaphore, we indicate
    // that the actual Dealer struct has been dropped.
    notify_drop: Semaphore,
}

impl DealerShared {
    fn dispatch_message(&self, msg: Message) {
        if let Some(split) = split_uri(&msg.uri) {
            self.message_handlers
                .lock()
                .retain(split, &mut |tx| tx.send(msg.clone()).is_ok());
        }
    }

    fn dispatch_request(&self, request: Request, send_tx: &mpsc::UnboundedSender<WsMessage>) {
        // ResponseSender will automatically send "success: false" if it is dropped without an answer.
        let responder = Responder::new(request.key.clone(), send_tx.clone());

        let split = if let Some(split) = split_uri(&request.message_ident) {
            split
        } else {
            warn!(
                "Dealer request with invalid message_ident: {}",
                &request.message_ident
            );
            return;
        };

        {
            let handler_map = self.request_handlers.lock();

            if let Some(handler) = handler_map.get(split) {
                handler.handle_request(request, responder);
                return;
            }
        }

        warn!("No handler for message_ident: {}", &request.message_ident);
    }

    fn dispatch(&self, m: MessageOrRequest, send_tx: &mpsc::UnboundedSender<WsMessage>) {
        match m {
            MessageOrRequest::Message(m) => self.dispatch_message(m),
            MessageOrRequest::Request(r) => self.dispatch_request(r, send_tx),
        }
    }

    async fn closed(&self) {
        if self.notify_drop.acquire().await.is_ok() {
            error!("should never have gotten a permit");
        }
    }

    fn is_closed(&self) -> bool {
        self.notify_drop.is_closed()
    }
}

pub struct Dealer {
    shared: Arc<DealerShared>,
    handle: TimeoutOnDrop<()>,
}

impl Dealer {
    pub fn add_handler<H>(&self, uri: &str, handler: H) -> Result<(), Error>
    where
        H: RequestHandler,
    {
        add_handler(&mut self.shared.request_handlers.lock(), uri, handler)
    }

    pub fn remove_handler(&self, uri: &str) -> Option<Box<dyn RequestHandler>> {
        remove_handler(&mut self.shared.request_handlers.lock(), uri)
    }

    pub fn subscribe(&self, uris: &[&str]) -> Result<Subscription, Error> {
        subscribe(&mut self.shared.message_handlers.lock(), uris)
    }

    pub async fn close(mut self) {
        debug!("closing dealer");

        self.shared.notify_drop.close();

        if let Some(handle) = self.handle.take() {
            if let Err(e) = CancelOnDrop(handle).await {
                error!("error aborting dealer operations: {}", e);
            }
        }
    }
}

/// Initializes a connection and returns futures that will finish when the connection is closed/lost.
async fn connect(
    address: &Url,
    proxy: Option<&Url>,
    shared: &Arc<DealerShared>,
) -> WsResult<(JoinHandle<()>, JoinHandle<()>)> {
    let host = address
        .host_str()
        .ok_or(WsError::Url(UrlError::NoHostName))?;

    let default_port = match address.scheme() {
        "ws" => 80,
        "wss" => 443,
        _ => return Err(WsError::Url(UrlError::UnsupportedUrlScheme)),
    };

    let port = address.port().unwrap_or(default_port);

    let stream = socket::connect(host, port, proxy).await?;

    let (mut ws_tx, ws_rx) = tokio_tungstenite::client_async_tls(address.as_str(), stream)
        .await?
        .0
        .split();

    let (send_tx, mut send_rx) = mpsc::unbounded_channel::<WsMessage>();

    // Spawn a task that will forward messages from the channel to the websocket.
    let send_task = {
        let shared = Arc::clone(shared);

        tokio::spawn(async move {
            let result = loop {
                select! {
                    biased;
                    () = shared.closed() => {
                        break Ok(None);
                    }
                    msg = send_rx.recv() => {
                        if let Some(msg) = msg {
                            // New message arrived through channel
                            if let WsMessage::Close(close_frame) = msg {
                                break Ok(close_frame);
                            }

                            if let Err(e) = ws_tx.feed(msg).await  {
                                break Err(e);
                            }
                        } else {
                            break Ok(None);
                        }
                    },
                    e = keep_flushing(&mut ws_tx) => {
                        break Err(e)
                    }
                    else => (),
                }
            };

            send_rx.close();

            // I don't trust in tokio_tungstenite's implementation of Sink::close.
            let result = match result {
                Ok(close_frame) => ws_tx.send(WsMessage::Close(close_frame)).await,
                Err(WsError::AlreadyClosed) | Err(WsError::ConnectionClosed) => ws_tx.flush().await,
                Err(e) => {
                    warn!("Dealer finished with an error: {}", e);
                    ws_tx.send(WsMessage::Close(None)).await
                }
            };

            if let Err(e) = result {
                warn!("Error while closing websocket: {}", e);
            }

            debug!("Dropping send task");
        })
    };

    let shared = Arc::clone(shared);

    // A task that receives messages from the web socket.
    let receive_task = tokio::spawn(async {
        let pong_received = AtomicBool::new(true);
        let send_tx = send_tx;
        let shared = shared;

        let receive_task = async {
            let mut ws_rx = ws_rx;

            loop {
                match ws_rx.next().await {
                    Some(Ok(msg)) => match msg {
                        WsMessage::Text(t) => match serde_json::from_str(&t) {
                            Ok(m) => shared.dispatch(m, &send_tx),
                            Err(e) => info!("Received invalid message: {}", e),
                        },
                        WsMessage::Binary(_) => {
                            info!("Received invalid binary message");
                        }
                        WsMessage::Pong(_) => {
                            debug!("Received pong");
                            pong_received.store(true, atomic::Ordering::Relaxed);
                        }
                        _ => (), // tungstenite handles Close and Ping automatically
                    },
                    Some(Err(e)) => {
                        warn!("Websocket connection failed: {}", e);
                        break;
                    }
                    None => {
                        debug!("Websocket connection closed.");
                        break;
                    }
                }
            }
        };

        // Sends pings and checks whether a pong comes back.
        let ping_task = async {
            use tokio::time::{interval, sleep};

            let mut timer = interval(PING_INTERVAL);

            loop {
                timer.tick().await;

                pong_received.store(false, atomic::Ordering::Relaxed);
                if send_tx.send(WsMessage::Ping(vec![])).is_err() {
                    // The sender is closed.
                    break;
                }

                debug!("Sent ping");

                sleep(PING_TIMEOUT).await;

                if !pong_received.load(atomic::Ordering::SeqCst) {
                    // No response
                    warn!("Websocket peer does not respond.");
                    break;
                }
            }
        };

        // Exit this task as soon as one our subtasks fails.
        // In both cases the connection is probably lost.
        select! {
            () = ping_task => (),
            () = receive_task => ()
        }

        // Try to take send_task down with us, in case it's still alive.
        let _ = send_tx.send(WsMessage::Close(None));

        debug!("Dropping receive task");
    });

    Ok((send_task, receive_task))
}

/// The main background task for `Dealer`, which coordinates reconnecting.
async fn run<F, Fut>(
    shared: Arc<DealerShared>,
    initial_tasks: Option<(JoinHandle<()>, JoinHandle<()>)>,
    mut get_url: F,
    proxy: Option<Url>,
) where
    Fut: Future<Output = Url> + Send + 'static,
    F: (FnMut() -> Fut) + Send + 'static,
{
    let init_task = |t| Some(TimeoutOnDrop::new(t, WEBSOCKET_CLOSE_TIMEOUT));

    let mut tasks = if let Some((s, r)) = initial_tasks {
        (init_task(s), init_task(r))
    } else {
        (None, None)
    };

    while !shared.is_closed() {
        match &mut tasks {
            (Some(t0), Some(t1)) => {
                select! {
                    () = shared.closed() => break,
                    r = t0 => {
                        if let Err(e) = r {
                            error!("timeout on task 0: {}", e);
                        }
                        tasks.0.take();
                    },
                    r = t1 => {
                        if let Err(e) = r {
                            error!("timeout on task 1: {}", e);
                        }
                        tasks.1.take();
                    }
                }
            }
            _ => {
                let url = select! {
                    () = shared.closed() => {
                        break
                    },
                    e = get_url() => e
                };

                match connect(&url, proxy.as_ref(), &shared).await {
                    Ok((s, r)) => tasks = (init_task(s), init_task(r)),
                    Err(e) => {
                        error!("Error while connecting: {}", e);
                        tokio::time::sleep(RECONNECT_INTERVAL).await;
                    }
                }
            }
        }
    }

    let tasks = tasks.0.into_iter().chain(tasks.1);

    let _ = join_all(tasks).await;
}
