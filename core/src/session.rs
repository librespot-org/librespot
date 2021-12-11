use std::collections::HashMap;
use std::future::Future;
use std::io;
use std::pin::Pin;
use std::process::exit;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock, Weak};
use std::task::Context;
use std::task::Poll;
use std::time::{SystemTime, UNIX_EPOCH};

use byteorder::{BigEndian, ByteOrder};
use bytes::Bytes;
use futures_core::TryStream;
use futures_util::{future, ready, StreamExt, TryStreamExt};
use num_traits::FromPrimitive;
use once_cell::sync::OnceCell;
use quick_xml::events::Event;
use thiserror::Error;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::apresolve::ApResolver;
use crate::audio_key::AudioKeyManager;
use crate::authentication::Credentials;
use crate::cache::Cache;
use crate::channel::ChannelManager;
use crate::config::SessionConfig;
use crate::connection::{self, AuthenticationError};
use crate::http_client::HttpClient;
use crate::mercury::MercuryManager;
use crate::packet::PacketType;
use crate::spclient::SpClient;
use crate::token::TokenProvider;

#[derive(Debug, Error)]
pub enum SessionError {
    #[error(transparent)]
    AuthenticationError(#[from] AuthenticationError),
    #[error("Cannot create session: {0}")]
    IoError(#[from] io::Error),
}

pub type UserAttributes = HashMap<String, String>;

#[derive(Debug, Clone, Default)]
pub struct UserData {
    pub country: String,
    pub canonical_username: String,
    pub attributes: UserAttributes,
}

#[derive(Debug, Clone, Default)]
struct SessionData {
    time_delta: i64,
    invalid: bool,
    user_data: UserData,
}

struct SessionInternal {
    config: SessionConfig,
    data: RwLock<SessionData>,

    http_client: HttpClient,
    tx_connection: mpsc::UnboundedSender<(u8, Vec<u8>)>,

    apresolver: OnceCell<ApResolver>,
    audio_key: OnceCell<AudioKeyManager>,
    channel: OnceCell<ChannelManager>,
    mercury: OnceCell<MercuryManager>,
    spclient: OnceCell<SpClient>,
    token_provider: OnceCell<TokenProvider>,
    cache: Option<Arc<Cache>>,

    handle: tokio::runtime::Handle,

    session_id: usize,
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
        let http_client = HttpClient::new(config.proxy.as_ref());
        let (sender_tx, sender_rx) = mpsc::unbounded_channel();
        let session_id = SESSION_COUNTER.fetch_add(1, Ordering::Relaxed);

        debug!("new Session[{}]", session_id);

        let session = Session(Arc::new(SessionInternal {
            config,
            data: RwLock::new(SessionData::default()),
            http_client,
            tx_connection: sender_tx,
            cache: cache.map(Arc::new),
            apresolver: OnceCell::new(),
            audio_key: OnceCell::new(),
            channel: OnceCell::new(),
            mercury: OnceCell::new(),
            spclient: OnceCell::new(),
            token_provider: OnceCell::new(),
            handle: tokio::runtime::Handle::current(),
            session_id,
        }));

        let ap = session.apresolver().resolve("accesspoint").await;
        info!("Connecting to AP \"{}:{}\"", ap.0, ap.1);
        let mut transport =
            connection::connect(&ap.0, ap.1, session.config().proxy.as_ref()).await?;

        let reusable_credentials =
            connection::authenticate(&mut transport, credentials, &session.config().device_id)
                .await?;
        info!("Authenticated as \"{}\" !", reusable_credentials.username);
        session.0.data.write().unwrap().user_data.canonical_username =
            reusable_credentials.username.clone();
        if let Some(cache) = session.cache() {
            cache.save_credentials(&reusable_credentials);
        }

        let (sink, stream) = transport.split();
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

        Ok(session)
    }

    pub fn apresolver(&self) -> &ApResolver {
        self.0
            .apresolver
            .get_or_init(|| ApResolver::new(self.weak()))
    }

    pub fn audio_key(&self) -> &AudioKeyManager {
        self.0
            .audio_key
            .get_or_init(|| AudioKeyManager::new(self.weak()))
    }

    pub fn channel(&self) -> &ChannelManager {
        self.0
            .channel
            .get_or_init(|| ChannelManager::new(self.weak()))
    }

    pub fn http_client(&self) -> &HttpClient {
        &self.0.http_client
    }

    pub fn mercury(&self) -> &MercuryManager {
        self.0
            .mercury
            .get_or_init(|| MercuryManager::new(self.weak()))
    }

    pub fn spclient(&self) -> &SpClient {
        self.0.spclient.get_or_init(|| SpClient::new(self.weak()))
    }

    pub fn token_provider(&self) -> &TokenProvider {
        self.0
            .token_provider
            .get_or_init(|| TokenProvider::new(self.weak()))
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

    fn check_catalogue(attributes: &UserAttributes) {
        if let Some(account_type) = attributes.get("type") {
            if account_type != "premium" {
                error!("librespot does not support {:?} accounts.", account_type);
                info!("Please support Spotify and your artists and sign up for a premium account.");

                // TODO: logout instead of exiting
                exit(1);
            }
        }
    }

    fn dispatch(&self, cmd: u8, data: Bytes) {
        use PacketType::*;
        let packet_type = FromPrimitive::from_u8(cmd);
        match packet_type {
            Some(Ping) => {
                let server_timestamp = BigEndian::read_u32(data.as_ref()) as i64;
                let timestamp = match SystemTime::now().duration_since(UNIX_EPOCH) {
                    Ok(dur) => dur,
                    Err(err) => err.duration(),
                }
                .as_secs() as i64;

                self.0.data.write().unwrap().time_delta = server_timestamp - timestamp;

                self.debug_info();
                self.send_packet(Pong, vec![0, 0, 0, 0]);
            }
            Some(CountryCode) => {
                let country = String::from_utf8(data.as_ref().to_owned()).unwrap();
                info!("Country: {:?}", country);
                self.0.data.write().unwrap().user_data.country = country;
            }
            Some(StreamChunkRes) | Some(ChannelError) => {
                self.channel().dispatch(packet_type.unwrap(), data);
            }
            Some(AesKey) | Some(AesKeyError) => {
                self.audio_key().dispatch(packet_type.unwrap(), data);
            }
            Some(MercuryReq) | Some(MercurySub) | Some(MercuryUnsub) | Some(MercuryEvent) => {
                self.mercury().dispatch(packet_type.unwrap(), data);
            }
            Some(ProductInfo) => {
                let data = std::str::from_utf8(&data).unwrap();
                let mut reader = quick_xml::Reader::from_str(data);

                let mut buf = Vec::new();
                let mut current_element = String::new();
                let mut user_attributes: UserAttributes = HashMap::new();

                loop {
                    match reader.read_event(&mut buf) {
                        Ok(Event::Start(ref element)) => {
                            current_element =
                                std::str::from_utf8(element.name()).unwrap().to_owned()
                        }
                        Ok(Event::End(_)) => {
                            current_element = String::new();
                        }
                        Ok(Event::Text(ref value)) => {
                            if !current_element.is_empty() {
                                let _ = user_attributes.insert(
                                    current_element.clone(),
                                    value.unescape_and_decode(&reader).unwrap(),
                                );
                            }
                        }
                        Ok(Event::Eof) => break,
                        Ok(_) => (),
                        Err(e) => error!(
                            "Error parsing XML at position {}: {:?}",
                            reader.buffer_position(),
                            e
                        ),
                    }
                }

                trace!("Received product info: {:?}", user_attributes);
                Self::check_catalogue(&user_attributes);

                self.0.data.write().unwrap().user_data.attributes = user_attributes;
            }
            Some(PongAck)
            | Some(SecretBlock)
            | Some(LegacyWelcome)
            | Some(UnknownDataAllZeros)
            | Some(LicenseVersion) => {}
            _ => {
                if let Some(packet_type) = PacketType::from_u8(cmd) {
                    trace!("Ignoring {:?} packet with data {:?}", packet_type, data);
                } else {
                    trace!("Ignoring unknown packet {:x}", cmd);
                }
            }
        }
    }

    pub fn send_packet(&self, cmd: PacketType, data: Vec<u8>) {
        self.0.tx_connection.send((cmd as u8, data)).unwrap();
    }

    pub fn cache(&self) -> Option<&Arc<Cache>> {
        self.0.cache.as_ref()
    }

    pub fn config(&self) -> &SessionConfig {
        &self.0.config
    }

    pub fn user_data(&self) -> UserData {
        self.0.data.read().unwrap().user_data.clone()
    }

    pub fn device_id(&self) -> &str {
        &self.config().device_id
    }

    pub fn username(&self) -> String {
        self.0
            .data
            .read()
            .unwrap()
            .user_data
            .canonical_username
            .clone()
    }

    pub fn set_user_attribute(&self, key: &str, value: &str) -> Option<String> {
        let mut dummy_attributes = UserAttributes::new();
        dummy_attributes.insert(key.to_owned(), value.to_owned());
        Self::check_catalogue(&dummy_attributes);

        self.0
            .data
            .write()
            .unwrap()
            .user_data
            .attributes
            .insert(key.to_owned(), value.to_owned())
    }

    pub fn set_user_attributes(&self, attributes: UserAttributes) {
        Self::check_catalogue(&attributes);

        self.0
            .data
            .write()
            .unwrap()
            .user_data
            .attributes
            .extend(attributes)
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
