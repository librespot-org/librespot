//! A unified `Service<Uri>` connector for librespot's HTTP client that
//! supports:
//!
//! * direct connections (no proxy),
//! * HTTP CONNECT proxies (optionally with `Proxy-Authorization: Basic`),
//! * SOCKS5 proxies (`socks5://` and `socks5h://`, optionally with RFC 1929
//!   username/password auth), where the destination hostname is handed to the
//!   proxy for remote DNS resolution.
//!
//! TLS is layered on top of the (possibly tunnelled) TCP stream for `https`
//! destinations, using whichever TLS backend librespot-core was compiled with.

use std::{
    future::Future,
    io,
    pin::Pin,
    task::{Context, Poll},
};

use hyper::Uri;
use hyper_util::rt::TokioIo;
use tower_service::Service;
use url::Url;

use crate::proxytunnel;

#[cfg(all(feature = "__rustls", not(feature = "native-tls")))]
use tokio_rustls::TlsConnector as RustlsConnector;

// ---------------------------------------------------------------------------
// Stream
// ---------------------------------------------------------------------------

/// A stream that is either plain TCP or TLS (over native-tls or rustls),
/// possibly tunnelled through a proxy underneath. `TokioIo` bridges the
/// tokio IO traits to hyper's runtime traits.
pub enum ProxyStream {
    Tcp(TokioIo<tokio::net::TcpStream>),
    #[cfg(feature = "native-tls")]
    NativeTls(TokioIo<Box<tokio_native_tls::TlsStream<tokio::net::TcpStream>>>),
    #[cfg(all(feature = "__rustls", not(feature = "native-tls")))]
    Rustls(TokioIo<Box<tokio_rustls::client::TlsStream<tokio::net::TcpStream>>>),
}

impl hyper::rt::Read for ProxyStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: hyper::rt::ReadBufCursor<'_>,
    ) -> Poll<io::Result<()>> {
        match self.get_mut() {
            ProxyStream::Tcp(stream) => Pin::new(stream).poll_read(cx, buf),
            #[cfg(feature = "native-tls")]
            ProxyStream::NativeTls(stream) => Pin::new(stream).poll_read(cx, buf),
            #[cfg(all(feature = "__rustls", not(feature = "native-tls")))]
            ProxyStream::Rustls(stream) => Pin::new(stream).poll_read(cx, buf),
        }
    }
}

impl hyper::rt::Write for ProxyStream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        match self.get_mut() {
            ProxyStream::Tcp(stream) => Pin::new(stream).poll_write(cx, buf),
            #[cfg(feature = "native-tls")]
            ProxyStream::NativeTls(stream) => Pin::new(stream).poll_write(cx, buf),
            #[cfg(all(feature = "__rustls", not(feature = "native-tls")))]
            ProxyStream::Rustls(stream) => Pin::new(stream).poll_write(cx, buf),
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match self.get_mut() {
            ProxyStream::Tcp(stream) => Pin::new(stream).poll_flush(cx),
            #[cfg(feature = "native-tls")]
            ProxyStream::NativeTls(stream) => Pin::new(stream).poll_flush(cx),
            #[cfg(all(feature = "__rustls", not(feature = "native-tls")))]
            ProxyStream::Rustls(stream) => Pin::new(stream).poll_flush(cx),
        }
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match self.get_mut() {
            ProxyStream::Tcp(stream) => Pin::new(stream).poll_shutdown(cx),
            #[cfg(feature = "native-tls")]
            ProxyStream::NativeTls(stream) => Pin::new(stream).poll_shutdown(cx),
            #[cfg(all(feature = "__rustls", not(feature = "native-tls")))]
            ProxyStream::Rustls(stream) => Pin::new(stream).poll_shutdown(cx),
        }
    }
}

impl hyper_util::client::legacy::connect::Connection for ProxyStream {
    fn connected(&self) -> hyper_util::client::legacy::connect::Connected {
        hyper_util::client::legacy::connect::Connected::new()
    }
}

// ---------------------------------------------------------------------------
// Connector
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
enum ProxyScheme {
    /// HTTP CONNECT proxy.
    Http,
    /// SOCKS5 proxy with remote (proxy-side) DNS resolution.
    Socks5,
}

#[derive(Clone, Debug)]
struct ProxyConfig {
    scheme: ProxyScheme,
    host: String,
    port: u16,
    socks5_auth: Option<proxytunnel::Socks5Auth>,
    http_basic_auth: Option<String>,
}

/// A `Service<Uri>` that connects directly or through a proxy.
#[derive(Clone)]
pub struct ProxyConnector {
    proxy: Option<ProxyConfig>,
    #[cfg(feature = "native-tls")]
    native_tls: Option<tokio_native_tls::TlsConnector>,
    #[cfg(all(feature = "__rustls", not(feature = "native-tls")))]
    rustls: Option<std::sync::Arc<RustlsConnector>>,
}

impl ProxyConnector {
    /// Build a connector for the given proxy URL (or `None` for a direct
    /// connection). Returns an error for proxy URLs that cannot be used.
    pub fn new(proxy_url: Option<&Url>) -> io::Result<Self> {
        let proxy = match proxy_url {
            Some(proxy_url) => {
                let scheme = match proxy_url.scheme() {
                    "socks5" | "socks5h" | "socks" => ProxyScheme::Socks5,
                    "http" | "https" => ProxyScheme::Http,
                    "socks4" | "socks4a" => {
                        return Err(io::Error::other(
                            "SOCKS4 proxies are not supported, use a SOCKS5 proxy",
                        ));
                    }
                    other => {
                        return Err(io::Error::other(format!(
                            "unsupported proxy scheme \"{other}\""
                        )));
                    }
                };

                let default_port = match scheme {
                    ProxyScheme::Socks5 => 1080,
                    ProxyScheme::Http => proxy_url.port_or_known_default().unwrap_or(8080),
                };
                let port = proxy_url.port().unwrap_or(default_port);
                let host = proxy_url
                    .host_str()
                    .ok_or_else(|| io::Error::other("proxy URL has no host"))?
                    .to_owned();

                let (socks5_auth, http_basic_auth) = match scheme {
                    ProxyScheme::Socks5 => (proxytunnel::Socks5Auth::from_url(proxy_url), None),
                    ProxyScheme::Http => (None, proxytunnel::http_basic_auth(proxy_url)),
                };

                Some(ProxyConfig {
                    scheme,
                    host,
                    port,
                    socks5_auth,
                    http_basic_auth,
                })
            }
            None => None,
        };

        let connector = Self {
            proxy,
            #[cfg(feature = "native-tls")]
            native_tls: Some(tokio_native_tls::TlsConnector::from(
                native_tls::TlsConnector::new().map_err(io::Error::other)?,
            )),
            #[cfg(all(feature = "__rustls", not(feature = "native-tls")))]
            rustls: Some(Self::rustls_connector()?),
        };
        Ok(connector)
    }

    #[cfg(all(feature = "__rustls", not(feature = "native-tls")))]
    fn rustls_connector() -> io::Result<std::sync::Arc<RustlsConnector>> {
        use std::sync::Arc;

        let mut roots = rustls::RootCertStore::empty();
        #[cfg(feature = "rustls-tls-native-roots")]
        {
            let result = rustls_native_certs::load_native_certs();
            for error in &result.errors {
                warn!("failed to load a native root certificate: {error}");
            }
            for cert in result.certs {
                roots
                    .add(cert)
                    .map_err(|e| io::Error::other(e.to_string()))?;
            }
        }
        #[cfg(feature = "rustls-tls-webpki-roots")]
        roots.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

        let config = rustls::ClientConfig::builder()
            .with_root_certificates(roots)
            .with_no_client_auth();
        Ok(Arc::new(RustlsConnector::from(Arc::new(config))))
    }

    async fn connect(&self, uri: &Uri) -> io::Result<ProxyStream> {
        let host = uri
            .host()
            .ok_or_else(|| io::Error::other("request URI has no host"))?
            .to_owned();
        let port = uri.port_u16().unwrap_or(match uri.scheme_str() {
            Some("http") => 80,
            _ => 443,
        });

        let tcp_stream = match &self.proxy {
            None => {
                // Try every resolved address, like HttpConnector does.
                let addrs = tokio::net::lookup_host((host.as_str(), port)).await?;
                let mut last_err = None;
                let mut stream = None;
                for addr in addrs {
                    match tokio::net::TcpStream::connect(addr).await {
                        Ok(s) => {
                            stream = Some(s);
                            break;
                        }
                        Err(e) => last_err = Some(e),
                    }
                }
                match stream {
                    Some(stream) => stream,
                    None => {
                        return Err(last_err.unwrap_or_else(|| {
                            io::Error::new(io::ErrorKind::NotFound, "No address resolved")
                        }));
                    }
                }
            }
            Some(proxy) => {
                let socket_addr = tokio::net::lookup_host((proxy.host.as_str(), proxy.port))
                    .await?
                    .next()
                    .ok_or_else(|| {
                        io::Error::new(
                            io::ErrorKind::NotFound,
                            "Can't resolve proxy server address",
                        )
                    })?;
                let mut stream = tokio::net::TcpStream::connect(socket_addr).await?;
                match proxy.scheme {
                    ProxyScheme::Socks5 => {
                        proxytunnel::socks5_connect(
                            &mut stream,
                            &host,
                            port,
                            proxy.socks5_auth.as_ref(),
                        )
                        .await?;
                    }
                    ProxyScheme::Http => {
                        proxytunnel::proxy_connect(
                            &mut stream,
                            &host,
                            &port.to_string(),
                            proxy.http_basic_auth.as_deref(),
                        )
                        .await?;
                    }
                }
                stream
            }
        };

        // Layer TLS for https destinations (over the tunnel, if proxied).
        if uri.scheme_str() == Some("https") {
            #[cfg(feature = "native-tls")]
            {
                let connector = self
                    .native_tls
                    .as_ref()
                    .expect("native TLS connector is set up in `new`");
                let tls_stream = connector
                    .connect(&host, tcp_stream)
                    .await
                    .map_err(io::Error::other)?;
                Ok(ProxyStream::NativeTls(TokioIo::new(Box::new(tls_stream))))
            }
            #[cfg(all(feature = "__rustls", not(feature = "native-tls")))]
            {
                use rustls::pki_types::ServerName;
                let connector = self
                    .rustls
                    .as_ref()
                    .expect("rustls connector is set up in `new`");
                let server_name = ServerName::try_from(host.clone())
                    .map_err(|_| io::Error::other("invalid server name for TLS"))?;
                let tls_stream = connector.connect(server_name, tcp_stream).await?;
                Ok(ProxyStream::Rustls(TokioIo::new(Box::new(tls_stream))))
            }
            #[cfg(not(any(feature = "native-tls", feature = "__rustls")))]
            {
                Err(io::Error::other(
                    "https requested but no TLS backend is compiled in",
                ))
            }
        } else {
            Ok(ProxyStream::Tcp(TokioIo::new(tcp_stream)))
        }
    }
}

impl Service<Uri> for ProxyConnector {
    type Response = ProxyStream;
    type Error = io::Error;
    type Future = Pin<Box<dyn Future<Output = Result<ProxyStream, io::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, uri: Uri) -> Self::Future {
        let connector = self.clone();
        Box::pin(async move { connector.connect(&uri).await })
    }
}
