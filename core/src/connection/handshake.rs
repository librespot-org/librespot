use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use futures::io::ReadExact;
use futures::io::Window;
use futures::io::WriteAll;
use futures::io::{AsyncReadExt, AsyncWriteExt};
use futures::Future;
use hmac::{Hmac, Mac};
use protobuf::{self, Message};
use rand::thread_rng;
use sha1::Sha1;
use std::io::{self, Read};
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use tokio::io::AsyncRead;
use tokio::io::AsyncWrite;
use tokio_util::codec::{Decoder, Framed};
use tokio_util::compat::{Tokio02AsyncReadCompatExt, Tokio02AsyncWriteCompatExt};
// use tokio_codec::{Decoder, Framed};
// use tokio_io::io::{read_exact, write_all, ReadExact, Window, WriteAll};
// use tokio_io::{AsyncRead, AsyncWrite};

use super::codec::APCodec;
use crate::diffie_hellman::DHLocalKeys;
use crate::protocol;
use crate::protocol::keyexchange::{APResponseMessage, ClientHello, ClientResponsePlaintext};
use crate::util;

pub struct Handshake<'a, T> {
    keys: DHLocalKeys,
    state: HandshakeState<'a, T>,
}

enum HandshakeState<'a, T> {
    ClientHello(WriteAll<'a, T>),
    APResponse(RecvPacket<'a, T, APResponseMessage>),
    ClientResponse(Option<APCodec>, WriteAll<'a, T>),
}

pub fn handshake<'a, T: AsyncRead + AsyncWrite>(connection: T) -> Handshake<'a, T> {
    let local_keys = DHLocalKeys::random(&mut thread_rng());
    let client_hello = client_hello(connection, local_keys.public_key());

    Handshake {
        keys: local_keys,
        state: HandshakeState::ClientHello(client_hello),
    }
}

impl<'a, T: AsyncRead + AsyncWrite> Future for Handshake<'a, T> {
    type Output = Result<Framed<T, APCodec>, io::Error>;

    fn poll(&mut self) -> Poll<Self::Output> {
        use self::HandshakeState::*;
        loop {
            self.state = match self.state {
                ClientHello(ref mut write) => {
                    let (connection, accumulator) = ready!(write.poll());

                    let read = recv_packet(connection, accumulator);
                    APResponse(read)
                }

                APResponse(ref mut read) => {
                    let (connection, message, accumulator) = ready!(read.poll());
                    let remote_key = message
                        .get_challenge()
                        .get_login_crypto_challenge()
                        .get_diffie_hellman()
                        .get_gs()
                        .to_owned();

                    let shared_secret = self.keys.shared_secret(&remote_key);
                    let (challenge, send_key, recv_key) =
                        compute_keys(&shared_secret, &accumulator);
                    let codec = APCodec::new(&send_key, &recv_key);

                    let write = client_response(connection, challenge);
                    ClientResponse(Some(codec), write)
                }

                ClientResponse(ref mut codec, ref mut write) => {
                    let (connection, _) = ready!(write.poll());
                    let codec = codec.take().unwrap();
                    let framed = codec.framed(connection);
                    return Poll::Ready(Ok(framed));
                }
            }
        }
    }
}

fn client_hello<'a, T: AsyncWrite>(connection: T, gc: Vec<u8>) -> WriteAll<'a, T> {
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
    packet.set_client_nonce(util::rand_vec(&mut thread_rng(), 0x10));
    packet.set_padding(vec![0x1e]);

    let mut buffer = vec![0, 4];
    let size = 2 + 4 + packet.compute_size();
    buffer.write_u32::<BigEndian>(size).unwrap();
    packet.write_to_vec(&mut buffer).unwrap();

    connection.write_all(buffer)
}

fn client_response<'a, T: AsyncWrite>(connection: T, challenge: Vec<u8>) -> WriteAll<'a, T> {
    let mut packet = ClientResponsePlaintext::new();
    packet
        .mut_login_crypto_response()
        .mut_diffie_hellman()
        .set_hmac(challenge);
    packet.mut_pow_response();
    packet.mut_crypto_response();

    let mut buffer = vec![];
    let size = 4 + packet.compute_size();
    buffer.write_u32::<BigEndian>(size).unwrap();
    packet.write_to_vec(&mut buffer).unwrap();

    connection.write_all(buffer)
}

enum RecvPacket<'a, T, M: Message> {
    Header(ReadExact<'a, T>, PhantomData<M>),
    Body(ReadExact<'a, T>, PhantomData<M>),
}

fn recv_packet<'a, T: AsyncRead, M>(connection: T, acc: Vec<u8>) -> RecvPacket<'a, T, M>
where
    T: Read,
    M: Message,
{
    RecvPacket::Header(read_into_accumulator(connection, 4, acc), PhantomData)
}

impl<'a, T: AsyncRead, M> Future for RecvPacket<'a, T, M>
where
    T: Read,
    M: Message,
{
    type Output = Result<(T, M, Vec<u8>), io::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        use self::RecvPacket::*;
        loop {
            *self = match *self {
                Header(ref mut read, _) => {
                    let (connection, header) = ready!(read.poll());
                    let size = BigEndian::read_u32(header.as_ref()) as usize;

                    let acc = header.into_inner();
                    let read = read_into_accumulator(connection, size - 4, acc);
                    RecvPacket::Body(read, PhantomData)
                }

                Body(ref mut read, _) => {
                    let (connection, data) = ready!(read.poll());
                    let message = protobuf::parse_from_bytes(data.as_ref()).unwrap();

                    let acc = data.into_inner();
                    return Poll::Ready(Ok((connection, message, acc)));
                }
            }
        }
    }
}

fn read_into_accumulator<'a, T: AsyncRead>(
    connection: T,
    size: usize,
    mut acc: Vec<u8>,
) -> ReadExact<'a, T> {
    let offset = acc.len();
    acc.resize(offset + size, 0);

    let mut window = Window::new(acc);
    window.set_start(offset);

    connection.read_exact(window)
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
