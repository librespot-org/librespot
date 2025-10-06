use librespot_core::{Error, SpotifyUri};
use regex::{Captures, Regex};
use std::sync::LazyLock;
use std::{
    collections::HashMap,
    fs,
    fs::File,
    io,
    path::{Path, PathBuf},
    time::Duration,
};
use symphonia::{
    core::formats::FormatOptions,
    core::io::MediaSourceStream,
    core::meta::{MetadataOptions, StandardTagKey, Tag},
    core::probe::{Hint, ProbeResult},
};

// "Spotify supports .mp3, .mp4, and .m4p files. It doesn’t support .mp4 files that contain video,
// or the iTunes lossless format (M4A)."
// https://community.spotify.com/t5/FAQs/Local-Files/ta-p/5186118
//
// There are some indications online that FLAC is supported, so check for this as well.
const SUPPORTED_FILE_EXTENSIONS: &[&str; 4] = &["mp3", "mp4", "m4p", "flac"];

#[derive(Default)]
pub struct LocalFileLookup(HashMap<SpotifyUri, PathBuf>);

impl LocalFileLookup {
    pub fn get(&self, uri: &SpotifyUri) -> Option<&Path> {
        self.0.get(uri).map(|p| p.as_path())
    }
}

pub fn create_local_file_lookup(directories: &[PathBuf]) -> LocalFileLookup {
    let mut lookup = LocalFileLookup(HashMap::new());

    for path in directories {
        if !path.is_dir() {
            warn!(
                "Ignoring local file source {}: not a directory",
                path.display()
            );
            continue;
        }

        if let Err(e) = visit_dir(path, &mut lookup) {
            warn!(
                "Failed to load entries from local file source {}: {}",
                path.display(),
                e
            );
        }
    }

    lookup
}

fn visit_dir(dir: &Path, accumulator: &mut LocalFileLookup) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            visit_dir(&path, accumulator)?;
        } else {
            let Some(extension) = path.extension().and_then(|e| e.to_str()) else {
                continue;
            };

            let lowercase_extension = extension.to_lowercase();

            if SUPPORTED_FILE_EXTENSIONS.contains(&lowercase_extension.as_str()) {
                let uri = match get_uri_from_file(path.as_path(), extension) {
                    Ok(uri) => uri,
                    Err(e) => {
                        warn!(
                            "Failed to determine URI of local file {}: {}",
                            path.display(),
                            e
                        );
                        continue;
                    }
                };

                accumulator.0.insert(uri, path);
            }
        }
    }

    Ok(())
}

fn get_uri_from_file(audio_path: &Path, extension: &str) -> Result<SpotifyUri, Error> {
    let src = File::open(audio_path)?;
    let mss = MediaSourceStream::new(Box::new(src), Default::default());

    let mut hint = Hint::new();
    hint.with_extension(extension);

    let meta_opts: MetadataOptions = Default::default();
    let fmt_opts: FormatOptions = Default::default();

    let mut probed = symphonia::default::get_probe()
        .format(&hint, mss, &fmt_opts, &meta_opts)
        .map_err(|_| Error::internal("Failed to probe file"))?;

    let mut artist: Option<String> = None;
    let mut album_title: Option<String> = None;
    let mut track_title: Option<String> = None;

    let tags = get_tags(&mut probed).ok_or(Error::internal("Failed to probe audio tags"))?;

    for tag in tags {
        if let Some(std_key) = tag.std_key {
            match std_key {
                StandardTagKey::Album => {
                    album_title.replace(tag.value.to_string());
                }
                StandardTagKey::Artist => {
                    artist.replace(tag.value.to_string());
                }
                StandardTagKey::TrackTitle => {
                    track_title.replace(tag.value.to_string());
                }
                _ => {
                    continue;
                }
            }
        }
    }

    let first_track = probed
        .format
        .default_track()
        .ok_or(Error::internal("Failed to find an audio track"))?;

    let time_base = first_track
        .codec_params
        .time_base
        .ok_or(Error::internal("Failed to calculate track duration"))?;

    let num_frames = first_track
        .codec_params
        .n_frames
        .ok_or(Error::internal("Failed to calculate track duration"))?;

    let time = time_base.calc_time(num_frames);

    fn url_encode(input: &str) -> String {
        static ENCODE_REGEX: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"[#$&'()*+,/:;=?@\[\]\s]").unwrap());

        ENCODE_REGEX
            .replace_all(input, |caps: &Captures| match &caps[0] {
                " " => "+".to_owned(),
                _ => format!("%{:X}", &caps[0].as_bytes()[0]),
            })
            .into_owned()
    }

    fn format_uri_part(input: Option<String>) -> String {
        input.as_deref().map(url_encode).unwrap_or("".to_owned())
    }

    Ok(SpotifyUri::Local {
        artist: format_uri_part(artist),
        album_title: format_uri_part(album_title),
        track_title: format_uri_part(track_title),
        duration: Duration::from_secs(time.seconds),
    })
}

fn get_tags(probed: &mut ProbeResult) -> Option<Vec<Tag>> {
    if let Some(metadata_rev) = probed.format.metadata().current() {
        return Some(metadata_rev.tags().to_vec());
    }

    if let Some(metadata_rev) = probed.metadata.get().as_ref().and_then(|m| m.current()) {
        return Some(metadata_rev.tags().to_vec());
    }

    None
}
