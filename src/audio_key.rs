use std::collections::HashMap;
use std::sync::{mpsc, Future};
use std::io::{Cursor, Write};
use byteorder::{BigEndian, ByteOrder, ReadBytesExt, WriteBytesExt};
use readall::ReadAllExt;

use util::{SpotifyId, FileId, IgnoreExt};
use session::Session;
use connection::PacketHandler;

pub type AudioKey = [u8; 16];
type AudioKeyId = u32;

pub struct AudioKeyManager {
    next_seq: AudioKeyId,
    callbacks: HashMap<AudioKeyId, mpsc::Sender<AudioKey>>,
}

impl AudioKeyManager {
    pub fn new() -> AudioKeyManager {
        AudioKeyManager {
            next_seq: 1,
            callbacks: HashMap::new(),
        }
    }

    pub fn request(&mut self, session: &Session, track: SpotifyId, file: FileId)
        -> Future<AudioKey> {
        let (tx, rx) = mpsc::channel();

        let seq = self.next_seq;
        self.next_seq += 1;

        let mut data : Vec<u8> = Vec::new();
        data.write(&file).unwrap();
        data.write(&track.to_raw()).unwrap();
        data.write_u32::<BigEndian>(seq).unwrap();
        data.write_u16::<BigEndian>(0x0000).unwrap();

        session.send_packet(0xc, &data).unwrap();

        self.callbacks.insert(seq, tx);

        Future::from_receiver(rx)
    }
}

impl PacketHandler for AudioKeyManager {
    fn handle(&mut self, cmd: u8, data: Vec<u8>) {
        assert_eq!(cmd, 0xd);

        let mut data = Cursor::new(data);
        let seq = data.read_u32::<BigEndian>().unwrap();
        let mut key = [0u8; 16];
        data.read_all(&mut key).unwrap();

        match self.callbacks.remove(&seq) {
            Some(callback) => callback.send(key).ignore(),
            None => ()
        };
    }
}

