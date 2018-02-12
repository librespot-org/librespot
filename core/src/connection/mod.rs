mod codec;
mod handshake;

pub use self::codec::APCodec;
pub use self::handshake::handshake;

use futures::{Future, Sink, Stream};
use protobuf::{self, Message};
use std::io;
use std::net::ToSocketAddrs;
use tokio_core::net::TcpStream;
use tokio_core::reactor::Handle;
use tokio_io::codec::Framed;

use authentication::Credentials;
use version;

pub type Transport = Framed<TcpStream, APCodec>;

pub fn connect<A: ToSocketAddrs>(
    addr: A,
    handle: &Handle,
) -> Box<Future<Item = Transport, Error = io::Error>> {
    let addr = addr.to_socket_addrs().unwrap().next().unwrap();
    let socket = TcpStream::connect(&addr, handle);
    let connection = socket.and_then(|socket| handshake(socket));

    Box::new(connection)
}

pub fn authenticate(
    transport: Transport,
    credentials: Credentials,
    device_id: String,
) -> Box<Future<Item = (Transport, Credentials), Error = io::Error>> {
    use protocol::authentication::{APWelcome, ClientResponseEncrypted, CpuFamily, Os};

    let packet = {
        let mut msg = ClientResponseEncrypted::new();
        {
            let mut msg = msg.mut_login_credentials();
            msg.set_username(credentials.username);
            msg.set_typ(credentials.auth_type);
            msg.set_auth_data(credentials.auth_data);
            msg
        };
        {
            let mut msg = msg.mut_system_info();
            msg.set_cpu_family(CpuFamily::CPU_UNKNOWN);
            msg.set_os(Os::OS_UNKNOWN);
            msg.set_system_information_string(format!(
                "librespot_{}_{}",
                version::short_sha(),
                version::build_id()
            ));
            msg.set_device_id(device_id);
            msg
        };
        msg.set_version_string(version::version_string());
        msg
    };

    let cmd = 0xab;
    let data = packet.write_to_bytes().unwrap();

    Box::new(
        transport
            .send((cmd, data))
            .and_then(|transport| transport.into_future().map_err(|(err, _stream)| err))
            .and_then(|(packet, transport)| match packet {
                Some((0xac, data)) => {
                    let welcome_data: APWelcome = protobuf::parse_from_bytes(data.as_ref()).unwrap();

                    let reusable_credentials = Credentials {
                        username: welcome_data.get_canonical_username().to_owned(),
                        auth_type: welcome_data.get_reusable_auth_credentials_type(),
                        auth_data: welcome_data.get_reusable_auth_credentials().to_owned(),
                    };

                    Ok((transport, reusable_credentials))
                }

                Some((0xad, _)) => panic!("Authentication failed"),
                Some((cmd, _)) => panic!("Unexpected packet {:?}", cmd),
                None => panic!("EOF"),
            }),
    )
}
