use util;

use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian};
use std::io::{Write,Read};
use std::net::TcpStream;

pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub fn connect() -> Connection {
        Connection {
            stream: TcpStream::connect("lon3-accesspoint-a26.ap.spotify.com:4070").unwrap(),
        }
    }

    pub fn send_packet(&mut self, data: &[u8]) -> Vec<u8> {
        self.send_packet_prefix(&[], data)
    }

    pub fn send_packet_prefix(&mut self, prefix: &[u8], data: &[u8]) -> Vec<u8> {
        let size = prefix.len() + 4 + data.len();
        let mut buf = Vec::with_capacity(size);

        buf.write(prefix).unwrap();
        buf.write_u32::<BigEndian>(size as u32).unwrap();
        buf.write(data).unwrap();
        self.stream.write(&buf).unwrap();

        buf
    }

    pub fn recv_packet(&mut self) -> Vec<u8> {
        let size : usize = self.stream.read_u32::<BigEndian>().unwrap() as usize;
        let mut buffer = util::alloc_buffer(size - 4);

        self.stream.read(&mut buffer).unwrap();

        buffer
    }
}

