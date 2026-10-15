use std::io;

use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use url::Url;

/// Credentials for SOCKS5 username/password authentication (RFC 1929).
#[derive(Clone, Debug, Default)]
pub struct Socks5Auth {
    pub username: String,
    pub password: String,
}

impl Socks5Auth {
    /// Extract username/password credentials from a proxy URL, percent-decoding
    /// them on the way.
    pub fn from_url(proxy_url: &Url) -> Option<Self> {
        let (username, password) = credentials_from_url(proxy_url)?;
        Some(Self { username, password })
    }
}

/// Extract percent-decoded credentials from a proxy URL.
fn credentials_from_url(proxy_url: &Url) -> Option<(String, String)> {
    if proxy_url.username().is_empty() && proxy_url.password().is_none() {
        return None;
    }
    Some((
        percent_decode(proxy_url.username()),
        proxy_url.password().map(percent_decode).unwrap_or_default(),
    ))
}

/// Build a `Proxy-Authorization: Basic` header value from a proxy URL.
pub fn http_basic_auth(proxy_url: &Url) -> Option<String> {
    use base64::Engine;
    let (username, password) = credentials_from_url(proxy_url)?;
    let credentials = format!("{username}:{password}");
    Some(base64::engine::general_purpose::STANDARD.encode(credentials))
}

fn percent_decode(value: &str) -> String {
    let bytes = value.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            let hex = |b: u8| match b {
                b'0'..=b'9' => Some(b - b'0'),
                b'a'..=b'f' => Some(b - b'a' + 10),
                b'A'..=b'F' => Some(b - b'A' + 10),
                _ => None,
            };
            if let (Some(h), Some(l)) = (
                bytes.get(i + 1).copied().and_then(hex),
                bytes.get(i + 2).copied().and_then(hex),
            ) {
                out.push(h * 16 + l);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// Perform a SOCKS5 handshake (RFC 1928) over `proxy_connection`, asking the
/// proxy to connect to `connect_host:connect_port`.
///
/// The hostname is passed to the proxy verbatim (ATYP = domain name), i.e. the
/// proxy performs the DNS resolution. This is the `socks5h` semantics from
/// curl, and it is applied for both `socks5://` and `socks5h://` proxy URLs,
/// because resolving hostnames locally is usually exactly what users of a
/// SOCKS proxy want to avoid.
pub async fn socks5_connect<T: AsyncRead + AsyncWrite + Unpin>(
    mut proxy_connection: T,
    connect_host: &str,
    connect_port: u16,
    auth: Option<&Socks5Auth>,
) -> io::Result<T> {
    // Greeting: offered auth methods (0x00 = none, 0x02 = user/pass).
    let mut greeting = vec![0x05, 0x01, 0x00];
    if auth.is_some() {
        greeting = vec![0x05, 0x02, 0x00, 0x02];
    }
    proxy_connection.write_all(&greeting).await?;

    let mut response = [0u8; 2];
    proxy_connection.read_exact(&mut response).await?;
    if response[0] != 0x05 {
        return Err(io::Error::other("Malformed SOCKS5 greeting response"));
    }

    match response[1] {
        0x00 => {} // no auth required
        0x02 => {
            // RFC 1929 username/password subnegotiation.
            let auth = auth.expect("proxy demanded auth but none was offered");
            let username = auth.username.as_bytes();
            let password = auth.password.as_bytes();
            if username.len() > 255 || password.len() > 255 {
                return Err(io::Error::other("SOCKS5 proxy credentials are too long"));
            }
            let mut request = Vec::with_capacity(3 + username.len() + password.len());
            request.push(0x01);
            request.push(username.len() as u8);
            request.extend_from_slice(username);
            request.push(password.len() as u8);
            request.extend_from_slice(password);
            proxy_connection.write_all(&request).await?;

            let mut auth_response = [0u8; 2];
            proxy_connection.read_exact(&mut auth_response).await?;
            if auth_response[1] != 0x00 {
                return Err(io::Error::other(
                    "SOCKS5 proxy rejected the provided credentials",
                ));
            }
        }
        0xff => {
            return Err(io::Error::other(
                "SOCKS5 proxy offers no acceptable auth method",
            ));
        }
        method => {
            return Err(io::Error::other(format!(
                "SOCKS5 proxy selected unsupported auth method {method:#04x}"
            )));
        }
    }

    // CONNECT request with the hostname passed through for remote resolution.
    if connect_host.len() > 255 {
        return Err(io::Error::other("Destination hostname is too long"));
    }
    let mut request = Vec::with_capacity(7 + connect_host.len());
    request.extend_from_slice(&[0x05, 0x01, 0x00, 0x03]);
    request.push(connect_host.len() as u8);
    request.extend_from_slice(connect_host.as_bytes());
    request.extend_from_slice(&connect_port.to_be_bytes());
    proxy_connection.write_all(&request).await?;

    // Read the fixed part of the reply, then discard the bound address.
    let mut reply = [0u8; 4];
    proxy_connection.read_exact(&mut reply).await?;
    if reply[0] != 0x05 {
        return Err(io::Error::other("Malformed SOCKS5 reply"));
    }
    if reply[1] != 0x00 {
        let reason = match reply[1] {
            0x01 => "general SOCKS server failure",
            0x02 => "connection not allowed by ruleset",
            0x03 => "network unreachable",
            0x04 => "host unreachable",
            0x05 => "connection refused",
            0x06 => "TTL expired",
            0x07 => "command not supported",
            0x08 => "address type not supported",
            code => return Err(io::Error::other(format!("SOCKS5 error code {code:#04x}"))),
        };
        return Err(io::Error::other(format!("SOCKS5 proxy: {reason}")));
    }
    match reply[3] {
        0x01 => {
            let mut addr = [0u8; 4];
            proxy_connection.read_exact(&mut addr).await?;
        }
        0x03 => {
            let mut len = [0u8; 1];
            proxy_connection.read_exact(&mut len).await?;
            let mut addr = vec![0u8; len[0] as usize];
            proxy_connection.read_exact(&mut addr).await?;
        }
        0x04 => {
            let mut addr = [0u8; 16];
            proxy_connection.read_exact(&mut addr).await?;
        }
        atyp => {
            return Err(io::Error::other(format!(
                "SOCKS5 reply has unknown address type {atyp:#04x}"
            )));
        }
    }
    let mut port = [0u8; 2];
    proxy_connection.read_exact(&mut port).await?;

    Ok(proxy_connection)
}

pub async fn proxy_connect<T: AsyncRead + AsyncWrite + Unpin>(
    mut proxy_connection: T,
    connect_host: &str,
    connect_port: &str,
    basic_auth: Option<&str>,
) -> io::Result<T> {
    let mut buffer = Vec::new();
    buffer.extend_from_slice(b"CONNECT ");
    buffer.extend_from_slice(connect_host.as_bytes());
    buffer.push(b':');
    buffer.extend_from_slice(connect_port.as_bytes());
    buffer.extend_from_slice(b" HTTP/1.1\r\n");
    if let Some(basic_auth) = basic_auth {
        buffer.extend_from_slice(b"Proxy-Authorization: Basic ");
        buffer.extend_from_slice(basic_auth.as_bytes());
        buffer.extend_from_slice(b"\r\n");
    }
    buffer.extend_from_slice(b"\r\n");

    proxy_connection.write_all(buffer.as_ref()).await?;

    buffer.resize(buffer.capacity(), 0);

    let mut offset = 0;
    loop {
        let bytes_read = proxy_connection.read(&mut buffer[offset..]).await?;
        if bytes_read == 0 {
            return Err(io::Error::other("Early EOF from proxy"));
        }
        offset += bytes_read;

        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut response = httparse::Response::new(&mut headers);

        let status = response
            .parse(&buffer[..offset])
            .map_err(io::Error::other)?;

        if status.is_complete() {
            return match response.code {
                Some(200) => Ok(proxy_connection), // Proxy says all is well
                Some(code) => {
                    let reason = response.reason.unwrap_or("no reason");
                    let msg = format!("Proxy responded with {code}: {reason}");
                    Err(io::Error::other(msg))
                }
                None => Err(io::Error::other("Malformed response from proxy")),
            };
        }

        if offset >= buffer.len() {
            buffer.resize(buffer.len() + 100, 0);
        }
    }
}
