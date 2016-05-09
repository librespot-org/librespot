use eventual;
use std::io::Write;
use byteorder::{WriteBytesExt, BigEndian};

use session::Session;
use util::FileId;
use stream;

pub struct AlbumCover {
    file_id: FileId,
    data: Vec<u8>,
    cover_tx: eventual::Complete<Vec<u8>, ()>,
}

impl stream::Handler for AlbumCover {
    fn on_create(self, channel_id: stream::ChannelId, session: &Session) -> stream::Response<Self> {
        let mut req: Vec<u8> = Vec::new();
        req.write_u16::<BigEndian>(channel_id).unwrap();
        req.write_u16::<BigEndian>(0).unwrap();
        req.write(&self.file_id.0).unwrap();
        session.send_packet(0x19, &req).unwrap();

        stream::Response::Continue(self)
    }

    fn on_header(self, _header_id: u8, _header_data: &[u8], _session: &Session) -> stream::Response<Self> {
        stream::Response::Continue(self)
    }

    fn on_data(mut self, data: &[u8], _session: &Session) -> stream::Response<Self> {
        self.data.extend_from_slice(data);
        stream::Response::Continue(self)
    }

    fn on_close(self, _session: &Session) -> stream::Response<Self> {
        // End of chunk, request a new one
        self.cover_tx.complete(self.data);
        stream::Response::Close
    }

    fn on_error(self, _session: &Session) -> stream::Response<Self> {
        self.cover_tx.fail(());
        stream::Response::Close
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

impl AlbumCover {
    pub fn get(file_id: FileId, session: &Session) -> eventual::Future<Vec<u8>, ()> {
        let (tx, rx) = eventual::Future::pair();
        session.stream(Box::new(AlbumCover {
            file_id: file_id,
            data: Vec::new(),
            cover_tx: tx,
        }));

        rx
    }
}
