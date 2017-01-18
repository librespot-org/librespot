use crypto::digest::Digest;
use crypto::sha1::Sha1;
use eventual;
use eventual::Future;
use eventual::Async;
use std::io::{self, Read, Cursor};
use std::result::Result;
use std::sync::{Mutex, RwLock, Arc, mpsc};
use std::str::FromStr;
use futures::Future as Future_;
use futures::Stream;
use tokio_core::reactor::Handle;

use album_cover::AlbumCover;
use apresolve::apresolve_or_fallback;
use audio_file::AudioFile;
use audio_key::{AudioKeyManager, AudioKey, AudioKeyError};
use authentication::Credentials;
use cache::Cache;
use connection::{self, adaptor};
use mercury::{MercuryManager, MercuryRequest, MercuryResponse};
use metadata::{MetadataManager, MetadataRef, MetadataTrait};
use stream::StreamManager;
use util::{SpotifyId, FileId, ReadSeek};

use stream;

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
    mercury: Mutex<MercuryManager>,
    metadata: Mutex<MetadataManager>,
    stream: Mutex<StreamManager>,
    audio_key: Mutex<AudioKeyManager>,
    rx_connection: Mutex<adaptor::StreamAdaptor<(u8, Vec<u8>), io::Error>>,
    tx_connection: Mutex<adaptor::SinkAdaptor<(u8, Vec<u8>)>>,
}

#[derive(Clone)]
pub struct Session(pub Arc<SessionInternal>);

pub fn device_id(name: &str) -> String {
    let mut h = Sha1::new();
    h.input_str(&name);
    h.result_str()
}

impl Session {
    pub fn connect(config: Config, credentials: Credentials,
                   cache: Box<Cache + Send + Sync>, handle: Handle)
        -> Box<Future_<Item=(Session, Box<Future_<Item=(), Error=io::Error>>), Error=io::Error>>
    {
        let access_point = apresolve_or_fallback::<io::Error>(&handle);

        let connection = access_point.and_then(move |addr| {
            info!("Connecting to AP \"{}\"", addr);
            connection::connect::<&str>(&addr, &handle)
        });

        let device_id = config.device_id.clone();
        let authentication = connection.and_then(move |connection| {
            connection::authenticate(connection, credentials, device_id)
        });

        let result = authentication.map(move |(transport, reusable_credentials)| {
            info!("Authenticated !");
            cache.put_credentials(&reusable_credentials);

            let (session, task) = Session::create(transport, config, cache, reusable_credentials.username.clone());
            (session, task)
        });
        
        Box::new(result)
    }

    fn create(transport: connection::Transport, config: Config,
              cache: Box<Cache + Send + Sync>, username: String) -> (Session, Box<Future_<Item=(), Error=io::Error>>)
    {
        let transport = transport.map(|(cmd, data)| (cmd, data.as_ref().to_owned()));
        let (tx, rx, task) = adaptor::adapt(transport);

        let session = Session(Arc::new(SessionInternal {
            config: config,
            data: RwLock::new(SessionData {
                country: String::new(),
                canonical_username: username,
            }),

            rx_connection: Mutex::new(rx),
            tx_connection: Mutex::new(tx),

            cache: cache,
            mercury: Mutex::new(MercuryManager::new()),
            metadata: Mutex::new(MetadataManager::new()),
            stream: Mutex::new(StreamManager::new()),
            audio_key: Mutex::new(AudioKeyManager::new()),
        }));

        (session, task)
    }


    pub fn poll(&self) {
        let (cmd, data) = self.recv();

        match cmd {
            0x4 => self.send_packet(0x49, data),
            0x4a => (),
            0x9 | 0xa => self.0.stream.lock().unwrap().handle(cmd, data, self),
            0xd | 0xe => self.0.audio_key.lock().unwrap().handle(cmd, data, self),
            0x1b => {
                self.0.data.write().unwrap().country = String::from_utf8(data).unwrap();
            }
            0xb2...0xb6 => self.0.mercury.lock().unwrap().handle(cmd, data, self),
            _ => (),
        }
    }

    pub fn recv(&self) -> (u8, Vec<u8>) {
        self.0.rx_connection.lock().unwrap().recv().unwrap()
    }

    pub fn send_packet(&self, cmd: u8, data: Vec<u8>) {
        self.0.tx_connection.lock().unwrap().send((cmd, data))
    }

    pub fn audio_key(&self, track: SpotifyId, file_id: FileId) -> Future<AudioKey, AudioKeyError> {
        self.0.cache
            .get_audio_key(track, file_id)
            .map(Future::of)
            .unwrap_or_else(|| {
                let self_ = self.clone();
                self.0.audio_key.lock().unwrap()
                    .request(self, track, file_id)
                    .map(move |key| {
                        self_.0.cache.put_audio_key(track, file_id, key);
                        key
                    })
            })
    }

    pub fn audio_file(&self, file_id: FileId) -> Box<ReadSeek> {
        self.0.cache
            .get_file(file_id)
            .unwrap_or_else(|| {
                let (audio_file, complete_rx) = AudioFile::new(self, file_id);

                let self_ = self.clone();
                complete_rx.map(move |mut complete_file| {
                    self_.0.cache.put_file(file_id, &mut complete_file)
                }).fire();

                Box::new(audio_file.await().unwrap())
            })
    }

    pub fn album_cover(&self, file_id: FileId) -> eventual::Future<Vec<u8>, ()> {
        self.0.cache
            .get_file(file_id)
            .map(|mut f| {
                let mut data = Vec::new();
                f.read_to_end(&mut data).unwrap();
                Future::of(data)
            })
            .unwrap_or_else(|| {
                  let self_ = self.clone();
                  AlbumCover::get(file_id, self)
                      .map(move |data| {
                          self_.0.cache.put_file(file_id, &mut Cursor::new(&data));
                          data
                      })
              })
    }

    pub fn stream(&self, handler: Box<stream::Handler>) {
        self.0.stream.lock().unwrap().create(handler, self)
    }

    pub fn metadata<T: MetadataTrait>(&self, id: SpotifyId) -> MetadataRef<T> {
        self.0.metadata.lock().unwrap().get(self, id)
    }

    pub fn mercury(&self, req: MercuryRequest) -> Future<MercuryResponse, ()> {
        self.0.mercury.lock().unwrap().request(self, req)
    }

    pub fn mercury_sub(&self, uri: String) -> mpsc::Receiver<MercuryResponse> {
        self.0.mercury.lock().unwrap().subscribe(self, uri)
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
}

pub trait PacketHandler {
    fn handle(&mut self, cmd: u8, data: Vec<u8>, session: &Session);
}
