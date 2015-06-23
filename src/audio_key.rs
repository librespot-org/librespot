use std::collections::HashMap;
use std::sync::mpsc;
use std::io::{Cursor, Write};
use byteorder::{BigEndian, ByteOrder, ReadBytesExt, WriteBytesExt};
use readall::ReadAllExt;

use connection::Packet;
use util::{SpotifyId, FileId};
use util::Either::{Left, Right};
use subsystem::Subsystem;

pub struct AudioKeyRequest {
    pub track: SpotifyId,
    pub file: FileId,
    pub callback: AudioKeyCallback,
}
pub type AudioKey = [u8; 16];
pub struct AudioKeyResponse(pub AudioKey);
pub type AudioKeyCallback = mpsc::Sender<AudioKeyResponse>;

type AudioKeyId = u32;
pub struct AudioKeyManager {
    next_seq: AudioKeyId,
    callbacks: HashMap<AudioKeyId, AudioKeyCallback>,

    requests: mpsc::Receiver<AudioKeyRequest>,
    packet_rx: mpsc::Receiver<Packet>,
    packet_tx: mpsc::Sender<Packet>,
}

impl AudioKeyManager {
    pub fn new(tx: mpsc::Sender<Packet>) -> (AudioKeyManager,
                                             mpsc::Sender<AudioKeyRequest>,
                                             mpsc::Sender<Packet>) {
        let (req_tx, req_rx) = mpsc::channel();
        let (pkt_tx, pkt_rx) = mpsc::channel();

        (AudioKeyManager {
            next_seq: 1,
            callbacks: HashMap::new(),

            requests: req_rx,
            packet_rx: pkt_rx,
            packet_tx: tx
        }, req_tx, pkt_tx)
    }

    fn request(&mut self, req: AudioKeyRequest) {
        let seq = self.next_seq;
        self.next_seq += 1;

        let mut data : Vec<u8> = Vec::new();
        data.write(&req.file).unwrap();
        data.write(&req.track.to_raw()).unwrap();
        data.write_u32::<BigEndian>(seq).unwrap();
        data.write_u16::<BigEndian>(0x0000).unwrap();

        self.packet_tx.send(Packet {
            cmd: 0xc,
            data: data
        }).unwrap();

        self.callbacks.insert(seq, req.callback);
    }

    fn packet(&mut self, packet: Packet) {
        assert_eq!(packet.cmd, 0xd);

        let mut data = Cursor::new(&packet.data as &[u8]);
        let seq = data.read_u32::<BigEndian>().unwrap();
        let mut key = [0u8; 16];
        data.read_all(&mut key).unwrap();

        match self.callbacks.remove(&seq) {
            Some(callback) => callback.send(AudioKeyResponse(key)).unwrap(),
            None => ()
        };
    }
}


impl Subsystem for AudioKeyManager {
    fn run(mut self) {
        loop {
            match {
                let requests = &self.requests;
                let packets = &self.packet_rx;

                select!{
                    r = requests.recv() => {
                        Left(r.unwrap())
                    },
                    p = packets.recv() => {
                        Right(p.unwrap())
                    }
                }
            } {
                Left(req) => {
                    self.request(req);
                }
                Right(pkt) => {
                    self.packet(pkt);
                }
            }

        }
    }
}

