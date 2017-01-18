use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use eventual;
use std::collections::HashMap;
use std::io::{Cursor, Read, Write};

use util::{SpotifyId, FileId};
use session::{Session, PacketHandler};

pub type AudioKey = [u8; 16];
#[derive(Debug,Hash,PartialEq,Eq,Copy,Clone)]
pub struct AudioKeyError;

#[derive(Debug,Hash,PartialEq,Eq,Copy,Clone)]
struct AudioKeyId(SpotifyId, FileId);

pub struct AudioKeyManager {
    next_seq: u32,
    pending: HashMap<u32, AudioKeyId>,
    cache: HashMap<AudioKeyId, Vec<eventual::Complete<AudioKey, AudioKeyError>>>,
}

impl AudioKeyManager {
    pub fn new() -> AudioKeyManager {
        AudioKeyManager {
            next_seq: 1,
            pending: HashMap::new(),
            cache: HashMap::new(),
        }
    }

    fn send_key_request(&mut self, session: &Session, track: SpotifyId, file: FileId) -> u32 {
        let seq = self.next_seq;
        self.next_seq += 1;

        let mut data: Vec<u8> = Vec::new();
        data.write(&file.0).unwrap();
        data.write(&track.to_raw()).unwrap();
        data.write_u32::<BigEndian>(seq).unwrap();
        data.write_u16::<BigEndian>(0x0000).unwrap();

        session.send_packet(0xc, data);

        seq
    }

    pub fn request(&mut self,
                   session: &Session,
                   track: SpotifyId,
                   file: FileId)
                   -> eventual::Future<AudioKey, AudioKeyError> {

        let id = AudioKeyId(track, file);
        self.cache
            .get_mut(&id)
            .map(|ref mut requests| {
                let (tx, rx) = eventual::Future::pair();
                requests.push(tx);
                rx
            })
            .unwrap_or_else(|| {
                let seq = self.send_key_request(session, track, file);
                self.pending.insert(seq, id.clone());

                let (tx, rx) = eventual::Future::pair();
                self.cache.insert(id, vec![tx]);
                rx
            })
    }
}

impl PacketHandler for AudioKeyManager {
    fn handle(&mut self, cmd: u8, data: Vec<u8>, _session: &Session) {
        let mut data = Cursor::new(data);
        let seq = data.read_u32::<BigEndian>().unwrap();

        if let Some(callbacks) = self.pending.remove(&seq).and_then(|id| self.cache.remove(&id)) {
            if cmd == 0xd {
                let mut key = [0u8; 16];
                data.read_exact(&mut key).unwrap();

                for cb in callbacks {
                    cb.complete(key);
                }
            } else if cmd == 0xe {
                let error = AudioKeyError;
                for cb in callbacks {
                    cb.fail(error);
                }
            }
        }
    }
}
