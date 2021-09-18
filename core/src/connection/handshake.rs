use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use hmac::{Hmac, Mac, NewMac};
use protobuf::{self, Message};
use rand::{thread_rng, RngCore};
use sha1::Sha1;
use std::io;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio_util::codec::{Decoder, Framed};

use super::codec::ApCodec;
use crate::diffie_hellman::DhLocalKeys;
use crate::protocol;
use crate::protocol::keyexchange::{APResponseMessage, ClientHello, ClientResponsePlaintext};

pub async fn handshake<T: AsyncRead + AsyncWrite + Unpin>(
    mut connection: T,
) -> io::Result<Framed<T, ApCodec>> {
    let local_keys = DhLocalKeys::random(&mut thread_rng());
    let gc = local_keys.public_key();
    let mut accumulator = client_hello(&mut connection, gc).await?;
    let message: APResponseMessage = recv_packet(&mut connection, &mut accumulator).await?;
    let remote_key = message
        .get_challenge()
        .get_login_crypto_challenge()
        .get_diffie_hellman()
        .get_gs()
        .to_owned();

    let shared_secret = local_keys.shared_secret(&remote_key);
    let (challenge, send_key, recv_key) = compute_keys(&shared_secret, &accumulator);
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

    let mut packet = ClientHello::new();
    packet
        .mut_build_info()
        .set_product(protocol::keyexchange::Product::PRODUCT_PARTNER);
    packet
        .mut_build_info()
        .set_platform(protocol::keyexchange::Platform::PLATFORM_LINUX_X86);
    packet.mut_build_info().set_version(109800078);
    packet
        .mut_cryptosuites_supported()
        .push(protocol::keyexchange::Cryptosuite::CRYPTO_SUITE_SHANNON);
    packet
        .mut_login_crypto_hello()
        .mut_diffie_hellman()
        .set_gc(gc);
    packet
        .mut_login_crypto_hello()
        .mut_diffie_hellman()
        .set_server_keys_known(1);
    packet.set_client_nonce(client_nonce);
    packet.set_padding(vec![0x1e]);

    let mut buffer = vec![0, 4];
    let size = 2 + 4 + packet.compute_size();
    <Vec<u8> as WriteBytesExt>::write_u32::<BigEndian>(&mut buffer, size).unwrap();
    packet.write_to_vec(&mut buffer).unwrap();

    connection.write_all(&buffer[..]).await?;
    Ok(buffer)
}

async fn client_response<T>(connection: &mut T, challenge: Vec<u8>) -> io::Result<()>
where
    T: AsyncWrite + Unpin,
{
    let mut packet = ClientResponsePlaintext::new();
    packet
        .mut_login_crypto_response()
        .mut_diffie_hellman()
        .set_hmac(challenge);
    packet.mut_pow_response();
    packet.mut_crypto_response();

    let mut buffer = vec![];
    let size = 4 + packet.compute_size();
    <Vec<u8> as WriteBytesExt>::write_u32::<BigEndian>(&mut buffer, size).unwrap();
    packet.write_to_vec(&mut buffer).unwrap();

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
    let message = M::parse_from_bytes(data).unwrap();
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

fn compute_keys(shared_secret: &[u8], packets: &[u8]) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    type HmacSha1 = Hmac<Sha1>;

    let mut data = Vec::with_capacity(0x64);
    for i in 1..6 {
        let mut mac =
            HmacSha1::new_from_slice(shared_secret).expect("HMAC can take key of any size");
        mac.update(packets);
        mac.update(&[i]);
        data.extend_from_slice(&mac.finalize().into_bytes());
    }

    let mut mac = HmacSha1::new_from_slice(&data[..0x14]).expect("HMAC can take key of any size");
    mac.update(packets);

    (
        mac.finalize().into_bytes().to_vec(),
        data[0x14..0x34].to_vec(),
        data[0x34..0x54].to_vec(),
    )
}
