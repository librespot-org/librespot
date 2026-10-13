//! Spotify's DJ talks around the tracks it plays. A DJ context carries the
//! script of each line in its tracks' metadata (`narration.intro.ssml` and
//! friends); Spotify synthesizes it on request, and the line plays before or
//! after its track, never over it.

use std::{collections::HashMap, io::Cursor, time::Duration};

use librespot_core::Session;
use symphonia::core::probe::Hint;

use crate::{
    config::PlayerConfig,
    decoder::{AudioDecoder, AudioPacket, AudioPacketPosition, DecoderResult, SymphoniaDecoder},
    player::db_to_ratio,
};

/// A track's `narration.*` metadata, as its context entry carries it.
pub type Narration = HashMap<String, String>;

/// Bounds synthesis plus download. The line is a nicety; the track must not
/// wait on it indefinitely.
const TIMEOUT: Duration = Duration::from_secs(8);

/// Spotify's loudness target, and the levels DJ declares for its own lines,
/// used when a line arrives without them.
const TARGET_LUFS: f64 = -14.0;
const DEFAULT_LOUDNESS: (f64, f64) = (-16.0, -3.0);

/// Wraps a track's decoder with its DJ lines, or returns it unchanged when it
/// has none. A line that fails to arrive is dropped: losing it is much smaller
/// than losing the music.
///
/// `track_factor` is the normalisation the player will apply to every packet,
/// lines included, so the lines are pre-scaled to land at their own level.
pub(crate) async fn narrate(
    session: &Session,
    config: &PlayerConfig,
    narration: &Narration,
    position_ms: u32,
    track_factor: f64,
    track: Box<dyn AudioDecoder + Send>,
) -> Box<dyn AudioDecoder + Send> {
    // Resuming mid-track skips the introduction; it belongs to the start.
    let intro = if position_ms == 0 {
        // The in-sequence intro, or the `jump` line when that is all there is.
        // Telling a skip to this track apart would need spirc to say how it got here.
        let kind = if has_script(narration, "intro") {
            "intro"
        } else {
            "jump"
        };
        line(session, config, narration, kind, track_factor).await
    } else {
        None
    };
    let outro = line(session, config, narration, "outro", track_factor).await;

    if intro.is_none() && outro.is_none() {
        return track;
    }
    Box::new(Narrated {
        intro,
        track,
        track_done: false,
        outro,
        end_ms: position_ms,
    })
}

fn has_script(narration: &Narration, kind: &str) -> bool {
    narration
        .get(&format!("narration.{kind}.ssml"))
        .is_some_and(|ssml| !ssml.is_empty())
}

async fn line(
    session: &Session,
    config: &PlayerConfig,
    narration: &Narration,
    kind: &str,
    track_factor: f64,
) -> Option<Line> {
    if !has_script(narration, kind) {
        return None;
    }
    let field = |key: &str| {
        narration
            .get(&format!("narration.{kind}.{key}"))
            .map(String::as_str)
            .unwrap_or_default()
    };

    let spclient = session.spclient();
    let fetch = async {
        let url = spclient
            .get_narration_url(field("ssml"), field("voice"), field("tts_provider"))
            .await?;
        spclient.request_url(&url).await
    };
    let mp3 = match tokio::time::timeout(TIMEOUT, fetch).await {
        Ok(Ok(mp3)) => mp3,
        Ok(Err(e)) => {
            warn!("DJ {kind} line unavailable, skipping it: {e}");
            return None;
        }
        Err(_) => {
            warn!(
                "DJ {kind} line took over {}s, skipping it",
                TIMEOUT.as_secs()
            );
            return None;
        }
    };

    let mut hint = Hint::new();
    hint.with_extension("mp3");
    let decoder = match SymphoniaDecoder::new(Cursor::new(mp3.to_vec()), hint) {
        Ok(decoder) => decoder,
        Err(e) => {
            warn!("DJ {kind} line undecodable, skipping it: {e}");
            return None;
        }
    };

    let loudness = field("loudness").parse::<f64>().ok();
    let true_peak = field("true_peak").parse::<f64>().ok();
    let (loudness, true_peak) = loudness.zip(true_peak).unwrap_or(DEFAULT_LOUDNESS);
    let gain = line_factor(config, loudness, true_peak) / track_factor;
    info!("DJ {kind} line ready, {} bytes, gain {gain:.2}", mp3.len());

    Some(Line {
        decoder: Box::new(decoder),
        gain,
    })
}

/// What a line needs to sit at the level normalised music plays at: Spotify's
/// target plus the listener's pregain, held below full scale at its peak.
/// Without normalisation the music plays as mastered and the line as rendered.
fn line_factor(config: &PlayerConfig, loudness: f64, true_peak: f64) -> f64 {
    if !config.normalisation {
        return 1.0;
    }
    let factor = db_to_ratio(TARGET_LUFS - loudness + config.normalisation_pregain_db);
    factor.min(db_to_ratio(-true_peak))
}

struct Line {
    decoder: Box<dyn AudioDecoder + Send>,
    gain: f64,
}

impl Line {
    /// The next packet of the line, or None once it ends or fails.
    fn next(&mut self) -> Option<AudioPacket> {
        match self.decoder.next_packet() {
            Ok(Some((_, AudioPacket::Samples(mut samples)))) => {
                samples.iter_mut().for_each(|s| *s *= self.gain);
                Some(AudioPacket::Samples(samples))
            }
            Ok(Some((_, packet))) => Some(packet),
            Ok(None) => None,
            Err(e) => {
                warn!("DJ line failed mid-play, dropping the rest: {e}");
                None
            }
        }
    }
}

/// Plays the introduction, then the track, then the closing remark. Position
/// stays the track's throughout: 0 while it is introduced, its end while the
/// DJ signs off.
struct Narrated {
    intro: Option<Line>,
    track: Box<dyn AudioDecoder + Send>,
    track_done: bool,
    outro: Option<Line>,
    end_ms: u32,
}

impl AudioDecoder for Narrated {
    /// Seeking means the listener wants the music, so an introduction still
    /// playing is abandoned. The closing remark stays: it follows the end.
    fn seek(&mut self, position_ms: u32) -> Result<u32, crate::decoder::DecoderError> {
        self.intro = None;
        self.track_done = false;
        self.track.seek(position_ms)
    }

    fn next_packet(&mut self) -> DecoderResult<Option<(AudioPacketPosition, AudioPacket)>> {
        let at = |position_ms| AudioPacketPosition {
            position_ms,
            skipped: false,
        };

        if let Some(intro) = &mut self.intro {
            match intro.next() {
                Some(packet) => return Ok(Some((at(self.end_ms), packet))),
                None => self.intro = None,
            }
        }
        if !self.track_done {
            match self.track.next_packet()? {
                Some((position, packet)) => {
                    self.end_ms = position.position_ms;
                    return Ok(Some((position, packet)));
                }
                None => self.track_done = true,
            }
        }
        if let Some(outro) = &mut self.outro {
            match outro.next() {
                Some(packet) => return Ok(Some((at(self.end_ms), packet))),
                None => self.outro = None,
            }
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decoder::DecoderError;

    /// A track of `n` one-sample packets, 10 ms apart.
    struct Fake {
        next: u32,
        n: u32,
    }

    impl AudioDecoder for Fake {
        fn seek(&mut self, position_ms: u32) -> Result<u32, DecoderError> {
            self.next = position_ms / 10;
            Ok(position_ms)
        }

        fn next_packet(&mut self) -> DecoderResult<Option<(AudioPacketPosition, AudioPacket)>> {
            if self.next >= self.n {
                return Ok(None);
            }
            self.next += 1;
            let position = AudioPacketPosition {
                position_ms: (self.next - 1) * 10,
                skipped: false,
            };
            Ok(Some((position, AudioPacket::Samples(vec![1.0]))))
        }
    }

    fn drain(decoder: &mut dyn AudioDecoder) -> Vec<u32> {
        std::iter::from_fn(|| decoder.next_packet().unwrap())
            .map(|(position, _)| position.position_ms)
            .collect()
    }

    #[test]
    fn plays_the_track_through_without_lines() {
        let mut narrated = Narrated {
            intro: None,
            track: Box::new(Fake { next: 0, n: 3 }),
            track_done: false,
            outro: None,
            end_ms: 0,
        };
        assert_eq!(drain(&mut narrated), [0, 10, 20]);
    }

    fn line(n: u32) -> Option<Line> {
        Some(Line {
            decoder: Box::new(Fake { next: 0, n }),
            gain: 0.5,
        })
    }

    #[test]
    fn the_dj_speaks_before_and_after_at_the_tracks_edges() {
        let mut narrated = Narrated {
            intro: line(2),
            track: Box::new(Fake { next: 0, n: 3 }),
            track_done: false,
            outro: line(2),
            end_ms: 0,
        };
        let packets: Vec<_> = std::iter::from_fn(|| narrated.next_packet().unwrap())
            .map(|(position, packet)| (position.position_ms, packet.samples().unwrap()[0]))
            .collect();
        assert_eq!(
            packets,
            [
                (0, 0.5),
                (0, 0.5),
                (0, 1.0),
                (10, 1.0),
                (20, 1.0),
                (20, 0.5),
                (20, 0.5)
            ]
        );
    }

    #[test]
    fn seeking_skips_the_intro_but_keeps_the_outro() {
        let mut narrated = Narrated {
            intro: line(5),
            track: Box::new(Fake { next: 0, n: 5 }),
            track_done: false,
            outro: line(1),
            end_ms: 0,
        };
        narrated.next_packet().unwrap();
        narrated.seek(30).unwrap();
        assert_eq!(drain(&mut narrated), [30, 40, 40]);
    }

    #[test]
    fn seeking_resumes_the_track() {
        let mut narrated = Narrated {
            intro: None,
            track: Box::new(Fake { next: 0, n: 5 }),
            track_done: false,
            outro: None,
            end_ms: 0,
        };
        assert_eq!(drain(&mut narrated), [0, 10, 20, 30, 40]);
        narrated.seek(20).unwrap();
        assert_eq!(drain(&mut narrated), [20, 30, 40]);
    }

    #[test]
    fn a_line_sits_at_the_target_below_full_scale() {
        let config = PlayerConfig {
            normalisation: true,
            normalisation_pregain_db: 0.0,
            ..PlayerConfig::default()
        };
        // -16 LUFS needs +2 dB to reach -14.
        assert!((line_factor(&config, -16.0, -3.0) - db_to_ratio(2.0)).abs() < 1e-9);
        // A peak at -1 dBTP caps the gain at +1 dB.
        assert!((line_factor(&config, -20.0, -1.0) - db_to_ratio(1.0)).abs() < 1e-9);
        let off = PlayerConfig {
            normalisation: false,
            ..PlayerConfig::default()
        };
        assert_eq!(line_factor(&off, -16.0, -3.0), 1.0);
    }
}
