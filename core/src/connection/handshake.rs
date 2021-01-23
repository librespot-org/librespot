use super::codec::APCodec;
use crate::{
    diffie_hellman::DHLocalKeys,
    protocol,
    protocol::keyexchange::{APResponseMessage, ClientHello, ClientResponsePlaintext},
    util,
};

use hmac::{Hmac, Mac};
use protobuf::{self, Message};
use rand::thread_rng;
use sha1::Sha1;
use std::{io, marker::Unpin};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio_util::codec::{Decoder, Framed};

// struct handshake {
//     keys: DHLocalKeys,
//     connection: T,
//     accumulator: Vec<u8>,
// }

pub async fn handshake<T: AsyncRead + AsyncWrite + Unpin>(
    mut connection: T,
) -> Result<Framed<T, APCodec>, io::Error> {
    let local_keys = DHLocalKeys::random(&mut thread_rng());
    // Send ClientHello
    let client_hello: Vec<u8> = client_hello(local_keys.public_key()).await?;
    connection.write_all(&client_hello).await?;

    // Receive APResponseMessage
    let size = connection.read_u32().await?;
    let mut buffer = Vec::with_capacity(size as usize - 4);
    let bytes = connection.read_buf(&mut buffer).await?;
    let message = protobuf::parse_from_bytes::<APResponseMessage>(&buffer[..bytes])?;

    let mut accumulator = client_hello.clone();
    accumulator.extend_from_slice(&size.to_be_bytes());
    accumulator.extend_from_slice(&buffer);
    let remote_key = message
        .get_challenge()
        .get_login_crypto_challenge()
        .get_diffie_hellman()
        .get_gs()
        .to_owned();

    // Solve the challenge
    let shared_secret = local_keys.shared_secret(&remote_key);
    let (challenge, send_key, recv_key) = compute_keys(&shared_secret, &accumulator);
    let codec = APCodec::new(&send_key, &recv_key);

    let buffer: Vec<u8> = client_response(challenge).await?;
    connection.write_all(&buffer).await?;
    let framed = codec.framed(connection);
    Ok(framed)
}

// async fn recv_packet<T: AsyncRead + Unpin, Message: protobuf::Message>(
//     mut connection: T,
// ) -> Result<(Message, &Vec<u8>), io::Error> {
//     let size = connection.read_u32().await?;
//     let mut buffer = Vec::with_capacity(size as usize - 4);
//     let bytes = connection.read_buf(&mut buffer).await?;
//     let proto = protobuf::parse_from_bytes(&buffer[..bytes])?;
//     Ok(proto)
// }

async fn client_hello(gc: Vec<u8>) -> Result<Vec<u8>, io::Error> {
    let mut packet = ClientHello::new();
    packet
        .mut_build_info()
        .set_product(protocol::keyexchange::Product::PRODUCT_PARTNER);
    packet
        .mut_build_info()
        .set_platform(protocol::keyexchange::Platform::PLATFORM_LINUX_X86);
    packet.mut_build_info().set_version(109_800_078);
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
    packet.set_client_nonce(util::rand_vec(&mut thread_rng(), 0x10));
    packet.set_padding(vec![0x1e]);

    let size = 2 + 4 + packet.compute_size();
    let mut buffer = Vec::with_capacity(size as usize);
    buffer.extend(&[0, 4]);
    buffer.write_u32(size).await?;
    buffer.extend(packet.write_to_bytes()?);
    Ok(buffer)
}

async fn client_response(challenge: Vec<u8>) -> Result<Vec<u8>, io::Error> {
    let mut packet = ClientResponsePlaintext::new();
    packet
        .mut_login_crypto_response()
        .mut_diffie_hellman()
        .set_hmac(challenge);
    packet.mut_pow_response();
    packet.mut_crypto_response();

    // let mut buffer = vec![];
    let size = 4 + packet.compute_size();
    let mut buffer = Vec::with_capacity(size as usize);
    buffer.write_u32(size).await?;
    // This seems to reallocate
    // packet.write_to_vec(&mut buffer)?;
    buffer.extend(packet.write_to_bytes()?);
    Ok(buffer)
}

fn compute_keys(shared_secret: &[u8], packets: &[u8]) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    type HmacSha1 = Hmac<Sha1>;

    let mut data = Vec::with_capacity(0x64);
    for i in 1..6 {
        let mut mac = HmacSha1::new_varkey(&shared_secret).expect("HMAC can take key of any size");
        mac.input(packets);
        mac.input(&[i]);
        data.extend_from_slice(&mac.result().code());
    }

    let mut mac = HmacSha1::new_varkey(&data[..0x14]).expect("HMAC can take key of any size");
    mac.input(packets);

    (
        mac.result().code().to_vec(),
        data[0x14..0x34].to_vec(),
        data[0x34..0x54].to_vec(),
    )
}
