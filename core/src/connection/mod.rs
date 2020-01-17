mod codec;
mod handshake;

pub use self::codec::APCodec;
pub use self::handshake::handshake;
use tokio::net::TcpStream;

use futures::{AsyncRead, AsyncWrite, Future, Sink, SinkExt, Stream, StreamExt};
use protobuf::{self, Message};
use std::io;
use std::net::ToSocketAddrs;
use tokio_util::codec::Framed;
// use futures::compat::{AsyncWrite01CompatExt, AsyncRead01CompatExt};
use tokio_util::compat::{self, Tokio02AsyncReadCompatExt, Tokio02AsyncWriteCompatExt};
// use tokio_codec::Framed;
// use tokio_core::net::TcpStream;
// use tokio_core::reactor::Handle;
use url::Url;

use crate::authentication::Credentials;
use crate::version;

use crate::proxytunnel;

pub type Transport = Framed<TcpStream, APCodec>;

pub async fn connect(addr: String, proxy: &Option<Url>) -> Result<Transport, io::Error> {
    let (addr, connect_url) = match *proxy {
        Some(ref url) => {
            info!("Using proxy \"{}\"", url);

            let mut iter = url.to_socket_addrs()?;
            let socket_addr = iter.next().ok_or(io::Error::new(
                io::ErrorKind::NotFound,
                "Can't resolve proxy server address",
            ))?;
            (socket_addr, Some(addr))
        }
        None => {
            let mut iter = addr.to_socket_addrs()?;
            let socket_addr = iter.next().ok_or(io::Error::new(
                io::ErrorKind::NotFound,
                "Can't resolve server address",
            ))?;
            (socket_addr, None)
        }
    };

    let connection = TcpStream::connect(&addr).await?;
    if let Some(connect_url) = connect_url {
        let connection = proxytunnel::connect(connection, &connect_url).await?;
        let connection = handshake(connection).await?;
        Ok(connection)
    } else {
        let connection = handshake(connection).await?;
        Ok(connection)
    }
}

pub async fn authenticate(
    transport: Transport,
    credentials: Credentials,
    device_id: String,
) -> Result<(Transport, Credentials), io::Error> {
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
    packet.mut_system_info().set_device_id(device_id);
    packet.set_version_string(version::version_string());

    let cmd: u8 = 0xab;
    let data = packet.write_to_bytes().unwrap();

    transport.send((cmd, data)).await;

    let packet = transport.next().await;
    // let (packet, transport) = transport
    //     .into_future()
    //     .map_err(|(err, _stream)| err)
    //     .await?;
    match packet {
        Some(Ok((0xac, data))) => {
            let welcome_data: APWelcome = protobuf::parse_from_bytes(data.as_ref()).unwrap();

            let reusable_credentials = Credentials {
                username: welcome_data.get_canonical_username().to_owned(),
                auth_type: welcome_data.get_reusable_auth_credentials_type(),
                auth_data: welcome_data.get_reusable_auth_credentials().to_owned(),
            };

            Ok((transport, reusable_credentials))
        }

        Some(Ok((0xad, data))) => {
            let error_data: APLoginFailed = protobuf::parse_from_bytes(data.as_ref()).unwrap();
            panic!(
                "Authentication failed with reason: {:?}",
                error_data.get_error_code()
            )
        }

        Some(Ok((cmd, _))) => panic!("Unexpected packet {:?}", cmd),
        Some(err @ Err(_)) => panic!("Packet error: {:?}", err),
        None => panic!("EOF"),
    }
}
