use byteorder::{BigEndian, ByteOrder, ReadBytesExt};
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::io::{Cursor, Seek, SeekFrom};
use session::{Session, PacketHandler};

pub enum Response<H, S = H> {
    Continue(H),
    Spawn(S),
    Close,
}

impl <H: Handler + 'static> Response<H> {
    pub fn boxed(self) -> Response<Box<Handler>> {
        match self {
            Response::Continue(handler) => Response::Continue(Box::new(handler)),
            Response::Spawn(handler) => Response::Spawn(Box::new(handler)),
            Response::Close => Response::Close,
        }
    }
}

pub trait Handler: Send {
    fn on_create(self, channel_id: ChannelId, session: &Session) -> Response<Self> where Self: Sized;
    fn on_header(self, header_id: u8, header_data: &[u8], session: &Session) -> Response<Self> where Self: Sized;
    fn on_data(self, data: &[u8], session: &Session) -> Response<Self> where Self: Sized;
    fn on_error(self, session: &Session) -> Response<Self> where Self: Sized;
    fn on_close(self, session: &Session) -> Response<Self> where Self: Sized;

    fn box_on_create(self: Box<Self>, channel_id: ChannelId, session: &Session) -> Response<Box<Handler>>;
    fn box_on_header(self: Box<Self>, header_id: u8, header_data: &[u8], session: &Session) -> Response<Box<Handler>>;
    fn box_on_data(self: Box<Self>, data: &[u8], session: &Session) -> Response<Box<Handler>>;
    fn box_on_error(self: Box<Self>, session: &Session) -> Response<Box<Handler>>;
    fn box_on_close(self: Box<Self>, session: &Session) -> Response<Box<Handler>>;
}

pub type ChannelId = u16;

enum ChannelMode {
    Header,
    Data
}

struct Channel(ChannelMode, Box<Handler>);

impl Channel {
    fn handle_packet(self, cmd: u8, data: Vec<u8>, session: &Session) -> Response<Self, Box<Handler>> {
        let Channel(mode, mut handler) = self;

        let mut packet = Cursor::new(&data as &[u8]);
        packet.read_u16::<BigEndian>().unwrap(); // Skip channel id

        if cmd == 0xa {
            println!("error: {} {}", data.len(), packet.read_u16::<BigEndian>().unwrap());
            return match handler.box_on_error(session) {
                Response::Continue(_) => Response::Close,
                Response::Spawn(f) => Response::Spawn(f),
                Response::Close => Response::Close,
            };
        }

        match mode {
            ChannelMode::Header => {
                let mut length = 0;

                while packet.position() < data.len() as u64 {
                    length = packet.read_u16::<BigEndian>().unwrap();
                    if length > 0 {
                        let header_id = packet.read_u8().unwrap();
                        let header_data = &data[packet.position() as usize .. packet.position() as usize + length as usize - 1];

                        handler = match handler.box_on_header(header_id, header_data, session) {
                            Response::Continue(handler) => handler,
                            Response::Spawn(f) => return Response::Spawn(f),
                            Response::Close => return Response::Close,
                        };

                        packet.seek(SeekFrom::Current(length as i64 - 1)).unwrap();
                    }
                }

                if length == 0 {
                    Response::Continue(Channel(ChannelMode::Data, handler))
                } else {
                    Response::Continue(Channel(ChannelMode::Header, handler))
                }
            }
            ChannelMode::Data => {
                if packet.position() < data.len() as u64 {
                    let event_data = &data[packet.position() as usize..];
                    match handler.box_on_data(event_data, session) {
                        Response::Continue(handler) => Response::Continue(Channel(ChannelMode::Data, handler)),
                        Response::Spawn(f) => Response::Spawn(f),
                        Response::Close => Response::Close,
                    }
                } else {
                    match handler.box_on_close(session) {
                        Response::Continue(_) => Response::Close,
                        Response::Spawn(f) => Response::Spawn(f),
                        Response::Close => Response::Close,
                    }
                }
            }
        }
    }
}

pub struct StreamManager {
    next_id: ChannelId,
    channels: HashMap<ChannelId, Option<Channel>>,
}

impl StreamManager {
    pub fn new() -> StreamManager {
        StreamManager {
            next_id: 0,
            channels: HashMap::new(),
        }
    }

    pub fn create(&mut self, handler: Box<Handler>, session: &Session) {
        let channel_id = self.next_id;
        self.next_id += 1;

        trace!("allocated stream {}", channel_id);

        match handler.box_on_create(channel_id, session) {
            Response::Continue(handler) => {
                self.channels.insert(channel_id,  Some(Channel(ChannelMode::Header, handler)));
            }
            Response::Spawn(handler) => self.create(handler, session),
            Response::Close => (),
        }
    }
}

impl PacketHandler for StreamManager {
    fn handle(&mut self, cmd: u8, data: Vec<u8>, session: &Session) {
        let id: ChannelId = BigEndian::read_u16(&data[0..2]);

        let spawn = if let Entry::Occupied(mut entry) = self.channels.entry(id) {
            if let Some(channel) = entry.get_mut().take() {
                match channel.handle_packet(cmd, data, session) {
                    Response::Continue(channel) => {
                        entry.insert(Some(channel));
                        None
                    }
                    Response::Spawn(f) => {
                        entry.remove();
                        Some(f)
                    }
                    Response::Close => {
                        entry.remove();
                        None
                    }
                }
            } else {
                None
            }
        } else {
            None
        };


        if let Some(s) = spawn {
            self.create(s, session);
        }
    }
}
