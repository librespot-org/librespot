use log::info;
use std::{
    env,
    fs::OpenOptions,
    io::{self, Seek, SeekFrom, Write},
    process::exit,
};

use librespot::{
    core::{
        authentication::Credentials, config::SessionConfig, session::Session,
        spotify_uri::SpotifyUri,
    },
    metadata::{Metadata, Playlist},
    playback::{
        NUM_CHANNELS, SAMPLE_RATE,
        audio_backend::{Open, Sink, SinkAsBytes, SinkError, SinkResult},
        config::{AudioFormat, PlayerConfig},
        convert::Converter,
        decoder::AudioPacket,
        mixer::NoOpVolume,
        player::{Player, PlayerEvent},
    },
};

/// A sink that writes the decoded audio stream to a RIFF/WAV file.
struct WavSink {
    file: Option<io::BufWriter<std::fs::File>>,
    path: String,
    format: AudioFormat,
    data_bytes: u64,
}

impl WavSink {
    fn new(path: String, format: AudioFormat) -> Self {
        Self {
            file: None,
            path,
            format,
            data_bytes: 0,
        }
    }

    fn bits_per_sample(&self) -> u16 {
        match self.format {
            AudioFormat::F64 => 64,
            AudioFormat::F32 => 32,
            AudioFormat::S32 => 32,
            AudioFormat::S24 | AudioFormat::S24_3 => 24,
            AudioFormat::S16 => 16,
        }
    }

    /// Actual number of bytes written per sample (matches `write`).
    fn bytes_per_sample(&self) -> u32 {
        match self.format {
            AudioFormat::F64 => 8,
            AudioFormat::F32 | AudioFormat::S32 | AudioFormat::S24 => 4,
            AudioFormat::S24_3 => 3,
            AudioFormat::S16 => 2,
        }
    }

    fn bytes_per_second(&self) -> u32 {
        SAMPLE_RATE * NUM_CHANNELS as u32 * self.bytes_per_sample()
    }

    fn write_header(&mut self) -> io::Result<()> {
        let bps = self.bits_per_sample();
        let bytes_per_sample = self.bytes_per_sample();
        let bytes_per_sample_total = bytes_per_sample * NUM_CHANNELS as u32;
        let data_len = self.data_bytes as u32;
        let bytes_per_second = self.bytes_per_second();

        let file = self
            .file
            .as_mut()
            .ok_or_else(|| io::Error::other("WAV file not open"))?;

        file.seek(SeekFrom::Start(0))?;
        file.write_all(b"RIFF")?;
        file.write_all(&(36 + data_len).to_le_bytes())?;
        file.write_all(b"WAVE")?;
        file.write_all(b"fmt ")?;
        file.write_all(&16u32.to_le_bytes())?; // fmt chunk size
        file.write_all(&1u16.to_le_bytes())?; // PCM
        file.write_all(&(NUM_CHANNELS as u16).to_le_bytes())?;
        file.write_all(&SAMPLE_RATE.to_le_bytes())?;
        file.write_all(&bytes_per_second.to_le_bytes())?;
        file.write_all(&(bytes_per_sample_total as u16).to_le_bytes())?; // block align
        file.write_all(&bps.to_le_bytes())?; // bits per sample
        file.write_all(b"data")?;
        file.write_all(&data_len.to_le_bytes())?;

        Ok(())
    }
}

impl Open for WavSink {
    fn open(device: Option<String>, format: AudioFormat) -> Self {
        // The SinkBuilder interface only passes a device string; we repurpose it as the
        // destination WAV file path (see the `Player::new` closure in `main`).
        let path = device.unwrap_or_else(|| "output.wav".to_string());
        Self::new(path, format)
    }
}

impl Sink for WavSink {
    fn start(&mut self) -> SinkResult<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.path)
            .map_err(|e| SinkError::ConnectionRefused(format!("{}: {e}", self.path)))?;
        self.file = Some(io::BufWriter::new(file));
        self.data_bytes = 0;
        // Reserve space for the 44-byte RIFF/WAV header.
        self.file
            .as_mut()
            .unwrap()
            .write_all(&[0u8; 44])
            .map_err(|e| SinkError::OnWrite(e.to_string()))?;
        info!("Recording WAV to {}", self.path);
        Ok(())
    }

    fn stop(&mut self) -> SinkResult<()> {
        self.write_header()
            .map_err(|e| SinkError::OnWrite(e.to_string()))?;
        if let Some(mut file) = self.file.take() {
            file.flush()
                .map_err(|e| SinkError::OnWrite(e.to_string()))?;
        }
        info!("Finished writing {} bytes of audio to {}", self.data_bytes, self.path);
        Ok(())
    }

    fn write(&mut self, packet: AudioPacket, converter: &mut Converter) -> SinkResult<()> {
        fn as_bytes<T: Sized>(slice: &[T]) -> &[u8] {
            unsafe {
                std::slice::from_raw_parts(
                    slice.as_ptr() as *const u8,
                    std::mem::size_of_val(slice),
                )
            }
        }

        match packet {
            AudioPacket::Samples(samples) => match self.format {
                AudioFormat::F64 => self.write_bytes(as_bytes(&samples)),
                AudioFormat::F32 => {
                    let s: &[f32] = &converter.f64_to_f32(&samples);
                    self.write_bytes(as_bytes(s))
                }
                AudioFormat::S32 => {
                    let s: &[i32] = &converter.f64_to_s32(&samples);
                    self.write_bytes(as_bytes(s))
                }
                AudioFormat::S24 => {
                    let s: &[i32] = &converter.f64_to_s24(&samples);
                    self.write_bytes(as_bytes(s))
                }
                AudioFormat::S24_3 => {
                    use librespot::playback::convert::i24;
                    let s: Vec<i24> = converter.f64_to_s24_3(&samples);
                    self.write_bytes(as_bytes(&s))
                }
                AudioFormat::S16 => {
                    let s: &[i16] = &converter.f64_to_s16(&samples);
                    self.write_bytes(as_bytes(s))
                }
            },
            AudioPacket::Raw(samples) => self.write_bytes(&samples),
        }
    }
}

impl SinkAsBytes for WavSink {
    fn write_bytes(&mut self, data: &[u8]) -> SinkResult<()> {
        let file = self
            .file
            .as_mut()
            .ok_or_else(|| SinkError::NotConnected("WAV file not open".to_string()))?;
        file.write_all(data)
            .map_err(|e| SinkError::OnWrite(e.to_string()))?;
        self.data_bytes += data.len() as u64;
        Ok(())
    }
}

fn sanitize_filename(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    for c in name.chars() {
        match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => out.push('_'),
            _ => out.push(c),
        }
    }
    out
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let session_config = SessionConfig::default();
    let player_config = PlayerConfig::default();
    let audio_format = AudioFormat::default();

    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} ACCESS_TOKEN PLAYLIST", args[0]);
        return;
    }
    let credentials = Credentials::with_access_token(&args[1]);

    let plist_uri = SpotifyUri::from_uri(&args[2]).unwrap_or_else(|_| {
        eprintln!(
            "PLAYLIST should be a playlist URI such as: \
                \"spotify:playlist:37i9dQZF1DXec50AjHrNTq\""
        );
        exit(1);
    });

    println!("Connecting...");
    let session = Session::new(session_config, None);
    if let Err(e) = session.connect(credentials, false).await {
        println!("Error connecting: {e}");
        exit(1);
    }

    let plist = Playlist::get(&session, &plist_uri).await.unwrap();
    let tracks: Vec<SpotifyUri> = plist.tracks().cloned().collect();
    if tracks.is_empty() {
        eprintln!("Playlist \"{}\" has no tracks", plist.name());
        exit(1);
    }

    let output_path = format!("{}.wav", sanitize_filename(plist.name()));
    println!(
        "Backing up playlist \"{}\" ({} tracks) to {}",
        plist.name(),
        tracks.len(),
        output_path
    );

    let filename = output_path.clone();
    let player = Player::new(player_config, session, Box::new(NoOpVolume), move || {
        Box::new(WavSink::open(Some(filename), audio_format))
    });

    let mut current = 0usize;
    player.load(tracks[current].clone(), true, 0);

    let mut events = player.get_player_event_channel();
    let mut done = false;
    while let Some(event) = events.recv().await {
        match event {
            PlayerEvent::TimeToPreloadNextTrack { .. } => {
                if let Some(next) = tracks.get(current + 1) {
                    player.preload(next.clone());
                }
            }
            PlayerEvent::EndOfTrack { .. } => {
                if let Some(next) = tracks.get(current + 1) {
                    current += 1;
                    player.load(next.clone(), true, 0);
                } else {
                    println!("Done");
                    done = true;
                    break;
                }
            }
            PlayerEvent::Unavailable { track_id, .. } => {
                eprintln!("Track unavailable: {track_id}");
                if let Some(idx) = tracks.iter().position(|t| *t == track_id) {
                    if idx >= current {
                        current = idx + 1;
                    }
                } else {
                    current += 1;
                }
                if let Some(next) = tracks.get(current) {
                    player.load(next.clone(), true, 0);
                } else {
                    println!("Done");
                    done = true;
                    break;
                }
            }
            PlayerEvent::TrackChanged { audio_item } => {
                println!("Now playing: {}", audio_item.name);
            }
            _ => (),
        }
    }

    if done {
        // Stop the player so the sink's `stop()` runs and the WAV header is written.
        player.stop();
        let mut channel = player.get_player_event_channel();
        while let Some(event) = channel.recv().await {
            if matches!(event, PlayerEvent::Stopped { .. }) {
                break;
            }
        }
    }
}
