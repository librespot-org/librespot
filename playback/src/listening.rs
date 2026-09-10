use std::time::{Duration, SystemTime};

use librespot_core::{
    Error, FileId, Session, SpotifyUri,
    listening::{EndReason, ListeningReporter, PlaybackReport},
};
use librespot_metadata::audio::AudioFileFormat;
use tokio::sync::{mpsc, oneshot};

pub(crate) enum ReportCommand {
    Report(u64, Session, Box<PlaybackReport>),
    Flush(oneshot::Sender<Result<(), Error>>),
}

pub(crate) struct PendingFlush {
    pub sender: mpsc::Sender<ReportCommand>,
    pub final_report: Option<ReportCommand>,
    pub previous_error: Option<Error>,
}

impl PendingFlush {
    pub async fn flush(self) -> Result<(), Error> {
        // Wait for queue capacity on the caller's runtime, never the audio thread.
        if let Some(report) = self.final_report {
            self.sender.send(report).await?;
        }
        let (done, completed) = oneshot::channel();
        self.sender.send(ReportCommand::Flush(done)).await?;
        completed.await??;
        self.previous_error.map_or(Ok(()), Err)
    }
}

trait ReportSink {
    fn report(
        &mut self,
        generation: u64,
        session: Session,
        report: PlaybackReport,
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send;
}

struct SessionReporter {
    generation: u64,
    reporter: ListeningReporter,
}

impl ReportSink for SessionReporter {
    async fn report(
        &mut self,
        generation: u64,
        session: Session,
        report: PlaybackReport,
    ) -> Result<(), Error> {
        if generation != self.generation {
            self.generation = generation;
            self.reporter = ListeningReporter::new(session);
        }
        tokio::time::timeout(Duration::from_secs(60), self.reporter.report(&report))
            .await
            .map_err(|_| Error::deadline_exceeded("Listening report timed out"))??;
        debug!(
            "Reported completed listening for {} ({} ms)",
            report.uri,
            report.played.as_millis()
        );
        Ok(())
    }
}

async fn run_reports(mut receiver: mpsc::Receiver<ReportCommand>, mut reporter: impl ReportSink) {
    let mut pending_error = None;
    while let Some(command) = receiver.recv().await {
        match command {
            ReportCommand::Report(generation, session, playback) => {
                if let Err(error) = reporter.report(generation, session, *playback).await {
                    warn!("Unable to report listening: {error}");
                    pending_error = Some(error);
                }
            }
            ReportCommand::Flush(done) => {
                let _ = done.send(pending_error.take().map_or(Ok(()), Err));
            }
        }
    }
}

pub(crate) fn reporter(session: Session) -> mpsc::Sender<ReportCommand> {
    let (sender, receiver) = mpsc::channel(32);
    let reporter = SessionReporter {
        generation: 0,
        reporter: ListeningReporter::new(session.clone()),
    };
    session.spawn(run_reports(receiver, reporter));
    sender
}

#[derive(Clone, Copy)]
pub(crate) struct LoadedAudioFile {
    pub id: FileId,
    pub format: AudioFileFormat,
}

pub(crate) struct PlaybackStatistics {
    uri: SpotifyUri,
    file: LoadedAudioFile,
    context_uri: Option<String>,
    started_at: Option<SystemTime>,
    samples: u64,
}

impl PlaybackStatistics {
    pub fn new(uri: SpotifyUri, file: LoadedAudioFile, context_uri: Option<String>) -> Self {
        Self {
            uri,
            file,
            context_uri,
            started_at: None,
            samples: 0,
        }
    }

    pub fn restart(&self) -> Self {
        Self::new(self.uri.clone(), self.file, self.context_uri.clone())
    }

    pub fn written(&mut self, samples: usize, at: SystemTime) {
        if samples > 0 {
            self.started_at.get_or_insert(at);
            self.samples = self.samples.saturating_add(samples as u64);
        }
    }

    pub fn finish(self, reason: EndReason, at: SystemTime) -> Option<PlaybackReport> {
        let started_at = self.started_at?;
        let format = match self.file.format {
            AudioFileFormat::OGG_VORBIS_96 => "Vorbis 96 kbps",
            AudioFileFormat::OGG_VORBIS_160 => "Vorbis 160 kbps",
            AudioFileFormat::OGG_VORBIS_320 => "Vorbis 320 kbps",
            AudioFileFormat::MP3_96 => "MP3 96 kbps",
            AudioFileFormat::MP3_160 => "MP3 160 kbps",
            AudioFileFormat::MP3_256 => "MP3 256 kbps",
            AudioFileFormat::MP3_320 => "MP3 320 kbps",
            _ => return None,
        };
        Some(PlaybackReport {
            uri: self.uri,
            file_id: self.file.id,
            context_uri: self.context_uri,
            audio_format: format.into(),
            started_at,
            ended_at: at,
            played: Duration::from_millis(
                self.samples.saturating_mul(1000) / crate::SAMPLES_PER_SECOND as u64,
            ),
            reason,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SAMPLES_PER_SECOND;

    fn loaded() -> PlaybackStatistics {
        PlaybackStatistics::new(
            SpotifyUri::from_uri("spotify:track:72AZ3V52rs9NfgMNhALxln").unwrap(),
            LoadedAudioFile {
                id: FileId([1; 20]),
                format: AudioFileFormat::OGG_VORBIS_320,
            },
            None,
        )
    }

    #[tokio::test]
    async fn flush_waits_for_the_pending_report() {
        let (sender, receiver) = mpsc::channel(4);
        let (release, released) = oneshot::channel();
        let (done, mut completed) = oneshot::channel();
        let mut stats = loaded();
        stats.written(SAMPLES_PER_SECOND as usize, SystemTime::now());
        sender
            .send(ReportCommand::Report(
                0,
                Session::new(Default::default(), None),
                Box::new(stats.finish(EndReason::EndPlay, SystemTime::now()).unwrap()),
            ))
            .await
            .unwrap();
        sender.send(ReportCommand::Flush(done)).await.unwrap();
        struct DelayedReport(Option<oneshot::Receiver<()>>);
        impl ReportSink for DelayedReport {
            async fn report(&mut self, _: u64, _: Session, _: PlaybackReport) -> Result<(), Error> {
                self.0.take().unwrap().await.unwrap();
                Ok(())
            }
        }
        let worker = tokio::spawn(run_reports(receiver, DelayedReport(Some(released))));
        tokio::task::yield_now().await;
        assert!(matches!(
            completed.try_recv(),
            Err(oneshot::error::TryRecvError::Empty)
        ));
        release.send(()).unwrap();
        completed.await.unwrap().unwrap();
        drop(sender);
        worker.await.unwrap();
    }

    #[tokio::test]
    async fn saturated_queue_flush_preserves_the_final_listen() {
        let (sender, mut receiver) = mpsc::channel(1);
        let make_report = || {
            let mut stats = loaded();
            stats.written(SAMPLES_PER_SECOND as usize, SystemTime::now());
            ReportCommand::Report(
                0,
                Session::new(Default::default(), None),
                Box::new(stats.finish(EndReason::EndPlay, SystemTime::now()).unwrap()),
            )
        };
        sender.send(make_report()).await.unwrap();
        let pending = PendingFlush {
            sender,
            final_report: Some(make_report()),
            previous_error: None,
        };
        let flush = pending.flush();
        tokio::pin!(flush);
        assert!(
            tokio::time::timeout(Duration::from_millis(10), &mut flush)
                .await
                .is_err()
        );
        assert!(matches!(
            receiver.recv().await,
            Some(ReportCommand::Report(..))
        ));
        let drain = async {
            assert!(matches!(
                receiver.recv().await,
                Some(ReportCommand::Report(..))
            ));
            let Some(ReportCommand::Flush(done)) = receiver.recv().await else {
                panic!("missing flush barrier")
            };
            done.send(Ok(())).unwrap();
        };
        let (result, ()) = tokio::join!(flush, drain);
        result.unwrap();
    }

    #[test]
    fn session_replacement_separates_delivered_audio() {
        let mut original = loaded();
        let now = SystemTime::now();
        original.written(SAMPLES_PER_SECOND as usize, now);
        let mut replacement = original.restart();
        assert_eq!(
            original.finish(EndReason::EndPlay, now).unwrap().played,
            Duration::from_secs(1)
        );
        assert!(
            replacement
                .restart()
                .finish(EndReason::EndPlay, now)
                .is_none()
        );
        replacement.written(SAMPLES_PER_SECOND as usize * 2, now);
        assert_eq!(
            replacement.finish(EndReason::EndPlay, now).unwrap().played,
            Duration::from_secs(2)
        );
    }

    #[test]
    fn loading_without_delivering_audio_does_not_report_a_listen() {
        assert!(
            loaded()
                .finish(EndReason::EndPlay, SystemTime::now())
                .is_none()
        );
    }

    #[test]
    fn pause_and_seek_time_do_not_inflate_played_audio() {
        let start = SystemTime::UNIX_EPOCH + Duration::from_secs(1000);
        let mut stats = loaded();
        // Each packet is shorter than a millisecond. Preserve the remainder
        // instead of truncating the duration of every packet independently.
        for _ in 0..1000 {
            stats.written(SAMPLES_PER_SECOND as usize / 1000, start);
        }
        // Pauses and seeks move wall time / position without writing samples.
        stats.written(
            SAMPLES_PER_SECOND as usize * 2,
            start + Duration::from_secs(60),
        );
        let report = stats
            .finish(EndReason::TrackDone, start + Duration::from_secs(61))
            .unwrap();
        assert_eq!(report.played.as_millis(), 2997);
        assert_eq!(report.started_at, start);
        assert_eq!(report.audio_format, "Vorbis 320 kbps");
    }
}
