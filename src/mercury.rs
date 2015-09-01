use byteorder::{BigEndian, ByteOrder, ReadBytesExt, WriteBytesExt};
use eventual;
use protobuf::{self, Message};
use std::collections::HashMap;
use std::io::{Cursor, Read, Write};
use std::mem::replace;
use std::sync::mpsc;

use librespot_protocol as protocol;
use session::Session;
use connection::PacketHandler;
use util::IgnoreExt;

#[derive(Debug, PartialEq, Eq)]
pub enum MercuryMethod {
    GET,
    SUB,
    UNSUB,
    SEND,
}

pub struct MercuryRequest {
    pub method: MercuryMethod,
    pub uri: String,
    pub content_type: Option<String>,
    pub payload: Vec<Vec<u8>>
}

#[derive(Debug)]
pub struct MercuryResponse {
    pub uri: String,
    pub payload: Vec<Vec<u8>>
}

pub struct MercuryPending {
    parts: Vec<Vec<u8>>,
    partial: Option<Vec<u8>>,
    callback: Option<eventual::Complete<MercuryResponse, ()>>
}

pub struct MercuryManager {
    next_seq: u32,
    pending: HashMap<Vec<u8>, MercuryPending>,
    subscriptions: HashMap<String, mpsc::Sender<MercuryResponse>>,
}

impl ToString for MercuryMethod {
    fn to_string(&self) -> String {
        match *self {
            MercuryMethod::GET => "GET",
            MercuryMethod::SUB => "SUB",
            MercuryMethod::UNSUB => "UNSUB",
            MercuryMethod::SEND => "SEND"
        }.to_owned()
    }
}

impl MercuryManager {
    pub fn new() -> MercuryManager {
        MercuryManager {
            next_seq: 0,
            pending: HashMap::new(),
            subscriptions: HashMap::new(),
        }
    }

    pub fn request(&mut self, session: &Session, req: MercuryRequest)
        -> eventual::Future<MercuryResponse, ()> {

        let mut seq = [0u8; 4];
        BigEndian::write_u32(&mut seq, self.next_seq);
        self.next_seq += 1;
        let data = self.encode_request(&seq, &req);

        let cmd = match req.method {
            MercuryMethod::SUB => 0xb3,
            MercuryMethod::UNSUB => 0xb4,
            _ => 0xb2,
        };

        session.send_packet(cmd, &data).unwrap();

        let (tx, rx) = eventual::Future::pair();
        self.pending.insert(seq.to_vec(), MercuryPending{
            parts: Vec::new(),
            partial: None,
            callback: Some(tx),
        });

        rx
    }

    pub fn subscribe(&mut self, session: &Session, uri: String)
        -> mpsc::Receiver<MercuryResponse> {
        let (tx, rx) = mpsc::channel();
        self.subscriptions.insert(uri.clone(), tx);

        self.request(session, MercuryRequest{
            method: MercuryMethod::SUB,
            uri: uri,
            content_type: None,
            payload: Vec::new()
        });

        rx
    }

    fn parse_part(mut s: &mut Read) -> Vec<u8> {
        let size = s.read_u16::<BigEndian>().unwrap() as usize;
        let mut buffer = vec![0; size];
        s.read_exact(&mut buffer).unwrap();

        buffer
    }

    fn complete_request(&mut self, cmd: u8, mut pending: MercuryPending) {
        let header_data = pending.parts.remove(0);
        let header : protocol::mercury::Header =
            protobuf::parse_from_bytes(&header_data).unwrap();

        let response = MercuryResponse {
            uri: header.get_uri().to_owned(),
            payload: pending.parts
        };

        if cmd == 0xb5 {
            self.subscriptions.get(header.get_uri()).map(|ch| ch.send(response).ignore());
        } else {
            pending.callback.map(|cb| cb.complete(response));
        }
    }

    fn encode_request(&self, seq: &[u8], req: &MercuryRequest) -> Vec<u8> {
        let mut packet = Vec::new();
        packet.write_u16::<BigEndian>(seq.len() as u16).unwrap();
        packet.write_all(seq).unwrap();
        packet.write_u8(1).unwrap(); // Flags: FINAL
        packet.write_u16::<BigEndian>(1 + req.payload.len() as u16).unwrap(); // Part count

        let mut header = protobuf_init!(protocol::mercury::Header::new(), {
            uri: req.uri.clone(),
            method: req.method.to_string(),
        });
        if let Some(ref content_type) = req.content_type {
            header.set_content_type(content_type.clone());
        }

        packet.write_u16::<BigEndian>(header.compute_size() as u16).unwrap();
        header.write_to_writer(&mut packet).unwrap();

        for p in &req.payload {
            packet.write_u16::<BigEndian>(p.len() as u16).unwrap();
            packet.write(&p).unwrap();
        }

        packet
    }
}

impl PacketHandler for MercuryManager {
    fn handle(&mut self, cmd: u8, data: Vec<u8>) {
        let mut packet = Cursor::new(data);

        let seq = {
            let seq_length = packet.read_u16::<BigEndian>().unwrap() as usize;
            let mut seq = vec![0; seq_length];
            packet.read_exact(&mut seq).unwrap();
            seq
        };
        let flags = packet.read_u8().unwrap();
        let count = packet.read_u16::<BigEndian>().unwrap() as usize;

        let mut pending = if let Some(pending) = self.pending.remove(&seq) {
            pending
        } else if cmd == 0xb5 {
            MercuryPending {
                parts: Vec::new(),
                partial: None,
                callback: None,
            }
        } else {
            println!("Ignore seq {:?} cmd {}", seq, cmd);
            return
        };

        for i in 0..count {
            let mut part = Self::parse_part(&mut packet);
            if let Some(mut data) = replace(&mut pending.partial, None) {
                data.append(&mut part);
                part = data;
            }

            if i == count - 1 && (flags == 2) {
                pending.partial = Some(part)
            } else {
                pending.parts.push(part);
            }
        }

        if flags == 0x1 {
            self.complete_request(cmd, pending);
        } else {
            self.pending.insert(seq, pending);
        }
    }
}

