use super::{Open, Sink, SinkAsBytes, SinkError, SinkResult};
use crate::config::AudioFormat;
use crate::convert::Converter;
use crate::decoder::AudioPacket;
use crate::{NUM_CHANNELS, SAMPLE_RATE};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::thread;
use thiserror::Error;

use libspa::sys as spa_sys;
use pipewire as pw;
use pw::{properties::properties, spa};
use ringbuf::{
    HeapRb,
    traits::{Consumer, Producer, Split},
};
use spa::pod::Pod;

type RingProducer = ringbuf::wrap::caching::Caching<
    std::sync::Arc<ringbuf::SharedRb<ringbuf::storage::Heap<u8>>>,
    true,
    false,
>;
type RingConsumer = ringbuf::wrap::caching::Caching<
    std::sync::Arc<ringbuf::SharedRb<ringbuf::storage::Heap<u8>>>,
    false,
    true,
>;

// Ring buffer size: 1 second of audio at max quality (F64, stereo, 44.1kHz)
const RING_BUFFER_SIZE: usize = 44100 * 2 * 8;

#[derive(Debug, Error)]
enum PipeWireError {
    #[error("<PipeWireSink> Failed to Create Main Loop: {0}")]
    MainLoopCreation(String),

    #[error("<PipeWireSink> Failed to Create Stream: {0}")]
    StreamCreation(String),

    #[error("<PipeWireSink> Failed to Connect Stream: {0}")]
    StreamConnect(String),

    #[error("<PipeWireSink> Stream Not Connected")]
    NotConnected,
}

impl From<PipeWireError> for SinkError {
    fn from(e: PipeWireError) -> SinkError {
        use PipeWireError::*;
        let es = e.to_string();
        match e {
            MainLoopCreation(_) | StreamCreation(_) | StreamConnect(_) => {
                SinkError::ConnectionRefused(es)
            }
            NotConnected => SinkError::NotConnected(es),
        }
    }
}

pub struct PipeWireSink {
    format: AudioFormat,
    // Lock-free ring buffer producer for audio data
    producer: Option<RingProducer>,
    // Flag to signal thread to stop
    quit_flag: Arc<AtomicBool>,
    initialized: bool,
    _main_loop_handle: Option<thread::JoinHandle<()>>,
}

fn calculate_sample_size(format: AudioFormat) -> usize {
    use AudioFormat::*;
    match format {
        F64 => 8,
        F32 | S32 | S24 => 4,
        S24_3 => 3,
        S16 => 2,
    }
}

fn convert_audio_format(format: AudioFormat) -> spa::param::audio::AudioFormat {
    use AudioFormat::*;
    match format {
        F64 => spa::param::audio::AudioFormat::F64LE,
        F32 => spa::param::audio::AudioFormat::F32LE,
        S32 => spa::param::audio::AudioFormat::S32LE,
        S24 => spa::param::audio::AudioFormat::S24_32LE,
        S24_3 => spa::param::audio::AudioFormat::S24LE,
        S16 => spa::param::audio::AudioFormat::S16LE,
    }
}

impl Open for PipeWireSink {
    fn open(_device: Option<String>, format: AudioFormat) -> Self {
        info!("Using PipeWireSink with format: {format:?}");

        Self {
            format,
            producer: None,
            quit_flag: Arc::new(AtomicBool::new(false)),
            initialized: false,
            _main_loop_handle: None,
        }
    }
}

impl Sink for PipeWireSink {
    fn start(&mut self) -> SinkResult<()> {
        if self.initialized {
            return Ok(());
        }

        info!("Starting PipeWire sink...");

        let format = self.format;
        let quit_flag = Arc::clone(&self.quit_flag);

        // Create a lock-free ring buffer for real-time audio transfer
        let ring_buffer = HeapRb::<u8>::new(RING_BUFFER_SIZE);
        let (producer, consumer) = ring_buffer.split();

        // Store the producer for write_bytes
        self.producer = Some(producer);

        // Run PipeWire main loop in a separate thread with the consumer
        let handle = thread::spawn(move || {
            if let Err(e) = run_pipewire_loop(consumer, quit_flag, format) {
                error!("PipeWire loop error: {}", e);
            }
        });

        self._main_loop_handle = Some(handle);
        self.initialized = true;

        // Give the thread a moment to initialize
        thread::sleep(std::time::Duration::from_millis(100));

        info!("PipeWire sink started successfully");
        Ok(())
    }

    fn stop(&mut self) -> SinkResult<()> {
        if !self.initialized {
            return Ok(());
        }

        info!("Stopping PipeWire sink...");

        // Signal the thread to quit
        self.quit_flag.store(true, Ordering::Relaxed);

        // Drop the producer to signal the consumer thread to exit
        self.producer = None;

        // Wait for the thread to finish with a timeout
        if let Some(handle) = self._main_loop_handle.take() {
            // Give it a moment to exit gracefully
            thread::sleep(std::time::Duration::from_millis(100));
            let _ = handle.join();
        }

        // Reset the quit flag for potential restart
        self.quit_flag.store(false, Ordering::Relaxed);
        self.initialized = false;

        info!("PipeWire sink stopped");
        Ok(())
    }

    sink_as_bytes!();
}

impl SinkAsBytes for PipeWireSink {
    fn write_bytes(&mut self, data: &[u8]) -> SinkResult<()> {
        if !self.initialized {
            return Err(PipeWireError::NotConnected.into());
        }

        if let Some(ref mut producer) = self.producer {
            // Push data to the lock-free ring buffer in chunks
            // This is much more efficient than byte-by-byte and is wait-free
            let mut offset = 0;
            while offset < data.len() {
                let written = producer.push_slice(&data[offset..]);
                if written == 0 {
                    // Ring buffer is full, wait a tiny bit for consumer to catch up
                    thread::sleep(std::time::Duration::from_micros(100));
                } else {
                    offset += written;
                }
            }
            Ok(())
        } else {
            Err(PipeWireError::NotConnected.into())
        }
    }
}

impl Drop for PipeWireSink {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

impl PipeWireSink {
    pub const NAME: &'static str = "pipewire";
}

fn run_pipewire_loop(
    consumer: RingConsumer,
    quit_flag: Arc<AtomicBool>,
    format: AudioFormat,
) -> Result<(), PipeWireError> {
    // Initialize PipeWire
    pw::init();

    let mainloop = pw::main_loop::MainLoopRc::new(None)
        .map_err(|e| PipeWireError::MainLoopCreation(format!("{:?}", e)))?;

    let context = pw::context::ContextRc::new(&mainloop, None)
        .map_err(|e| PipeWireError::MainLoopCreation(format!("{:?}", e)))?;

    let core = context
        .connect_rc(None)
        .map_err(|e| PipeWireError::MainLoopCreation(format!("{:?}", e)))?;

    let stream = pw::stream::StreamBox::new(
        &core,
        "librespot-playback",
        properties! {
            *pw::keys::MEDIA_TYPE => "Audio",
            *pw::keys::MEDIA_ROLE => "Music",
            *pw::keys::MEDIA_CATEGORY => "Playback",
            *pw::keys::AUDIO_CHANNELS => NUM_CHANNELS.to_string().as_str(),
            *pw::keys::APP_NAME => "librespot",
        },
    )
    .map_err(|e| PipeWireError::StreamCreation(format!("{:?}", e)))?;

    let sample_size = calculate_sample_size(format);
    let stride = sample_size * NUM_CHANNELS as usize;

    // Clone mainloop for use in the listener callback
    let mainloop_quit = mainloop.clone();

    // Use PipeWire's real-time callback with lock-free ring buffer consumer
    // This ensures optimal performance and real-time safety
    let _listener = stream
        .add_local_listener_with_user_data((consumer, quit_flag.clone()))
        .process(move |stream, (consumer, quit_flag)| {
            // Check if we should quit
            if quit_flag.load(Ordering::Relaxed) {
                mainloop_quit.quit();
                return;
            }

            match stream.dequeue_buffer() {
                None => {
                    // No buffer available, this is normal
                }
                Some(mut buffer) => {
                    let datas = buffer.datas_mut();
                    let data = &mut datas[0];

                    let n_frames = if let Some(slice) = data.data() {
                        let n_frames = slice.len() / stride;
                        let total_bytes = n_frames * stride;

                        // Pop data from the lock-free ring buffer
                        // This is wait-free and real-time safe
                        let bytes_read = consumer.pop_slice(slice);

                        // Fill any remaining space with silence if underrun
                        if bytes_read < total_bytes {
                            slice[bytes_read..total_bytes].fill(0);
                        }

                        n_frames
                    } else {
                        0
                    };

                    // Configure the buffer chunk metadata
                    let chunk = data.chunk_mut();
                    *chunk.offset_mut() = 0;
                    *chunk.stride_mut() = stride as _;
                    *chunk.size_mut() = (stride * n_frames) as _;
                }
            }
        })
        .register()
        .map_err(|e| PipeWireError::StreamCreation(format!("{:?}", e)))?;

    // Setup audio format parameters - matches pipewire_tone_test.rs
    let mut audio_info = spa::param::audio::AudioInfoRaw::new();
    audio_info.set_format(convert_audio_format(format));
    audio_info.set_rate(SAMPLE_RATE);
    audio_info.set_channels(NUM_CHANNELS as u32);

    let mut position = [0; spa::param::audio::MAX_CHANNELS];
    position[0] = spa_sys::SPA_AUDIO_CHANNEL_FL;
    position[1] = spa_sys::SPA_AUDIO_CHANNEL_FR;
    audio_info.set_position(position);

    let values: Vec<u8> = pw::spa::pod::serialize::PodSerializer::serialize(
        std::io::Cursor::new(Vec::new()),
        &pw::spa::pod::Value::Object(pw::spa::pod::Object {
            type_: spa_sys::SPA_TYPE_OBJECT_Format,
            id: spa_sys::SPA_PARAM_EnumFormat,
            properties: audio_info.into(),
        }),
    )
    .map_err(|e| PipeWireError::StreamCreation(format!("{:?}", e)))?
    .0
    .into_inner();

    let mut params = [Pod::from_bytes(&values).unwrap()];

    // Connect stream - matches pipewire_tone_test.rs
    stream
        .connect(
            spa::utils::Direction::Output,
            None,
            pw::stream::StreamFlags::AUTOCONNECT
                | pw::stream::StreamFlags::MAP_BUFFERS
                | pw::stream::StreamFlags::RT_PROCESS,
            &mut params,
        )
        .map_err(|e| PipeWireError::StreamConnect(format!("{:?}", e)))?;

    // Run the main loop
    mainloop.run();

    Ok(())
}
