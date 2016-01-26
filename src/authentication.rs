use byteorder::{BigEndian, ByteOrder};
use crypto;
use crypto::aes;
use crypto::digest::Digest;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::pbkdf2::pbkdf2;
use crypto::sha1::Sha1;
use protobuf::{self, Message, ProtobufEnum};
use rand::thread_rng;
use std::io::{self, Read, Write};
use std::result::Result;
use rustc_serialize::base64::FromBase64;

use connection::{PlainConnection, CipherConnection};
use diffie_hellman::DHLocalKeys;
use librespot_protocol as protocol;
use librespot_protocol::authentication::AuthenticationType;
use session::Session;
use util;

fn read_u8<R: Read>(stream: &mut R) -> io::Result<u8> {
    let mut data = [0u8];
    try!(stream.read_exact(&mut data));
    Ok(data[0])
}

fn read_int<R: Read>(stream: &mut R) -> io::Result<u32> {
    let lo = try!(read_u8(stream)) as u32;
    if lo & 0x80 == 0 {
        return Ok(lo);
    }

    let hi = try!(read_u8(stream)) as u32;
    Ok(lo & 0x7f | hi << 7)
}

fn read_bytes<R: Read>(stream: &mut R) -> io::Result<Vec<u8>> {
    let length = try!(read_int(stream));
    let mut data = vec![0u8; length as usize];
    try!(stream.read_exact(&mut data));

    Ok(data)
}

impl Session {
    pub fn connect(&self) -> CipherConnection {
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

    fn login(&self,
             username: String,
             auth_data: Vec<u8>,
             typ: AuthenticationType)
             -> Result<(), ()> {

        let packet = protobuf_init!(protocol::authentication::ClientResponseEncrypted::new(), {
            login_credentials => {
                username: username,
                typ: typ,
                auth_data: auth_data,
            },
            system_info => {
                cpu_family: protocol::authentication::CpuFamily::CPU_UNKNOWN,
                os: protocol::authentication::Os::OS_UNKNOWN,
                system_information_string: "librespot".to_owned(),
                device_id: self.device_id().clone()
            },
            version_string: util::version::version_string(),
            appkey => {
                version: self.config().application_key[0] as u32,
                devkey: self.config().application_key[0x1..0x81].to_vec(),
                signature: self.config().application_key[0x81..0x141].to_vec(),
                useragent: self.config().user_agent.clone(),
                callback_hash: vec![0; 20],
            }
        });

        let mut connection = self.connect();
        connection.send_packet(0xab, &packet.write_to_bytes().unwrap()).unwrap();
        let (cmd, data) = connection.recv_packet().unwrap();

        match cmd {
            0xac => {
                let welcome_data: protocol::authentication::APWelcome =
                    protobuf::parse_from_bytes(&data).unwrap();

                let username = welcome_data.get_canonical_username().to_owned();
                self.authenticated(username, connection);

                eprintln!("Authenticated !");
                Ok(())
            }

            0xad => {
                let msg: protocol::keyexchange::APLoginFailed = protobuf::parse_from_bytes(&data)
                                                                    .unwrap();
                eprintln!("Authentication failed, {:?}", msg);
                Err(())
            }
            _ => {
                println!("Unexpected message {:x}", cmd);
                Err(())
            }
        }
    }

    pub fn login_password(&self, username: String, password: String) -> Result<(), ()> {
        self.login(username,
                   password.into_bytes(),
                   AuthenticationType::AUTHENTICATION_USER_PASS)
    }

    pub fn login_blob(&self, username: String, blob: &str) -> Result<(), ()> {
        let blob = blob.from_base64().unwrap();

        let secret = {
            let mut data = [0u8; 20];
            let mut h = crypto::sha1::Sha1::new();
            h.input(&self.device_id().as_bytes());
            h.result(&mut data);
            data
        };

        let key = {
            let mut data = [0u8; 24];
            let mut mac = Hmac::new(Sha1::new(), &secret);
            pbkdf2(&mut mac, username.as_bytes(), 0x100, &mut data[0..20]);

            let mut hash = Sha1::new();
            hash.input(&data[0..20]);
            hash.result(&mut data[0..20]);
            BigEndian::write_u32(&mut data[20..], 20);
            data
        };

        let blob = {
            // Anyone know what this block mode is ?
            let mut data = vec![0u8; blob.len()];
            let mut cipher = aes::ecb_decryptor(aes::KeySize::KeySize192,
                                                &key,
                                                crypto::blockmodes::NoPadding);
            cipher.decrypt(&mut crypto::buffer::RefReadBuffer::new(&blob),
                           &mut crypto::buffer::RefWriteBuffer::new(&mut data),
                           true)
                  .unwrap();

            let l = blob.len();
            for i in 0..l - 0x10 {
                data[l - i - 1] ^= data[l - i - 0x11];
            }

            data
        };

        let mut cursor = io::Cursor::new(&blob);
        read_u8(&mut cursor).unwrap();
        read_bytes(&mut cursor).unwrap();
        read_u8(&mut cursor).unwrap();
        let auth_type = read_int(&mut cursor).unwrap();
        let auth_type = AuthenticationType::from_i32(auth_type as i32).unwrap();
        read_u8(&mut cursor).unwrap();
        let auth_data = read_bytes(&mut cursor).unwrap();;

        self.login(username, auth_data, auth_type)
    }
}
