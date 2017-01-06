use crypto::digest::Digest;
use crypto::sha1::Sha1;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use eventual;
use eventual::Future;
use eventual::Async;
use protobuf::{self, Message};
use rand::thread_rng;
use std::io::{Read, Write, Cursor};
use std::result::Result;
use std::sync::{Mutex, RwLock, Arc, mpsc};

use album_cover::AlbumCover;
use apresolve::apresolve;
use audio_key::{AudioKeyManager, AudioKey, AudioKeyError};
use audio_file::AudioFile;
use authentication::Credentials;
use cache::Cache;
use connection::{self, PlainConnection, CipherConnection};
use diffie_hellman::DHLocalKeys;
use mercury::{MercuryManager, MercuryRequest, MercuryResponse};
use metadata::{MetadataManager, MetadataRef, MetadataTrait};
use protocol;
use stream::StreamManager;
use util::{self, SpotifyId, FileId, ReadSeek};
use version;

use stream;

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum Bitrate {
    Bitrate96,
    Bitrate160,
    Bitrate320,
}

pub struct Config {
    pub user_agent: String,
    pub device_name: String,
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
    device_id: String,
    data: RwLock<SessionData>,

    cache: Box<Cache + Send + Sync>,
    mercury: Mutex<MercuryManager>,
    metadata: Mutex<MetadataManager>,
    stream: Mutex<StreamManager>,
    audio_key: Mutex<AudioKeyManager>,
    rx_connection: Mutex<Option<CipherConnection>>,
    tx_connection: Mutex<Option<CipherConnection>>,
}

#[derive(Clone)]
pub struct Session(pub Arc<SessionInternal>);

impl Session {
    pub fn new(config: Config, cache: Box<Cache + Send + Sync>) -> Session {
        let device_id = {
            let mut h = Sha1::new();
            h.input_str(&config.device_name);
            h.result_str()
        };

        Session(Arc::new(SessionInternal {
            config: config,
            device_id: device_id,
            data: RwLock::new(SessionData {
                country: String::new(),
                canonical_username: String::new(),
            }),

            rx_connection: Mutex::new(None),
            tx_connection: Mutex::new(None),

            cache: cache,
            mercury: Mutex::new(MercuryManager::new()),
            metadata: Mutex::new(MetadataManager::new()),
            stream: Mutex::new(StreamManager::new()),
            audio_key: Mutex::new(AudioKeyManager::new()),
        }))
    }

    fn connect(&self) -> CipherConnection {
        let local_keys = DHLocalKeys::random(&mut thread_rng());

        let ap = apresolve();

        info!("Connecting to AP {}", ap);
        let mut connection = PlainConnection::connect(&ap).unwrap();

        let request = protobuf_init!(protocol::keyexchange::ClientHello::new(), {
            build_info => {
                product: protocol::keyexchange::Product::PRODUCT_PARTNER,
                platform: protocol::keyexchange::Platform::PLATFORM_LINUX_X86,
                version: 0x10800000000,
            },
            cryptosuites_supported => [
                protocol::keyexchange::Cryptosuite::CRYPTO_SUITE_SHANNON,
            ],
            login_crypto_hello.diffie_hellman => {
                gc: local_keys.public_key(),
                server_keys_known: 1,
            },
            client_nonce: util::rand_vec(&mut thread_rng(), 0x10),
            padding: vec![0x1e],
        });

        let init_client_packet = connection.send_packet_prefix(&[0, 4],
                                                               &request.write_to_bytes().unwrap())
                                           .unwrap();
        let init_server_packet = connection.recv_packet().unwrap();

        let response: protocol::keyexchange::APResponseMessage =
            protobuf::parse_from_bytes(&init_server_packet[4..]).unwrap();

        let remote_key = response.get_challenge()
                                 .get_login_crypto_challenge()
                                 .get_diffie_hellman()
                                 .get_gs();

        let shared_secret = local_keys.shared_secret(remote_key);
        let (challenge, send_key, recv_key) = {
            let mut data = Vec::with_capacity(0x64);
            let mut mac = Hmac::new(Sha1::new(), &shared_secret);

            for i in 1..6 {
                mac.input(&init_client_packet);
                mac.input(&init_server_packet);
                mac.input(&[i]);
                data.write(&mac.result().code()).unwrap();
                mac.reset();
            }

            mac = Hmac::new(Sha1::new(), &data[..0x14]);
            mac.input(&init_client_packet);
            mac.input(&init_server_packet);

            (mac.result().code().to_vec(),
             data[0x14..0x34].to_vec(),
             data[0x34..0x54].to_vec())
        };

        let packet = protobuf_init!(protocol::keyexchange::ClientResponsePlaintext::new(), {
            login_crypto_response.diffie_hellman => {
                hmac: challenge
            },
            pow_response => {},
            crypto_response => {},
        });


        connection.send_packet(&packet.write_to_bytes().unwrap()).unwrap();

        CipherConnection::new(connection.into_stream(),
                              &send_key,
                              &recv_key)
    }

    pub fn login(&self, credentials: Credentials) -> Result<Credentials, ()> {
        let packet = protobuf_init!(protocol::authentication::ClientResponseEncrypted::new(), {
            login_credentials => {
                username: credentials.username,
                typ: credentials.auth_type,
                auth_data: credentials.auth_data,
            },
            system_info => {
                cpu_family: protocol::authentication::CpuFamily::CPU_UNKNOWN,
                os: protocol::authentication::Os::OS_UNKNOWN,
                system_information_string: "librespot".to_owned(),
                device_id: self.device_id().to_owned(),
            },
            version_string: version::version_string(),
        });

        let mut connection = self.connect();
        connection.send_packet(0xab, &packet.write_to_bytes().unwrap()).unwrap();
        let (cmd, data) = connection.recv_packet().unwrap();

        match cmd {
            0xac => {
                let welcome_data: protocol::authentication::APWelcome =
                    protobuf::parse_from_bytes(&data).unwrap();

                let username = welcome_data.get_canonical_username().to_owned();
                self.0.data.write().unwrap().canonical_username = username.clone();
                *self.0.rx_connection.lock().unwrap() = Some(connection.clone());
                *self.0.tx_connection.lock().unwrap() = Some(connection);

                info!("Authenticated !");

                let reusable_credentials = Credentials {
                    username: username,
                    auth_type: welcome_data.get_reusable_auth_credentials_type(),
                    auth_data: welcome_data.get_reusable_auth_credentials().to_owned(),
                };

                self.0.cache.put_credentials(&reusable_credentials);

                Ok(reusable_credentials)
            }

            0xad => {
                let msg: protocol::keyexchange::APLoginFailed =
                    protobuf::parse_from_bytes(&data).unwrap();
                error!("Authentication failed, {:?}", msg);
                Err(())
            }
            _ => {
                error!("Unexpected message {:x}", cmd);
                Err(())
            }
        }
    }

    pub fn poll(&self) {
        let (cmd, data) = self.recv();

        match cmd {
            0x4 => self.send_packet(0x49, &data).unwrap(),
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
        self.0.rx_connection.lock().unwrap().as_mut().unwrap().recv_packet().unwrap()
    }

    pub fn send_packet(&self, cmd: u8, data: &[u8]) -> connection::Result<()> {
        self.0.tx_connection.lock().unwrap().as_mut().unwrap().send_packet(cmd, data)
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
        &self.0.device_id
    }
}

pub trait PacketHandler {
    fn handle(&mut self, cmd: u8, data: Vec<u8>, session: &Session);
}
