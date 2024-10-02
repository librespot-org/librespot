use std::{
    collections::HashMap,
    future::Future,
    io,
    pin::Pin,
    process::exit,
    sync::{Arc, Weak},
    task::{Context, Poll},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use byteorder::{BigEndian, ByteOrder};
use bytes::Bytes;
use futures_core::TryStream;
use futures_util::StreamExt;
use librespot_protocol::authentication::AuthenticationType;
use num_traits::FromPrimitive;
use once_cell::sync::OnceCell;
use parking_lot::RwLock;
use pin_project_lite::pin_project;
use quick_xml::events::Event;
use thiserror::Error;
use tokio::{
    sync::mpsc,
    time::{sleep, Duration as TokioDuration, Instant as TokioInstant, Sleep},
};
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::{
    apresolve::{ApResolver, SocketAddress},
    audio_key::AudioKeyManager,
    authentication::Credentials,
    cache::Cache,
    channel::ChannelManager,
    config::SessionConfig,
    connection::{self, AuthenticationError, Transport},
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
    auth_data: Vec<u8>,
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

/// A shared reference to a Spotify session.
///
/// After instantiating, you need to login via [Session::connect].
/// You can either implement the whole playback logic yourself by using
/// this structs interface directly or hand it to a
/// `Player`.
///
/// *Note*: [Session] instances cannot yet be reused once invalidated. After
/// an unexpectedly closed connection, you'll need to create a new [Session].
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

    async fn connect_inner(
        &self,
        access_point: &SocketAddress,
        credentials: Credentials,
    ) -> Result<(Credentials, Transport), Error> {
        const MAX_RETRIES: u8 = 1;
        let mut transport = connection::connect_with_retry(
            &access_point.0,
            access_point.1,
            self.config().proxy.as_ref(),
            MAX_RETRIES,
        )
        .await?;
        let mut reusable_credentials = connection::authenticate(
            &mut transport,
            credentials.clone(),
            &self.config().device_id,
        )
        .await?;

        // Might be able to remove this once keymaster is replaced with login5.
        if credentials.auth_type == AuthenticationType::AUTHENTICATION_SPOTIFY_TOKEN {
            trace!(
                "Reconnect using stored credentials as token authed sessions cannot use keymaster."
            );
            transport = connection::connect_with_retry(
                &access_point.0,
                access_point.1,
                self.config().proxy.as_ref(),
                MAX_RETRIES,
            )
            .await?;
            reusable_credentials = connection::authenticate(
                &mut transport,
                reusable_credentials.clone(),
                &self.config().device_id,
            )
            .await?;
        }

        Ok((reusable_credentials, transport))
    }

    pub async fn connect(
        &self,
        credentials: Credentials,
        store_credentials: bool,
    ) -> Result<(), Error> {
        // There currently happen to be 6 APs but anything will do to avoid an infinite loop.
        const MAX_AP_TRIES: u8 = 6;
        let mut num_ap_tries = 0;
        let (reusable_credentials, transport) = loop {
            let ap = self.apresolver().resolve("accesspoint").await?;
            info!("Connecting to AP \"{}:{}\"", ap.0, ap.1);
            match self.connect_inner(&ap, credentials.clone()).await {
                Ok(ct) => break ct,
                Err(e) => {
                    num_ap_tries += 1;
                    if MAX_AP_TRIES == num_ap_tries {
                        error!("Tried too many access points");
                        return Err(e);
                    }
                    if let Some(AuthenticationError::LoginFailed(ErrorCode::TryAnotherAP)) =
                        e.error.downcast_ref::<AuthenticationError>()
                    {
                        warn!("Instructed to try another access point...");
                        continue;
                    } else if let Some(AuthenticationError::LoginFailed(..)) =
                        e.error.downcast_ref::<AuthenticationError>()
                    {
                        return Err(e);
                    } else {
                        warn!("Try another access point...");
                        continue;
                    }
                }
            }
        };

        let username = reusable_credentials
            .username
            .as_ref()
            .map_or("UNKNOWN", |s| s.as_str());
        info!("Authenticated as '{username}' !");
        self.set_username(username);
        self.set_auth_data(&reusable_credentials.auth_data);
        if let Some(cache) = self.cache() {
            if store_credentials {
                let cred_changed = cache
                    .credentials()
                    .map(|c| c != reusable_credentials)
                    .unwrap_or(true);
                if cred_changed {
                    cache.save_credentials(&reusable_credentials);
                }
            }
        }

        // This channel serves as a buffer for packets and serializes access to the TcpStream, such
        // that `self.send_packet` can return immediately and needs no additional synchronization.
        let (tx_connection, rx_connection) = mpsc::unbounded_channel();
        self.0
            .tx_connection
            .set(tx_connection)
            .map_err(|_| SessionError::NotConnected)?;

        let (sink, stream) = transport.split();
        let sender_task = UnboundedReceiverStream::new(rx_connection)
            .map(Ok)
            .forward(sink);
        let session_weak = self.weak();
        tokio::spawn(async move {
            if let Err(e) = sender_task.await {
                error!("{}", e);
                if let Some(session) = session_weak.try_upgrade() {
                    if !session.is_invalid() {
                        session.shutdown();
                    }
                }
            }
        });

        tokio::spawn(DispatchTask::new(self.weak(), stream));

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
        client_id.clone_into(&mut self.0.data.write().client_id);
    }

    pub fn client_name(&self) -> String {
        self.0.data.read().client_name.clone()
    }

    pub fn set_client_name(&self, client_name: &str) {
        client_name.clone_into(&mut self.0.data.write().client_name);
    }

    pub fn client_brand_name(&self) -> String {
        self.0.data.read().client_brand_name.clone()
    }

    pub fn set_client_brand_name(&self, client_brand_name: &str) {
        client_brand_name.clone_into(&mut self.0.data.write().client_brand_name);
    }

    pub fn client_model_name(&self) -> String {
        self.0.data.read().client_model_name.clone()
    }

    pub fn set_client_model_name(&self, client_model_name: &str) {
        client_model_name.clone_into(&mut self.0.data.write().client_model_name);
    }

    pub fn connection_id(&self) -> String {
        self.0.data.read().connection_id.clone()
    }

    pub fn set_connection_id(&self, connection_id: &str) {
        connection_id.clone_into(&mut self.0.data.write().connection_id);
    }

    pub fn username(&self) -> String {
        self.0.data.read().user_data.canonical_username.clone()
    }

    pub fn set_username(&self, username: &str) {
        username.clone_into(&mut self.0.data.write().user_data.canonical_username);
    }

    pub fn auth_data(&self) -> Vec<u8> {
        self.0.data.read().auth_data.clone()
    }

    pub fn set_auth_data(&self, auth_data: &[u8]) {
        self.0.data.write().auth_data = auth_data.to_owned();
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
        self.0.data.read().user_data.attributes.get(key).cloned()
    }

    fn weak(&self) -> SessionWeak {
        SessionWeak(Arc::downgrade(&self.0))
    }

    pub fn shutdown(&self) {
        debug!("Shutdown: Invalidating session");
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

#[derive(Clone, Copy, Default, Debug, PartialEq)]
enum KeepAliveState {
    #[default]
    // Expecting a Ping from the server, either after startup or after a PongAck.
    ExpectingPing,

    // We need to send a Pong at the given time.
    PendingPong,

    // We just sent a Pong and wait for it be ACK'd.
    ExpectingPongAck,
}

const INITIAL_PING_TIMEOUT: TokioDuration = TokioDuration::from_secs(5);
const PING_TIMEOUT: TokioDuration = TokioDuration::from_secs(65);
const PONG_DELAY: TokioDuration = TokioDuration::from_secs(60);
const PONG_ACK_TIMEOUT: TokioDuration = TokioDuration::from_secs(5);

impl KeepAliveState {
    fn debug(&self, sleep: &Sleep) {
        let delay = sleep
            .deadline()
            .checked_duration_since(TokioInstant::now())
            .map(|t| t.as_secs_f64())
            .unwrap_or(f64::INFINITY);

        trace!("keep-alive state: {:?}, timeout in {:.1}", self, delay);
    }
}

pin_project! {
    struct DispatchTask<S>
    where
        S: TryStream<Ok = (u8, Bytes)>
    {
        session: SessionWeak,
        keep_alive_state: KeepAliveState,
        #[pin]
        stream: S,
        #[pin]
        timeout: Sleep,
    }

    impl<S> PinnedDrop for DispatchTask<S>
    where
        S: TryStream<Ok = (u8, Bytes)>
    {
        fn drop(_this: Pin<&mut Self>) {
            debug!("drop Dispatch");
        }
    }
}

impl<S> DispatchTask<S>
where
    S: TryStream<Ok = (u8, Bytes)>,
{
    fn new(session: SessionWeak, stream: S) -> Self {
        Self {
            session,
            keep_alive_state: KeepAliveState::ExpectingPing,
            stream,
            timeout: sleep(INITIAL_PING_TIMEOUT),
        }
    }

    fn dispatch(
        mut self: Pin<&mut Self>,
        session: &Session,
        cmd: u8,
        data: Bytes,
    ) -> Result<(), Error> {
        use KeepAliveState::*;
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
                trace!("Received Ping");
                if self.keep_alive_state != ExpectingPing {
                    warn!("Received unexpected Ping from server")
                }
                let mut this = self.as_mut().project();
                *this.keep_alive_state = PendingPong;
                this.timeout
                    .as_mut()
                    .reset(TokioInstant::now() + PONG_DELAY);
                this.keep_alive_state.debug(&this.timeout);

                let server_timestamp = BigEndian::read_u32(data.as_ref()) as i64;
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or(Duration::ZERO)
                    .as_secs() as i64;
                {
                    let mut data = session.0.data.write();
                    data.time_delta = server_timestamp.saturating_sub(timestamp);
                }

                session.debug_info();

                Ok(())
            }
            Some(PongAck) => {
                trace!("Received PongAck");
                if self.keep_alive_state != ExpectingPongAck {
                    warn!("Received unexpected PongAck from server")
                }
                let mut this = self.as_mut().project();
                *this.keep_alive_state = ExpectingPing;
                this.timeout
                    .as_mut()
                    .reset(TokioInstant::now() + PING_TIMEOUT);
                this.keep_alive_state.debug(&this.timeout);

                Ok(())
            }
            Some(CountryCode) => {
                let country = String::from_utf8(data.as_ref().to_owned())?;
                info!("Country: {:?}", country);
                session.0.data.write().user_data.country = country;
                Ok(())
            }
            Some(StreamChunkRes) | Some(ChannelError) => session.channel().dispatch(cmd, data),
            Some(AesKey) | Some(AesKeyError) => session.audio_key().dispatch(cmd, data),
            Some(MercuryReq) | Some(MercurySub) | Some(MercuryUnsub) | Some(MercuryEvent) => {
                session.mercury().dispatch(cmd, data)
            }
            Some(ProductInfo) => {
                let data = std::str::from_utf8(&data)?;
                let mut reader = quick_xml::Reader::from_str(data);

                let mut buf = Vec::new();
                let mut current_element = String::new();
                let mut user_attributes: UserAttributes = HashMap::new();

                loop {
                    match reader.read_event_into(&mut buf) {
                        Ok(Event::Start(ref element)) => {
                            std::str::from_utf8(element)?.clone_into(&mut current_element)
                        }
                        Ok(Event::End(_)) => {
                            current_element = String::new();
                        }
                        Ok(Event::Text(ref value)) => {
                            if !current_element.is_empty() {
                                let _ = user_attributes
                                    .insert(current_element.clone(), value.unescape()?.to_string());
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
                Session::check_catalogue(&user_attributes);

                session.0.data.write().user_data.attributes = user_attributes;
                Ok(())
            }
            Some(SecretBlock)
            | Some(LegacyWelcome)
            | Some(UnknownDataAllZeros)
            | Some(LicenseVersion) => Ok(()),
            _ => {
                trace!("Ignoring {:?} packet with data {:#?}", cmd, data);
                Err(SessionError::Packet(cmd as u8).into())
            }
        }
    }
}

impl<S> Future for DispatchTask<S>
where
    S: TryStream<Ok = (u8, Bytes), Error = std::io::Error>,
    <S as TryStream>::Ok: std::fmt::Debug,
{
    type Output = Result<(), S::Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        use KeepAliveState::*;

        let session = match self.session.try_upgrade() {
            Some(session) => session,
            None => return Poll::Ready(Ok(())),
        };

        // Process all messages that are immediately ready
        loop {
            match self.as_mut().project().stream.try_poll_next(cx) {
                Poll::Ready(Some(Ok((cmd, data)))) => {
                    let result = self.as_mut().dispatch(&session, cmd, data);
                    if let Err(e) = result {
                        debug!("could not dispatch command: {}", e);
                    }
                }
                Poll::Ready(None) => {
                    warn!("Connection to server closed.");
                    session.shutdown();
                    return Poll::Ready(Ok(()));
                }
                Poll::Ready(Some(Err(e))) => {
                    error!("Connection to server closed.");
                    session.shutdown();
                    return Poll::Ready(Err(e));
                }
                Poll::Pending => break,
            }
        }

        // Handle the keee-alive sequence, returning an error when we haven't received a
        // Ping/PongAck for too long.
        //
        // The expected keepalive sequence is
        // - Server: Ping
        // - wait 60s
        // - Client: Pong
        // - Server: PongAck
        // - wait 60s
        // - repeat
        //
        // This means that we silently lost connection to Spotify servers if
        // - we don't receive a Ping 60s after the last PongAck, or
        // - we don't receive a PongAck immediately after our Pong.
        //
        // Currently, we add a safety margin of 5s to these expected deadlines.
        let mut this = self.as_mut().project();
        if let Poll::Ready(()) = this.timeout.as_mut().poll(cx) {
            match this.keep_alive_state {
                ExpectingPing | ExpectingPongAck => {
                    if !session.is_invalid() {
                        session.shutdown();
                    }
                    // TODO: Optionally reconnect (with cached/last credentials?)
                    return Poll::Ready(Err(io::Error::new(
                        io::ErrorKind::TimedOut,
                        "session lost connection to server",
                    )));
                }
                PendingPong => {
                    trace!("Sending Pong");
                    let _ = session.send_packet(PacketType::Pong, vec![0, 0, 0, 0]);
                    *this.keep_alive_state = ExpectingPongAck;
                    this.timeout
                        .as_mut()
                        .reset(TokioInstant::now() + PONG_ACK_TIMEOUT);
                    this.keep_alive_state.debug(&this.timeout);
                }
            }
        }

        Poll::Pending
    }
}
