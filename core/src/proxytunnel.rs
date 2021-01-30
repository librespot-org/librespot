use std::io;

use hyper::Uri;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

pub async fn connect<T: AsyncRead + AsyncWrite + Unpin>(
    mut connection: T,
    connect_url: &str,
) -> io::Result<T> {
    let uri = connect_url.parse::<Uri>().unwrap();
    let mut buffer = format!(
        "CONNECT {0}:{1} HTTP/1.1\r\n\
         \r\n",
        uri.host().unwrap_or_else(|| panic!("No host in {}", uri)),
        uri.port().unwrap_or_else(|| panic!("No port in {}", uri))
    )
    .into_bytes();
    connection.write_all(buffer.as_ref()).await?;

    buffer.resize(buffer.capacity(), 0);

    let mut offset = 0;
    loop {
        let bytes_read = connection.read(&mut buffer[offset..]).await?;
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
                Some(200) => Ok(connection), // Proxy says all is well
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
