use portaudio;
use std::sync::mpsc;
use vorbis;

use audio_key::{AudioKeyRequest, AudioKeyResponse};
use metadata::TrackRef;
use session::Session;
use audio_file::AudioFile;
use audio_decrypt::AudioDecrypt;
use util::Subfile;

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

        let mut decoder = 
            vorbis::Decoder::new(
                Subfile::new(
                        AudioDecrypt::new(key,
                            AudioFile::new(file_id, session.stream.clone())), 0xa7)).unwrap();
        //decoder.time_seek(60f64).unwrap();

        portaudio::initialize().unwrap();

        let stream = portaudio::stream::Stream::<i16>::open_default(
                0,
                2,
                44100.0,
                portaudio::stream::FRAMES_PER_BUFFER_UNSPECIFIED,
                None
                ).unwrap();
        stream.start().unwrap();

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

