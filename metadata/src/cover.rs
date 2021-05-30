use librespot_core::packet;

use librespot_core::channel::ChannelData;
use librespot_core::session::Session;
use librespot_core::spotify_id::FileId;

pub fn get(session: &Session, file: FileId) -> ChannelData {
    let (channel_id, channel) = session.channel().allocate();
    let (_headers, data) = channel.split();

    let packet = packet!(
        (u16) channel_id,
        (u16) 0,
        ([u8; 20]) &file.0
    );

    session.send_packet(0x19, packet);

    data
}
