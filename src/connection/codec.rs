use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use shannon::Shannon;
use std::io;
use tokio_core::io::{Codec, EasyBuf};

const HEADER_SIZE: usize = 3;
const MAC_SIZE: usize = 4;

#[derive(Debug)]
enum DecodeState {
    Header,
    Payload(u8, usize),
}

pub struct APCodec {
    encode_nonce: u32,
    encode_cipher: Shannon,

    decode_nonce: u32,
    decode_cipher: Shannon,
    decode_state: DecodeState,
}

impl APCodec {
    pub fn new(send_key: &[u8], recv_key: &[u8]) -> APCodec {
        APCodec {
            encode_nonce: 0,
            encode_cipher: Shannon::new(send_key),

            decode_nonce: 0,
            decode_cipher: Shannon::new(recv_key),
            decode_state: DecodeState::Header,
        }
    }
}

impl Codec for APCodec {
    type Out = (u8, Vec<u8>);
    type In = (u8, EasyBuf);

    fn encode(&mut self, item: (u8, Vec<u8>), buf: &mut Vec<u8>) -> io::Result<()> {
        let (cmd, payload) = item;
        let offset = buf.len();

        buf.write_u8(cmd).unwrap();
        buf.write_u16::<BigEndian>(payload.len() as u16).unwrap();
        buf.extend_from_slice(&payload);

        self.encode_cipher.nonce_u32(self.encode_nonce);
        self.encode_nonce += 1;

        self.encode_cipher.encrypt(&mut buf[offset..]);

        let mut mac = [0u8; MAC_SIZE];
        self.encode_cipher.finish(&mut mac);
        buf.extend_from_slice(&mac);

        Ok(())
    }

    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<(u8, EasyBuf)>> {
        if let DecodeState::Header = self.decode_state {
            if buf.len() >= HEADER_SIZE {
                let mut header = [0u8; HEADER_SIZE];
                header.copy_from_slice(buf.drain_to(HEADER_SIZE).as_slice());

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

                let mut payload = buf.drain_to(size + MAC_SIZE);

                self.decode_cipher.decrypt(&mut payload.get_mut()[..size]);
                let mac = payload.split_off(size);
                self.decode_cipher.check_mac(mac.as_slice())?;

                return Ok(Some((cmd, payload)));
            }
        }


        Ok(None)
    }
}
