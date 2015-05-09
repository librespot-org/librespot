use connection::{PlainConnection, CipherConnection};
use keys::PrivateKeys;
use librespot_protocol as protocol;
use util;

use crypto::sha1::Sha1;
use crypto::digest::Digest;
use protobuf::*;
use rand::thread_rng;

pub struct Config {
    pub application_key: Vec<u8>,
    pub user_agent: String,
    pub device_id: String,
}

pub struct Session {
    config: Config,
    connection: CipherConnection,
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
            parse_from_bytes(&init_server_packet[4..]).unwrap();

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

        Session {
            config: config,
            connection: connection.setup_cipher(shared_keys)
        }
    }

    pub fn login(&mut self, username: String, password: String) {
        let packet = protobuf_init!(protocol::authentication::ClientResponseEncrypted::new(), {
            login_credentials => {
                username: username,
                typ: protocol::authentication::Type::AUTHENTICATION_USER_PASS,
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

        self.connection.send_encrypted_packet(
            0xab,
            &packet.write_to_bytes().unwrap()).unwrap();

        loop {
            let (cmd, data) = self.connection.recv_packet().unwrap();
            println!("{:x}", cmd);
        }
    }
}

