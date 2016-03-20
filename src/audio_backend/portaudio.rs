use super::{Open, Sink};
use std::io;
use portaudio;

pub struct PortAudioSink<'a>(portaudio::stream::Stream<'a, i16, i16>);

impl <'a> Open for PortAudioSink<'a> {
    fn open() -> PortAudioSink<'a> {
        portaudio::initialize().unwrap();

        let stream = portaudio::stream::Stream::open_default(
                0, 2, 44100.0,
                portaudio::stream::FRAMES_PER_BUFFER_UNSPECIFIED,
                None
        ).unwrap();

        PortAudioSink(stream)
    }
}

impl <'a> Sink for PortAudioSink<'a> {
    fn start(&self) -> io::Result<()> {
        self.0.start().unwrap();
        Ok(())
    }
    fn stop(&self) -> io::Result<()> {
        self.0.stop().unwrap();
        Ok(())
    }
    fn write(&self, data: &[i16]) -> io::Result<()> {
        match self.0.write(&data) {
            Ok(_) => (),
            Err(portaudio::PaError::OutputUnderflowed) => eprintln!("Underflow"),
            Err(e) => panic!("PA Error {}", e),
        };

        Ok(())
    }
}

impl <'a> Drop for PortAudioSink<'a> {
    fn drop(&mut self) {
        portaudio::terminate().unwrap();
    }
}
