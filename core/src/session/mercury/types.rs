use protobuf::Message;

use crate::protocol;
use crate::util::PacketBuilder;
use crate::util::Proto;
use crate::util::{IntoPacketData, PacketData};

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

#[derive(Copy, Clone)]
struct Payload<'a>(&'a [Vec<u8>]);

impl PacketData for Payload<'_> {
    #[inline]
    fn size_hint(&self) -> usize {
        self.0.iter().map(|x| 2 + x.len()).sum()
    }

    #[inline]
    fn write(self, vec: &mut Vec<u8>) {
        for p in self.0 {
            PacketBuilder::new()
                .append(p.len() as u16)
                .append(p.as_slice())
                .into_inner()
                .write(vec);
        }
    }
}

impl<'a> IntoPacketData<&'a [Vec<u8>]> for Payload<'a> {
    type Data = Self;

    fn convert(data: &'a [Vec<u8>]) -> Self {
        Self(data)
    }
}

impl MercuryRequest {
    pub fn encode(&self, seq: &[u8]) -> Vec<u8> {
        let mut header = protocol::mercury::Header::new();
        header.set_uri(self.uri.clone());
        header.set_method(self.method.to_string());

        if let Some(ref content_type) = self.content_type {
            header.set_content_type(content_type.clone());
        }

        crate::packet!(
            (u16) seq.len() as u16,
            ([u8]) seq,
            (u8) 1,
            (u16) 1 + self.payload.len() as u16,
            (u16) header.compute_size() as u16,
            (Proto) &header,
            (Payload) &self.payload
        )
    }
}
