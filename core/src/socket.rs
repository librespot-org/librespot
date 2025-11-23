use std::io;

use tokio::net::TcpStream;
use url::Url;

use crate::proxytunnel;

pub async fn connect(host: &str, port: u16, proxy: Option<&Url>) -> io::Result<TcpStream> {
    if let Some(proxy_url) = proxy {
        info!("Using proxy \"{proxy_url}\"");

        let socket_addrs = proxy_url.socket_addrs(|| None)?;
        let socket = TcpStream::connect(&*socket_addrs).await?;

        proxytunnel::proxy_connect(socket, host, &port.to_string()).await
    } else {
        TcpStream::connect((host, port)).await
    }
}
