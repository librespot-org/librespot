use crypto::digest::Digest;
use crypto::sha1::Sha1;
use protobuf::{self, Message};
use rand::thread_rng;
use std::sync::mpsc;
use std::thread;

use audio_key;
use connection::{PlainConnection, Packet, PacketDispatch, SendThread, RecvThread};
use keys::PrivateKeys;
use librespot_protocol as protocol;
use mercury;
use metadata;
use stream;
use subsystem::Subsystem;
use util;

pub struct Config {
    pub application_key: Vec<u8>,
    pub user_agent: String,
    pub device_id: String,
}

pub struct Session {
    pub config: Config,

    packet_rx: mpsc::Receiver<Packet>,
    pub packet_tx: mpsc::Sender<Packet>,

    pub audio_key: mpsc::Sender<audio_key::AudioKeyRequest>,
    pub mercury: mpsc::Sender<mercury::MercuryRequest>,
    pub metadata: mpsc::Sender<metadata::MetadataRequest>,
    pub stream: mpsc::Sender<stream::StreamRequest>,
}

impl Session {
    pub fn new(mut config: Config) -> Session {
        config.device_id = {
            let mut h = Sha1::new();
            h.input_str(&config.device_id);
            h.result_str()
        };

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

        let (send_thread, tx) = SendThread::new(cipher_connection.clone());

        let (main_tx, rx) = mpsc::channel();
        let (mercury, mercury_req, mercury_pkt)
            = mercury::MercuryManager::new(tx.clone());
        let (metadata, metadata_req)
            = metadata::MetadataManager::new(mercury_req.clone());
        let (stream, stream_req, stream_pkt)
            = stream::StreamManager::new(tx.clone());
        let (audio_key, audio_key_req, audio_key_pkt)
            = audio_key::AudioKeyManager::new(tx.clone());

        let recv_thread = RecvThread::new(cipher_connection, PacketDispatch {
            main: main_tx,
            stream: stream_pkt,
            mercury: mercury_pkt,
            audio_key: audio_key_pkt
        });

        thread::spawn(move || send_thread.run());
        thread::spawn(move || recv_thread.run());

        mercury.start();
        metadata.start();
        stream.start();
        audio_key.start();

        Session {
            config: config,
            packet_rx: rx,
            packet_tx: tx,
            mercury: mercury_req,
            metadata: metadata_req,
            stream: stream_req,
            audio_key: audio_key_req,
        }
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
                system_information_string: "librespot".to_string(),
                device_id: self.config.device_id.clone()
            },
            version_string: util::version::version_string(),
            appkey => {
                version: self.config.application_key[0] as u32,
                devkey: self.config.application_key[0x1..0x81].to_vec(),
                signature: self.config.application_key[0x81..0x141].to_vec(),
                useragent: self.config.user_agent.clone(),
                callback_hash: vec![0; 20],
            }
        });

        self.packet_tx.send(Packet {
            cmd: 0xab,
            data: packet.write_to_bytes().unwrap()
        }).unwrap();
    }

    pub fn poll(&self) {
        let packet = self.packet_rx.recv().unwrap();

        match packet.cmd {
            0x4 => { // PING
                self.packet_tx.send(Packet {
                    cmd: 0x49,
                    data: packet.data
                }).unwrap();
            }
            0x4a => { // PONG
            }
            0xac => { // AUTHENTICATED
                eprintln!("Authentication succeedded");
            }
            0xad => {
                eprintln!("Authentication failed");
            }
            _ => ()
        };
    }
}

