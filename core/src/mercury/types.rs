use byteorder::{BigEndian, WriteBytesExt};
use protobuf::Message;
use std::io::Write;

use crate::protocol;

#[derive(Debug, PartialEq, Eq)]
pub enum MercuryMethod {
    Get,
    Sub,
    Unsub,
    Send,
}

#[derive(Debug)]
pub struct MercuryRequest {
    pub method: MercuryMethod,
    pub uri: String,
    pub content_type: Option<String>,
    pub payload: Vec<Vec<u8>>,
}

#[derive(Debug, Clone)]
pub struct MercuryResponse {
    pub uri: String,
    pub status_code: i32,
    pub payload: Vec<Vec<u8>>,
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct MercuryError;

impl ToString for MercuryMethod {
    fn to_string(&self) -> String {
        match *self {
            MercuryMethod::Get => "GET",
            MercuryMethod::Sub => "SUB",
            MercuryMethod::Unsub => "UNSUB",
            MercuryMethod::Send => "SEND",
        }
        .to_owned()
    }
}

impl MercuryMethod {
    pub fn command(&self) -> u8 {
        match *self {
            MercuryMethod::Get | MercuryMethod::Send => 0xb2,
            MercuryMethod::Sub => 0xb3,
            MercuryMethod::Unsub => 0xb4,
        }
    }
}

impl MercuryRequest {
    pub fn encode(&self, seq: &[u8]) -> Vec<u8> {
        let mut packet = Vec::new();
        packet.write_u16::<BigEndian>(seq.len() as u16).unwrap();
        packet.write_all(seq).unwrap();
        packet.write_u8(1).unwrap(); // Flags: FINAL
        packet
            .write_u16::<BigEndian>(1 + self.payload.len() as u16)
            .unwrap(); // Part count

        let mut header = protocol::mercury::Header::new();
        header.set_uri(self.uri.clone());
        header.set_method(self.method.to_string());

        if let Some(ref content_type) = self.content_type {
            header.set_content_type(content_type.clone());
        }

        packet
            .write_u16::<BigEndian>(header.compute_size() as u16)
            .unwrap();
        header.write_to_writer(&mut packet).unwrap();

        for p in &self.payload {
            packet.write_u16::<BigEndian>(p.len() as u16).unwrap();
            packet.write(p).unwrap();
        }

        packet
    }
}
