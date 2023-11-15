use crate::{
    audio_backend::{Sink, SinkResult},
    config::PlayerConfig,
    convert::Converter,
    decoder::AudioPacket,
    mixer::VolumeGetter,
    normaliser::Normaliser,
    player::NormalisationData,
    resampler::StereoInterleavedResampler,
    MS_PER_PAGE,
};

pub enum SamplePipeline {
    PassThrough(Bypass),
    Process(Pipeline),
}

impl SamplePipeline {
    pub fn new(
        config: &PlayerConfig,
        sink: Box<dyn Sink>,
        volume_getter: Box<dyn VolumeGetter>,
    ) -> Self {
        if config.passthrough {
            SamplePipeline::PassThrough(Bypass::new(config, sink))
        } else {
            SamplePipeline::Process(Pipeline::new(config, sink, volume_getter))
        }
    }

    pub fn get_latency_ms(&mut self) -> u32 {
        use SamplePipeline::*;

        match self {
            PassThrough(_) => 0,
            Process(ref mut p) => p.get_latency_ms(),
        }
    }

    pub fn start(&mut self) -> SinkResult<()> {
        use SamplePipeline::*;

        match self {
            PassThrough(ref mut p) => p.start()?,
            Process(ref mut p) => p.start()?,
        }

        Ok(())
    }

    pub fn stop(&mut self) -> SinkResult<()> {
        use SamplePipeline::*;

        match self {
            PassThrough(ref mut p) => p.stop()?,
            Process(ref mut p) => p.stop()?,
        }

        Ok(())
    }

    pub fn update_normalisation_data(
        &mut self,
        auto_normalise_as_album: bool,
        data: NormalisationData,
    ) {
        use SamplePipeline::*;

        match self {
            PassThrough(_) => (),
            Process(ref mut p) => p.update_normalisation_data(auto_normalise_as_album, data),
        }
    }

    pub fn write(&mut self, packet: AudioPacket) -> SinkResult<()> {
        use SamplePipeline::*;

        match self {
            PassThrough(ref mut p) => p.write(packet)?,
            Process(ref mut p) => p.write(packet)?,
        }

        Ok(())
    }
}

pub struct Bypass {
    converter: Converter,
    sink: Box<dyn Sink>,
}

impl Bypass {
    fn new(config: &PlayerConfig, sink: Box<dyn Sink>) -> Self {
        let converter = Converter::new(config.ditherer);

        Self { converter, sink }
    }

    fn start(&mut self) -> SinkResult<()> {
        self.sink.start()?;

        Ok(())
    }

    fn stop(&mut self) -> SinkResult<()> {
        self.sink.stop()?;

        Ok(())
    }

    fn write(&mut self, packet: AudioPacket) -> SinkResult<()> {
        self.sink.write(packet, &mut self.converter)?;

        Ok(())
    }
}

pub struct Pipeline {
    resampler: StereoInterleavedResampler,
    normaliser: Normaliser,
    converter: Converter,
    sink: Box<dyn Sink>,
}

impl Pipeline {
    fn new(
        config: &PlayerConfig,
        sink: Box<dyn Sink>,
        volume_getter: Box<dyn VolumeGetter>,
    ) -> Self {
        let resampler = StereoInterleavedResampler::new(config.sample_rate);

        let normaliser = Normaliser::new(config, volume_getter);
        let converter = Converter::new(config.ditherer);

        Self {
            resampler,
            normaliser,
            converter,
            sink,
        }
    }

    fn get_latency_ms(&mut self) -> u32 {
        let total_latency_pcm = self.sink.get_latency_pcm() + self.resampler.get_latency_pcm();

        (total_latency_pcm as f64 * MS_PER_PAGE) as u32
    }

    fn start(&mut self) -> SinkResult<()> {
        self.sink.start()?;

        Ok(())
    }

    fn stop(&mut self) -> SinkResult<()> {
        self.resampler
            .drain()
            .map(|processed_samples| self.normaliser.normalise(processed_samples))
            .map(|new_packet| self.sink.write(new_packet, &mut self.converter))
            .transpose()?;

        self.resampler.stop();
        self.normaliser.stop();

        self.sink.stop()?;

        Ok(())
    }

    fn update_normalisation_data(
        &mut self,
        auto_normalise_as_album: bool,
        data: NormalisationData,
    ) {
        self.normaliser
            .update_normalisation_data(auto_normalise_as_album, data);
    }

    fn write(&mut self, packet: AudioPacket) -> SinkResult<()> {
        if let AudioPacket::Samples(samples) = packet {
            self.resampler
                .resample(samples)
                .map(|processed_samples| self.normaliser.normalise(processed_samples))
                .map(|new_packet| self.sink.write(new_packet, &mut self.converter))
                .transpose()?;
        }

        Ok(())
    }
}
