use std::{
    collections::HashMap,
    future::Future,
    io,
    pin::Pin,
    process::exit,
    sync::{Arc, Weak},
    task::{Context, Poll},
    time::{SystemTime, UNIX_EPOCH},
};

use byteorder::{BigEndian, ByteOrder};
use bytes::Bytes;
use futures_core::TryStream;
use futures_util::{future, ready, StreamExt, TryStreamExt};
use num_traits::FromPrimitive;
use once_cell::sync::OnceCell;
use parking_lot::RwLock;
use quick_xml::events::Event;
use thiserror::Error;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::{
    apresolve::ApResolver,
    audio_key::AudioKeyManager,
    authentication::Credentials,
    cache::Cache,
    channel::ChannelManager,
    config::SessionConfig,
    connection::{self, AuthenticationError},
    http_client::HttpClient,
    mercury::MercuryManager,
    packet::PacketType,
    protocol::keyexchange::ErrorCode,
    spclient::SpClient,
    token::TokenProvider,
    Error,
};

#[derive(Debug, Error)]
pub enum SessionError {
    #[error(transparent)]
    AuthenticationError(#[from] AuthenticationError),
    #[error("Cannot create session: {0}")]
    IoError(#[from] io::Error),
    #[error("Session is not connected")]
    NotConnected,
    #[error("packet {0} unknown")]
    Packet(u8),
}

impl From<SessionError> for Error {
    fn from(err: SessionError) -> Self {
        match err {
            SessionError::AuthenticationError(_) => Error::unauthenticated(err),
            SessionError::IoError(_) => Error::unavailable(err),
            SessionError::NotConnected => Error::unavailable(err),
            SessionError::Packet(_) => Error::unimplemented(err),
        }
    }
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
    client_id: String,
    client_name: String,
    client_brand_name: String,
    client_model_name: String,
    connection_id: String,
    time_delta: i64,
    invalid: bool,
    user_data: UserData,
}

struct SessionInternal {
    config: SessionConfig,
    data: RwLock<SessionData>,

    http_client: HttpClient,
    tx_connection: OnceCell<mpsc::UnboundedSender<(u8, Vec<u8>)>>,

    apresolver: OnceCell<ApResolver>,
    audio_key: OnceCell<AudioKeyManager>,
    channel: OnceCell<ChannelManager>,
    mercury: OnceCell<MercuryManager>,
    spclient: OnceCell<SpClient>,
    token_provider: OnceCell<TokenProvider>,
    cache: Option<Arc<Cache>>,

    handle: tokio::runtime::Handle,
}

#[derive(Clone)]
pub struct Session(Arc<SessionInternal>);

impl Session {
    pub fn new(config: SessionConfig, cache: Option<Cache>) -> Self {
        let http_client = HttpClient::new(config.proxy.as_ref());

        debug!("new Session");

        let session_data = SessionData {
            client_id: config.client_id.clone(),
            ..SessionData::default()
        };

        Self(Arc::new(SessionInternal {
            config,
            data: RwLock::new(session_data),
            http_client,
            tx_connection: OnceCell::new(),
            cache: cache.map(Arc::new),
            apresolver: OnceCell::new(),
            audio_key: OnceCell::new(),
            channel: OnceCell::new(),
            mercury: OnceCell::new(),
            spclient: OnceCell::new(),
            token_provider: OnceCell::new(),
            handle: tokio::runtime::Handle::current(),
        }))
    }

    pub async fn connect(
        &self,
        credentials: Credentials,
        store_credentials: bool,
    ) -> Result<(), Error> {
        let (reusable_credentials, transport) = loop {
            let ap = self.apresolver().resolve("accesspoint").await?;
            info!("Connecting to AP \"{}:{}\"", ap.0, ap.1);
            let mut transport =
                connection::connect(&ap.0, ap.1, self.config().proxy.as_ref()).await?;

            match connection::authenticate(
                &mut transport,
                credentials.clone(),
                &self.config().device_id,
            )
            .await
            {
                Ok(creds) => break (creds, transport),
                Err(e) => {
                    if let Some(AuthenticationError::LoginFailed(ErrorCode::TryAnotherAP)) =
                        e.error.downcast_ref::<AuthenticationError>()
                    {
                        warn!("Instructed to try another access point...");
                        continue;
                    } else {
                        return Err(e);
                    }
                }
            }
        };

        info!("Authenticated as \"{}\" !", reusable_credentials.username);
        self.set_username(&reusable_credentials.username);
        if let Some(cache) = self.cache() {
            if store_credentials {
                cache.save_credentials(&reusable_credentials);
            }
        }

        let (tx_connection, rx_connection) = mpsc::unbounded_channel();
        self.0
            .tx_connection
            .set(tx_connection)
            .map_err(|_| SessionError::NotConnected)?;

        let (sink, stream) = transport.split();
        let sender_task = UnboundedReceiverStream::new(rx_connection)
            .map(Ok)
            .forward(sink);
        let receiver_task = DispatchTask(stream, self.weak());

        tokio::spawn(async move {
            let result = future::try_join(sender_task, receiver_task).await;

            if let Err(e) = result {
                error!("{}", e);
            }
        });

        Ok(())
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
        self.0.data.read().time_delta
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
            "Session strong={} weak={}",
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

    fn dispatch(&self, cmd: u8, data: Bytes) -> Result<(), Error> {
        use PacketType::*;

        let packet_type = FromPrimitive::from_u8(cmd);
        let cmd = match packet_type {
            Some(cmd) => cmd,
            None => {
                trace!("Ignoring unknown packet {:x}", cmd);
                return Err(SessionError::Packet(cmd).into());
            }
        };

        match packet_type {
            Some(Ping) => {
                let server_timestamp = BigEndian::read_u32(data.as_ref()) as i64;
                let timestamp = match SystemTime::now().duration_since(UNIX_EPOCH) {
                    Ok(dur) => dur,
                    Err(err) => err.duration(),
                }
                .as_secs() as i64;

                self.0.data.write().time_delta = server_timestamp - timestamp;

                self.debug_info();
                self.send_packet(Pong, vec![0, 0, 0, 0])
            }
            Some(CountryCode) => {
                let country = String::from_utf8(data.as_ref().to_owned())?;
                info!("Country: {:?}", country);
                self.0.data.write().user_data.country = country;
                Ok(())
            }
            Some(StreamChunkRes) | Some(ChannelError) => self.channel().dispatch(cmd, data),
            Some(AesKey) | Some(AesKeyError) => self.audio_key().dispatch(cmd, data),
            Some(MercuryReq) | Some(MercurySub) | Some(MercuryUnsub) | Some(MercuryEvent) => {
                self.mercury().dispatch(cmd, data)
            }
            Some(ProductInfo) => {
                let data = std::str::from_utf8(&data)?;
                let mut reader = quick_xml::Reader::from_str(data);

                let mut buf = Vec::new();
                let mut current_element = String::new();
                let mut user_attributes: UserAttributes = HashMap::new();

                loop {
                    match reader.read_event(&mut buf) {
                        Ok(Event::Start(ref element)) => {
                            current_element = std::str::from_utf8(element.name())?.to_owned()
                        }
                        Ok(Event::End(_)) => {
                            current_element = String::new();
                        }
                        Ok(Event::Text(ref value)) => {
                            if !current_element.is_empty() {
                                let _ = user_attributes.insert(
                                    current_element.clone(),
                                    value.unescape_and_decode(&reader)?,
                                );
                            }
                        }
                        Ok(Event::Eof) => break,
                        Ok(_) => (),
                        Err(e) => warn!(
                            "Error parsing XML at position {}: {:?}",
                            reader.buffer_position(),
                            e
                        ),
                    }
                }

                trace!("Received product info: {:#?}", user_attributes);
                Self::check_catalogue(&user_attributes);

                self.0.data.write().user_data.attributes = user_attributes;
                Ok(())
            }
            Some(PongAck)
            | Some(SecretBlock)
            | Some(LegacyWelcome)
            | Some(UnknownDataAllZeros)
            | Some(LicenseVersion) => Ok(()),
            _ => {
                trace!("Ignoring {:?} packet with data {:#?}", cmd, data);
                Err(SessionError::Packet(cmd as u8).into())
            }
        }
    }

    pub fn send_packet(&self, cmd: PacketType, data: Vec<u8>) -> Result<(), Error> {
        match self.0.tx_connection.get() {
            Some(tx) => Ok(tx.send((cmd as u8, data))?),
            None => Err(SessionError::NotConnected.into()),
        }
    }

    pub fn cache(&self) -> Option<&Arc<Cache>> {
        self.0.cache.as_ref()
    }

    pub fn config(&self) -> &SessionConfig {
        &self.0.config
    }

    // This clones a fairly large struct, so use a specific getter or setter unless
    // you need more fields at once, in which case this can spare multiple `read`
    // locks.
    pub fn user_data(&self) -> UserData {
        self.0.data.read().user_data.clone()
    }

    pub fn device_id(&self) -> &str {
        &self.config().device_id
    }

    pub fn client_id(&self) -> String {
        self.0.data.read().client_id.clone()
    }

    pub fn set_client_id(&self, client_id: &str) {
        self.0.data.write().client_id = client_id.to_owned();
    }

    pub fn client_name(&self) -> String {
        self.0.data.read().client_name.clone()
    }

    pub fn set_client_name(&self, client_name: &str) {
        self.0.data.write().client_name = client_name.to_owned();
    }

    pub fn client_brand_name(&self) -> String {
        self.0.data.read().client_brand_name.clone()
    }

    pub fn set_client_brand_name(&self, client_brand_name: &str) {
        self.0.data.write().client_brand_name = client_brand_name.to_owned();
    }

    pub fn client_model_name(&self) -> String {
        self.0.data.read().client_model_name.clone()
    }

    pub fn set_client_model_name(&self, client_model_name: &str) {
        self.0.data.write().client_model_name = client_model_name.to_owned();
    }

    pub fn connection_id(&self) -> String {
        self.0.data.read().connection_id.clone()
    }

    pub fn set_connection_id(&self, connection_id: &str) {
        self.0.data.write().connection_id = connection_id.to_owned();
    }

    pub fn username(&self) -> String {
        self.0.data.read().user_data.canonical_username.clone()
    }

    pub fn set_username(&self, username: &str) {
        self.0.data.write().user_data.canonical_username = username.to_owned();
    }

    pub fn country(&self) -> String {
        self.0.data.read().user_data.country.clone()
    }

    pub fn filter_explicit_content(&self) -> bool {
        match self.get_user_attribute("filter-explicit-content") {
            Some(value) => matches!(&*value, "1"),
            None => false,
        }
    }

    pub fn autoplay(&self) -> bool {
        if let Some(overide) = self.config().autoplay {
            return overide;
        }

        match self.get_user_attribute("autoplay") {
            Some(value) => matches!(&*value, "1"),
            None => false,
        }
    }

    pub fn set_user_attribute(&self, key: &str, value: &str) -> Option<String> {
        let mut dummy_attributes = UserAttributes::new();
        dummy_attributes.insert(key.to_owned(), value.to_owned());
        Self::check_catalogue(&dummy_attributes);

        self.0
            .data
            .write()
            .user_data
            .attributes
            .insert(key.to_owned(), value.to_owned())
    }

    pub fn set_user_attributes(&self, attributes: UserAttributes) {
        Self::check_catalogue(&attributes);

        self.0.data.write().user_data.attributes.extend(attributes)
    }

    pub fn get_user_attribute(&self, key: &str) -> Option<String> {
        self.0
            .data
            .read()
            .user_data
            .attributes
            .get(key)
            .map(Clone::clone)
    }

    fn weak(&self) -> SessionWeak {
        SessionWeak(Arc::downgrade(&self.0))
    }

    pub fn shutdown(&self) {
        debug!("Invalidating session");
        self.0.data.write().invalid = true;
        self.mercury().shutdown();
        self.channel().shutdown();
    }

    pub fn is_invalid(&self) -> bool {
        self.0.data.read().invalid
    }
}

#[derive(Clone)]
pub struct SessionWeak(Weak<SessionInternal>);

impl SessionWeak {
    fn try_upgrade(&self) -> Option<Session> {
        self.0.upgrade().map(Session)
    }

    pub(crate) fn upgrade(&self) -> Session {
        self.try_upgrade()
            .expect("session was dropped and so should have this component")
    }
}

impl Drop for SessionInternal {
    fn drop(&mut self) {
        debug!("drop Session");
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

            if let Err(e) = session.dispatch(cmd, data) {
                debug!("could not dispatch command: {}", e);
            }
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
