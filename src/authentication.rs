use byteorder::{BigEndian, ByteOrder};
use crypto;
use crypto::aes;
use crypto::digest::Digest;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::pbkdf2::pbkdf2;
use crypto::sha1::Sha1;
use protobuf::ProtobufEnum;
use std::io::{self, Read};
use rustc_serialize::base64::FromBase64;

use protocol::authentication::AuthenticationType;

pub struct Credentials {
    pub username: String,
    pub auth_type: AuthenticationType,
    pub auth_data: Vec<u8>,
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
}
