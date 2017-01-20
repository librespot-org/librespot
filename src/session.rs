use crypto::digest::Digest;
use crypto::sha1::Sha1;
use futures::Future;
use futures::sync::mpsc;
use futures::{Stream, BoxFuture, IntoFuture};
use std::io;
use std::result::Result;
use std::str::FromStr;
use std::sync::{RwLock, Arc, Weak};
use tokio_core::reactor::{Handle, Remote};

use apresolve::apresolve_or_fallback;
use authentication::Credentials;
use cache::Cache;
use component::Lazy;
use connection;

use audio_key::AudioKeyManager;
use channel::ChannelManager;
use mercury::MercuryManager;
use metadata::MetadataManager;
use audio_file::AudioFileManager;

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum Bitrate {
    Bitrate96,
    Bitrate160,
    Bitrate320,
}
impl FromStr for Bitrate {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "96" => Ok(Bitrate::Bitrate96),
            "160" => Ok(Bitrate::Bitrate160),
            "320" => Ok(Bitrate::Bitrate320),
            _ => Err(s.into()),
        }
    }
}

pub struct Config {
    pub user_agent: String,
    pub name: String,
    pub device_id: String,
    pub bitrate: Bitrate,
    pub onstart: Option<String>,
    pub onstop: Option<String>,
}

pub struct SessionData {
    country: String,
    canonical_username: String,
}

pub struct SessionInternal {
    config: Config,
    data: RwLock<SessionData>,

    cache: Box<Cache + Send + Sync>,

    tx_connection: mpsc::UnboundedSender<(u8, Vec<u8>)>,

    audio_key: Lazy<AudioKeyManager>,
    audio_file: Lazy<AudioFileManager>,
    channel: Lazy<ChannelManager>,
    mercury: Lazy<MercuryManager>,
    metadata: Lazy<MetadataManager>,

    handle: Remote,
}

#[derive(Clone)]
pub struct Session(pub Arc<SessionInternal>);

#[derive(Clone)]
pub struct SessionWeak(pub Weak<SessionInternal>);

pub fn device_id(name: &str) -> String {
    let mut h = Sha1::new();
    h.input_str(&name);
    h.result_str()
}

impl Session {
    pub fn connect(config: Config, credentials: Credentials,
                   cache: Box<Cache + Send + Sync>, handle: Handle)
        -> Box<Future<Item=Session, Error=io::Error>>
    {
        let access_point = apresolve_or_fallback::<io::Error>(&handle);

        let handle_ = handle.clone();
        let connection = access_point.and_then(move |addr| {
            info!("Connecting to AP \"{}\"", addr);
            connection::connect::<&str>(&addr, &handle_)
        });

        let device_id = config.device_id.clone();
        let authentication = connection.and_then(move |connection| {
            connection::authenticate(connection, credentials, device_id)
        });

        let result = authentication.map(move |(transport, reusable_credentials)| {
            info!("Authenticated !");
            cache.put_credentials(&reusable_credentials);

            let (session, task) = Session::create(
                &handle, transport, config, cache, reusable_credentials.username.clone()
            );

            handle.spawn(task.map_err(|e| panic!(e)));

            session
        });
        
        Box::new(result)
    }

    fn create(handle: &Handle, transport: connection::Transport,
              config: Config, cache: Box<Cache + Send + Sync>,
              username: String)
        -> (Session, BoxFuture<(), io::Error>)
    {
        let transport = transport.map(|(cmd, data)| (cmd, data.as_ref().to_owned()));
        let (sink, stream) = transport.split();

        let (sender_tx, sender_rx) = mpsc::unbounded();

        let sender_task = sender_rx
            .map_err(|e| -> io::Error { panic!(e) })
            .forward(sink).map(|_| ());

        let session = Session(Arc::new(SessionInternal {
            config: config,
            data: RwLock::new(SessionData {
                country: String::new(),
                canonical_username: username,
            }),

            tx_connection: sender_tx,

            cache: cache,

            audio_key: Lazy::new(),
            audio_file: Lazy::new(),
            channel: Lazy::new(),
            mercury: Lazy::new(),
            metadata: Lazy::new(),

            handle: handle.remote().clone(),
        }));

        let receiver_task = {
            let session = session.clone();
            stream.for_each(move |(cmd, data)| {
                session.dispatch(cmd, data);
                Ok(())
            })
        };

        let task = (receiver_task, sender_task).into_future()
            .map(|((), ())| ()).boxed();

        (session, task)
    }

    pub fn audio_key(&self) -> &AudioKeyManager {
        self.0.audio_key.get(|| AudioKeyManager::new(self.weak()))
    }

    pub fn audio_file(&self) -> &AudioFileManager {
        self.0.audio_file.get(|| AudioFileManager::new(self.weak()))
    }

    pub fn channel(&self) -> &ChannelManager {
        self.0.channel.get(|| ChannelManager::new(self.weak()))
    }

    pub fn mercury(&self) -> &MercuryManager {
        self.0.mercury.get(|| MercuryManager::new(self.weak()))
    }

    pub fn metadata(&self) -> &MetadataManager {
        self.0.metadata.get(|| MetadataManager::new(self.weak()))
    }

    pub fn spawn<F, R>(&self, f: F)
        where F: FnOnce(&Handle) -> R + Send + 'static,
              R: IntoFuture<Item=(), Error=()>,
              R::Future: 'static
    {
        self.0.handle.spawn(f)
    }

    fn dispatch(&self, cmd: u8, data: Vec<u8>) {
        match cmd {
            0x4 => self.send_packet(0x49, data),
            0x4a => (),
            0x1b => {
                self.0.data.write().unwrap().country = String::from_utf8(data).unwrap();
            }

            0x9 | 0xa => self.channel().dispatch(cmd, data),
            0xd | 0xe => self.audio_key().dispatch(cmd, data),
            0xb2...0xb6 => self.mercury().dispatch(cmd, data),
            _ => (),
        }
    }

    pub fn send_packet(&self, cmd: u8, data: Vec<u8>) {
        self.0.tx_connection.send((cmd, data)).unwrap();
    }

    pub fn cache(&self) -> &Cache {
        self.0.cache.as_ref()
    }

    pub fn config(&self) -> &Config {
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

    pub fn weak(&self) -> SessionWeak {
        SessionWeak(Arc::downgrade(&self.0))
    }
}

impl SessionWeak {
    pub fn upgrade(&self) -> Session {
        Session(self.0.upgrade().expect("Session died"))
    }
}
