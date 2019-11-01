mod codec;
mod handshake;

pub use self::codec::APCodec;
pub use self::handshake::handshake;

use futures::{Future, Sink, Stream};
use protobuf::{self, Message};
use std::io;
use std::net::ToSocketAddrs;
use tokio_codec::Framed;
use tokio_core::net::TcpStream;
use tokio_core::reactor::Handle;
use url::Url;

use crate::authentication::Credentials;
use crate::version;

use crate::proxytunnel;

pub type Transport = Framed<TcpStream, APCodec>;

pub fn connect(
    addr: String,
    handle: &Handle,
    proxy: &Option<Url>,
) -> Box<dyn Future<Item = Transport, Error = io::Error>> {
    let (addr, connect_url) = match *proxy {
        Some(ref url) => {
            info!("Using proxy \"{}\"", url);
            (url.to_socket_addrs().unwrap().next().unwrap(), Some(addr))
        }
        None => (addr.to_socket_addrs().unwrap().next().unwrap(), None),
    };

    let socket = TcpStream::connect(&addr, handle);
    if let Some(connect_url) = connect_url {
        let connection = socket
            .and_then(move |socket| proxytunnel::connect(socket, &connect_url).and_then(handshake));
        Box::new(connection)
    } else {
        let connection = socket.and_then(handshake);
        Box::new(connection)
    }
}

pub fn authenticate(
    transport: Transport,
    credentials: Credentials,
    device_id: String,
) -> Box<dyn Future<Item = (Transport, Credentials), Error = io::Error>> {
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

    let cmd = 0xab;
    let data = packet.write_to_bytes().unwrap();

    Box::new(
        transport
            .send((cmd, data))
            .and_then(|transport| transport.into_future().map_err(|(err, _stream)| err))
            .and_then(|(packet, transport)| match packet {
                Some((0xac, data)) => {
                    let welcome_data: APWelcome =
                        protobuf::parse_from_bytes(data.as_ref()).unwrap();

                    let reusable_credentials = Credentials {
                        username: welcome_data.get_canonical_username().to_owned(),
                        auth_type: welcome_data.get_reusable_auth_credentials_type(),
                        auth_data: welcome_data.get_reusable_auth_credentials().to_owned(),
                    };

                    Ok((transport, reusable_credentials))
                }

                Some((0xad, data)) => {
                    let error_data: APLoginFailed =
                        protobuf::parse_from_bytes(data.as_ref()).unwrap();
                    panic!(
                        "Authentication failed with reason: {:?}",
                        error_data.get_error_code()
                    )
                }

                Some((cmd, _)) => panic!("Unexpected packet {:?}", cmd),
                None => panic!("EOF"),
            }),
    )
}
