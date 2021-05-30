use byteorder::{BigEndian, ByteOrder};
use bytes::Bytes;
use std::collections::HashMap;
use tokio::sync::oneshot;

use crate::packet;
use crate::spotify_id::{FileId, SpotifyId};
use crate::util::SeqGenerator;

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct AudioKey(pub [u8; 16]);

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct AudioKeyError;

component! {
    AudioKeyManager<'_> : AudioKeyManagerInner {
        sequence: SeqGenerator<u32> = SeqGenerator::new(0),
        pending: HashMap<u32, oneshot::Sender<Result<AudioKey, AudioKeyError>>> = HashMap::new(),
    }
}

impl AudioKeyManager<'_> {
    pub(super) fn dispatch(self, cmd: u8, mut data: Bytes) {
        let seq = BigEndian::read_u32(data.split_to(4).as_ref());

        let sender = self.lock(|inner| inner.pending.remove(&seq));

        if let Some(sender) = sender {
            match cmd {
                0xd => {
                    let mut key = [0u8; 16];
                    key.copy_from_slice(data.as_ref());
                    let _ = sender.send(Ok(AudioKey(key)));
                }
                0xe => {
                    warn!(
                        "error audio key {:x} {:x}",
                        data.as_ref()[0],
                        data.as_ref()[1]
                    );
                    let _ = sender.send(Err(AudioKeyError));
                }
                _ => (),
            }
        }
    }

    pub async fn request(self, track: SpotifyId, file: FileId) -> Result<AudioKey, AudioKeyError> {
        let (tx, rx) = oneshot::channel();

        let seq = self.lock(move |inner| {
            let seq = inner.sequence.get();
            inner.pending.insert(seq, tx);
            seq
        });

        self.send_key_request(seq, track, file);
        rx.await.map_err(|_| AudioKeyError)?
    }

    fn send_key_request(self, seq: u32, track: SpotifyId, file: FileId) {
        let data = packet!(
            ([u8; 20]) &file.0,
            ([u8; 16]) &track.to_raw(),
            (u32) seq,
            (u16) 0x0000
        );

        self.send_packet(0xc, data)
    }
}
