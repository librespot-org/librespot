use byteorder::{BigEndian, ByteOrder, ReadBytesExt, WriteBytesExt};
use std::collections::HashMap;
use std::io::{Cursor, Seek, SeekFrom, Write};
use std::sync::mpsc;

use connection::Packet;
use util::{ArcVec, FileId};
use util::Either::{Left, Right};
use subsystem::Subsystem;

pub type StreamCallback = mpsc::Sender<StreamEvent>;
pub struct StreamRequest {
    pub id: FileId,
    pub offset: u32,
    pub size: u32,
    pub callback: StreamCallback
}

#[derive(Debug)]
pub enum StreamEvent {
    Header(u8, ArcVec<u8>),
    Data(ArcVec<u8>),
}

type ChannelId = u16;

enum ChannelMode {
    Header,
    Data
}

struct Channel {
    mode: ChannelMode,
    callback: StreamCallback
}

pub struct StreamManager {
    next_id: ChannelId,
    channels: HashMap<ChannelId, Channel>,

    requests: mpsc::Receiver<StreamRequest>,
    packet_rx: mpsc::Receiver<Packet>,
    packet_tx: mpsc::Sender<Packet>,
}

impl StreamManager {
    pub fn new(tx: mpsc::Sender<Packet>) -> (StreamManager,
                                             mpsc::Sender<StreamRequest>,
                                             mpsc::Sender<Packet>) {
        let (req_tx, req_rx) = mpsc::channel();
        let (pkt_tx, pkt_rx) = mpsc::channel();

        (StreamManager {
            next_id: 0,
            channels: HashMap::new(),

            requests: req_rx,
            packet_rx: pkt_rx,
            packet_tx: tx
        }, req_tx, pkt_tx)
    }

    fn request(&mut self, req: StreamRequest) {
        let channel_id = self.next_id;
        self.next_id += 1;

        let mut data : Vec<u8> = Vec::new();
        data.write_u16::<BigEndian>(channel_id).unwrap();
        data.write_u8(0).unwrap();
        data.write_u8(1).unwrap();
        data.write_u16::<BigEndian>(0x0000).unwrap();
        data.write_u32::<BigEndian>(0x00000000).unwrap();
        data.write_u32::<BigEndian>(0x00009C40).unwrap();
        data.write_u32::<BigEndian>(0x00020000).unwrap();
        data.write(&req.id).unwrap();
        data.write_u32::<BigEndian>(req.offset).unwrap();
        data.write_u32::<BigEndian>(req.offset + req.size).unwrap();

        self.packet_tx.send(Packet {
            cmd: 0x8,
            data: data
        }).unwrap();

        self.channels.insert(channel_id, Channel {
            mode: ChannelMode::Header,
            callback: req.callback
        });
    }

    fn packet(&mut self, data: Vec<u8>) {
        let data = ArcVec::new(data);
        let mut packet = Cursor::new(&data as &[u8]);

        let id : ChannelId = packet.read_u16::<BigEndian>().unwrap();
        let channel = match self.channels.get_mut(&id) {
            Some(ch) => ch,
            None => { return; }
        };

        match channel.mode {
            ChannelMode::Header => {
                let mut length = 0;

                while packet.position() < data.len() as u64 {
                    length = packet.read_u16::<BigEndian>().unwrap();
                    if length > 0 {
                        let header_id = packet.read_u8().unwrap();
                        channel.callback.send(StreamEvent::Header(
                                header_id,
                                data.clone()
                                    .offset(packet.position() as usize)
                                    .limit(length as usize - 1)
                            )).unwrap();

                        packet.seek(SeekFrom::Current(length as i64 - 1)).unwrap();
                    }
                }
                
                if length == 0 {
                    channel.mode = ChannelMode::Data;
                }
            }

            ChannelMode::Data => {
                if packet.position() < data.len() as u64 {
                    channel.callback.send(StreamEvent::Data(
                            data.clone().offset(packet.position() as usize))).unwrap();
                } else {
                    // TODO: close the channel
                }
            }
        }
    }
}

impl Subsystem for StreamManager {
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
                Left(req) => self.request(req),
                Right(pkt) => self.packet(pkt.data)
            }
        }
    }
}


