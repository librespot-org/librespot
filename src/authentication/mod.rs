use byteorder::{BigEndian, ByteOrder};
use crypto;
use crypto::aes;
use crypto::digest::Digest;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::pbkdf2::pbkdf2;
use crypto::sha1::Sha1;
use protobuf::ProtobufEnum;
use std::io::{self, Read, Write};
use std::fs::File;
use std::path::Path;
use rustc_serialize::base64::{self, FromBase64, ToBase64};
use rustc_serialize::json;

use protocol::authentication::AuthenticationType;

#[derive(Debug, Clone)]
pub struct Credentials {
    pub username: String,
    pub auth_type: AuthenticationType,
    pub auth_data: Vec<u8>,
}

#[derive(Debug, Clone)]
#[derive(RustcDecodable, RustcEncodable)]
struct StoredCredentials {
    pub username: String,
    pub auth_type: i32,
    pub auth_data: String,
}

impl Credentials {
    pub fn with_password(username: String, password: String) -> Credentials {
        Credentials {
            username: username,
            auth_type: AuthenticationType::AUTHENTICATION_USER_PASS,
            auth_data: password.into_bytes(),
        }
    }

    pub fn with_blob(username: String, encrypted_blob: &str, device_id: &str) -> Credentials {
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

        let encrypted_blob = encrypted_blob.from_base64().unwrap();

        let secret = {
            let mut data = [0u8; 20];
            let mut h = crypto::sha1::Sha1::new();
            h.input(device_id.as_bytes());
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
            let mut data = vec![0u8; encrypted_blob.len()];
            let mut cipher = aes::ecb_decryptor(aes::KeySize::KeySize192,
                                                &key,
                                                crypto::blockmodes::NoPadding);
            cipher.decrypt(&mut crypto::buffer::RefReadBuffer::new(&encrypted_blob),
                           &mut crypto::buffer::RefWriteBuffer::new(&mut data),
                           true)
                  .unwrap();

            let l = encrypted_blob.len();
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

        Credentials {
            username: username,
            auth_type: auth_type,
            auth_data: auth_data,
        }
    }

    pub fn from_reader<R: Read>(mut reader: R) -> Credentials {
        let mut contents = String::new();
        reader.read_to_string(&mut contents).unwrap();

        json::decode::<StoredCredentials>(&contents).unwrap().into()
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Option<Credentials> {
        File::open(path).ok().map(Credentials::from_reader)
    }

    pub fn save_to_writer<W: Write>(&self, writer: &mut W) {
        let contents = json::encode::<StoredCredentials>(&self.clone().into()).unwrap();
        writer.write_all(contents.as_bytes()).unwrap();
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) {
        let mut file = File::create(path).unwrap();
        self.save_to_writer(&mut file)
    }
}

impl From<Credentials> for StoredCredentials {
    fn from(credentials: Credentials) -> StoredCredentials {
        StoredCredentials {
            username: credentials.username,
            auth_type: credentials.auth_type.value(),
            auth_data: credentials.auth_data.to_base64(base64::STANDARD),
        }
    }
}

impl From<StoredCredentials> for Credentials {
    fn from(credentials: StoredCredentials) -> Credentials {
        Credentials {
            username: credentials.username,
            auth_type: AuthenticationType::from_i32(credentials.auth_type).unwrap(),
            auth_data: credentials.auth_data.from_base64().unwrap(),
        }
    }
}

#[cfg(feature = "discovery")]
mod discovery;
#[cfg(feature = "discovery")]
pub use self::discovery::discovery_login;
#[cfg(not(feature = "discovery"))]
pub fn discovery_login(device_name: &str, device_id: &str) -> Result<Credentials, ()> {
    Err(())
}

#[cfg(feature = "facebook")]
mod facebook;
#[cfg(feature = "facebook")]
pub use self::facebook::facebook_login;
#[cfg(not(feature = "facebook"))]
pub fn facebook_login() -> Result<Credentials, ()> {
    Err(())
}
