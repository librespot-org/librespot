use std::{
    collections::{vec_deque, VecDeque},
    process::exit,
    sync::atomic::Ordering,
    sync::mpsc,
    thread,
};

use crate::{
    config::SampleRate, player::PLAYER_COUNTER, RESAMPLER_INPUT_SIZE,
    SAMPLE_RATE as SOURCE_SAMPLE_RATE,
};

struct DelayLine {
    buffer: VecDeque<f64>,
    coefficients_length: usize,
}

impl DelayLine {
    fn new(coefficients_length: usize) -> DelayLine {
        Self {
            buffer: VecDeque::with_capacity(coefficients_length),
            coefficients_length,
        }
    }

    fn push(&mut self, sample: f64) {
        self.buffer.push_back(sample);

        while self.buffer.len() > self.coefficients_length {
            self.buffer.pop_front();
        }
    }

    fn clear(&mut self) {
        self.buffer.clear();
    }
}

impl<'a> IntoIterator for &'a DelayLine {
    type Item = &'a f64;
    type IntoIter = vec_deque::Iter<'a, f64>;

    fn into_iter(self) -> Self::IntoIter {
        self.buffer.iter()
    }
}

struct ConvolutionFilter {
    coefficients: Vec<f64>,
    delay_line: DelayLine,
}

impl ConvolutionFilter {
    fn new(coefficients: Vec<f64>) -> Self {
        let delay_line = DelayLine::new(coefficients.len());

        Self {
            coefficients,
            delay_line,
        }
    }

    fn convolute(&mut self, sample: f64) -> f64 {
        self.delay_line.push(sample);

        // Temporal convolution
        self.coefficients
            .iter()
            .zip(&self.delay_line)
            .fold(0.0, |acc, (coefficient, delay_line_sample)| {
                acc + coefficient * delay_line_sample
            })
    }

    fn clear(&mut self) {
        self.delay_line.clear();
    }
}

struct MonoSincResampler {
    interpolator: ConvolutionFilter,
    input_buffer: Vec<f64>,
    resample_factor_reciprocal: f64,
    delay_line_latency: u64,
    interpolation_output_size: usize,
}

impl MonoSincResampler {
    fn new(sample_rate: SampleRate) -> Self {
        let coefficients = sample_rate
            .get_interpolation_coefficients()
            .unwrap_or_default();

        let resample_factor_reciprocal = sample_rate
            .get_resample_factor_reciprocal()
            .unwrap_or_default();

        let interpolation_output_size = sample_rate
            .get_interpolation_output_size()
            .unwrap_or_default();

        let delay_line_latency = (coefficients.len() as f64 * resample_factor_reciprocal) as u64;

        Self {
            interpolator: ConvolutionFilter::new(coefficients),
            input_buffer: Vec::with_capacity(SOURCE_SAMPLE_RATE as usize),
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

    fn resample(&mut self, samples: &[f64]) -> Option<Vec<f64>> {
        self.input_buffer.extend_from_slice(samples);

        let num_buffer_chunks = self.input_buffer.len().saturating_div(RESAMPLER_INPUT_SIZE);

        if num_buffer_chunks == 0 {
            return None;
        }

        let input_size = num_buffer_chunks * RESAMPLER_INPUT_SIZE;
        // The size of the output after interpolation.
        let output_size = num_buffer_chunks * self.interpolation_output_size;

        let mut output = Vec::with_capacity(output_size);

        output.extend((0..output_size).map(|ouput_index| {
            // The factional weights are already calculated and factored
            // into our interpolation coefficients so all we have to
            // do is pretend we're doing nearest-neighbor interpolation
            // and push samples though the Interpolator and what comes
            // out the other side is Sinc Windowed Interpolated samples.
            let sample_index = (ouput_index as f64 * self.resample_factor_reciprocal) as usize;
            let sample = self.input_buffer[sample_index];
            self.interpolator.convolute(sample)
        }));

        self.input_buffer.drain(..input_size);

        Some(output)
    }
}

enum ResampleTask {
    Stop,
    Terminate,
    Resample(Vec<f64>),
}

struct ResampleWorker {
    task_sender: Option<mpsc::Sender<ResampleTask>>,
    result_receiver: Option<mpsc::Receiver<(Option<Vec<f64>>, u64)>>,
    handle: Option<thread::JoinHandle<()>>,
}

impl ResampleWorker {
    fn new(mut resampler: MonoSincResampler, name: String) -> Self {
        let (task_sender, task_receiver) = mpsc::channel();
        let (result_sender, result_receiver) = mpsc::channel();

        let builder = thread::Builder::new().name(name.clone());

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
                    ResampleTask::Resample(samples) => {
                        let resampled = resampler.resample(&samples);
                        let latency = resampler.get_latency_pcm();

                        result_sender.send((resampled, latency)).ok();
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
                let player_id = PLAYER_COUNTER.load(Ordering::SeqCst).saturating_sub(1);

                Resampler::Worker {
                    left_resampler: ResampleWorker::new(
                        MonoSincResampler::new(sample_rate),
                        format!("resampler:{player_id}:left"),
                    ),
                    right_resampler: ResampleWorker::new(
                        MonoSincResampler::new(sample_rate),
                        format!("resampler:{player_id}:right"),
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

                let (left_resampled, left_latency_pcm) = left_resampler.get_resampled();
                let (right_resampled, right_latency_pcm) = right_resampler.get_resampled();

                // They should always be equal
                self.latency_pcm = left_latency_pcm.max(right_latency_pcm);

                left_resampled.and_then(|left_samples| {
                    right_resampled.map(|right_samples| {
                        Self::interleave_samples(&left_samples, &right_samples)
                    })
                })
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
