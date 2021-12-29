use std::{io, net::ToSocketAddrs};

use tokio::net::TcpStream;
use url::Url;

use crate::proxytunnel;

pub async fn connect(host: &str, port: u16, proxy: Option<&Url>) -> io::Result<TcpStream> {
    let socket = if let Some(proxy_url) = proxy {
        info!("Using proxy \"{}\"", proxy_url);

        let socket_addr = proxy_url.socket_addrs(|| None).and_then(|addrs| {
            addrs.into_iter().next().ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    "Can't resolve proxy server address",
                )
            })
        })?;
        let socket = TcpStream::connect(&socket_addr).await?;

        proxytunnel::proxy_connect(socket, host, &port.to_string()).await?
    } else {
        let socket_addr = (host, port).to_socket_addrs()?.next().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                "Can't resolve access point address",
            )
        })?;

        TcpStream::connect(&socket_addr).await?
    };
    Ok(socket)
}
