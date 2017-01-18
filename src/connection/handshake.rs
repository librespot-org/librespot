use crypto::sha1::Sha1;
use crypto::hmac::Hmac;
use crypto::mac::Mac;use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use protobuf::{self, Message, MessageStatic};
use rand::thread_rng;
use std::io::{self, Read, Write};
use std::marker::PhantomData;
use tokio_core::io::{Io, Framed, write_all, WriteAll, read_exact, ReadExact, Window};
use futures::{Poll, Async, Future};

use diffie_hellman::DHLocalKeys;
use protocol;
use protocol::keyexchange::{ClientHello, APResponseMessage, ClientResponsePlaintext};
use util;
use super::codec::APCodec;

pub struct Handshake<T> {
    keys: DHLocalKeys,
    state: HandshakeState<T>,
}

enum HandshakeState<T> {
    ClientHello(WriteAll<T, Vec<u8>>),
    APResponse(RecvPacket<T, APResponseMessage>),
    ClientResponse(Option<APCodec>, WriteAll<T, Vec<u8>>),
}

pub fn handshake<T: Io>(connection: T) -> Handshake<T> {
    let local_keys = DHLocalKeys::random(&mut thread_rng());
    let client_hello = client_hello(connection, local_keys.public_key());

    Handshake {
        keys: local_keys,
        state: HandshakeState::ClientHello(client_hello),
    }
}

impl <T: Io> Future for Handshake<T> {
    type Item = Framed<T, APCodec>;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, io::Error> {
        use self::HandshakeState::*;
        loop {
            self.state = match self.state {
                ClientHello(ref mut write) => {
                    let (connection, accumulator) = try_ready!(write.poll());
                    
                    let read = recv_packet(connection, accumulator);
                    APResponse(read)
                }

                APResponse(ref mut read) => {
                    let (connection, message, accumulator) = try_ready!(read.poll());
                    let remote_key = message.get_challenge()
                        .get_login_crypto_challenge()
                        .get_diffie_hellman()
                        .get_gs()
                        .to_owned();

                    let shared_secret = self.keys.shared_secret(&remote_key);
                    let (challenge, send_key, recv_key) = compute_keys(&shared_secret,
                                                                       &accumulator);
                    let codec = APCodec::new(&send_key, &recv_key);

                    let write = client_response(connection, challenge);
                    ClientResponse(Some(codec), write)
                }

                ClientResponse(ref mut codec, ref mut write) => {
                    let (connection, _) = try_ready!(write.poll());
                    let codec = codec.take().unwrap();
                    let framed = connection.framed(codec);
                    return Ok(Async::Ready(framed));
                }
            }
        }
    }
}

fn client_hello<T: Write>(connection: T, gc: Vec<u8>) -> WriteAll<T, Vec<u8>> {
    let packet = protobuf_init!(ClientHello::new(), {
        build_info => {
            product: protocol::keyexchange::Product::PRODUCT_PARTNER,
            platform: protocol::keyexchange::Platform::PLATFORM_LINUX_X86,
            version: 0x10800000000,
        },
        cryptosuites_supported => [
            protocol::keyexchange::Cryptosuite::CRYPTO_SUITE_SHANNON,
        ],
        login_crypto_hello.diffie_hellman => {
            gc: gc,
            server_keys_known: 1,
        },
        client_nonce: util::rand_vec(&mut thread_rng(), 0x10),
        padding: vec![0x1e],
    });

    let mut buffer = vec![0, 4];
    let size = 2 + 4 + packet.compute_size();
    buffer.write_u32::<BigEndian>(size).unwrap();
    packet.write_to_vec(&mut buffer).unwrap();

    write_all(connection, buffer)
}

fn client_response<T: Write>(connection: T, challenge: Vec<u8>) -> WriteAll<T, Vec<u8>> {
    let packet = protobuf_init!(ClientResponsePlaintext::new(), {
        login_crypto_response.diffie_hellman => {
            hmac: challenge
        },
        pow_response => {},
        crypto_response => {},
    });

    let mut buffer = vec![];
    let size = 4 + packet.compute_size();
    buffer.write_u32::<BigEndian>(size).unwrap();
    packet.write_to_vec(&mut buffer).unwrap();

    write_all(connection, buffer)
}

enum RecvPacket<T, M: MessageStatic> {
    Header(ReadExact<T, Window<Vec<u8>>>, PhantomData<M>),
    Body(ReadExact<T, Window<Vec<u8>>>, PhantomData<M>),
}

fn recv_packet<T, M>(connection: T, acc: Vec<u8>) -> RecvPacket<T, M>
    where T: Read,
          M: MessageStatic
{
    RecvPacket::Header(read_into_accumulator(connection, 4, acc), PhantomData)
}

impl <T, M> Future for RecvPacket<T, M>
    where T: Read,
          M: MessageStatic
{
    type Item = (T, M, Vec<u8>);
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, io::Error> {
        use self::RecvPacket::*;
        loop {
            *self = match *self {
                Header(ref mut read, _) => {
                    let (connection, header) = try_ready!(read.poll());
                    let size = BigEndian::read_u32(header.as_ref()) as usize;

                    let acc = header.into_inner();
                    let read = read_into_accumulator(connection, size - 4, acc);
                    RecvPacket::Body(read, PhantomData)
                }

                Body(ref mut read, _) => {
                    let (connection, data) = try_ready!(read.poll());
                    let message = protobuf::parse_from_bytes(data.as_ref()).unwrap();

                    let acc = data.into_inner();
                    return Ok(Async::Ready((connection, message, acc)));
                }
            }
        }
    }
}

fn read_into_accumulator<T: Read>(connection: T, size: usize, mut acc: Vec<u8>) -> ReadExact<T, Window<Vec<u8>>> {
    let offset = acc.len();
    acc.resize(offset + size, 0);

    let mut window = Window::new(acc);
    window.set_start(offset);

    read_exact(connection, window)
}

fn compute_keys(shared_secret: &[u8], packets: &[u8]) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let mut data = Vec::with_capacity(0x64);
    let mut mac = Hmac::new(Sha1::new(), &shared_secret);

    for i in 1..6 {
        mac.input(packets);
        mac.input(&[i]);
        data.extend_from_slice(&mac.result().code());
        mac.reset();
    }

    mac = Hmac::new(Sha1::new(), &data[..0x14]);
    mac.input(packets);

    (mac.result().code().to_vec(), data[0x14..0x34].to_vec(), data[0x34..0x54].to_vec())
}
