use std::{
    collections::VecDeque, process::exit, sync::atomic::Ordering::SeqCst, sync::mpsc, thread,
};

use crate::{config::SampleRate, player::PLAYER_COUNTER, RESAMPLER_INPUT_SIZE};

struct ConvolutionFilter {
    coefficients: Vec<f64>,
    coefficients_length: usize,
    delay_line: VecDeque<f64>,
}

impl ConvolutionFilter {
    fn new(coefficients: Vec<f64>) -> Self {
        let coefficients_length = coefficients.len();
        let delay_line = VecDeque::with_capacity(coefficients_length);

        Self {
            coefficients,
            coefficients_length,
            delay_line,
        }
    }

    fn get_convoluted_sample(&mut self) -> f64 {
        let output_sample = self
            .coefficients
            .iter()
            .zip(&self.delay_line)
            .fold(0.0, |acc, (coefficient, delay_line_sample)| {
                acc + coefficient * delay_line_sample
            });

        self.delay_line.pop_front();

        output_sample
    }

    fn convolute(&mut self, sample: f64) -> f64 {
        self.delay_line.push_back(sample);

        if self.delay_line.len() == self.coefficients_length {
            self.get_convoluted_sample()
        } else {
            0.0
        }
    }

    fn drain(&mut self) -> Vec<f64> {
        let delay_line_len = self.delay_line.len();
        let mut output = Vec::with_capacity(delay_line_len);

        for _ in 0..delay_line_len {
            output.push(self.get_convoluted_sample());
        }

        output
    }

    fn clear(&mut self) {
        self.delay_line.clear();
    }
}

struct MonoSincResampler {
    interpolator: ConvolutionFilter,
    input_buffer: Vec<f64>,
    resample_factor: f64,
    resample_factor_reciprocal: f64,
    delay_line_latency: u64,
    interpolation_output_size: usize,
}

impl MonoSincResampler {
    fn new(sample_rate: SampleRate) -> Self {
        let coefficients = sample_rate
            .get_interpolation_coefficients()
            .unwrap_or_default();

        let resample_factor = sample_rate.get_resample_factor().unwrap_or_default();

        let resample_factor_reciprocal = sample_rate
            .get_resample_factor_reciprocal()
            .unwrap_or_default();

        let interpolation_output_size = sample_rate
            .get_interpolation_output_size()
            .unwrap_or_default();

        let delay_line_latency = (coefficients.len() as f64 * resample_factor_reciprocal) as u64;

        Self {
            interpolator: ConvolutionFilter::new(coefficients),
            input_buffer: Vec::with_capacity(RESAMPLER_INPUT_SIZE),
            resample_factor,
            resample_factor_reciprocal,
            delay_line_latency,
            interpolation_output_size,
        }
    }

    fn get_latency_pcm(&mut self) -> u64 {
        self.input_buffer.len() as u64 + self.delay_line_latency
    }

    fn stop(&mut self) {
        self.interpolator.clear();
        self.input_buffer.clear();
    }

    fn drain(&mut self) -> (Option<Vec<f64>>, u64) {
        // On drain the interpolation isn't perfect for a couple reasons:
        // 1. buffer len * resample_factor more than likely isn't an integer.
        // 2. As you drain the delay line there are less and less samples to use for interpolation.
        let output_len = (self.input_buffer.len() as f64 * self.resample_factor) as usize;
        let mut output = Vec::with_capacity(output_len);

        output.extend((0..output_len).map(|ouput_index| {
            self.interpolator.convolute(
                *self
                    .input_buffer
                    .get((ouput_index as f64 * self.resample_factor_reciprocal) as usize)
                    .unwrap_or(&0.0),
            )
        }));

        let interpolator_drainage = self.interpolator.drain();

        output.reserve_exact(interpolator_drainage.len());

        output.extend(interpolator_drainage.iter());

        let output_len = output.len() as f64;

        // Do a simple linear fade out of the drainage (about 5ms) to hide/prevent audible artifacts.
        for (index, sample) in output.iter_mut().enumerate() {
            let fade_factor = 1.0 - (index as f64) / output_len;
            *sample *= fade_factor;
        }

        (Some(output), 0)
    }

    fn resample(&mut self, samples: &[f64]) -> (Option<Vec<f64>>, u64) {
        self.input_buffer.extend_from_slice(samples);

        let num_buffer_chunks = self.input_buffer.len().saturating_div(RESAMPLER_INPUT_SIZE);

        if num_buffer_chunks == 0 {
            return (None, self.get_latency_pcm());
        }

        let input_size = num_buffer_chunks * RESAMPLER_INPUT_SIZE;

        let output_size = num_buffer_chunks * self.interpolation_output_size;

        let mut output = Vec::with_capacity(output_size);

        output.extend((0..output_size).map(|ouput_index| {
            // Since the interpolation coefficients are pre-calculated we can pretend like
            // we're doing nearest neighbor interpolation and then push the samples though
            // the interpolator as if it were a simple FIR filter (which it actually also is).
            // What comes out the other side is anti-aliased windowed sinc interpolated samples.
            self.interpolator.convolute(
                *self
                    .input_buffer
                    .get((ouput_index as f64 * self.resample_factor_reciprocal) as usize)
                    .unwrap_or(&0.0),
            )
        }));

        self.input_buffer.drain(..input_size);

        (Some(output), self.get_latency_pcm())
    }
}

enum ResampleTask {
    Stop,
    Drain,
    Terminate,
    Resample(Vec<f64>),
}

struct ResampleWorker {
    task_sender: Option<mpsc::Sender<ResampleTask>>,
    result_receiver: Option<mpsc::Receiver<(Option<Vec<f64>>, u64)>>,
    handle: Option<thread::JoinHandle<()>>,
}

impl ResampleWorker {
    fn new(sample_rate: SampleRate, name: String) -> Self {
        let (task_sender, task_receiver) = mpsc::channel();
        let (result_sender, result_receiver) = mpsc::channel();

        let builder = thread::Builder::new().name(name.clone());

        let mut resampler = MonoSincResampler::new(sample_rate);

        let handle = match builder.spawn(move || loop {
            match task_receiver.recv() {
                Err(e) => {
                    match thread::current().name() {
                        Some(name) => error!("Error in <ResampleWorker> [{name}] thread: {e}"),
                        None => error!("Error in <ResampleWorker> thread: {e}"),
                    }

                    exit(1);
                }
                Ok(task) => match task {
                    ResampleTask::Stop => resampler.stop(),
                    ResampleTask::Drain => {
                        result_sender.send(resampler.drain()).ok();
                    }
                    ResampleTask::Resample(samples) => {
                        result_sender.send(resampler.resample(&samples)).ok();
                    }
                    ResampleTask::Terminate => {
                        loop {
                            let drained = task_receiver.recv().ok();

                            if drained.is_none() {
                                break;
                            }
                        }

                        match thread::current().name() {
                            Some(name) => debug!("<ResampleWorker> [{name}] thread finished"),
                            None => debug!("<ResampleWorker> thread finished"),
                        }

                        break;
                    }
                },
            }
        }) {
            Ok(handle) => {
                debug!("Created <ResampleWorker> [{name}] thread");
                handle
            }
            Err(e) => {
                error!("Error creating <ResampleWorker> [{name}] thread: {e}");
                exit(1);
            }
        };

        Self {
            task_sender: Some(task_sender),
            result_receiver: Some(result_receiver),
            handle: Some(handle),
        }
    }

    fn stop(&mut self) {
        self.task_sender
            .as_mut()
            .and_then(|sender| sender.send(ResampleTask::Stop).ok());
    }

    fn drain(&mut self) {
        self.task_sender
            .as_mut()
            .and_then(|sender| sender.send(ResampleTask::Drain).ok());
    }

    fn resample(&mut self, samples: Vec<f64>) {
        self.task_sender
            .as_mut()
            .and_then(|sender| sender.send(ResampleTask::Resample(samples)).ok());
    }

    fn get_resampled(&mut self) -> (Option<Vec<f64>>, u64) {
        self.result_receiver
            .as_mut()
            .and_then(|result_receiver| result_receiver.recv().ok())
            .unwrap_or((None, 0))
    }
}

impl Drop for ResampleWorker {
    fn drop(&mut self) {
        debug!("Shutting down <ResampleWorker> thread ...");
        self.task_sender
            .take()
            .and_then(|sender| sender.send(ResampleTask::Terminate).ok());

        self.result_receiver
            .take()
            .and_then(|result_receiver| loop {
                let drained = result_receiver.recv().ok();

                if drained.is_none() {
                    break drained;
                }
            });

        self.handle.take().and_then(|handle| handle.join().ok());
    }
}

enum Resampler {
    Bypass,
    Worker {
        left_resampler: ResampleWorker,
        right_resampler: ResampleWorker,
    },
}

pub struct StereoInterleavedResampler {
    resampler: Resampler,
    latency_pcm: u64,
}

impl StereoInterleavedResampler {
    pub fn new(sample_rate: SampleRate) -> Self {
        debug!("Sample Rate: {sample_rate}");

        let resampler = match sample_rate {
            SampleRate::Hz44100 => {
                debug!("Interpolation Type: Bypass");
                debug!("No <ResampleWorker> threads required");

                Resampler::Bypass
            }
            _ => {
                debug!("Interpolation Type: Windowed Sinc");

                // The player increments the player id when it gets it...
                let player_id = PLAYER_COUNTER.load(SeqCst).saturating_sub(1);

                Resampler::Worker {
                    left_resampler: ResampleWorker::new(
                        sample_rate,
                        format!("resampler:L:{player_id}"),
                    ),
                    right_resampler: ResampleWorker::new(
                        sample_rate,
                        format!("resampler:R:{player_id}"),
                    ),
                }
            }
        };

        Self {
            resampler,
            latency_pcm: 0,
        }
    }

    pub fn get_latency_pcm(&mut self) -> u64 {
        self.latency_pcm
    }

    pub fn drain(&mut self) -> Option<Vec<f64>> {
        match &mut self.resampler {
            // Bypass is basically a no-op.
            Resampler::Bypass => None,
            Resampler::Worker {
                left_resampler,
                right_resampler,
            } => {
                left_resampler.drain();
                right_resampler.drain();

                let (resampled, latency_pcm) = Self::get_resampled(left_resampler, right_resampler);

                self.latency_pcm = latency_pcm;

                resampled
            }
        }
    }

    pub fn resample(&mut self, input_samples: Vec<f64>) -> Option<Vec<f64>> {
        match &mut self.resampler {
            // Bypass is basically a no-op.
            Resampler::Bypass => Some(input_samples),
            Resampler::Worker {
                left_resampler,
                right_resampler,
            } => {
                let (left_samples, right_samples) = Self::deinterleave_samples(&input_samples);

                left_resampler.resample(left_samples);
                right_resampler.resample(right_samples);

                let (resampled, latency_pcm) = Self::get_resampled(left_resampler, right_resampler);

                self.latency_pcm = latency_pcm;

                resampled
            }
        }
    }

    pub fn stop(&mut self) {
        self.latency_pcm = 0;

        match &mut self.resampler {
            // Stop does nothing
            // if we're bypassed.
            Resampler::Bypass => (),
            Resampler::Worker {
                left_resampler,
                right_resampler,
            } => {
                left_resampler.stop();
                right_resampler.stop();
            }
        }
    }

    fn get_resampled(
        left_resampler: &mut ResampleWorker,
        right_resampler: &mut ResampleWorker,
    ) -> (Option<Vec<f64>>, u64) {
        let (left_resampled, left_latency_pcm) = left_resampler.get_resampled();
        let (right_resampled, right_latency_pcm) = right_resampler.get_resampled();

        let resampled = left_resampled.and_then(|left_samples| {
            right_resampled
                .map(|right_samples| Self::interleave_samples(&left_samples, &right_samples))
        });

        // They should always be equal
        let latency_pcm = left_latency_pcm.max(right_latency_pcm);

        (resampled, latency_pcm)
    }

    fn interleave_samples(left_samples: &[f64], right_samples: &[f64]) -> Vec<f64> {
        // Re-interleave the resampled channels.
        let mut output = Vec::with_capacity(left_samples.len() + right_samples.len());

        output.extend(
            left_samples
                .iter()
                .zip(right_samples.iter())
                .flat_map(|(&left, &right)| std::iter::once(left).chain(std::iter::once(right))),
        );

        output
    }

    fn deinterleave_samples(samples: &[f64]) -> (Vec<f64>, Vec<f64>) {
        // Split the stereo interleaved samples into left and right channels.
        let samples_len = samples.len() / 2;

        let mut left_samples = Vec::with_capacity(samples_len);
        let mut right_samples = Vec::with_capacity(samples_len);

        left_samples.extend(samples.iter().step_by(2));
        right_samples.extend(samples.iter().skip(1).step_by(2));

        (left_samples, right_samples)
    }
}
