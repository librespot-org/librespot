use std::{error::Error, io, net::IpAddr, time::Duration};

use http::Uri;
use hyper_util::{client::legacy::connect::HttpConnector, rt::TokioIo};
use tokio::net::TcpStream;
use tower_service::Service;
use url::Url;

use crate::proxytunnel;

const CONNECT_TIMEOUT: Duration = Duration::from_secs(5);

pub async fn connect(host: &str, port: u16, proxy: Option<&Url>) -> io::Result<TcpStream> {
    connect_with(HttpConnector::new(), host, port, proxy).await
}

async fn connect_with<R>(
    mut connector: HttpConnector<R>,
    host: &str,
    port: u16,
    proxy: Option<&Url>,
) -> io::Result<TcpStream>
where
    HttpConnector<R>: Service<Uri, Response = TokioIo<TcpStream>>,
    <HttpConnector<R> as Service<Uri>>::Error: Error + Send + Sync + 'static,
{
    // Hyper already supplies off-thread DNS, all-address fallback and a
    // 300 ms Happy Eyeballs delay. This establishes only a raw TCP stream;
    // the caller still owns Spotify's handshake or verified WebSocket TLS.
    connector.set_connect_timeout(Some(CONNECT_TIMEOUT));
    tokio::time::timeout(CONNECT_TIMEOUT, async {
        let (connect_host, connect_port) = match proxy {
            Some(proxy) => {
                debug!("Using a proxy for the TCP connection");
                let host = proxy.host_str().ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "Proxy has no host")
                })?;
                let port = proxy.port_or_known_default().ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "Proxy has no port")
                })?;
                (host, port)
            }
            None => (host, port),
        };
        let authority = match connect_host.parse::<IpAddr>() {
            Ok(IpAddr::V6(address)) => format!("[{address}]:{connect_port}"),
            _ => format!("{connect_host}:{connect_port}"),
        };
        let address = Uri::builder()
            .scheme("http")
            .authority(authority)
            .path_and_query("/")
            .build()
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error))?;
        std::future::poll_fn(|cx| connector.poll_ready(cx))
            .await
            .map_err(connect_error)?;
        let socket = connector
            .call(address)
            .await
            .map(TokioIo::into_inner)
            .map_err(connect_error)?;

        if proxy.is_some() {
            // Resolve only the proxy locally. Never bypass it after a refused
            // CONNECT, and never put proxy credentials in connection logs.
            proxytunnel::proxy_connect(socket, host, &port.to_string()).await
        } else {
            Ok(socket)
        }
    })
    .await?
}

fn connect_error(error: impl Error + Send + Sync + 'static) -> io::Error {
    let cause = error.source();
    let kind = cause
        .and_then(|cause| cause.downcast_ref::<io::Error>())
        .map_or(io::ErrorKind::Other, io::Error::kind);
    // ConnectError's Display alone omits the useful DNS/OS failure reason.
    let message = cause.map_or_else(|| error.to_string(), |cause| format!("{error}: {cause}"));
    io::Error::new(kind, message)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        future::Future,
        net::SocketAddr,
        pin::Pin,
        sync::{
            Arc, Mutex,
            atomic::{AtomicBool, Ordering},
        },
        task::{Context, Poll},
    };

    use hyper_util::client::legacy::connect::dns::Name;
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::TcpListener,
    };

    #[derive(Clone)]
    enum Answer {
        Addresses(Vec<SocketAddr>),
        Error,
        PendingReady,
        PendingLookup,
    }

    #[derive(Clone)]
    struct Resolver {
        answer: Answer,
        names: Arc<Mutex<Vec<String>>>,
        lookup_dropped: Arc<AtomicBool>,
    }

    impl Resolver {
        fn new(answer: Answer) -> Self {
            Self {
                answer,
                names: Arc::default(),
                lookup_dropped: Arc::default(),
            }
        }
    }

    struct MarkDropped(Arc<AtomicBool>);

    impl Drop for MarkDropped {
        fn drop(&mut self) {
            self.0.store(true, Ordering::SeqCst);
        }
    }

    impl Service<Name> for Resolver {
        type Response = std::vec::IntoIter<SocketAddr>;
        type Error = io::Error;
        type Future = Pin<Box<dyn Future<Output = io::Result<Self::Response>> + Send>>;

        fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<io::Result<()>> {
            if matches!(self.answer, Answer::PendingReady) {
                Poll::Pending
            } else {
                Poll::Ready(Ok(()))
            }
        }

        fn call(&mut self, name: Name) -> Self::Future {
            self.names.lock().unwrap().push(name.as_str().to_owned());
            let answer = self.answer.clone();
            let dropped = MarkDropped(self.lookup_dropped.clone());
            Box::pin(async move {
                let _dropped = dropped;
                match answer {
                    Answer::Addresses(addresses) => Ok(addresses.into_iter()),
                    Answer::Error => {
                        Err(io::Error::new(io::ErrorKind::NotFound, "fixture DNS error"))
                    }
                    Answer::PendingLookup => std::future::pending().await,
                    Answer::PendingReady => panic!("unready resolver called"),
                }
            })
        }
    }

    async fn assert_fallback(first: &str) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let resolver = Resolver::new(Answer::Addresses(vec![
            first.parse().unwrap(),
            "127.0.0.1:0".parse().unwrap(),
        ]));
        let mut client = connect_with(
            HttpConnector::new_with_resolver(resolver.clone()),
            "playback.invalid",
            port,
            None,
        )
        .await
        .unwrap();
        assert_eq!(client.peer_addr().unwrap(), listener.local_addr().unwrap());
        assert_eq!(*resolver.names.lock().unwrap(), ["playback.invalid"]);
        let (mut server, _) = listener.accept().await.unwrap();
        client.write_all(b"raw TCP").await.unwrap();
        let mut received = [0; 7];
        server.read_exact(&mut received).await.unwrap();
        assert_eq!(&received, b"raw TCP");
    }

    #[tokio::test]
    async fn connects_to_later_address_in_same_family() {
        assert_fallback("127.0.0.2:0").await;
    }

    #[tokio::test]
    async fn connects_to_ipv4_when_first_ipv6_address_fails() {
        assert_fallback("[::1]:0").await;
    }

    #[tokio::test]
    async fn literal_address_does_not_need_dns() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let resolver = Resolver::new(Answer::Error);
        connect_with(
            HttpConnector::new_with_resolver(resolver.clone()),
            "127.0.0.1",
            listener.local_addr().unwrap().port(),
            None,
        )
        .await
        .unwrap();
        assert!(resolver.names.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn dns_failure_and_empty_answers_return_errors() {
        for (answer, expected) in [
            (Answer::Error, io::ErrorKind::NotFound),
            (Answer::Addresses(vec![]), io::ErrorKind::NotConnected),
        ] {
            let resolver = Resolver::new(answer);
            let error = connect_with(
                HttpConnector::new_with_resolver(resolver.clone()),
                "playback.invalid",
                4070,
                None,
            )
            .await
            .unwrap_err();
            assert_eq!(error.kind(), expected);
            if expected == io::ErrorKind::NotFound {
                assert!(error.to_string().contains("fixture DNS error"));
            }
            assert_eq!(*resolver.names.lock().unwrap(), ["playback.invalid"]);
        }
    }

    #[tokio::test(start_paused = true)]
    async fn stalled_dns_readiness_and_lookup_obey_whole_attempt_deadline() {
        for answer in [Answer::PendingReady, Answer::PendingLookup] {
            let resolver = Resolver::new(answer);
            let start = tokio::time::Instant::now();
            let error = tokio::time::timeout(
                CONNECT_TIMEOUT + Duration::from_secs(2),
                connect_with(
                    HttpConnector::new_with_resolver(resolver.clone()),
                    "playback.invalid",
                    4070,
                    None,
                ),
            )
            .await
            .expect("socket setup exceeded its deadline")
            .unwrap_err();
            assert_eq!(error.kind(), io::ErrorKind::TimedOut);
            assert_eq!(start.elapsed(), CONNECT_TIMEOUT);
            if matches!(resolver.answer, Answer::PendingLookup) {
                assert!(resolver.lookup_dropped.load(Ordering::SeqCst));
            } else {
                assert!(resolver.names.lock().unwrap().is_empty());
            }
        }
    }

    async fn read_connect(server: &mut TcpStream) -> Vec<u8> {
        let mut request = Vec::new();
        while !request.ends_with(b"\r\n\r\n") {
            request.push(server.read_u8().await.unwrap());
            assert!(request.len() < 256);
        }
        request
    }

    #[tokio::test]
    async fn proxy_uses_address_fallback_and_resolves_only_proxy() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let proxy = Url::parse(&format!(
            "http://proxy.invalid:{}",
            listener.local_addr().unwrap().port()
        ))
        .unwrap();
        let resolver = Resolver::new(Answer::Addresses(vec![
            "127.0.0.2:0".parse().unwrap(),
            "127.0.0.1:0".parse().unwrap(),
        ]));
        let server = tokio::spawn(async move {
            let (mut server, _) = listener.accept().await.unwrap();
            assert_eq!(
                read_connect(&mut server).await,
                b"CONNECT unresolved-target.invalid:4070 HTTP/1.1\r\n\r\n"
            );
            server.write_all(b"HTTP/1.1 200 OK\r\n\r\n").await.unwrap();
            assert_eq!(server.read_u8().await.unwrap(), 42);
            server.write_u8(43).await.unwrap();
        });
        let mut client = connect_with(
            HttpConnector::new_with_resolver(resolver.clone()),
            "unresolved-target.invalid",
            4070,
            Some(&proxy),
        )
        .await
        .unwrap();
        client.write_u8(42).await.unwrap();
        assert_eq!(client.read_u8().await.unwrap(), 43);
        server.await.unwrap();
        assert_eq!(*resolver.names.lock().unwrap(), ["proxy.invalid"]);
    }

    #[tokio::test]
    async fn rejected_proxy_never_connects_directly_or_exposes_proxy_credentials() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let direct = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let proxy = Url::parse(&format!(
            "http://fixture-user:fixture-password@proxy.invalid:{}",
            listener.local_addr().unwrap().port()
        ))
        .unwrap();
        let resolver = Resolver::new(Answer::Addresses(vec!["127.0.0.1:0".parse().unwrap()]));
        let server = tokio::spawn(async move {
            let (mut server, _) = listener.accept().await.unwrap();
            let request = read_connect(&mut server).await;
            assert!(!String::from_utf8(request).unwrap().contains("fixture-"));
            server
                .write_all(b"HTTP/1.1 407 Proxy Authentication Required\r\n\r\n")
                .await
                .unwrap();
        });
        let error = connect_with(
            HttpConnector::new_with_resolver(resolver.clone()),
            "127.0.0.1",
            direct.local_addr().unwrap().port(),
            Some(&proxy),
        )
        .await
        .unwrap_err();
        assert!(error.to_string().contains("407"));
        assert!(!format!("{error:?}").contains("fixture-"));
        server.await.unwrap();
        assert!(
            tokio::time::timeout(Duration::from_millis(20), direct.accept())
                .await
                .is_err()
        );
        assert_eq!(*resolver.names.lock().unwrap(), ["proxy.invalid"]);
    }

    #[tokio::test]
    async fn stalled_proxy_times_out_and_closes_socket() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let proxy = Url::parse(&format!("http://{}", listener.local_addr().unwrap())).unwrap();
        let server = tokio::spawn(async move {
            let (mut server, _) = listener.accept().await.unwrap();
            read_connect(&mut server).await;
            let mut byte = [0];
            assert_eq!(server.read(&mut byte).await.unwrap(), 0);
        });
        let error = tokio::time::timeout(
            CONNECT_TIMEOUT + Duration::from_secs(2),
            connect("unresolved-target.invalid", 4070, Some(&proxy)),
        )
        .await
        .expect("proxy setup exceeded its deadline")
        .unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::TimedOut);
        tokio::time::timeout(Duration::from_secs(2), server)
            .await
            .unwrap()
            .unwrap();
    }
}
