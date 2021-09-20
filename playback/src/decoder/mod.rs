use thiserror::Error;

mod lewton_decoder;
pub use lewton_decoder::VorbisDecoder;

mod passthrough_decoder;
pub use passthrough_decoder::PassthroughDecoder;

#[derive(Error, Debug)]
pub enum DecoderError {
    #[error("Lewton Decoder Error: {0}")]
    LewtonDecoder(String),
    #[error("Passthrough Decoder Error: {0}")]
    PassthroughDecoder(String),
}

pub type DecoderResult<T> = Result<T, DecoderError>;

#[derive(Error, Debug)]
pub enum AudioPacketError {
    #[error("Decoder OggData Error: Can't return OggData on Samples")]
    OggData,
    #[error("Decoder Samples Error: Can't return Samples on OggData")]
    Samples,
}

pub type AudioPacketResult<T> = Result<T, AudioPacketError>;

pub enum AudioPacket {
    Samples(Vec<f64>),
    OggData(Vec<u8>),
}

impl AudioPacket {
    pub fn samples_from_f32(f32_samples: Vec<f32>) -> Self {
        let f64_samples = f32_samples.iter().map(|sample| *sample as f64).collect();
        AudioPacket::Samples(f64_samples)
    }

    pub fn samples(&self) -> AudioPacketResult<&[f64]> {
        match self {
            AudioPacket::Samples(s) => Ok(s),
            AudioPacket::OggData(_) => Err(AudioPacketError::OggData),
        }
    }

    pub fn oggdata(&self) -> AudioPacketResult<&[u8]> {
        match self {
            AudioPacket::OggData(d) => Ok(d),
            AudioPacket::Samples(_) => Err(AudioPacketError::Samples),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            AudioPacket::Samples(s) => s.is_empty(),
            AudioPacket::OggData(d) => d.is_empty(),
        }
    }
}

pub trait AudioDecoder {
    fn seek(&mut self, absgp: u64) -> DecoderResult<()>;
    fn next_packet(&mut self) -> DecoderResult<Option<AudioPacket>>;
}
