use super::{Open, Sink, SinkAsBytes, SinkError, SinkResult};
use crate::config::AudioFormat;
use crate::convert::Converter;
use crate::decoder::AudioPacket;
use crate::{NUM_CHANNELS, SAMPLE_RATE};
use std::sync::mpsc::{sync_channel, SyncSender, Receiver};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use thiserror::Error;

use pipewire as pw;
use pw::{properties::properties, spa};
use spa::pod::Pod;
use libspa::sys as spa_sys;

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
    // Channel sender for audio data
    sender: Option<SyncSender<u8>>,
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
        S24 => spa::param::audio::AudioFormat::S24LE,
        S24_3 => spa::param::audio::AudioFormat::S24_32LE,
        S16 => spa::param::audio::AudioFormat::S16LE,
    }
}

impl Open for PipeWireSink {
    fn open(_device: Option<String>, format: AudioFormat) -> Self {
        info!("Using PipeWireSink with format: {format:?}");

        Self {
            format,
            sender: None,
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
        
        // Create a sync channel - buffer size for ~1 second of audio to prevent underruns
        let sample_size = calculate_sample_size(format);
        let buffer_size = SAMPLE_RATE as usize * NUM_CHANNELS as usize * sample_size;
        let (sender, receiver) = sync_channel::<u8>(buffer_size);
        
        // Store the sender for write_bytes
        self.sender = Some(sender);
        
        // Run PipeWire main loop in a separate thread with the receiver
        let handle = thread::spawn(move || {
            if let Err(e) = run_pipewire_loop(receiver, quit_flag, format) {
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
        
        // Drop the sender to signal the receiver thread to exit
        self.sender = None;

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

        if let Some(ref sender) = self.sender {
            // Send data through the channel - use blocking send to ensure all data is queued
            for &byte in data {
                sender.send(byte).map_err(|_| PipeWireError::NotConnected)?;
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
    receiver: Receiver<u8>,
    quit_flag: Arc<AtomicBool>,
    format: AudioFormat,
) -> Result<(), PipeWireError> {
    // Initialize PipeWire
    pw::init();
    
    let mainloop = pw::main_loop::MainLoopRc::new(None)
        .map_err(|e| PipeWireError::MainLoopCreation(format!("{:?}", e)))?;
    
    let context = pw::context::ContextRc::new(&mainloop, None)
        .map_err(|e| PipeWireError::MainLoopCreation(format!("{:?}", e)))?;
        
    let core = context.connect_rc(None)
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
    
    // This is the key: the listener is based on the working pipewire_tone_test.rs example
    let _listener = stream
        .add_local_listener_with_user_data((receiver, quit_flag.clone()))
        .process(move |stream, (receiver, quit_flag)| {
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
                        
                        // Read from channel - try non-blocking first, then blocking if needed
                        let mut bytes_read = 0;
                        
                        // First, try to read without blocking
                        for i in 0..total_bytes {
                            match receiver.try_recv() {
                                Ok(byte) => {
                                    slice[i] = byte;
                                    bytes_read += 1;
                                }
                                Err(_) => break,
                            }
                        }
                        
                        // If we didn't get enough data, block to get more
                        // This prevents underruns and stuttering
                        if bytes_read < total_bytes {
                            for i in bytes_read..total_bytes {
                                match receiver.recv() {
                                    Ok(byte) => {
                                        slice[i] = byte;
                                    }
                                    Err(_) => {
                                        // Channel disconnected, fill rest with silence and signal quit
                                        for j in i..total_bytes {
                                            slice[j] = 0;
                                        }
                                        quit_flag.store(true, Ordering::Relaxed);
                                        break;
                                    }
                                }
                            }
                        }
                        
                        n_frames
                    } else {
                        0
                    };
                    
                    // This matches the working pipewire_tone_test.rs example exactly
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
    stream.connect(
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
