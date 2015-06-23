use portaudio;
use std::sync::mpsc;
use std::thread;
use vorbis;

use audio_key::{AudioKeyRequest, AudioKeyResponse};
use metadata::TrackRef;
use session::Session;
use audio_file::{AudioFileRef, AudioFileReader};
use audio_decrypt::AudioDecrypt;

pub struct Player;

impl Player {
    pub fn play(session: &Session, track: TrackRef) {
        let file_id = *track.wait().unwrap().files.first().unwrap();

        let key = {
            let (tx, rx) = mpsc::channel();

            session.audio_key.send(AudioKeyRequest {
                track: track.id(),
                file: file_id,
                callback: tx
            }).unwrap();
            
            let AudioKeyResponse(key) = rx.recv().unwrap();
            key
        };

        let reader = {
            let file = AudioFileRef::new(file_id, session.stream.clone());
            let f = file.clone();
            let s = session.stream.clone();
            thread::spawn( move || { f.fetch(s) });
            AudioDecrypt::new(key, AudioFileReader::new(&file))
        };


        portaudio::initialize().unwrap();

        let stream = portaudio::stream::Stream::<i16>::open_default(
                0,
                2,
                44100.0,
                portaudio::stream::FRAMES_PER_BUFFER_UNSPECIFIED,
                None
                ).unwrap();
        stream.start().unwrap();

        let mut decoder = vorbis::Decoder::new(reader).unwrap();

        for pkt in decoder.packets() {
            match pkt {
                Ok(packet) => {
                    match stream.write(&packet.data) {
                        Ok(_) => (),
                        Err(portaudio::PaError::OutputUnderflowed)
                            => eprintln!("Underflow"),
                        Err(e) => panic!("PA Error {}", e)
                    };
                },
                Err(vorbis::VorbisError::Hole) => (),
                Err(e) => panic!("Vorbis error {:?}", e)
            }
        }

        drop(stream);

        portaudio::terminate().unwrap();
    }
}

