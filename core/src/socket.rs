use std::io;
use std::net::SocketAddr;
use std::time::Duration;

use tokio::net::TcpStream;
use url::Url;

use crate::proxytunnel;

// Bounds each address attempt so one blackholed address (e.g. an IPv6 route
// that silently drops SYNs) can't consume the caller's entire timeout budget
// before the remaining addresses (e.g. IPv4) get a chance.
const CONNECT_ATTEMPT_TIMEOUT: Duration = Duration::from_secs(3);

async fn connect_attempts(addrs: impl Iterator<Item = SocketAddr>) -> io::Result<TcpStream> {
    let mut last_err = None;

    for addr in addrs {
        match tokio::time::timeout(CONNECT_ATTEMPT_TIMEOUT, TcpStream::connect(addr)).await {
            Ok(Ok(stream)) => return Ok(stream),
            Ok(Err(e)) => last_err = Some(e),
            Err(_) => {
                last_err = Some(io::Error::new(
                    io::ErrorKind::TimedOut,
                    format!("connection to {addr} timed out"),
                ))
            }
        }
    }

    Err(last_err
        .unwrap_or_else(|| io::Error::new(io::ErrorKind::NotFound, "no addresses to connect to")))
}

pub async fn connect(host: &str, port: u16, proxy: Option<&Url>) -> io::Result<TcpStream> {
    if let Some(proxy_url) = proxy {
        info!("Using proxy \"{proxy_url}\"");

        let socket_addrs = proxy_url.socket_addrs(|| None)?;
        let socket = connect_attempts(socket_addrs.into_iter()).await?;

        proxytunnel::proxy_connect(socket, host, &port.to_string()).await
    } else {
        let socket_addrs = tokio::net::lookup_host((host, port)).await?;
        connect_attempts(socket_addrs).await
    }
}
