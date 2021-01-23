use std::io;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, RwLock, Weak};
use std::time::{SystemTime, UNIX_EPOCH};

use byteorder::{BigEndian, ByteOrder};
use bytes::Bytes;
use futures::sync::mpsc;
use futures::{Async, Future, IntoFuture, Poll, Stream};
use tokio::runtime::{current_thread, current_thread::Handle};

use crate::apresolve::apresolve_or_fallback;
use crate::audio_key::AudioKeyManager;
use crate::authentication::Credentials;
use crate::cache::Cache;
use crate::channel::ChannelManager;
use crate::component::Lazy;
use crate::config::SessionConfig;
use crate::connection;
use crate::mercury::MercuryManager;

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

    audio_key: Lazy<AudioKeyManager>,
    channel: Lazy<ChannelManager>,
    mercury: Lazy<MercuryManager>,
    cache: Option<Arc<Cache>>,

    handle: Mutex<Handle>,
    session_id: usize,
}

static SESSION_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone)]
pub struct Session(Arc<SessionInternal>);

impl Session {
    pub fn connect(
        config: SessionConfig,
        credentials: Credentials,
        cache: Option<Cache>,
        handle: Handle,
    ) -> Box<dyn Future<Item = Session, Error = io::Error>> {
        let access_point = apresolve_or_fallback::<io::Error>(&config.proxy, &config.ap_port);

        let proxy = config.proxy.clone();
        let connection = access_point.and_then(move |addr| {
            info!("Connecting to AP \"{}\"", addr);
            connection::connect(addr, &proxy)
        });

        let device_id = config.device_id.clone();
        let authentication = connection.and_then(move |connection| {
            connection::authenticate(connection, credentials, device_id)
        });

        let result = authentication.map(move |(transport, reusable_credentials)| {
            info!("Authenticated as \"{}\" !", reusable_credentials.username);
            if let Some(ref cache) = cache {
                cache.save_credentials(&reusable_credentials);
            }

            let (session, task) = Session::create(
                &handle,
                transport,
                config,
                cache,
                reusable_credentials.username.clone(),
            );

            current_thread::spawn(task.map_err(|e| {
                error!("SessionError: {}", e.to_string());
                std::process::exit(0);
            }));

            session
        });

        Box::new(result)
    }

    fn create(
        handle: &Handle,
        transport: connection::Transport,
        config: SessionConfig,
        cache: Option<Cache>,
        username: String,
    ) -> (Session, Box<dyn Future<Item = (), Error = io::Error>>) {
        let (sink, stream) = transport.split();

        let (sender_tx, sender_rx) = mpsc::unbounded();
        let session_id = SESSION_COUNTER.fetch_add(1, Ordering::Relaxed);

        debug!("new Session[{}]", session_id);

        let session = Session(Arc::new(SessionInternal {
            config: config,
            data: RwLock::new(SessionData {
                country: String::new(),
                canonical_username: username,
                invalid: false,
                time_delta: 0,
            }),

            tx_connection: sender_tx,

            cache: cache.map(Arc::new),

            audio_key: Lazy::new(),
            channel: Lazy::new(),
            mercury: Lazy::new(),

            handle: Mutex::new(handle.clone()),
            session_id: session_id,
        }));

        let sender_task = sender_rx
            .map_err(|e| -> io::Error { panic!(e) })
            .forward(sink)
            .map(|_| ());
        let receiver_task = DispatchTask(stream, session.weak());

        let task = Box::new(
            (receiver_task, sender_task)
                .into_future()
                .map(|((), ())| ()),
        );

        (session, task)
    }

    pub fn audio_key(&self) -> &AudioKeyManager {
        self.0.audio_key.get(|| AudioKeyManager::new(self.weak()))
    }

    pub fn channel(&self) -> &ChannelManager {
        self.0.channel.get(|| ChannelManager::new(self.weak()))
    }

    pub fn mercury(&self) -> &MercuryManager {
        self.0.mercury.get(|| MercuryManager::new(self.weak()))
    }

    pub fn time_delta(&self) -> i64 {
        self.0.data.read().unwrap().time_delta
    }

    // Spawn a future directly
    pub fn spawn<F>(&self, f: F)
    where
        F: Future<Item = (), Error = ()> + Send + 'static,
    {
        let handle = self.0.handle.lock().unwrap();
        let spawn_res = handle.spawn(f);
        match spawn_res {
            Ok(_) => (),
            Err(e) => error!("Session SpawnErr {:?}", e),
        }
    }

    // pub fn spawn<F, R>(&self, f: F)
    // where
    //     F: FnOnce() -> R + Send + 'static,
    //     R: Future<Item = (), Error = ()> + Send + 'static,
    // {
    //     // This fails when called from a different thread
    //     // current_thread::spawn(future::lazy(|| f()));
    //
    //     // These fail when the Future doesn't implement Send
    //     let handle = self.0.handle.lock().unwrap();
    //     let spawn_res = handle.spawn(lazy(|| f()));
    //
    //     // let mut te = current_thread::TaskExecutor::current();
    //     // let spawn_res = te.spawn_local(Box::new(future::lazy(|| f())));
    //
    //     match spawn_res {
    //         Ok(_) => (),
    //         Err(e) => error!("Session SpawnErr {:?}", e),
    //     }
    // }

    fn debug_info(&self) {
        debug!(
            "Session[{}] strong={} weak={}",
            self.0.session_id,
            Arc::strong_count(&self.0),
            Arc::weak_count(&self.0)
        );
    }

    #[cfg_attr(feature = "cargo-clippy", allow(match_same_arms))]
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
            _ => trace!("Unknown dispatch cmd :{:?} {:?}", cmd, data),
        }
    }

    pub fn send_packet(&self, cmd: u8, data: Vec<u8>) {
        self.0.tx_connection.unbounded_send((cmd, data)).unwrap();
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
pub struct SessionWeak(Weak<SessionInternal>);

impl SessionWeak {
    fn try_upgrade(&self) -> Option<Session> {
        self.0.upgrade().map(Session)
    }

    pub(crate) fn upgrade(&self) -> Session {
        self.try_upgrade().expect("Session died")
    }
}

impl Drop for SessionInternal {
    fn drop(&mut self) {
        debug!("drop Session[{}]", self.session_id);
    }
}

struct DispatchTask<S>(S, SessionWeak)
where
    S: Stream<Item = (u8, Bytes)>;

impl<S> Future for DispatchTask<S>
where
    S: Stream<Item = (u8, Bytes)>,
    <S as Stream>::Error: ::std::fmt::Debug,
{
    type Item = ();
    type Error = S::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let session = match self.1.try_upgrade() {
            Some(session) => session,
            None => return Ok(Async::Ready(())),
        };

        loop {
            let (cmd, data) = match self.0.poll() {
                Ok(Async::Ready(Some(t))) => t,
                Ok(Async::Ready(None)) => {
                    warn!("Connection to server closed.");
                    session.shutdown();
                    return Ok(Async::Ready(()));
                }
                Ok(Async::NotReady) => return Ok(Async::NotReady),
                Err(e) => {
                    session.shutdown();
                    return Err(From::from(e));
                }
            };

            session.dispatch(cmd, data);
        }
    }
}

impl<S> Drop for DispatchTask<S>
where
    S: Stream<Item = (u8, Bytes)>,
{
    fn drop(&mut self) {
        debug!("drop Dispatch");
    }
}
