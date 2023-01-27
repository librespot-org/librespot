use std::io;

use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

pub async fn proxy_connect<T: AsyncRead + AsyncWrite + Unpin>(
    mut proxy_connection: T,
    connect_host: &str,
    connect_port: &str,
) -> io::Result<T> {
    let mut buffer = Vec::new();
    buffer.extend_from_slice(b"CONNECT ");
    buffer.extend_from_slice(connect_host.as_bytes());
    buffer.push(b':');
    buffer.extend_from_slice(connect_port.as_bytes());
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
                    let msg = format!("Proxy responded with {code}: {reason}");
                    Err(io::Error::new(io::ErrorKind::Other, msg))
                }
                None => Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Malformed response from proxy",
                )),
            };
        }

        if offset >= buffer.len() {
            buffer.resize(buffer.len() + 100, 0);
        }
    }
}
