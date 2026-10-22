use crate::symphonia_util;
use librespot_core::{Error, SpotifyUri};
use std::{
    collections::{HashMap, HashSet},
    fs,
    fs::File,
    io,
    path::{Path, PathBuf},
    time::Duration,
};
use symphonia::core::{
    formats::FormatOptions,
    io::MediaSourceStream,
    meta::{MetadataOptions, StandardTagKey, Tag},
    probe::{Hint, ProbeResult},
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
        self.0.get(uri).map(PathBuf::as_path)
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
    // Keep traversal off the call stack and do not follow nested links. The
    // canonical set also handles directory aliases such as Windows junctions.
    let mut pending = vec![dir.to_path_buf()];
    let mut visited = HashSet::new();
    while let Some(directory) = pending.pop() {
        let directory = fs::canonicalize(directory)?;
        if !visited.insert(directory.clone()) {
            continue;
        }
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let kind = entry.file_type()?;
            let path = entry.path();
            if kind.is_symlink() {
                continue;
            }
            if kind.is_dir() {
                pending.push(path);
                continue;
            }
            if !kind.is_file() {
                continue;
            }
            let Some(file_extension) = path.extension().and_then(|e| e.to_str()) else {
                continue;
            };

            let lowercase_extension = file_extension.to_lowercase();

            if SUPPORTED_FILE_EXTENSIONS.contains(&lowercase_extension.as_str()) {
                let uri = match get_uri_from_file(path.as_path(), file_extension) {
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

fn get_uri_from_file(audio_path: &Path, file_extension: &str) -> Result<SpotifyUri, Error> {
    let src = File::open(audio_path)?;
    let mss = MediaSourceStream::new(Box::new(src), Default::default());

    let mut hint = Hint::new();
    hint.with_extension(file_extension);

    let meta_opts: MetadataOptions = Default::default();
    let fmt_opts: FormatOptions = Default::default();

    let mut probed = symphonia::default::get_probe()
        .format(&hint, mss, &fmt_opts, &meta_opts)
        .map_err(|_| Error::internal("Failed to probe file"))?;

    let mut artist: Option<String> = None;
    let mut album_title: Option<String> = None;
    let mut track_title: Option<String> = None;

    fn get_tags(probed: &mut ProbeResult) -> Option<Vec<Tag>> {
        let metadata = symphonia_util::get_latest_metadata(probed)?;
        let metadata_rev = metadata.current()?;
        Some(metadata_rev.tags().to_vec())
    }

    for tag in get_tags(&mut probed).ok_or(Error::internal("Failed to probe audio tags"))? {
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

    fn format_uri_part(input: Option<String>) -> String {
        input
            .map(|s| {
                let bytes = s.into_bytes();
                let encoded = form_urlencoded::byte_serialize(bytes.as_slice());
                encoded.collect::<String>()
            })
            .unwrap_or("".to_owned())
    }

    Ok(SpotifyUri::Local {
        artist: format_uri_part(artist),
        album_title: format_uri_part(album_title),
        track_title: format_uri_part(track_title),
        duration: Duration::from_secs(time.seconds),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directory_links_do_not_recurse_into_ancestors() {
        let root = std::env::temp_dir().join(format!(
            "librespot-lookup-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let child = root.join("child");
        fs::create_dir_all(&child).unwrap();
        #[cfg(unix)]
        let link = std::os::unix::fs::symlink(&root, child.join("ancestor"));
        #[cfg(windows)]
        let link = std::os::windows::fs::symlink_dir(&root, child.join("ancestor"));
        if let Err(error) = link {
            fs::remove_dir_all(&root).unwrap();
            if cfg!(windows) && error.kind() == io::ErrorKind::PermissionDenied {
                return;
            }
            panic!("{error}");
        }
        let mut lookup = LocalFileLookup::default();
        let result = visit_dir(&root, &mut lookup);
        fs::remove_dir_all(&root).unwrap();
        result.unwrap();
        assert!(lookup.0.is_empty());
    }
}
