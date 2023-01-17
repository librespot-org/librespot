mod codec;
mod handshake;

pub use self::{codec::ApCodec, handshake::handshake};

use std::io;

use futures_util::{SinkExt, StreamExt};
use num_traits::FromPrimitive;
use protobuf::{self, Message};
use thiserror::Error;
use tokio::net::TcpStream;
use tokio_util::codec::Framed;
use url::Url;

use crate::{authentication::Credentials, packet::PacketType, version, Error};

use crate::protocol::keyexchange::{APLoginFailed, ErrorCode};

pub type Transport = Framed<TcpStream, ApCodec>;

fn login_error_message(code: &ErrorCode) -> &'static str {
    pub use ErrorCode::*;
    match code {
        ProtocolError => "Protocol error",
        TryAnotherAP => "Try another access point",
        BadConnectionId => "Bad connection ID",
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
    #[error("invalid packet {0}")]
    Packet(u8),
    #[error("transport returned no data")]
    Transport,
}

impl From<AuthenticationError> for Error {
    fn from(err: AuthenticationError) -> Self {
        match err {
            AuthenticationError::LoginFailed(_) => Error::permission_denied(err),
            AuthenticationError::Packet(_) => Error::unimplemented(err),
            AuthenticationError::Transport => Error::unavailable(err),
        }
    }
}

impl From<APLoginFailed> for AuthenticationError {
    fn from(login_failure: APLoginFailed) -> Self {
        Self::LoginFailed(login_failure.error_code())
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
) -> Result<Credentials, Error> {
    use crate::protocol::authentication::{APWelcome, ClientResponseEncrypted, CpuFamily, Os};

    let cpu_family = match std::env::consts::ARCH {
        "blackfin" => CpuFamily::CPU_BLACKFIN,
        "arm" | "arm64" => CpuFamily::CPU_ARM,
        "ia64" => CpuFamily::CPU_IA64,
        "mips" => CpuFamily::CPU_MIPS,
        "ppc" => CpuFamily::CPU_PPC,
        "ppc64" => CpuFamily::CPU_PPC_64,
        "sh" => CpuFamily::CPU_SH,
        "x86" => CpuFamily::CPU_X86,
        "x86_64" => CpuFamily::CPU_X86_64,
        _ => CpuFamily::CPU_UNKNOWN,
    };

    let os = match std::env::consts::OS {
        "android" => Os::OS_ANDROID,
        "freebsd" | "netbsd" | "openbsd" => Os::OS_FREEBSD,
        "ios" => Os::OS_IPHONE,
        "linux" => Os::OS_LINUX,
        "macos" => Os::OS_OSX,
        "windows" => Os::OS_WINDOWS,
        _ => Os::OS_UNKNOWN,
    };

    let mut packet = ClientResponseEncrypted::new();
    packet
        .login_credentials
        .mut_or_insert_default()
        .set_username(credentials.username);
    packet
        .login_credentials
        .mut_or_insert_default()
        .set_typ(credentials.auth_type);
    packet
        .login_credentials
        .mut_or_insert_default()
        .set_auth_data(credentials.auth_data);
    packet
        .system_info
        .mut_or_insert_default()
        .set_cpu_family(cpu_family);
    packet.system_info.mut_or_insert_default().set_os(os);
    packet
        .system_info
        .mut_or_insert_default()
        .set_system_information_string(format!(
            "librespot-{}-{}",
            version::SHA_SHORT,
            version::BUILD_ID
        ));
    packet
        .system_info
        .mut_or_insert_default()
        .set_device_id(device_id.to_string());
    packet.set_version_string(format!("librespot {}", version::SEMVER));

    let cmd = PacketType::Login;
    let data = packet.write_to_bytes()?;

    transport.send((cmd as u8, data)).await?;
    let (cmd, data) = transport
        .next()
        .await
        .ok_or(AuthenticationError::Transport)??;
    let packet_type = FromPrimitive::from_u8(cmd);
    let result = match packet_type {
        Some(PacketType::APWelcome) => {
            let welcome_data = APWelcome::parse_from_bytes(data.as_ref())?;

            let reusable_credentials = Credentials {
                username: welcome_data.canonical_username().to_owned(),
                auth_type: welcome_data.reusable_auth_credentials_type(),
                auth_data: welcome_data.reusable_auth_credentials().to_owned(),
            };

            Ok(reusable_credentials)
        }
        Some(PacketType::AuthFailure) => {
            let error_data = APLoginFailed::parse_from_bytes(data.as_ref())?;
            Err(error_data.into())
        }
        _ => {
            trace!(
                "Did not expect {:?} AES key packet with data {:#?}",
                cmd,
                data
            );
            Err(AuthenticationError::Packet(cmd))
        }
    };
    Ok(result?)
}
