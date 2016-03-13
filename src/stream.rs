use byteorder::{BigEndian, ByteOrder, ReadBytesExt, WriteBytesExt};
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::io::{Cursor, Seek, SeekFrom, Write};
use eventual::{self, Async};

use util::{ArcVec, FileId};
use connection::PacketHandler;
use session::Session;

#[derive(Debug)]
pub enum StreamEvent {
    Header(u8, ArcVec<u8>),
    Data(ArcVec<u8>),
}

#[derive(Debug,Hash,PartialEq,Eq,Copy,Clone)]
pub struct StreamError;

pub type ChannelId = u16;

enum ChannelMode {
    Header,
    Data,
}

struct Channel {
    mode: ChannelMode,
    callback: Option<eventual::Sender<StreamEvent, StreamError>>,
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

    pub fn allocate_stream(&mut self) -> (ChannelId, eventual::Stream<StreamEvent, StreamError>) {
        let (tx, rx) = eventual::Stream::pair();

        let channel_id = self.next_id;
        self.next_id += 1;

        self.channels.insert(channel_id,
                             Channel {
                                 mode: ChannelMode::Header,
                                 callback: Some(tx),
                             });

        (channel_id, rx)
    }

    pub fn request(&mut self,
                   session: &Session,
                   file: FileId,
                   offset: u32,
                   size: u32)
                   -> eventual::Stream<StreamEvent, StreamError> {

        let (channel_id, rx) = self.allocate_stream();

        let mut data: Vec<u8> = Vec::new();
        data.write_u16::<BigEndian>(channel_id).unwrap();
        data.write_u8(0).unwrap();
        data.write_u8(1).unwrap();
        data.write_u16::<BigEndian>(0x0000).unwrap();
        data.write_u32::<BigEndian>(0x00000000).unwrap();
        data.write_u32::<BigEndian>(0x00009C40).unwrap();
        data.write_u32::<BigEndian>(0x00020000).unwrap();
        data.write(&file.0).unwrap();
        data.write_u32::<BigEndian>(offset).unwrap();
        data.write_u32::<BigEndian>(offset + size).unwrap();

        session.send_packet(0x8, &data).unwrap();

        rx
    }
}

impl Channel {
    fn handle_packet(&mut self, cmd: u8, data: Vec<u8>) {
        let data = ArcVec::new(data);
        let mut packet = Cursor::new(&data as &[u8]);
        packet.read_u16::<BigEndian>().unwrap(); // Skip channel id

        if cmd == 0xa {
            self.callback.take().map(|c| c.fail(StreamError));
        } else {
            match self.mode {
                ChannelMode::Header => {
                    let mut length = 0;

                    while packet.position() < data.len() as u64 {
                        length = packet.read_u16::<BigEndian>().unwrap();
                        if length > 0 {
                            let header_id = packet.read_u8().unwrap();
                            let header_data = data.clone()
                                                  .offset(packet.position() as usize)
                                                  .limit(length as usize - 1);

                            let header = StreamEvent::Header(header_id, header_data);

                            self.callback = self.callback.take().and_then(|c| c.send(header).await().ok());

                            packet.seek(SeekFrom::Current(length as i64 - 1)).unwrap();
                        }
                    }

                    if length == 0 {
                        self.mode = ChannelMode::Data;
                    }
                }

                ChannelMode::Data => {
                    if packet.position() < data.len() as u64 {
                        let event_data = data.clone().offset(packet.position() as usize);
                        let event = StreamEvent::Data(event_data);

                        self.callback = self.callback.take().and_then(|c| c.send(event).await().ok());
                    } else {
                        self.callback = None;
                    }
                }
            }
        }
    }
}

impl PacketHandler for StreamManager {
    fn handle(&mut self, cmd: u8, data: Vec<u8>) {
        let id: ChannelId = BigEndian::read_u16(&data[0..2]);

        if let Entry::Occupied(mut entry) = self.channels.entry(id) {
            entry.get_mut().handle_packet(cmd, data);

            if entry.get().callback.is_none() {
                entry.remove();
            }
        }
    }
}
