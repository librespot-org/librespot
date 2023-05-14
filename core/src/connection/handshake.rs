use std::{env::consts::ARCH, io};

use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use hmac::{Hmac, Mac};
use protobuf::{self, Message};
use rand::{thread_rng, RngCore};
use rsa::{BigUint, Pkcs1v15Sign, RsaPublicKey};
use sha1::{Digest, Sha1};
use thiserror::Error;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio_util::codec::{Decoder, Framed};

use super::codec::ApCodec;

use crate::{diffie_hellman::DhLocalKeys, version};

use crate::protocol;
use crate::protocol::keyexchange::{
    APResponseMessage, ClientHello, ClientResponsePlaintext, Platform, ProductFlags,
};

const SERVER_KEY: [u8; 256] = [
    0xac, 0xe0, 0x46, 0x0b, 0xff, 0xc2, 0x30, 0xaf, 0xf4, 0x6b, 0xfe, 0xc3, 0xbf, 0xbf, 0x86, 0x3d,
    0xa1, 0x91, 0xc6, 0xcc, 0x33, 0x6c, 0x93, 0xa1, 0x4f, 0xb3, 0xb0, 0x16, 0x12, 0xac, 0xac, 0x6a,
    0xf1, 0x80, 0xe7, 0xf6, 0x14, 0xd9, 0x42, 0x9d, 0xbe, 0x2e, 0x34, 0x66, 0x43, 0xe3, 0x62, 0xd2,
    0x32, 0x7a, 0x1a, 0x0d, 0x92, 0x3b, 0xae, 0xdd, 0x14, 0x02, 0xb1, 0x81, 0x55, 0x05, 0x61, 0x04,
    0xd5, 0x2c, 0x96, 0xa4, 0x4c, 0x1e, 0xcc, 0x02, 0x4a, 0xd4, 0xb2, 0x0c, 0x00, 0x1f, 0x17, 0xed,
    0xc2, 0x2f, 0xc4, 0x35, 0x21, 0xc8, 0xf0, 0xcb, 0xae, 0xd2, 0xad, 0xd7, 0x2b, 0x0f, 0x9d, 0xb3,
    0xc5, 0x32, 0x1a, 0x2a, 0xfe, 0x59, 0xf3, 0x5a, 0x0d, 0xac, 0x68, 0xf1, 0xfa, 0x62, 0x1e, 0xfb,
    0x2c, 0x8d, 0x0c, 0xb7, 0x39, 0x2d, 0x92, 0x47, 0xe3, 0xd7, 0x35, 0x1a, 0x6d, 0xbd, 0x24, 0xc2,
    0xae, 0x25, 0x5b, 0x88, 0xff, 0xab, 0x73, 0x29, 0x8a, 0x0b, 0xcc, 0xcd, 0x0c, 0x58, 0x67, 0x31,
    0x89, 0xe8, 0xbd, 0x34, 0x80, 0x78, 0x4a, 0x5f, 0xc9, 0x6b, 0x89, 0x9d, 0x95, 0x6b, 0xfc, 0x86,
    0xd7, 0x4f, 0x33, 0xa6, 0x78, 0x17, 0x96, 0xc9, 0xc3, 0x2d, 0x0d, 0x32, 0xa5, 0xab, 0xcd, 0x05,
    0x27, 0xe2, 0xf7, 0x10, 0xa3, 0x96, 0x13, 0xc4, 0x2f, 0x99, 0xc0, 0x27, 0xbf, 0xed, 0x04, 0x9c,
    0x3c, 0x27, 0x58, 0x04, 0xb6, 0xb2, 0x19, 0xf9, 0xc1, 0x2f, 0x02, 0xe9, 0x48, 0x63, 0xec, 0xa1,
    0xb6, 0x42, 0xa0, 0x9d, 0x48, 0x25, 0xf8, 0xb3, 0x9d, 0xd0, 0xe8, 0x6a, 0xf9, 0x48, 0x4d, 0xa1,
    0xc2, 0xba, 0x86, 0x30, 0x42, 0xea, 0x9d, 0xb3, 0x08, 0x6c, 0x19, 0x0e, 0x48, 0xb3, 0x9d, 0x66,
    0xeb, 0x00, 0x06, 0xa2, 0x5a, 0xee, 0xa1, 0x1b, 0x13, 0x87, 0x3c, 0xd7, 0x19, 0xe6, 0x55, 0xbd,
];

#[derive(Debug, Error)]
pub enum HandshakeError {
    #[error("invalid key length")]
    InvalidLength,
    #[error("server key verification failed")]
    VerificationFailed,
}

pub async fn handshake<T: AsyncRead + AsyncWrite + Unpin>(
    mut connection: T,
) -> io::Result<Framed<T, ApCodec>> {
    let local_keys = DhLocalKeys::random(&mut thread_rng());
    let gc = local_keys.public_key();
    let mut accumulator = client_hello(&mut connection, gc).await?;
    let message: APResponseMessage = recv_packet(&mut connection, &mut accumulator).await?;
    let remote_key = message
        .challenge
        .get_or_default()
        .login_crypto_challenge
        .get_or_default()
        .diffie_hellman
        .get_or_default()
        .gs()
        .to_owned();
    let remote_signature = message
        .challenge
        .get_or_default()
        .login_crypto_challenge
        .get_or_default()
        .diffie_hellman
        .get_or_default()
        .gs_signature()
        .to_owned();

    // Prevent man-in-the-middle attacks: check server signature
    let n = BigUint::from_bytes_be(&SERVER_KEY);
    let e = BigUint::new(vec![65537]);
    let public_key = RsaPublicKey::new(n, e).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            HandshakeError::VerificationFailed,
        )
    })?;

    let hash = Sha1::digest(&remote_key);
    let padding = Pkcs1v15Sign::new::<Sha1>();
    public_key
        .verify(padding, &hash, &remote_signature)
        .map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                HandshakeError::VerificationFailed,
            )
        })?;

    // OK to proceed
    let shared_secret = local_keys.shared_secret(&remote_key);
    let (challenge, send_key, recv_key) = compute_keys(&shared_secret, &accumulator)?;
    let codec = ApCodec::new(&send_key, &recv_key);

    client_response(&mut connection, challenge).await?;

    Ok(codec.framed(connection))
}

async fn client_hello<T>(connection: &mut T, gc: Vec<u8>) -> io::Result<Vec<u8>>
where
    T: AsyncWrite + Unpin,
{
    let mut client_nonce = vec![0; 0x10];
    thread_rng().fill_bytes(&mut client_nonce);

    let platform = match std::env::consts::OS {
        "android" => Platform::PLATFORM_ANDROID_ARM,
        "freebsd" | "netbsd" | "openbsd" => match ARCH {
            "x86_64" => Platform::PLATFORM_FREEBSD_X86_64,
            _ => Platform::PLATFORM_FREEBSD_X86,
        },
        "ios" => match ARCH {
            "aarch64" => Platform::PLATFORM_IPHONE_ARM64,
            _ => Platform::PLATFORM_IPHONE_ARM,
        },
        "linux" => match ARCH {
            "arm" | "aarch64" => Platform::PLATFORM_LINUX_ARM,
            "blackfin" => Platform::PLATFORM_LINUX_BLACKFIN,
            "mips" => Platform::PLATFORM_LINUX_MIPS,
            "sh" => Platform::PLATFORM_LINUX_SH,
            "x86_64" => Platform::PLATFORM_LINUX_X86_64,
            _ => Platform::PLATFORM_LINUX_X86,
        },
        "macos" => match ARCH {
            "ppc" | "ppc64" => Platform::PLATFORM_OSX_PPC,
            "x86_64" => Platform::PLATFORM_OSX_X86_64,
            _ => Platform::PLATFORM_OSX_X86,
        },
        "windows" => match ARCH {
            "arm" | "aarch64" => Platform::PLATFORM_WINDOWS_CE_ARM,
            "x86_64" => Platform::PLATFORM_WIN32_X86_64,
            _ => Platform::PLATFORM_WIN32_X86,
        },
        _ => Platform::PLATFORM_LINUX_X86,
    };

    #[cfg(debug_assertions)]
    const PRODUCT_FLAGS: ProductFlags = ProductFlags::PRODUCT_FLAG_DEV_BUILD;
    #[cfg(not(debug_assertions))]
    const PRODUCT_FLAGS: ProductFlags = ProductFlags::PRODUCT_FLAG_NONE;

    let mut packet = ClientHello::new();
    packet
        .build_info
        .mut_or_insert_default()
        // ProductInfo won't push autoplay and perhaps other settings
        // when set to anything else than PRODUCT_CLIENT
        .set_product(protocol::keyexchange::Product::PRODUCT_CLIENT);
    packet
        .build_info
        .mut_or_insert_default()
        .product_flags
        .push(PRODUCT_FLAGS.into());
    packet
        .build_info
        .mut_or_insert_default()
        .set_platform(platform);
    packet
        .build_info
        .mut_or_insert_default()
        .set_version(version::SPOTIFY_VERSION);
    packet
        .cryptosuites_supported
        .push(protocol::keyexchange::Cryptosuite::CRYPTO_SUITE_SHANNON.into());
    packet
        .login_crypto_hello
        .mut_or_insert_default()
        .diffie_hellman
        .mut_or_insert_default()
        .set_gc(gc);
    packet
        .login_crypto_hello
        .mut_or_insert_default()
        .diffie_hellman
        .mut_or_insert_default()
        .set_server_keys_known(1);
    packet.set_client_nonce(client_nonce);
    packet.set_padding(vec![0x1e]);

    let mut buffer = vec![0, 4];
    let size = 2 + 4 + packet.compute_size();
    <Vec<u8> as WriteBytesExt>::write_u32::<BigEndian>(&mut buffer, size.try_into().unwrap())?;
    packet.write_to_vec(&mut buffer)?;

    connection.write_all(&buffer[..]).await?;
    Ok(buffer)
}

async fn client_response<T>(connection: &mut T, challenge: Vec<u8>) -> io::Result<()>
where
    T: AsyncWrite + Unpin,
{
    let mut packet = ClientResponsePlaintext::new();
    packet
        .login_crypto_response
        .mut_or_insert_default()
        .diffie_hellman
        .mut_or_insert_default()
        .set_hmac(challenge);

    packet.pow_response.mut_or_insert_default();
    packet.crypto_response.mut_or_insert_default();

    let mut buffer = vec![];
    let size = 4 + packet.compute_size();
    <Vec<u8> as WriteBytesExt>::write_u32::<BigEndian>(&mut buffer, size.try_into().unwrap())?;
    packet.write_to_vec(&mut buffer)?;

    connection.write_all(&buffer[..]).await?;
    Ok(())
}

async fn recv_packet<T, M>(connection: &mut T, acc: &mut Vec<u8>) -> io::Result<M>
where
    T: AsyncRead + Unpin,
    M: Message,
{
    let header = read_into_accumulator(connection, 4, acc).await?;
    let size = BigEndian::read_u32(header) as usize;
    let data = read_into_accumulator(connection, size - 4, acc).await?;
    let message = M::parse_from_bytes(data)?;
    Ok(message)
}

async fn read_into_accumulator<'a, 'b, T: AsyncRead + Unpin>(
    connection: &'a mut T,
    size: usize,
    acc: &'b mut Vec<u8>,
) -> io::Result<&'b mut [u8]> {
    let offset = acc.len();
    acc.resize(offset + size, 0);

    connection.read_exact(&mut acc[offset..]).await?;
    Ok(&mut acc[offset..])
}

fn compute_keys(shared_secret: &[u8], packets: &[u8]) -> io::Result<(Vec<u8>, Vec<u8>, Vec<u8>)> {
    type HmacSha1 = Hmac<Sha1>;

    let mut data = Vec::with_capacity(0x64);
    for i in 1..6 {
        let mut mac = HmacSha1::new_from_slice(shared_secret).map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidData, HandshakeError::InvalidLength)
        })?;
        mac.update(packets);
        mac.update(&[i]);
        data.extend_from_slice(&mac.finalize().into_bytes());
    }

    let mut mac = HmacSha1::new_from_slice(&data[..0x14])
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, HandshakeError::InvalidLength))?;
    mac.update(packets);

    Ok((
        mac.finalize().into_bytes().to_vec(),
        data[0x14..0x34].to_vec(),
        data[0x34..0x54].to_vec(),
    ))
}
