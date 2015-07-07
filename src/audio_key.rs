use std::collections::{HashMap, LinkedList};
use std::sync::{mpsc, Future};
use std::io::{Cursor, Write};
use byteorder::{BigEndian, ByteOrder, ReadBytesExt, WriteBytesExt};
use readall::ReadAllExt;
use std::mem;

use util::{SpotifyId, FileId, IgnoreExt};
use session::Session;
use connection::PacketHandler;

pub type AudioKey = [u8; 16];

#[derive(Debug,Hash,PartialEq,Eq,Clone)]
struct AudioKeyId(SpotifyId, FileId);

enum AudioKeyStatus {
    Loading(LinkedList<mpsc::Sender<AudioKey>>),
    Loaded(AudioKey)
}

pub struct AudioKeyManager {
    next_seq: u32,
    pending: HashMap<u32, AudioKeyId>, 
    cache: HashMap<AudioKeyId, AudioKeyStatus>,
}

impl AudioKeyManager {
    pub fn new() -> AudioKeyManager {
        AudioKeyManager {
            next_seq: 1,
            pending: HashMap::new(),
            cache: HashMap::new()
        }
    }

    pub fn request(&mut self, session: &Session, track: SpotifyId, file: FileId)
        -> Future<AudioKey> {

        let id = AudioKeyId(track, file);
        self.cache.get_mut(&id).map(|status| match status {
            &mut AudioKeyStatus::Loaded(key) => {
                Future::from_value(key.clone())
            }
            &mut AudioKeyStatus::Loading(ref mut req) => {
                let (tx, rx) = mpsc::channel();
                req.push_front(tx);
                Future::from_receiver(rx)
            }
        }).unwrap_or_else(|| {
            let seq = self.next_seq;
            self.next_seq += 1;

            let mut data : Vec<u8> = Vec::new();
            data.write(&file.0).unwrap();
            data.write(&track.to_raw()).unwrap();
            data.write_u32::<BigEndian>(seq).unwrap();
            data.write_u16::<BigEndian>(0x0000).unwrap();

            session.send_packet(0xc, &data).unwrap();

            self.pending.insert(seq, id.clone());

            let (tx, rx) = mpsc::channel();
            let mut req = LinkedList::new();
            req.push_front(tx);
            self.cache.insert(id, AudioKeyStatus::Loading(req));
            Future::from_receiver(rx)
        })
    }
}

impl PacketHandler for AudioKeyManager {
    fn handle(&mut self, cmd: u8, data: Vec<u8>) {
        assert_eq!(cmd, 0xd);

        let mut data = Cursor::new(data);
        let seq = data.read_u32::<BigEndian>().unwrap();
        let mut key = [0u8; 16];
        data.read_all(&mut key).unwrap();

        if let Some(status) = self.pending.remove(&seq).and_then(|id| { self.cache.get_mut(&id) }) {
            let status = mem::replace(status, AudioKeyStatus::Loaded(key));

            if let AudioKeyStatus::Loading(cbs) = status {
                for cb in cbs {
                    cb.send(key).ignore();
                }
            }
        }
    }
}

