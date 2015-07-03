use byteorder::{BigEndian, ByteOrder, ReadBytesExt, WriteBytesExt};
use std::collections::HashMap;
use std::io::{Cursor, Seek, SeekFrom, Write};
use std::sync::mpsc;

use util::{ArcVec, FileId};
use connection::PacketHandler;
use session::Session;

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
    callback: mpsc::Sender<StreamEvent>
}

pub struct StreamManager {
    next_id: ChannelId,
    channels: HashMap<ChannelId, Channel>,
}

impl StreamManager {
    pub fn new() -> StreamManager {
        StreamManager {
            next_id: 0,
            channels: HashMap::new(),
        }
    }

    pub fn request(&mut self, session: &Session,
                   file: FileId, offset: u32, size: u32)
        -> mpsc::Receiver<StreamEvent> {

        let (tx, rx) = mpsc::channel();

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
        data.write(&file).unwrap();
        data.write_u32::<BigEndian>(offset).unwrap();
        data.write_u32::<BigEndian>(offset + size).unwrap();

        session.send_packet(0x8, &data).unwrap();

        self.channels.insert(channel_id, Channel {
            mode: ChannelMode::Header,
            callback: tx
        });

        rx
    }
}

impl PacketHandler for StreamManager {
    fn handle(&mut self, _cmd: u8, data: Vec<u8>) {
        let data = ArcVec::new(data);
        let mut packet = Cursor::new(&data as &[u8]);

        let id : ChannelId = packet.read_u16::<BigEndian>().unwrap();
        let mut close = false;
        {
            let channel = match self.channels.get_mut(&id) {
                Some(ch) => ch,
                None => { return; }
            };

            match channel.mode {
                ChannelMode::Header => {
                    let mut length = 0;

                    while packet.position() < data.len() as u64 && !close {
                        length = packet.read_u16::<BigEndian>().unwrap();
                        if length > 0 {
                            let header_id = packet.read_u8().unwrap();
                            channel.callback
                                .send(StreamEvent::Header(
                                        header_id,
                                        data.clone()
                                            .offset(packet.position() as usize)
                                            .limit(length as usize - 1)
                                            ))
                                .unwrap_or_else(|_| {
                                    close = true;
                                });

                            packet.seek(SeekFrom::Current(length as i64 - 1)).unwrap();
                        }
                    }
                    
                    if length == 0 {
                        channel.mode = ChannelMode::Data;
                    }
                }

                ChannelMode::Data => {
                    if packet.position() < data.len() as u64 {
                        channel.callback
                            .send(StreamEvent::Data(data.clone().offset(packet.position() as usize)))
                            .unwrap_or_else(|_| {
                                close = true;
                            });
                    } else {
                        close = true;
                    }
                }
            }
        }

        if close {
            self.channels.remove(&id);
        }
    }
}

