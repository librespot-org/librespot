use std::error::Error;
use std::io;
use std::str::FromStr;

use futures::{Async, Future, Poll};
use httparse;
use hyper::Uri;
use tokio_io::io::{read, write_all, Read, Window, WriteAll};
use tokio_io::{AsyncRead, AsyncWrite};

pub struct ProxyTunnel<T> {
    state: ProxyState<T>,
}

enum ProxyState<T> {
    ProxyConnect(WriteAll<T, Vec<u8>>),
    ProxyResponse(Read<T, Window<Vec<u8>>>),
}

pub fn connect<T: AsyncRead + AsyncWrite>(connection: T, connect_url: &str) -> ProxyTunnel<T> {
    let proxy = proxy_connect(connection, connect_url);
    ProxyTunnel {
        state: ProxyState::ProxyConnect(proxy),
    }
}

impl<T: AsyncRead + AsyncWrite> Future for ProxyTunnel<T> {
    type Item = T;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, io::Error> {
        use self::ProxyState::*;
        loop {
            self.state = match self.state {
                ProxyConnect(ref mut write) => {
                    let (connection, mut accumulator) = try_ready!(write.poll());

                    let capacity = accumulator.capacity();
                    accumulator.resize(capacity, 0);
                    let window = Window::new(accumulator);

                    let read = read(connection, window);
                    ProxyResponse(read)
                }

                ProxyResponse(ref mut read_f) => {
                    let (connection, mut window, bytes_read) = try_ready!(read_f.poll());

                    if bytes_read == 0 {
                        return Err(io::Error::new(io::ErrorKind::Other, "Early EOF from proxy"));
                    }

                    let data_end = window.start() + bytes_read;

                    let buf = window.get_ref()[0..data_end].to_vec();
                    let mut headers = [httparse::EMPTY_HEADER; 16];
                    let mut response = httparse::Response::new(&mut headers);
                    let status = match response.parse(&buf) {
                        Ok(status) => status,
                        Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err.description())),
                    };

                    if status.is_complete() {
                        if let Some(code) = response.code {
                            if code == 200 {
                                // Proxy says all is well
                                return Ok(Async::Ready(connection));
                            } else {
                                let reason = response.reason.unwrap_or("no reason");
                                let msg = format!("Proxy responded with {}: {}", code, reason);

                                return Err(io::Error::new(io::ErrorKind::Other, msg));
                            }
                        } else {
                            return Err(io::Error::new(
                                io::ErrorKind::Other,
                                "Malformed response from proxy",
                            ));
                        }
                    } else {
                        if data_end >= window.end() {
                            // Allocate some more buffer space
                            let newsize = data_end + 100;
                            window.get_mut().resize(newsize, 0);
                            window.set_end(newsize);
                        }
                        // We did not get a full header
                        window.set_start(data_end);
                        let read = read(connection, window);
                        ProxyResponse(read)
                    }
                }
            }
        }
    }
}

fn proxy_connect<T: AsyncWrite>(connection: T, connect_url: &str) -> WriteAll<T, Vec<u8>> {
    let uri = Uri::from_str(connect_url).unwrap();
    let buffer = format!(
        "CONNECT {0}:{1} HTTP/1.1\r\n\
         \r\n",
        uri.host().expect(&format!("No host in {}", uri)),
        uri.port().expect(&format!("No port in {}", uri))
    ).into_bytes();

    write_all(connection, buffer)
}
