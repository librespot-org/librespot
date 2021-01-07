use crate::audio::AudioPacket;
use std::io;

pub trait Open {
    fn open(_: Option<String>) -> Self;
}

pub trait Sink {
    fn start(&mut self) -> io::Result<()>;
    fn stop(&mut self) -> io::Result<()>;
    fn write(&mut self, packet: &AudioPacket) -> io::Result<()>;
}

fn mk_sink<S: Sink + Open + 'static>(device: Option<String>) -> Box<dyn Sink> {
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

#[cfg(feature = "jackaudio-backend")]
mod jackaudio;
#[cfg(feature = "jackaudio-backend")]
use self::jackaudio::JackSink;

#[cfg(all(
    feature = "rodiojack-backend",
    not(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd"))
))]
compile_error!("Rodio JACK backend is currently only supported on linux.");

#[cfg(all(
    feature = "rodiojack-backend",
    any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd")
))]
use self::rodio::JackRodioSink;

#[cfg(feature = "gstreamer-backend")]
mod gstreamer;
#[cfg(feature = "gstreamer-backend")]
use self::gstreamer::GstreamerSink;

#[cfg(any(feature = "rodio-backend", feature = "rodiojack-backend"))]
mod rodio;
#[cfg(feature = "rodio-backend")]
use self::rodio::RodioSink;

#[cfg(feature = "sdl-backend")]
mod sdl;
#[cfg(feature = "sdl-backend")]
use self::sdl::SdlSink;

mod pipe;
use self::pipe::StdoutSink;

mod subprocess;
use self::subprocess::SubprocessSink;

pub const BACKENDS: &'static [(&'static str, fn(Option<String>) -> Box<dyn Sink>)] = &[
    #[cfg(feature = "alsa-backend")]
    ("alsa", mk_sink::<AlsaSink>),
    #[cfg(feature = "portaudio-backend")]
    ("portaudio", mk_sink::<PortAudioSink>),
    #[cfg(feature = "pulseaudio-backend")]
    ("pulseaudio", mk_sink::<PulseAudioSink>),
    #[cfg(feature = "jackaudio-backend")]
    ("jackaudio", mk_sink::<JackSink>),
    #[cfg(all(
        feature = "rodiojack-backend",
        any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd")
    ))]
    ("rodiojack", mk_sink::<JackRodioSink>),
    #[cfg(feature = "gstreamer-backend")]
    ("gstreamer", mk_sink::<GstreamerSink>),
    #[cfg(feature = "rodio-backend")]
    ("rodio", mk_sink::<RodioSink>),
    #[cfg(feature = "sdl-backend")]
    ("sdl", mk_sink::<SdlSink>),
    ("pipe", mk_sink::<StdoutSink>),
    ("subprocess", mk_sink::<SubprocessSink>),
];

pub fn find(name: Option<String>) -> Option<fn(Option<String>) -> Box<dyn Sink>> {
    if let Some(name) = name {
        BACKENDS
            .iter()
            .find(|backend| name == backend.0)
            .map(|backend| backend.1)
    } else {
        Some(
            BACKENDS
                .first()
                .expect("No backends were enabled at build time")
                .1,
        )
    }
}
