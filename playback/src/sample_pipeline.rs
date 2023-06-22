use crate::{
    MS_PER_PAGE,
    audio_backend::{Sink, SinkResult},
    config::PlayerConfig,
    convert::Converter,
    decoder::AudioPacket,
    mixer::VolumeGetter,
    normaliser::Normaliser,
    player::NormalisationData,
    resampler::StereoInterleavedResampler,
};

pub struct SamplePipeline {
    resampler: StereoInterleavedResampler,
    normaliser: Normaliser,
    converter: Converter,
    sink: Box<dyn Sink>,
}

impl SamplePipeline {
    pub fn new(
        config: &PlayerConfig,
        sink: Box<dyn Sink>,
        volume_getter: Box<dyn VolumeGetter>,
    ) -> Self {
        let resampler =
            StereoInterleavedResampler::new(config.sample_rate, config.interpolation_quality);

        let normaliser = Normaliser::new(config, volume_getter);
        let converter = Converter::new(config.ditherer);

        Self {
            resampler,
            normaliser,
            converter,
            sink,
        }
    }

    pub fn get_latency_ms(&mut self) -> u32 {
        let total_latency_pcm = self.sink.get_latency_pcm() + self.resampler.get_latency_pcm();

        (total_latency_pcm as f64 * MS_PER_PAGE) as u32
    }

    pub fn start(&mut self) -> SinkResult<()> {
        self.sink.start()?;

        Ok(())
    }

    pub fn stop(&mut self) -> SinkResult<()> {
        self.resampler.stop();
        self.normaliser.stop();
        self.sink.stop()?;

        Ok(())
    }

    pub fn set_normalisation_factor(
        &mut self,
        auto_normalise_as_album: bool,
        data: NormalisationData,
    ) {
        self.normaliser.set_factor(auto_normalise_as_album, data);
    }

    pub fn write(&mut self, packet: AudioPacket) -> SinkResult<()> {
        if let AudioPacket::Samples(samples) = packet {
            self.resampler
                .process(&samples)
                .map(|processed_samples| self.normaliser.normalise(&processed_samples))
                .map(|new_packet| self.sink.write(new_packet, &mut self.converter))
                .transpose()?;
        } else {
            self.sink.write(packet, &mut self.converter)?;
        }

        Ok(())
    }
}
