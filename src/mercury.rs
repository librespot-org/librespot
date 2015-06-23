use byteorder::{BigEndian, ByteOrder, ReadBytesExt, WriteBytesExt};
use protobuf::{self, Message};
use readall::ReadAllExt;
use std::collections::{HashMap, LinkedList};
use std::io::{Cursor, Read, Write};
use std::fmt;
use std::mem::replace;
use std::sync::mpsc;

use connection::Packet;
use librespot_protocol as protocol;
use subsystem::Subsystem;
use util::Either::{Left, Right};

pub enum MercuryMethod {
    GET,
    GETX,
    SUB,
    UNSUB,
}

pub struct MercuryRequest {
    pub method: MercuryMethod,
    pub url: String,
    pub mime: Option<String>,
    pub callback: MercuryCallback
}

#[derive(Debug)]
pub struct MercuryResponse {
    pub url: String,
    pub payload: LinkedList<Vec<u8>>
}

pub type MercuryCallback = Option<mpsc::Sender<MercuryResponse>>;

pub struct MercuryPending {
    parts: LinkedList<Vec<u8>>,
    partial: Option<Vec<u8>>,
    callback: MercuryCallback,
}

pub struct MercuryManager {
    next_seq: u32,
    pending: HashMap<Vec<u8>, MercuryPending>,

    requests: mpsc::Receiver<MercuryRequest>,
    packet_tx: mpsc::Sender<Packet>,
    packet_rx: mpsc::Receiver<Packet>,
}

impl fmt::Display for MercuryMethod {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        formatter.write_str(match *self {
            MercuryMethod::GET => "GET",
            MercuryMethod::GETX => "GETX",
            MercuryMethod::SUB => "SUB",
            MercuryMethod::UNSUB => "UNSUB"
        })
    }
}

impl MercuryManager {
    pub fn new(tx: mpsc::Sender<Packet>) -> (MercuryManager,
                                             mpsc::Sender<MercuryRequest>,
                                             mpsc::Sender<Packet>) {
        let (req_tx, req_rx) = mpsc::channel();
        let (pkt_tx, pkt_rx) = mpsc::channel();

        (MercuryManager {
            next_seq: 0,
            pending: HashMap::new(),

            requests: req_rx,
            packet_rx: pkt_rx,
            packet_tx: tx,
        }, req_tx, pkt_tx)
    }

    fn request(&mut self, req: MercuryRequest) {
        let mut seq = [0u8; 4];
        BigEndian::write_u32(&mut seq, self.next_seq);
        self.next_seq += 1;
        let data = self.encode_request(&seq, &req);

        self.packet_tx.send(Packet {
            cmd: 0xb2,
            data: data
        }).unwrap();

        self.pending.insert(seq.to_vec(), MercuryPending{
            parts: LinkedList::new(),
            partial: None,
            callback: req.callback,
        });
    }

    fn parse_part(mut s: &mut Read) -> Vec<u8> {
        let size = s.read_u16::<BigEndian>().unwrap() as usize;
        let mut buffer = vec![0; size];
        s.read_all(&mut buffer).unwrap();

        buffer
    }

    fn complete_request(&mut self, seq: &[u8]) {
        let mut pending = self.pending.remove(seq).unwrap();

        let header_data = match pending.parts.pop_front() {
            Some(data) => data,
            None => panic!("No header part !")
        };

        let header : protocol::mercury::MercuryReply =
            protobuf::parse_from_bytes(&header_data).unwrap();

        match pending.callback {
            Some(ch) => {
                ch.send(MercuryResponse{
                    url: header.get_url().to_string(),
                    payload: pending.parts
                }).unwrap();
            }
            None => (),
        }
    }

    fn handle_packet(&mut self, _cmd: u8, data: Vec<u8>) {
        let mut packet = Cursor::new(data);

        let seq = {
            let seq_length = packet.read_u16::<BigEndian>().unwrap() as usize;
            let mut seq = vec![0; seq_length];
            packet.read_all(&mut seq).unwrap();
            seq
        };
        let flags = packet.read_u8().unwrap();
        let count = packet.read_u16::<BigEndian>().unwrap() as usize;

        {
            let pending : &mut MercuryPending = match self.pending.get_mut(&seq) {
                Some(pending) => pending,
                None => { return; }
            };

            for i in 0..count {
                let mut part = Self::parse_part(&mut packet);
                if pending.partial.is_some() {
                    let mut data = replace(&mut pending.partial, None).unwrap();
                    data.append(&mut part);
                    part = data;
                }

                if i == count -1 && (flags == 2) {
                    pending.partial = Some(part)
                } else {
                    pending.parts.push_back(part);
                }
            }
        }

        if flags == 0x1 {
            self.complete_request(&seq);
        }
    }

    fn encode_request(&self, seq: &[u8], req: &MercuryRequest) -> Vec<u8> {
        let mut packet = Vec::new();
        packet.write_u16::<BigEndian>(seq.len() as u16).unwrap();
        packet.write_all(seq).unwrap();
        packet.write_u8(1).unwrap(); // Flags: FINAL
        packet.write_u16::<BigEndian>(1).unwrap(); // Part count. Only header

        let mut header = protobuf_init!(protocol::mercury::MercuryRequest::new(), {
            url: req.url.clone(),
            method: req.method.to_string(),
        });
        req.mime.clone().map(|mime| header.set_mime(mime));

        packet.write_u16::<BigEndian>(header.compute_size() as u16).unwrap();
        header.write_to_writer(&mut packet).unwrap();

        packet
    }
}

impl Subsystem for MercuryManager {
    fn run(mut self) {
        loop {
            match {
                let requests = &self.requests;
                let packets = &self.packet_rx;

                select!{
                    r = requests.recv() => {
                        Left(r.unwrap())
                    },
                    p = packets.recv() => {
                        Right(p.unwrap())
                    }
                }
            } {
                Left(req) => {
                    self.request(req);
                }
                Right(pkt) => {
                    self.handle_packet(pkt.cmd, pkt.data);
                }
            }

        }
    }
}

