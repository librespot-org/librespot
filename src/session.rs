use crypto::digest::Digest;
use crypto::mac::Mac;
use crypto::sha1::Sha1;
use crypto::hmac::Hmac;
use eventual::Future;
use protobuf::{self, Message};
use rand::thread_rng;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Mutex, RwLock, Arc, mpsc};

use audio_key::{AudioKeyManager, AudioKey, AudioKeyError};
use audio_file::{AudioFileManager, AudioFile};
use connection::{self, PlainConnection, CipherConnection};
use connection::PacketHandler;
use diffie_hellman::DHLocalKeys;
use librespot_protocol as protocol;
use mercury::{MercuryManager, MercuryRequest, MercuryResponse};
use metadata::{MetadataManager, MetadataRef, MetadataTrait};
use stream::{StreamManager, StreamEvent};
use util::{SpotifyId, FileId, mkdir_existing};

use util;

pub struct Config {
    pub application_key: Vec<u8>,
    pub user_agent: String,
    pub device_name: String,
    pub cache_location: PathBuf,
}

pub struct SessionData {
    pub country: String,
    pub canonical_username: String,
    pub device_id: String,
}

pub struct SessionInternal {
    pub config: Config,
    pub data: RwLock<SessionData>,

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

    pub fn connect(&self) {
        let local_keys = DHLocalKeys::random(&mut thread_rng());

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
                gc: local_keys.public_key(),
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

        let cipher_connection = CipherConnection::new(
            connection.into_stream(),
            &send_key,
            &recv_key);

        *self.0.rx_connection.lock().unwrap() = Some(cipher_connection.clone());
        *self.0.tx_connection.lock().unwrap() = Some(cipher_connection);
    }

    pub fn poll(&self) {
        let (cmd, data) = self.recv();

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
            _ => ()
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
}

