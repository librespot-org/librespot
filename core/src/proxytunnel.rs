use std::io;

use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

pub async fn connect<T: AsyncRead + AsyncWrite + Unpin>(
    mut proxy_connection: T,
    connect_host: &str,
    connect_port: u16,
) -> io::Result<T> {
    let mut buffer = Vec::new();
    buffer.extend_from_slice(b"CONNECT ");
    buffer.extend_from_slice(connect_host.as_bytes());
    buffer.push(b':');
    buffer.extend_from_slice(connect_port.to_string().as_bytes());
    buffer.extend_from_slice(b" HTTP/1.1\r\n\r\n");

    proxy_connection.write_all(buffer.as_ref()).await?;

    buffer.resize(buffer.capacity(), 0);

    let mut offset = 0;
    loop {
        let bytes_read = proxy_connection.read(&mut buffer[offset..]).await?;
        if bytes_read == 0 {
            return Err(io::Error::new(io::ErrorKind::Other, "Early EOF from proxy"));
        }
        offset += bytes_read;

        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut response = httparse::Response::new(&mut headers);

        let status = response
            .parse(&buffer[..offset])
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

        if status.is_complete() {
            return match response.code {
                Some(200) => Ok(proxy_connection), // Proxy says all is well
                Some(code) => {
                    let reason = response.reason.unwrap_or("no reason");
                    let msg = format!("Proxy responded with {}: {}", code, reason);
                    Err(io::Error::new(io::ErrorKind::Other, msg))
                }
                None => Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Malformed response from proxy",
                )),
            };
        }

        if offset >= buffer.len() {
            buffer.resize(buffer.len() * 2, 0);
        }
    }
}

cfg_if! {
    if #[cfg(feature = "apresolve")] {
        use std::future::Future;
        use std::net::{SocketAddr, ToSocketAddrs};
        use std::pin::Pin;
        use std::task::Poll;

        use hyper::service::Service;
        use hyper::Uri;
        use tokio::net::TcpStream;

        #[derive(Clone)]
        pub struct ProxyTunnel {
            proxy_addr: SocketAddr,
        }

        impl ProxyTunnel {
            pub fn new<T: ToSocketAddrs>(addr: T) -> io::Result<Self> {
                let addr = addr.to_socket_addrs()?.next().ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "No socket address given")
                })?;
                Ok(Self { proxy_addr: addr })
            }
        }

        impl Service<Uri> for ProxyTunnel {
            type Response = TcpStream;
            type Error = io::Error;
            type Future = Pin<Box<dyn Future<Output = io::Result<TcpStream>> + Send>>;

            fn poll_ready(&mut self, _: &mut std::task::Context<'_>) -> Poll<io::Result<()>> {
                Poll::Ready(Ok(()))
            }

            fn call(&mut self, url: Uri) -> Self::Future {
                let proxy_addr = self.proxy_addr;
                let fut = async move {
                    let host = url
                        .host()
                        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Host is missing"))?;
                    let port = url
                        .port()
                        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Port is missing"))?;

                    let conn = TcpStream::connect(proxy_addr).await?;
                    connect(conn, host, port.as_u16()).await
                };

                Box::pin(fut)
            }
        }
    }
}
