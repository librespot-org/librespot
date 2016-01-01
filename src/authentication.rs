use byteorder::{BigEndian, ByteOrder};
use crypto;
use crypto::aes;
use crypto::pbkdf2::pbkdf2;
use crypto::sha1::Sha1;
use crypto::hmac::Hmac;
use crypto::digest::Digest;
use protobuf::{self, Message, ProtobufEnum};
use std::io::{self, Read};
use std::result::Result;
use rustc_serialize::base64::FromBase64;

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
        return Ok(lo)
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
    fn login(&self, username: String, auth_data: Vec<u8>, typ: AuthenticationType) -> Result<(), ()> {
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
                device_id: self.0.data.read().unwrap().device_id.clone()
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

        self.connect();
        self.send_packet(0xab, &packet.write_to_bytes().unwrap()).unwrap();
        let (cmd, data) = self.recv();
        match cmd {
            0xac => {
                let welcome_data : protocol::authentication::APWelcome = 
                    protobuf::parse_from_bytes(&data).unwrap();
                self.0.data.write().unwrap().canonical_username = 
                    welcome_data.get_canonical_username().to_string();

                eprintln!("Authenticated !");
                Ok(())
            }

            0xad => {
                let msg : protocol::keyexchange::APLoginFailed = 
                    protobuf::parse_from_bytes(&data).unwrap();
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
        self.login(username, password.into_bytes(),
                   AuthenticationType::AUTHENTICATION_USER_PASS)
    }

    pub fn login_blob(&self, username: String, blob: &str) -> Result<(), ()> {
        let blob = blob.from_base64().unwrap();

        let secret = {
            let mut data = [0u8; 20];
            let mut h = crypto::sha1::Sha1::new();
            h.input(&self.0.data.read().unwrap().device_id.as_bytes());
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
            let mut cipher = aes::ecb_decryptor(
                aes::KeySize::KeySize192, &key, crypto::blockmodes::NoPadding);
            cipher.decrypt(&mut crypto::buffer::RefReadBuffer::new(&blob),
                           &mut crypto::buffer::RefWriteBuffer::new(&mut data),
                           true).unwrap();

            let l = blob.len();
            for i in 0..l-0x10 {
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
