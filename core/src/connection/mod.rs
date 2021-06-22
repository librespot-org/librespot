mod codec;
mod handshake;

pub use self::codec::ApCodec;
pub use self::handshake::handshake;

use std::io::{self, ErrorKind};

use futures_util::{SinkExt, StreamExt};
use num_traits::FromPrimitive;
use protobuf::{self, Message, ProtobufError};
use thiserror::Error;
use tokio::net::TcpStream;
use tokio_util::codec::Framed;
use url::Url;

use crate::authentication::Credentials;
use crate::packet::PacketType;
use crate::protocol::keyexchange::{APLoginFailed, ErrorCode};
use crate::version;

pub type Transport = Framed<TcpStream, ApCodec>;

fn login_error_message(code: &ErrorCode) -> &'static str {
    pub use ErrorCode::*;
    match code {
        ProtocolError => "Protocol error",
        TryAnotherAP => "Try another AP",
        BadConnectionId => "Bad connection id",
        TravelRestriction => "Travel restriction",
        PremiumAccountRequired => "Premium account required",
        BadCredentials => "Bad credentials",
        CouldNotValidateCredentials => "Could not validate credentials",
        AccountExists => "Account exists",
        ExtraVerificationRequired => "Extra verification required",
        InvalidAppKey => "Invalid app key",
        ApplicationBanned => "Application banned",
    }
}

#[derive(Debug, Error)]
pub enum AuthenticationError {
    #[error("Login failed with reason: {}", login_error_message(.0))]
    LoginFailed(ErrorCode),
    #[error("Authentication failed: {0}")]
    IoError(#[from] io::Error),
}

impl From<ProtobufError> for AuthenticationError {
    fn from(e: ProtobufError) -> Self {
        io::Error::new(ErrorKind::InvalidData, e).into()
    }
}

impl From<APLoginFailed> for AuthenticationError {
    fn from(login_failure: APLoginFailed) -> Self {
        Self::LoginFailed(login_failure.get_error_code())
    }
}

pub async fn connect(host: &str, port: u16, proxy: Option<&Url>) -> io::Result<Transport> {
    let socket = crate::socket::connect(host, port, proxy).await?;

    handshake(socket).await
}

pub async fn authenticate(
    transport: &mut Transport,
    credentials: Credentials,
    device_id: &str,
) -> Result<Credentials, AuthenticationError> {
    use crate::protocol::authentication::{APWelcome, ClientResponseEncrypted, CpuFamily, Os};

    let mut packet = ClientResponseEncrypted::new();
    packet
        .mut_login_credentials()
        .set_username(credentials.username);
    packet
        .mut_login_credentials()
        .set_typ(credentials.auth_type);
    packet
        .mut_login_credentials()
        .set_auth_data(credentials.auth_data);
    packet
        .mut_system_info()
        .set_cpu_family(CpuFamily::CPU_UNKNOWN);
    packet.mut_system_info().set_os(Os::OS_UNKNOWN);
    packet
        .mut_system_info()
        .set_system_information_string(format!(
            "librespot_{}_{}",
            version::SHA_SHORT,
            version::BUILD_ID
        ));
    packet
        .mut_system_info()
        .set_device_id(device_id.to_string());
    packet.set_version_string(version::VERSION_STRING.to_string());

    let cmd = PacketType::Login;
    let data = packet.write_to_bytes().unwrap();

    transport.send((cmd as u8, data)).await?;
    let (cmd, data) = transport.next().await.expect("EOF")?;
    let packet_type = FromPrimitive::from_u8(cmd);
    match packet_type {
        Some(PacketType::APWelcome) => {
            let welcome_data = APWelcome::parse_from_bytes(data.as_ref())?;

            let reusable_credentials = Credentials {
                username: welcome_data.get_canonical_username().to_owned(),
                auth_type: welcome_data.get_reusable_auth_credentials_type(),
                auth_data: welcome_data.get_reusable_auth_credentials().to_owned(),
            };

            Ok(reusable_credentials)
        }
        Some(PacketType::AuthFailure) => {
            let error_data = APLoginFailed::parse_from_bytes(data.as_ref())?;
            Err(error_data.into())
        }
        _ => {
            let msg = format!("Received invalid packet: {}", cmd);
            Err(io::Error::new(ErrorKind::InvalidData, msg).into())
        }
    }
}
