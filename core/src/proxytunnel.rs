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

    buffer.clear();
    connection.read_to_end(&mut buffer).await?;
    if buffer.is_empty() {
        return Err(io::Error::new(io::ErrorKind::Other, "Early EOF from proxy"));
    }

    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut response = httparse::Response::new(&mut headers);

    response
        .parse(&buffer[..])
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err.to_string()))?;

    match response.code {
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
    }
}
