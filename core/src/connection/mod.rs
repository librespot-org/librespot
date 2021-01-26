mod codec;
mod handshake;

pub use self::codec::APCodec;
pub use self::handshake::handshake;

use futures::{SinkExt, StreamExt};
use protobuf::{self, Message};
use std::io;
use std::net::ToSocketAddrs;
use tokio::net::TcpStream;
use tokio_util::codec::Framed;
use url::Url;

use crate::authentication::Credentials;
use crate::version;

use crate::proxytunnel;

pub type Transport = Framed<TcpStream, APCodec>;

pub async fn connect(addr: String, proxy: &Option<Url>) -> io::Result<Transport> {
    let socket = if let Some(proxy) = proxy {
        info!("Using proxy \"{}\"", proxy);
        let socket_addr = proxy.to_socket_addrs().and_then(|mut iter| {
            iter.next().ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    "Can't resolve proxy server address",
                )
            })
        })?;
        let socket = TcpStream::connect(&socket_addr).await?;
        proxytunnel::connect(socket, &addr).await?
    } else {
        let socket_addr = addr.to_socket_addrs().and_then(|mut iter| {
            iter.next().ok_or_else(|| {
                io::Error::new(io::ErrorKind::NotFound, "Can't resolve server address")
            })
        })?;
        TcpStream::connect(&socket_addr).await?
    };

    handshake(socket).await
}

pub async fn authenticate(
    transport: &mut Transport,
    credentials: Credentials,
    device_id: &str,
) -> io::Result<Credentials> {
    use crate::protocol::authentication::{APWelcome, ClientResponseEncrypted, CpuFamily, Os};
    use crate::protocol::keyexchange::APLoginFailed;

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
            version::short_sha(),
            version::build_id()
        ));
    packet
        .mut_system_info()
        .set_device_id(device_id.to_string());
    packet.set_version_string(version::version_string());

    let cmd = 0xab;
    let data = packet.write_to_bytes().unwrap();

    transport.send((cmd, data)).await?;
    let (cmd, data) = transport.next().await.expect("EOF")?;
    match cmd {
        0xac => {
            let welcome_data: APWelcome = protobuf::parse_from_bytes(data.as_ref()).unwrap();

            let reusable_credentials = Credentials {
                username: welcome_data.get_canonical_username().to_owned(),
                auth_type: welcome_data.get_reusable_auth_credentials_type(),
                auth_data: welcome_data.get_reusable_auth_credentials().to_owned(),
            };

            Ok(reusable_credentials)
        }

        0xad => {
            let error_data: APLoginFailed = protobuf::parse_from_bytes(data.as_ref()).unwrap();
            panic!(
                "Authentication failed with reason: {:?}",
                error_data.get_error_code()
            )
        }

        _ => panic!("Unexpected packet {:?}", cmd),
    }
}
