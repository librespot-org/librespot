use session::Session;
use stream;
use util::FileId;

use byteorder::{BigEndian, WriteBytesExt};
use std::io::Write;

const CHUNK_SIZE: usize = 0x20000;

pub enum Response<H> {
//    Wait(H),
    Continue(H),
    Seek(H, usize),
    Close,
}

pub trait Handler : Sized + Send + 'static {
    fn on_header(self, header_id: u8, header_data: &[u8], session: &Session) -> Response<Self>;
    fn on_data(self, offset: usize, data: &[u8], session: &Session) -> Response<Self>;
    fn on_eof(self, session: &Session) -> Response<Self>;
    fn on_error(self, session: &Session);
}

pub struct AudioFile<H: Handler> {
    handler: H,
    file_id: FileId,
    offset: usize,
}

impl <H: Handler> AudioFile<H> {
    pub fn new(file_id: FileId, offset: usize, handler: H, session: &Session) {
        let handler = AudioFile {
            handler: handler,
            file_id: file_id,
            offset: offset,
        };

        session.stream(Box::new(handler));
    }
}

impl <H: Handler> stream::Handler for AudioFile<H> {
    fn on_create(self, channel_id: stream::ChannelId, session: &Session) -> stream::Response<Self> {
        debug!("Got channel {}", channel_id);

        let mut data: Vec<u8> = Vec::new();
        data.write_u16::<BigEndian>(channel_id).unwrap();
        data.write_u8(0).unwrap();
        data.write_u8(1).unwrap();
        data.write_u16::<BigEndian>(0x0000).unwrap();
        data.write_u32::<BigEndian>(0x00000000).unwrap();
        data.write_u32::<BigEndian>(0x00009C40).unwrap();
        data.write_u32::<BigEndian>(0x00020000).unwrap();
        data.write(&self.file_id.0).unwrap();
        data.write_u32::<BigEndian>(self.offset as u32 / 4).unwrap();
        data.write_u32::<BigEndian>((self.offset + CHUNK_SIZE) as u32 / 4).unwrap();

        session.send_packet(0x8, &data).unwrap();

        stream::Response::Continue(self)
    }

    fn on_header(mut self, header_id: u8, header_data: &[u8], session: &Session) -> stream::Response<Self> {
        match self.handler.on_header(header_id, header_data, session) {
            Response::Continue(handler) => {
                self.handler = handler;
                stream::Response::Continue(self)
            }
            Response::Seek(handler, offset) => {
                self.handler = handler;
                self.offset = offset;
                stream::Response::Spawn(self)
            }
            Response::Close => stream::Response::Close,
        }
    }

    fn on_data(mut self, data: &[u8], session: &Session) -> stream::Response<Self> {
        match self.handler.on_data(self.offset, data, session) {
            Response::Continue(handler) => {
                self.handler = handler;
                self.offset += data.len();
                stream::Response::Continue(self)
            }
            Response::Seek(handler, offset) => {
                self.handler = handler;
                self.offset = offset;
                stream::Response::Spawn(self)
            }
            Response::Close => stream::Response::Close,
        }
    }

    fn on_close(self, _session: &Session) -> stream::Response<Self> {
        // End of chunk, request a new one
        stream::Response::Spawn(self)
    }

    fn on_error(mut self, session: &Session) -> stream::Response<Self> {
        match self.handler.on_eof(session) {
            Response::Continue(_) => stream::Response::Close,
            Response::Seek(handler, offset) => {
                self.handler = handler;
                self.offset = offset;
                stream::Response::Spawn(self)
            }
            Response::Close => stream::Response::Close,
        }
    }

    fn box_on_create(self: Box<Self>, channel_id: stream::ChannelId, session: &Session) -> stream::Response<Box<stream::Handler>> {
        self.on_create(channel_id, session).boxed()
    }

    fn box_on_header(self: Box<Self>, header_id: u8, header_data: &[u8], session: &Session) -> stream::Response<Box<stream::Handler>> {
        self.on_header(header_id, header_data, session).boxed()
    }

    fn box_on_data(self: Box<Self>, data: &[u8], session: &Session) -> stream::Response<Box<stream::Handler>> {
        self.on_data(data, session).boxed()
    }

    fn box_on_error(self: Box<Self>, session: &Session) -> stream::Response<Box<stream::Handler>> {
        self.on_error(session).boxed()
    }

    fn box_on_close(self: Box<Self>, session: &Session) -> stream::Response<Box<stream::Handler>> {
        self.on_close(session).boxed()
    }
}

