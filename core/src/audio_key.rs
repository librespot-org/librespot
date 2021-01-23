use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use bytes::Bytes;
use futures::channel::oneshot;
use std::collections::HashMap;
use std::io::Write;
use thiserror::Error;

use crate::spotify_id::{FileId, SpotifyId};
use crate::util::SeqGenerator;

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct AudioKey(pub [u8; 16]);

#[derive(Error, Debug)]
pub enum AudioKeyError {
    #[error("AudioKey sender disconnected")]
    Cancelled(#[from] oneshot::Canceled),
    #[error("Unknown server response: `{0:?}`")]
    UnknownResponse(Vec<u8>),
}

component! {
    AudioKeyManager : AudioKeyManagerInner {
        sequence: SeqGenerator<u32> = SeqGenerator::new(0),
        pending: HashMap<u32, oneshot::Sender<Result<AudioKey, AudioKeyError>>> = HashMap::new(),
    }
}

impl AudioKeyManager {
    pub(crate) fn dispatch(&self, cmd: u8, mut data: Bytes) {
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
                    let _ = sender.send(Err(AudioKeyError::UnknownResponse(
                        data.as_ref()[..1].to_vec(),
                    )));
                }
                _ => warn!("Unexpected audioKey response: 0x{:x?} {:?}", cmd, data),
            }
        }
    }

    pub async fn request(&self, track: SpotifyId, file: FileId) -> Result<AudioKey, AudioKeyError> {
        let (tx, rx) = oneshot::channel();

        let seq = self.lock(move |inner| {
            let seq = inner.sequence.get();
            inner.pending.insert(seq, tx);
            seq
        });

        self.send_key_request(seq, track, file);
        rx.await?
    }

    fn send_key_request(&self, seq: u32, track: SpotifyId, file: FileId) {
        let mut data: Vec<u8> = Vec::new();
        data.write(&file.0).unwrap();
        data.write(&track.to_raw()).unwrap();
        data.write_u32::<BigEndian>(seq).unwrap();
        data.write_u16::<BigEndian>(0x0000).unwrap();

        self.session().send_packet(0xc, data)
    }
}
