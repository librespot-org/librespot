use portaudio;
use std::io;

pub trait Sink {
    fn start(&self) -> io::Result<()>;
    fn stop(&self) -> io::Result<()>;
    fn write(&self, data: &[i16]) -> io::Result<()>;
}

pub struct PortAudioSink<'a>(portaudio::stream::Stream<'a, i16, i16>);

impl <'a> PortAudioSink<'a> {
    pub fn open() -> PortAudioSink<'a> {
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
        self.0.write(&data).unwrap();
        Ok(())
    }
}

impl <'a> Drop for PortAudioSink<'a> {
    fn drop(&mut self) {
        portaudio::terminate().unwrap();
    }
}

