use aes::Aes192;
use byteorder::{BigEndian, ByteOrder};
use hmac::Hmac;
use pbkdf2::pbkdf2;
use protobuf::ProtobufEnum;
use sha1::{Digest, Sha1};
use std::io::{self, Read};

use crate::protocol::authentication::AuthenticationType;
use crate::protocol::keyexchange::{APLoginFailed, ErrorCode};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    pub username: String,

    #[serde(serialize_with = "serialize_protobuf_enum")]
    #[serde(deserialize_with = "deserialize_protobuf_enum")]
    pub auth_type: AuthenticationType,

    #[serde(alias = "encoded_auth_blob")]
    #[serde(serialize_with = "serialize_base64")]
    #[serde(deserialize_with = "deserialize_base64")]
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
            stream.read_exact(&mut data)?;
            Ok(data[0])
        }

        fn read_int<R: Read>(stream: &mut R) -> io::Result<u32> {
            let lo = read_u8(stream)? as u32;
            if lo & 0x80 == 0 {
                return Ok(lo);
            }

            let hi = read_u8(stream)? as u32;
            Ok(lo & 0x7f | hi << 7)
        }

        fn read_bytes<R: Read>(stream: &mut R) -> io::Result<Vec<u8>> {
            let length = read_int(stream)?;
            let mut data = vec![0u8; length as usize];
            stream.read_exact(&mut data)?;

            Ok(data)
        }

        let secret = Sha1::digest(device_id.as_bytes());

        let key = {
            let mut key = [0u8; 24];
            pbkdf2::<Hmac<Sha1>>(&secret, username.as_bytes(), 0x100, &mut key[0..20]);

            let hash = &Sha1::digest(&key[..20]);
            key[..20].copy_from_slice(hash);
            BigEndian::write_u32(&mut key[20..], 20);
            key
        };

        // decrypt data using ECB mode without padding
        let blob = {
            use aes::cipher::generic_array::typenum::Unsigned;
            use aes::cipher::generic_array::GenericArray;
            use aes::cipher::{BlockCipher, NewBlockCipher};

            let mut data = base64::decode(encrypted_blob).unwrap();
            let cipher = Aes192::new(GenericArray::from_slice(&key));
            let block_size = <Aes192 as BlockCipher>::BlockSize::to_usize();
            assert_eq!(data.len() % block_size, 0);
            // replace to chunks_exact_mut with MSRV bump to 1.31
            for chunk in data.chunks_mut(block_size) {
                cipher.decrypt_block(GenericArray::from_mut_slice(chunk));
            }

            let l = data.len();
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
        let auth_data = read_bytes(&mut cursor).unwrap();

        Credentials {
            username: username,
            auth_type: auth_type,
            auth_data: auth_data,
        }
    }
}

fn serialize_protobuf_enum<T, S>(v: &T, ser: S) -> Result<S::Ok, S::Error>
where
    T: ProtobufEnum,
    S: serde::Serializer,
{
    serde::Serialize::serialize(&v.value(), ser)
}

fn deserialize_protobuf_enum<'de, T, D>(de: D) -> Result<T, D::Error>
where
    T: ProtobufEnum,
    D: serde::Deserializer<'de>,
{
    let v: i32 = serde::Deserialize::deserialize(de)?;
    T::from_i32(v).ok_or_else(|| serde::de::Error::custom("Invalid enum value"))
}

fn serialize_base64<T, S>(v: &T, ser: S) -> Result<S::Ok, S::Error>
where
    T: AsRef<[u8]>,
    S: serde::Serializer,
{
    serde::Serialize::serialize(&base64::encode(v.as_ref()), ser)
}

fn deserialize_base64<'de, D>(de: D) -> Result<Vec<u8>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let v: String = serde::Deserialize::deserialize(de)?;
    base64::decode(&v).map_err(|e| serde::de::Error::custom(e.to_string()))
}

pub fn get_credentials<F: FnOnce(&String) -> String>(
    username: Option<String>,
    password: Option<String>,
    cached_credentials: Option<Credentials>,
    prompt: F,
) -> Option<Credentials> {
    match (username, password, cached_credentials) {
        (Some(username), Some(password), _) => Some(Credentials::with_password(username, password)),

        (Some(ref username), _, Some(ref credentials)) if *username == credentials.username => {
            Some(credentials.clone())
        }

        (Some(username), None, _) => Some(Credentials::with_password(
            username.clone(),
            prompt(&username),
        )),

        (None, _, Some(credentials)) => Some(credentials),

        (None, _, None) => None,
    }
}

error_chain! {
    types {
        AuthenticationError, AuthenticationErrorKind, AuthenticationResultExt, AuthenticationResult;
    }

    foreign_links {
        Io(::std::io::Error);
    }

    errors {
        BadCredentials {
            description("Bad credentials")
            display("Authentication failed with error: Bad credentials")
        }
        PremiumAccountRequired {
            description("Premium account required")
            display("Authentication failed with error: Premium account required")
        }
    }
}

impl From<APLoginFailed> for AuthenticationError {
    fn from(login_failure: APLoginFailed) -> Self {
        let error_code = login_failure.get_error_code();
        match error_code {
            ErrorCode::BadCredentials => Self::from_kind(AuthenticationErrorKind::BadCredentials),
            ErrorCode::PremiumAccountRequired => {
                Self::from_kind(AuthenticationErrorKind::PremiumAccountRequired)
            }
            _ => format!("Authentication failed with error: {:?}", error_code).into(),
        }
    }
}
