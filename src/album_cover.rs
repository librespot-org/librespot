use eventual;
use std::io::Write;
use byteorder::{WriteBytesExt, BigEndian};

use session::Session;
use util::FileId;
use stream::StreamEvent;

pub fn get_album_cover(session: &Session, file_id: FileId)
    -> eventual::Future<Vec<u8>, ()> {

    let (channel_id, rx) = session.allocate_stream();

    let mut req: Vec<u8> = Vec::new();
    req.write_u16::<BigEndian>(channel_id).unwrap();
    req.write_u16::<BigEndian>(0).unwrap();
    req.write(&file_id.0).unwrap();
    session.send_packet(0x19, &req).unwrap();

    rx.map_err(|_| ())
      .reduce(Vec::new(), |mut current, event| {
        if let StreamEvent::Data(data) = event {
            current.extend_from_slice(&data)
        }
        current
    })
}
