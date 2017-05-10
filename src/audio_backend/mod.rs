use std::io;

pub trait Open {
    fn open(Option<String>) -> Self;
}

pub trait Sink {
    fn start(&mut self) -> io::Result<()>;
    fn stop(&mut self) -> io::Result<()>;
    fn write(&mut self, data: &[i16]) -> io::Result<()>;
}

fn mk_sink<S: Sink + Open + 'static>(device: Option<String>) -> Box<Sink> {
    Box::new(S::open(device))
}

#[cfg(feature = "alsa-backend")]
mod alsa;
#[cfg(feature = "alsa-backend")]
use self::alsa::AlsaSink;

#[cfg(feature = "portaudio-backend")]
mod portaudio;
#[cfg(feature = "portaudio-backend")]
use self::portaudio::PortAudioSink;

#[cfg(feature = "pulseaudio-backend")]
mod pulseaudio;
#[cfg(feature = "pulseaudio-backend")]
use self::pulseaudio::PulseAudioSink;

mod pipe;
use self::pipe::StdoutSink;

pub const BACKENDS : &'static [
    (&'static str, fn(Option<String>) -> Box<Sink>)
] = &[
    #[cfg(feature = "alsa-backend")]
    ("alsa", mk_sink::<AlsaSink>),
    #[cfg(feature = "portaudio-backend")]
    ("portaudio", mk_sink::<PortAudioSink>),
    #[cfg(feature = "pulseaudio-backend")]
    ("pulseaudio", mk_sink::<PulseAudioSink>),
    ("pipe", mk_sink::<StdoutSink>),
];

pub fn find(name: Option<String>) -> Option<fn(Option<String>) -> Box<Sink>> {
    if let Some(name) = name {
        BACKENDS.iter().find(|backend| name == backend.0).map(|backend| backend.1)
    } else {
        Some(BACKENDS.first().expect("No backends were enabled at build time").1)
    }
}
