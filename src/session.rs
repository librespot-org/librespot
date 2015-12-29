use crypto::digest::Digest;
use crypto::sha1::Sha1;
use eventual::Future;
use protobuf::{self, Message};
use rand::thread_rng;
use std::sync::{Mutex, RwLock, Arc, mpsc};
use std::path::PathBuf;

use connection::{self, PlainConnection, CipherConnection};
use keys::PrivateKeys;
use librespot_protocol as protocol;
use util::{SpotifyId, FileId, mkdir_existing};

use mercury::{MercuryManager, MercuryRequest, MercuryResponse};
use metadata::{MetadataManager, MetadataRef, MetadataTrait};
use stream::{StreamManager, StreamEvent};
use audio_key::{AudioKeyManager, AudioKey};
use audio_file::{AudioFileManager, AudioFile};
use connection::PacketHandler;

use util;

pub struct Config {
    pub application_key: Vec<u8>,
    pub user_agent: String,
    pub device_id: String,
    pub cache_location: PathBuf,
}

pub struct SessionData {
    pub country: String,
}

pub struct SessionInternal {
    pub config: Config,
    pub data: RwLock<SessionData>,

    mercury: Mutex<MercuryManager>,
    metadata: Mutex<MetadataManager>,
    stream: Mutex<StreamManager>,
    audio_key: Mutex<AudioKeyManager>,
    audio_file: Mutex<AudioFileManager>,
    rx_connection: Mutex<CipherConnection>,
    tx_connection: Mutex<CipherConnection>,
}

#[derive(Clone)]
pub struct Session(pub Arc<SessionInternal>);

impl Session {
    pub fn new(mut config: Config) -> Session {
        config.device_id = {
            let mut h = Sha1::new();
            h.input_str(&config.device_id);
            h.result_str()
        };

        mkdir_existing(&config.cache_location).unwrap();

        let keys = PrivateKeys::new();
        let mut connection = PlainConnection::connect().unwrap();

        let request = protobuf_init!(protocol::keyexchange::ClientHello::new(), {
            build_info => {
                product: protocol::keyexchange::Product::PRODUCT_LIBSPOTIFY_EMBEDDED,
                platform: protocol::keyexchange::Platform::PLATFORM_LINUX_X86,
                version: 0x10800000000,
            },
            /*
            fingerprints_supported => [
                protocol::keyexchange::Fingerprint::FINGERPRINT_GRAIN
            ],
            */
            cryptosuites_supported => [
                protocol::keyexchange::Cryptosuite::CRYPTO_SUITE_SHANNON,
                //protocol::keyexchange::Cryptosuite::CRYPTO_SUITE_RC4_SHA1_HMAC
            ],
            /*
            powschemes_supported => [
                protocol::keyexchange::Powscheme::POW_HASH_CASH
            ],
            */
            login_crypto_hello.diffie_hellman => {
                gc: keys.public_key(),
                server_keys_known: 1,
            },
            client_nonce: util::rand_vec(&mut thread_rng(), 0x10),
            padding: vec![0x1e],
            feature_set => {
                autoupdate2: true,
            }
        });

        let init_client_packet =
            connection.send_packet_prefix(&[0,4], &request.write_to_bytes().unwrap()).unwrap();
        let init_server_packet =
            connection.recv_packet().unwrap();

        let response : protocol::keyexchange::APResponseMessage =
            protobuf::parse_from_bytes(&init_server_packet[4..]).unwrap();

        protobuf_bind!(response, {
            challenge => {
                login_crypto_challenge.diffie_hellman => {
                    gs: remote_key,
                }
            }
        });

        let shared_keys = keys.add_remote_key(remote_key, &init_client_packet, &init_server_packet);

        let packet = protobuf_init!(protocol::keyexchange::ClientResponsePlaintext::new(), {
            login_crypto_response.diffie_hellman => {
                hmac: shared_keys.challenge().to_vec()
            },
            pow_response => {},
            crypto_response => {},
        });

        connection.send_packet(&packet.write_to_bytes().unwrap()).unwrap();

        let cipher_connection = connection.setup_cipher(shared_keys);

        Session(Arc::new(SessionInternal {
            config: config,
            data: RwLock::new(SessionData {
                country: String::new(),
            }),

            rx_connection: Mutex::new(cipher_connection.clone()),
            tx_connection: Mutex::new(cipher_connection),

            mercury: Mutex::new(MercuryManager::new()),
            metadata: Mutex::new(MetadataManager::new()),
            stream: Mutex::new(StreamManager::new()),
            audio_key: Mutex::new(AudioKeyManager::new()),
            audio_file: Mutex::new(AudioFileManager::new()),
        }))
    }

    pub fn login(&self, username: String, password: String) {
        let packet = protobuf_init!(protocol::authentication::ClientResponseEncrypted::new(), {
            login_credentials => {
                username: username,
                typ: protocol::authentication::AuthenticationType::AUTHENTICATION_USER_PASS,
                auth_data: password.into_bytes(),
            },
            system_info => {
                cpu_family: protocol::authentication::CpuFamily::CPU_UNKNOWN,
                os: protocol::authentication::Os::OS_UNKNOWN,
                system_information_string: "librespot".to_owned(),
                device_id: self.0.config.device_id.clone()
            },
            version_string: util::version::version_string(),
            appkey => {
                version: self.0.config.application_key[0] as u32,
                devkey: self.0.config.application_key[0x1..0x81].to_vec(),
                signature: self.0.config.application_key[0x81..0x141].to_vec(),
                useragent: self.0.config.user_agent.clone(),
                callback_hash: vec![0; 20],
            }
        });

        self.send_packet(0xab, &packet.write_to_bytes().unwrap()).unwrap();
    }

    pub fn poll(&self) {
        let (cmd, data) =
            self.0.rx_connection.lock().unwrap().recv_packet().unwrap();

        match cmd {
            0x4 => self.send_packet(0x49, &data).unwrap(),
            0x4a => (),
            0x9  => self.0.stream.lock().unwrap().handle(cmd, data),
            0xd | 0xe => self.0.audio_key.lock().unwrap().handle(cmd, data),
            0x1b => {
                self.0.data.write().unwrap().country =
                    String::from_utf8(data).unwrap();
            },

            0xb2...0xb6 => self.0.mercury.lock().unwrap().handle(cmd, data),
            0xac => eprintln!("Authentication succeedded"),
            0xad => eprintln!("Authentication failed"),
            _ => ()
        }
    }

    pub fn send_packet(&self, cmd: u8, data: &[u8]) -> connection::Result<()> {
        self.0.tx_connection.lock().unwrap().send_packet(cmd, data)
    }

    pub fn audio_key(&self, track: SpotifyId, file: FileId) -> Future<AudioKey, ()> {
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
}

