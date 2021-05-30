use byteorder::{BigEndian, ByteOrder};
use bytes::{BufMut, Bytes, BytesMut};
use shannon::Shannon;
use std::io;
use tokio_util::codec::{Decoder, Encoder, FramedRead, FramedWrite};

const HEADER_SIZE: usize = 3;
const MAC_SIZE: usize = 4;

pub struct ApEncoder {
    encode_nonce: u32,
    encode_cipher: Shannon,
}

impl ApEncoder {
    pub fn new(send_key: &[u8]) -> Self {
        Self {
            encode_nonce: 0,
            encode_cipher: Shannon::new(send_key),
        }
    }
}

impl Encoder<(u8, Vec<u8>)> for ApEncoder {
    type Error = io::Error;

    fn encode(&mut self, item: (u8, Vec<u8>), buf: &mut BytesMut) -> io::Result<()> {
        let (cmd, payload) = item;
        let offset = buf.len();

        buf.reserve(3 + payload.len());
        buf.put_u8(cmd);
        buf.put_u16(payload.len() as u16);
        buf.extend_from_slice(&payload);

        self.encode_cipher.nonce_u32(self.encode_nonce);
        self.encode_nonce += 1;

        self.encode_cipher.encrypt(&mut buf[offset..]);

        let mut mac = [0u8; MAC_SIZE];
        self.encode_cipher.finish(&mut mac);
        buf.extend_from_slice(&mac);

        Ok(())
    }
}

#[derive(Debug)]
enum DecodeState {
    Header,
    Payload(u8, usize),
}

pub struct ApDecoder {
    decode_nonce: u32,
    decode_cipher: Shannon,
    decode_state: DecodeState,
}

pub type Decoded<R> = FramedRead<R, ApDecoder>;

impl ApDecoder {
    pub fn new(recv_key: &[u8]) -> Self {
        Self {
            decode_nonce: 0,
            decode_cipher: Shannon::new(recv_key),
            decode_state: DecodeState::Header,
        }
    }
}

impl Decoder for ApDecoder {
    type Item = (u8, Bytes);
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<(u8, Bytes)>> {
        if let DecodeState::Header = self.decode_state {
            if buf.len() >= HEADER_SIZE {
                let mut header = [0u8; HEADER_SIZE];
                header.copy_from_slice(buf.split_to(HEADER_SIZE).as_ref());

                self.decode_cipher.nonce_u32(self.decode_nonce);
                self.decode_nonce += 1;

                self.decode_cipher.decrypt(&mut header);

                let cmd = header[0];
                let size = BigEndian::read_u16(&header[1..]) as usize;
                self.decode_state = DecodeState::Payload(cmd, size);
            }
        }

        if let DecodeState::Payload(cmd, size) = self.decode_state {
            if buf.len() >= size + MAC_SIZE {
                self.decode_state = DecodeState::Header;

                let mut payload = buf.split_to(size + MAC_SIZE);

                self.decode_cipher
                    .decrypt(&mut payload.get_mut(..size).unwrap());
                let mac = payload.split_off(size);
                self.decode_cipher.check_mac(mac.as_ref())?;

                return Ok(Some((cmd, payload.freeze())));
            }
        }

        Ok(None)
    }
}

pub type Encoded<W> = FramedWrite<W, ApEncoder>;
