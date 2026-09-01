use id3::TagLike;
use log::info;
use std::{
    env,
    fs::OpenOptions,
    io::{self, Seek, SeekFrom, Write},
    process::exit,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};

use rand::Rng;

use librespot::{
    core::{
        authentication::Credentials, config::SessionConfig, session::Session,
        spotify_uri::SpotifyUri,
    },
    metadata::{Metadata, Playlist, Track},
    playback::{
        NUM_CHANNELS, SAMPLE_RATE,
        audio_backend::{Sink, SinkAsBytes, SinkError, SinkResult},
        config::{AudioFormat, PlayerConfig},
        convert::Converter,
        decoder::AudioPacket,
        mixer::NoOpVolume,
        player::{Player, PlayerEvent},
    },
};

/// Metadata for a single track that we will write to its own WAV file.
struct TrackTarget {
    /// Destination `.wav` file path.
    path: String,
    /// Track / song title.
    title: String,
    /// Artist name(s), joined.
    artist: String,
    /// Album name.
    album: String,
    /// Decoded album artwork (JPEG), if available.
    album_art: Option<Vec<u8>>,
}

/// A sink that writes the decoded audio stream to one RIFF/WAV file per track.
///
/// The sink is created once when the `Player` is built (the player keeps a single
/// persistent sink across the whole playlist). To produce a separate file for every
/// track we share the decoded metadata and a `current` track index with the main
/// task: whenever playback moves to the next track the index changes, and the sink
/// finalises the previous file and opens a new one for the track.
struct WavSink {
    tracks: Arc<Vec<TrackTarget>>,
    current: Arc<AtomicUsize>,
    format: AudioFormat,
    file: Option<io::BufWriter<std::fs::File>>,
    data_bytes: u64,
    open_idx: Option<usize>,
}

impl WavSink {
    fn new(tracks: Arc<Vec<TrackTarget>>, current: Arc<AtomicUsize>, format: AudioFormat) -> Self {
        Self {
            tracks,
            current,
            format,
            file: None,
            data_bytes: 0,
            open_idx: None,
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

    /// Write the RIFF/WAV header, close the file and attach ID3v2 metadata + artwork.
    fn close_track(&mut self) -> io::Result<()> {
        if let Some(idx) = self.open_idx.take() {
            let path = self.tracks[idx].path.clone();

            self.write_header()?;
            if let Some(mut file) = self.file.take() {
                file.flush()?;
            }
            info!(
                "Finished writing {} bytes of audio to {}",
                self.data_bytes, path
            );

            write_id3_tag(&self.tracks[idx], &path);
        }

        self.data_bytes = 0;
        Ok(())
    }

    /// Make sure the file for the track at `idx` is open. If another track's file
    /// is currently open, it is finalised first.
    fn ensure_track_open(&mut self, idx: usize) -> SinkResult<()> {
        if self.open_idx == Some(idx) {
            return Ok(());
        }

        self.close_track().map_err(|e| SinkError::OnWrite(e.to_string()))?;

        let path = self.tracks[idx].path.clone();
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)
            .map_err(|e| SinkError::ConnectionRefused(format!("{path}: {e}")))?;
        self.file = Some(io::BufWriter::new(file));
        self.open_idx = Some(idx);
        self.data_bytes = 0;
        // Reserve space for the 44-byte RIFF/WAV header.
        self.file
            .as_mut()
            .unwrap()
            .write_all(&[0u8; 44])
            .map_err(|e| SinkError::OnWrite(e.to_string()))?;
        info!("Recording WAV to {path} ({})", self.tracks[idx].title);

        Ok(())
    }
}

impl Sink for WavSink {
    fn start(&mut self) -> SinkResult<()> {
        self.file = None;
        self.data_bytes = 0;
        self.open_idx = None;
        Ok(())
    }

    fn stop(&mut self) -> SinkResult<()> {
        self.close_track()
            .map_err(|e| SinkError::OnWrite(e.to_string()))
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

        // The current track index is updated by the main task on `TrackChanged`.
        let idx = self.current.load(Ordering::SeqCst);
        let idx = idx.min(self.tracks.len().saturating_sub(1));
        self.ensure_track_open(idx)?;

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

/// Attach ID3v2 title/artist/album tags and, when available, the album artwork to a
/// finished `.wav` file using the `id3` crate.
fn write_id3_tag(meta: &TrackTarget, path: &str) {
    let mut tag = id3::Tag::new();
    tag.set_title(meta.title.clone());
    tag.set_artist(meta.artist.clone());
    tag.set_album(meta.album.clone());

    if let Some(art) = &meta.album_art {
        tag.add_frame(id3::frame::Picture {
            mime_type: "image/jpeg".to_string(),
            picture_type: id3::frame::PictureType::CoverFront,
            description: String::new(),
            data: art.clone(),
        });
    }

    if let Err(e) = tag.write_to_path(path, id3::Version::Id3v24) {
        eprintln!("Failed to write ID3 metadata to {path}: {e}");
        return;
    }
    info!("Wrote ID3 metadata to {path}");
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

/// Preload the metadata (title, artist, album) and album artwork bytes for every
/// track in the playlist so nothing has to be fetched while streaming.
async fn load_track_targets(session: &Session, tracks: &[SpotifyUri]) -> Vec<TrackTarget> {
    let mut out = Vec::with_capacity(tracks.len());
    for (i, uri) in tracks.iter().enumerate() {
        let (title, artist, album, album_art) = match Track::get(session, uri).await {
            Ok(track) => {
                let title = track.name;
                let artist = track
                    .artists
                    .iter()
                    .map(|a| a.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                let album = track.album.name.clone();

                // Pick the largest available cover for the best-quality artwork.
                let album_art = match track.album.covers.iter().max_by_key(|c| c.width * c.height)
                {
                    Some(cover) => match session.spclient().get_image(&cover.id).await {
                        Ok(bytes) => Some(bytes.to_vec()),
                        Err(e) => {
                            eprintln!("Failed to fetch artwork for {title}: {e}");
                            None
                        }
                    },
                    None => None,
                };

                (title, artist, album, album_art)
            }
            Err(e) => {
                eprintln!("Failed to fetch metadata for track {i}: {e}");
                (format!("Track {}", i + 1), String::new(), String::new(), None)
            }
        };

        let artist_title = format!("{artist} - {title}");
        let filename = format!(
            "{:02} - {}.wav",
            i + 1,
            sanitize_filename(&artist_title)
        );

        out.push(TrackTarget {
            path: filename,
            title,
            artist,
            album,
            album_art,
        });
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

    println!(
        "Backing up playlist \"{}\" ({} tracks)",
        plist.name(),
        tracks.len()
    );
    println!("Fetching metadata and artwork for each track...");
    let targets = Arc::new(load_track_targets(&session, &tracks).await);

    let current = Arc::new(AtomicUsize::new(0));

    let sink_targets = targets.clone();
    let sink_current = current.clone();
    let player = Player::new(player_config, session, Box::new(NoOpVolume), move || {
        Box::new(WavSink::new(sink_targets, sink_current, audio_format))
    });

    player.load(tracks[0].clone(), true, 0);

    let mut events = player.get_player_event_channel();
    let mut done = false;
    // Number of times we've retried loading the same track before giving up.
    const MAX_RETRIES: u32 = 3;
    // Identifies the track currently being retried, so consecutive failures for the
    // same track count together (and reset once the track changes or a load succeeds).
    let mut retry_track: Option<SpotifyUri> = None;
    let mut retry_count: u32 = 0;
    while let Some(event) = events.recv().await {
        match event {
            PlayerEvent::TimeToPreloadNextTrack { .. } => {
                if let Some(next) = tracks.get(current.load(Ordering::SeqCst) + 1) {
                    player.preload(next.clone());
                }
            }
            PlayerEvent::TrackChanged { audio_item } => {
                if let Some(idx) = tracks.iter().position(|t| t == &audio_item.track_id) {
                    current.store(idx.min(targets.len().saturating_sub(1)), Ordering::SeqCst);
                }
                // A track started playing, so any prior retry state is obsolete.
                retry_track = None;
                retry_count = 0;
                println!("Now playing: {}", audio_item.name);
            }
            PlayerEvent::EndOfTrack { .. } => {
                // A track finished successfully, so reset the retry state.
                retry_track = None;
                retry_count = 0;
                if let Some(next) = tracks.get(current.load(Ordering::SeqCst) + 1) {
                    player.load(next.clone(), true, 0);
                } else {
                    println!("Done");
                    done = true;
                    break;
                }
            }
            PlayerEvent::Unavailable { track_id, .. } => {
                // Retry loading the same track up to MAX_RETRIES times with a random
                // 1-3 second delay before giving up and skipping to the next track.
                let same_track = retry_track.as_ref() == Some(&track_id);
                retry_track = Some(track_id.clone());
                retry_count = if same_track { retry_count + 1 } else { 1 };

                if retry_count <= MAX_RETRIES {
                    let delay =
                        Duration::from_millis(rand::rng().random_range(1000..=3000));
                    eprintln!(
                        "Track unavailable: {track_id} (attempt {retry_count}/{MAX_RETRIES}), retrying in {} ms",
                        delay.as_millis()
                    );
                    tokio::time::sleep(delay).await;
                    player.load(track_id.clone(), true, 0);
                    continue;
                }

                eprintln!(
                    "Track unavailable: {track_id} (gave up after {MAX_RETRIES} attempts), skipping"
                );
                retry_track = None;
                retry_count = 0;
                let next_idx = tracks
                    .iter()
                    .position(|t| t == &track_id)
                    .map(|idx| idx + 1)
                    .unwrap_or_else(|| current.load(Ordering::SeqCst) + 1);
                if let Some(next) = tracks.get(next_idx) {
                    player.load(next.clone(), true, 0);
                } else {
                    println!("Done");
                    done = true;
                    break;
                }
            }
            _ => (),
        }
    }

    if done {
        // Stop the player so the sink's `stop()` runs and the final WAV header is written.
        player.stop();
        let mut channel = player.get_player_event_channel();
        while let Some(event) = channel.recv().await {
            if matches!(event, PlayerEvent::Stopped { .. }) {
                break;
            }
        }
    }
}
