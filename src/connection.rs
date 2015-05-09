use util;

use byteorder::{self, ReadBytesExt, WriteBytesExt, BigEndian, ByteOrder};
use keys::SharedKeys;
use readall::ReadAllExt;
use shannon::ShannonStream;
use std::convert;
use std::io;
use std::io::Write;
use std::net::TcpStream;
use std::result;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    Other
}

pub type Result<T> = result::Result<T, Error>;

impl convert::From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl convert::From<byteorder::Error> for Error {
    fn from(err: byteorder::Error) -> Error {
        match err {
            byteorder::Error::Io(e) => Error::IoError(e),
            _ => Error::Other
        }
    }
}

pub struct PlainConnection {
    stream: TcpStream
}

pub struct CipherConnection {
    stream: ShannonStream<TcpStream>,
}

impl PlainConnection {
    pub fn connect() -> Result<PlainConnection> {
        Ok(PlainConnection {
            stream: try!(TcpStream::connect("lon3-accesspoint-a26.ap.spotify.com:4070")),
        })
    }

    pub fn send_packet(&mut self, data: &[u8]) -> Result<Vec<u8>> {
        self.send_packet_prefix(&[], data)
    }

    pub fn send_packet_prefix(&mut self, prefix: &[u8], data: &[u8]) -> Result<Vec<u8>> {
        let size = prefix.len() + 4 + data.len();
        let mut buf = Vec::with_capacity(size);

        try!(buf.write(prefix));
        try!(buf.write_u32::<BigEndian>(size as u32));
        try!(buf.write(data));
        try!(self.stream.write(&buf));
        try!(self.stream.flush());

        Ok(buf)
    }

    pub fn recv_packet(&mut self) -> Result<Vec<u8>> {
        let size = try!(self.stream.read_u32::<BigEndian>()) as usize;
        let mut buffer = util::alloc_buffer(size);

        BigEndian::write_u32(&mut buffer, size as u32);
        try!(self.stream.read_all(&mut buffer[4..]));

        Ok(buffer)
    }

    pub fn setup_cipher(self, keys: SharedKeys) -> CipherConnection {
        CipherConnection{
            stream: ShannonStream::new(self.stream, &keys.send_key(), &keys.recv_key())
        }
    }
}

impl CipherConnection {
    pub fn send_encrypted_packet(&mut self, cmd: u8, data: &[u8]) -> Result<()> {
        try!(self.stream.write_u8(cmd)); try!(self.stream.write_u16::<BigEndian>(data.len() as u16));
        try!(self.stream.write(data));

        try!(self.stream.finish_send());
        try!(self.stream.flush());

        Ok(())
    }

    pub fn recv_packet(&mut self) -> Result<(u8, Vec<u8>)> {
        let cmd = try!(self.stream.read_u8());
        let size = try!(self.stream.read_u16::<BigEndian>()) as usize;

        let mut data = vec![0; size];
        try!(self.stream.read_all(&mut data));

        try!(self.stream.finish_recv());

        Ok((cmd, data))
    }
}



