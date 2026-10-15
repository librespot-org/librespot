use std::io;

use tokio::net::TcpStream;
use url::Url;

use crate::proxytunnel;

pub async fn connect(host: &str, port: u16, proxy: Option<&Url>) -> io::Result<TcpStream> {
    if let Some(proxy_url) = proxy {
        info!("Using proxy \"{proxy_url}\"");

        let socket_addrs = proxy_url.socket_addrs(|| None)?;
        let socket = TcpStream::connect(&*socket_addrs).await?;

        match proxy_url.scheme() {
            // SOCKS5 with remote DNS resolution: the destination hostname is
            // handed to the proxy verbatim. `socks5://` gets the same
            // treatment because resolving hostnames locally is usually
            // exactly what users of a SOCKS proxy want to avoid.
            "socks5" | "socks5h" | "socks" => {
                let auth = proxytunnel::Socks5Auth::from_url(proxy_url);
                proxytunnel::socks5_connect(socket, host, port, auth.as_ref()).await
            }
            "socks4" | "socks4a" => Err(io::Error::other(
                "SOCKS4 proxies are not supported, use a SOCKS5 proxy",
            )),
            // HTTP CONNECT proxy. The destination hostname is likewise
            // resolved by the proxy.
            _ => {
                let auth = proxytunnel::http_basic_auth(proxy_url);
                proxytunnel::proxy_connect(socket, host, &port.to_string(), auth.as_deref()).await
            }
        }
    } else {
        TcpStream::connect((host, port)).await
    }
}
