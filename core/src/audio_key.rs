use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use bytes::Bytes;
use futures::sync::oneshot;
use futures::{Async, Future, Poll};
use std::collections::HashMap;
use std::io::Write;

use spotify_id::{FileId, SpotifyId};
use util::SeqGenerator;

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct AudioKey(pub [u8; 16]);

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct AudioKeyError;

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
                    warn!("error audio key {:x} {:x}", data.as_ref()[0], data.as_ref()[1]);
                    let _ = sender.send(Err(AudioKeyError));
                }
                _ => (),
            }
        }
    }

    pub fn request(&self, track: SpotifyId, file: FileId) -> AudioKeyFuture<AudioKey> {
        let (tx, rx) = oneshot::channel();

        let seq = self.lock(move |inner| {
            let seq = inner.sequence.get();
            inner.pending.insert(seq, tx);
            seq
        });

        self.send_key_request(seq, track, file);
        AudioKeyFuture(rx)
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

pub struct AudioKeyFuture<T>(oneshot::Receiver<Result<T, AudioKeyError>>);
impl<T> Future for AudioKeyFuture<T> {
    type Item = T;
    type Error = AudioKeyError;

    fn poll(&mut self) -> Poll<T, AudioKeyError> {
        match self.0.poll() {
            Ok(Async::Ready(Ok(value))) => Ok(Async::Ready(value)),
            Ok(Async::Ready(Err(err))) => Err(err),
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(oneshot::Canceled) => Err(AudioKeyError),
        }
    }
}
