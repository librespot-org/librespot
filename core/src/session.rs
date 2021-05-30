#[macro_use]
mod component;

pub mod audio_key;
pub mod channel;
pub mod mercury;

use std::future::Future;
use std::io;
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, RwLock, Weak};
use std::task::Context;
use std::task::Poll;
use std::time::{SystemTime, UNIX_EPOCH};

use byteorder::{BigEndian, ByteOrder};
use bytes::Bytes;
use futures_core::TryStream;
use futures_util::{future, ready, StreamExt, TryStreamExt};
use thiserror::Error;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

use self::audio_key::{AudioKeyManager, AudioKeyManagerInner};
use self::channel::{ChannelManager, ChannelManagerInner};
use self::mercury::{MercuryManager, MercuryManagerInner};
use crate::apresolve::apresolve;
use crate::authentication::Credentials;
use crate::cache::Cache;
use crate::config::SessionConfig;
use crate::connection::{self, AuthenticationError, Transport};

#[derive(Debug, Error)]
pub enum SessionError {
    #[error(transparent)]
    AuthenticationError(#[from] AuthenticationError),
    #[error("Cannot create session: {0}")]
    IoError(#[from] io::Error),
}

struct SessionData {
    country: String,
    time_delta: i64,
    canonical_username: String,
    invalid: bool,
}

struct SessionInternal {
    config: SessionConfig,
    data: RwLock<SessionData>,

    tx_connection: mpsc::UnboundedSender<(u8, Vec<u8>)>,

    audio_key: Mutex<AudioKeyManagerInner>,
    channel: Mutex<ChannelManagerInner>,
    mercury: Mutex<MercuryManagerInner>,
    cache: Option<Arc<Cache>>,

    handle: tokio::runtime::Handle,

    session_id: usize,
}

impl SessionInternal {
    fn send_packet(&self, cmd: u8, data: Vec<u8>) {
        self.tx_connection.send((cmd, data)).unwrap();
    }
}

impl_components! {
    SessionInternal;
    AudioKeyManagerInner: .audio_key,
    ChannelManagerInner: .channel,
    MercuryManagerInner: .mercury
}

static SESSION_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone)]
pub struct Session(Arc<SessionInternal>);

impl Session {
    pub async fn connect(
        config: SessionConfig,
        credentials: Credentials,
        cache: Option<Cache>,
    ) -> Result<Session, SessionError> {
        let ap = apresolve(config.proxy.as_ref(), config.ap_port).await;

        info!("Connecting to AP \"{}\"", ap);
        let mut conn = connection::connect(ap, config.proxy.as_ref()).await?;

        let reusable_credentials =
            connection::authenticate(&mut conn, credentials, &config.device_id).await?;
        info!("Authenticated as \"{}\" !", reusable_credentials.username);
        if let Some(cache) = &cache {
            cache.save_credentials(&reusable_credentials);
        }

        let session = Session::create(
            conn,
            config,
            cache,
            reusable_credentials.username,
            tokio::runtime::Handle::current(),
        );

        Ok(session)
    }

    fn create(
        transport: Transport,
        config: SessionConfig,
        cache: Option<Cache>,
        username: String,
        handle: tokio::runtime::Handle,
    ) -> Session {
        let Transport {
            read: stream,
            write: sink,
        } = transport;

        let (sender_tx, sender_rx) = mpsc::unbounded_channel();
        let session_id = SESSION_COUNTER.fetch_add(1, Ordering::Relaxed);

        debug!("new Session[{}]", session_id);

        let session = Session(Arc::new(SessionInternal {
            config,
            data: RwLock::new(SessionData {
                country: String::new(),
                canonical_username: username,
                invalid: false,
                time_delta: 0,
            }),
            tx_connection: sender_tx,
            cache: cache.map(Arc::new),
            audio_key: Default::default(),
            channel: Default::default(),
            mercury: Default::default(),
            handle,
            session_id,
        }));

        let sender_task = UnboundedReceiverStream::new(sender_rx)
            .map(Ok)
            .forward(sink);
        let receiver_task = DispatchTask(stream, session.weak());

        tokio::spawn(async move {
            let result = future::try_join(sender_task, receiver_task).await;

            if let Err(e) = result {
                error!("{}", e);
            }
        });

        session
    }

    pub fn audio_key(&self) -> AudioKeyManager<'_> {
        AudioKeyManager(&self.0)
    }

    pub fn channel(&self) -> ChannelManager<'_> {
        ChannelManager(&self.0)
    }

    pub fn mercury(&self) -> MercuryManager<'_> {
        MercuryManager(&self.0)
    }

    pub fn time_delta(&self) -> i64 {
        self.0.data.read().unwrap().time_delta
    }

    pub fn spawn<T>(&self, task: T)
    where
        T: Future + Send + 'static,
        T::Output: Send + 'static,
    {
        self.0.handle.spawn(task);
    }

    fn debug_info(&self) {
        debug!(
            "Session[{}] strong={} weak={}",
            self.0.session_id,
            Arc::strong_count(&self.0),
            Arc::weak_count(&self.0)
        );
    }

    #[allow(clippy::match_same_arms)]
    fn dispatch(&self, cmd: u8, data: Bytes) {
        match cmd {
            0x4 => {
                let server_timestamp = BigEndian::read_u32(data.as_ref()) as i64;
                let timestamp = match SystemTime::now().duration_since(UNIX_EPOCH) {
                    Ok(dur) => dur,
                    Err(err) => err.duration(),
                }
                .as_secs() as i64;

                self.0.data.write().unwrap().time_delta = server_timestamp - timestamp;

                self.debug_info();
                self.send_packet(0x49, vec![0, 0, 0, 0]);
            }
            0x4a => (),
            0x1b => {
                let country = String::from_utf8(data.as_ref().to_owned()).unwrap();
                info!("Country: {:?}", country);
                self.0.data.write().unwrap().country = country;
            }

            0x9 | 0xa => self.channel().dispatch(cmd, data),
            0xd | 0xe => self.audio_key().dispatch(cmd, data),
            0xb2..=0xb6 => self.mercury().dispatch(cmd, data),
            _ => (),
        }
    }

    pub fn send_packet(&self, cmd: u8, data: Vec<u8>) {
        self.0.send_packet(cmd, data);
    }

    pub fn cache(&self) -> Option<&Arc<Cache>> {
        self.0.cache.as_ref()
    }

    fn config(&self) -> &SessionConfig {
        &self.0.config
    }

    pub fn username(&self) -> String {
        self.0.data.read().unwrap().canonical_username.clone()
    }

    pub fn country(&self) -> String {
        self.0.data.read().unwrap().country.clone()
    }

    pub fn device_id(&self) -> &str {
        &self.config().device_id
    }

    fn weak(&self) -> SessionWeak {
        SessionWeak(Arc::downgrade(&self.0))
    }

    pub fn session_id(&self) -> usize {
        self.0.session_id
    }

    pub fn shutdown(&self) {
        debug!("Invalidating session[{}]", self.0.session_id);
        self.0.data.write().unwrap().invalid = true;
        self.mercury().shutdown();
        self.channel().shutdown();
    }

    pub fn is_invalid(&self) -> bool {
        self.0.data.read().unwrap().invalid
    }
}

#[derive(Clone)]
struct SessionWeak(Weak<SessionInternal>);

impl SessionWeak {
    fn try_upgrade(&self) -> Option<Session> {
        self.0.upgrade().map(Session)
    }
}

impl Drop for SessionInternal {
    fn drop(&mut self) {
        debug!("drop Session[{}]", self.session_id);
    }
}

struct DispatchTask<S>(S, SessionWeak)
where
    S: TryStream<Ok = (u8, Bytes)> + Unpin;

impl<S> Future for DispatchTask<S>
where
    S: TryStream<Ok = (u8, Bytes)> + Unpin,
    <S as TryStream>::Ok: std::fmt::Debug,
{
    type Output = Result<(), S::Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let session = match self.1.try_upgrade() {
            Some(session) => session,
            None => return Poll::Ready(Ok(())),
        };

        loop {
            let (cmd, data) = match ready!(self.0.try_poll_next_unpin(cx)) {
                Some(Ok(t)) => t,
                None => {
                    warn!("Connection to server closed.");
                    session.shutdown();
                    return Poll::Ready(Ok(()));
                }
                Some(Err(e)) => {
                    session.shutdown();
                    return Poll::Ready(Err(e));
                }
            };

            session.dispatch(cmd, data);
        }
    }
}

impl<S> Drop for DispatchTask<S>
where
    S: TryStream<Ok = (u8, Bytes)> + Unpin,
{
    fn drop(&mut self) {
        debug!("drop Dispatch");
    }
}
