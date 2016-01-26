use crypto::digest::Digest;
use crypto::sha1::Sha1;
use eventual::Future;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Mutex, RwLock, Arc, mpsc};

use audio_key::{AudioKeyManager, AudioKey, AudioKeyError};
use audio_file::{AudioFileManager, AudioFile};
use connection::{self, CipherConnection};
use connection::PacketHandler;
use mercury::{MercuryManager, MercuryRequest, MercuryResponse};
use metadata::{MetadataManager, MetadataRef, MetadataTrait};
use stream::{StreamManager, StreamEvent};
use util::{SpotifyId, FileId, mkdir_existing};

pub enum Bitrate {
    Bitrate96,
    Bitrate160,
    Bitrate320,
}

pub struct Config {
    pub application_key: Vec<u8>,
    pub user_agent: String,
    pub device_name: String,
    pub cache_location: PathBuf,
    pub bitrate: Bitrate,
}

pub struct SessionData {
    country: String,
    canonical_username: String,
    device_id: String,
}

pub struct SessionInternal {
    config: Config,
    data: RwLock<SessionData>,

    mercury: Mutex<MercuryManager>,
    metadata: Mutex<MetadataManager>,
    stream: Mutex<StreamManager>,
    audio_key: Mutex<AudioKeyManager>,
    audio_file: Mutex<AudioFileManager>,
    rx_connection: Mutex<Option<CipherConnection>>,
    tx_connection: Mutex<Option<CipherConnection>>,
}

#[derive(Clone)]
pub struct Session(pub Arc<SessionInternal>);

impl Session {
    pub fn new(config: Config) -> Session {
        mkdir_existing(&config.cache_location).unwrap();

        let device_id = {
            let mut h = Sha1::new();
            h.input_str(&config.device_name);
            h.result_str()
        };

        Session(Arc::new(SessionInternal {
            config: config,
            data: RwLock::new(SessionData {
                country: String::new(),
                canonical_username: String::new(),
                device_id: device_id,
            }),

            rx_connection: Mutex::new(None),
            tx_connection: Mutex::new(None),

            mercury: Mutex::new(MercuryManager::new()),
            metadata: Mutex::new(MetadataManager::new()),
            stream: Mutex::new(StreamManager::new()),
            audio_key: Mutex::new(AudioKeyManager::new()),
            audio_file: Mutex::new(AudioFileManager::new()),
        }))
    }

    pub fn authenticated(&self, username: String, connection: CipherConnection) {
        self.0.data.write().unwrap().canonical_username = username;
        *self.0.rx_connection.lock().unwrap() = Some(connection.clone());
        *self.0.tx_connection.lock().unwrap() = Some(connection);
    }

    pub fn poll(&self) {
        let (cmd, data) = self.recv();

        match cmd {
            0x4 => self.send_packet(0x49, &data).unwrap(),
            0x4a => (),
            0x9 => self.0.stream.lock().unwrap().handle(cmd, data),
            0xd | 0xe => self.0.audio_key.lock().unwrap().handle(cmd, data),
            0x1b => {
                self.0.data.write().unwrap().country = String::from_utf8(data).unwrap();
            }
            0xb2...0xb6 => self.0.mercury.lock().unwrap().handle(cmd, data),
            _ => (),
        }
    }

    pub fn recv(&self) -> (u8, Vec<u8>) {
        self.0.rx_connection.lock().unwrap().as_mut().unwrap().recv_packet().unwrap()
    }

    pub fn send_packet(&self, cmd: u8, data: &[u8]) -> connection::Result<()> {
        self.0.tx_connection.lock().unwrap().as_mut().unwrap().send_packet(cmd, data)
    }

    pub fn audio_key(&self, track: SpotifyId, file: FileId) -> Future<AudioKey, AudioKeyError> {
        self.0.audio_key.lock().unwrap().request(self, track, file)
    }

    pub fn audio_file(&self, file: FileId) -> AudioFile {
        self.0.audio_file.lock().unwrap().request(self, file)
    }

    pub fn stream(&self, file: FileId, offset: u32, size: u32) -> mpsc::Receiver<StreamEvent> {
        self.0.stream.lock().unwrap().request(self, file, offset, size)
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

    pub fn config(&self) -> &Config {
        &self.0.config
    }

    pub fn username(&self) -> String {
        self.0.data.read().unwrap().canonical_username.clone()
    }

    pub fn country(&self) -> String {
        self.0.data.read().unwrap().country.clone()
    }

    pub fn device_id(&self) -> String {
        self.0.data.read().unwrap().device_id.clone()
    }
}
