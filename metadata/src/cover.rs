use byteorder::{BigEndian, WriteBytesExt};
use std::io::Write;

use librespot_core::channel::ChannelData;
use librespot_core::session::Session;
use librespot_core::spotify_id::FileId;

pub fn get(session: &Session, file: FileId) -> ChannelData {
    let (channel_id, channel) = session.channel().allocate();
    let (_headers, data) = channel.split();

    let mut packet: Vec<u8> = Vec::new();
    packet.write_u16::<BigEndian>(channel_id).unwrap();
    packet.write_u16::<BigEndian>(0).unwrap();
    packet.write(&file.0).unwrap();
    session.send_packet(0x19, packet);

    data
}
