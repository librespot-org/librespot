use super::{Mixer, MixerConfig};
use librespot_core::Error;
use portable_atomic::AtomicU16;
use shell_words::split;
use std::process::{Command, ExitStatus, Stdio};
use std::sync::atomic::Ordering;
use std::time::{Duration, Instant};
use thiserror::Error;

const EXTERNAL_VOLUME_QUERY_TIMEOUT: Duration = Duration::from_secs(1);

pub struct ExternalMixer {
    volume: AtomicU16,
    volume_query: Option<ExternalVolumeQuery>,
}

struct ExternalVolumeQuery {
    command: String,
    args: Vec<String>,
    display: String,
}

#[derive(Debug, Error)]
enum ExternalVolumeQueryError {
    #[error("missing command")]
    MissingCommand,
    #[error("failed to parse command args for {command}: {e}")]
    InvalidArgs {
        command: String,
        e: shell_words::ParseError,
    },
    #[error("failed to spawn command {command}: {e}")]
    SpawnFailure { command: String, e: std::io::Error },
    #[error("command exited unsuccessfully: {0}")]
    NonZeroExit(ExitStatus),
    #[error("command timed out after {0:?}")]
    TimedOut(Duration),
    #[error("failed to wait for command: {0}")]
    WaitFailure(std::io::Error),
    #[error("failed to kill timed-out command: {0}")]
    KillFailure(std::io::Error),
    #[error("command output was not UTF-8: {0}")]
    InvalidUtf8(#[from] std::str::Utf8Error),
    #[error("command output was empty")]
    EmptyOutput,
    #[error("command output must contain one integer, got {0:?}")]
    InvalidOutput(String),
    #[error("command volume {0} is outside the valid 0..=65535 range")]
    VolumeOutOfRange(u32),
}

impl Mixer for ExternalMixer {
    fn open(config: MixerConfig) -> Result<Self, Error> {
        let volume_query = config
            .external_volume_query
            .map(ExternalVolumeQuery::new)
            .transpose()
            .map_err(Error::invalid_argument)?;

        if volume_query.is_some() {
            info!("Mixing with external volume control and volume query");
        } else {
            info!("Mixing with external volume control");
        }

        Ok(Self {
            volume: AtomicU16::new(u16::MAX / 2),
            volume_query,
        })
    }

    fn volume(&self) -> u16 {
        self.volume.load(Ordering::Relaxed)
    }

    fn set_volume(&self, volume: u16) {
        self.volume.store(volume, Ordering::Relaxed);
    }

    fn refresh_volume(&self) -> Option<u16> {
        let query = self.volume_query.as_ref()?;

        match query.volume() {
            Ok(volume) => {
                self.volume.store(volume, Ordering::Relaxed);
                Some(volume)
            }
            Err(why) => {
                warn!("External volume query failed: {why}");
                None
            }
        }
    }
}

impl ExternalMixer {
    pub const NAME: &'static str = "external";
}

impl ExternalVolumeQuery {
    fn new(command: String) -> Result<Self, ExternalVolumeQueryError> {
        let mut command_parts =
            split(&command).map_err(|e| ExternalVolumeQueryError::InvalidArgs {
                command: command.clone(),
                e,
            })?;

        if command_parts.is_empty() {
            return Err(ExternalVolumeQueryError::MissingCommand);
        }

        Ok(Self {
            command: command_parts.remove(0),
            args: command_parts,
            display: command,
        })
    }

    fn volume(&self) -> Result<u16, ExternalVolumeQueryError> {
        let mut child = Command::new(&self.command)
            .args(&self.args)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| ExternalVolumeQueryError::SpawnFailure {
                command: self.display.clone(),
                e,
            })?;

        let deadline = Instant::now() + EXTERNAL_VOLUME_QUERY_TIMEOUT;
        loop {
            match child
                .try_wait()
                .map_err(ExternalVolumeQueryError::WaitFailure)?
            {
                Some(_) => {
                    let output = child
                        .wait_with_output()
                        .map_err(ExternalVolumeQueryError::WaitFailure)?;

                    if !output.status.success() {
                        return Err(ExternalVolumeQueryError::NonZeroExit(output.status));
                    }

                    return parse_external_volume(&output.stdout);
                }
                None => {
                    let now = Instant::now();
                    if now >= deadline {
                        child
                            .kill()
                            .map_err(ExternalVolumeQueryError::KillFailure)?;
                        let _ = child.wait();
                        return Err(ExternalVolumeQueryError::TimedOut(
                            EXTERNAL_VOLUME_QUERY_TIMEOUT,
                        ));
                    }

                    std::thread::sleep(
                        deadline
                            .saturating_duration_since(now)
                            .min(Duration::from_millis(10)),
                    );
                }
            }
        }
    }
}

fn parse_external_volume(output: &[u8]) -> Result<u16, ExternalVolumeQueryError> {
    let output = std::str::from_utf8(output)?;
    let trimmed = output.trim();

    if trimmed.is_empty() {
        return Err(ExternalVolumeQueryError::EmptyOutput);
    }

    if trimmed.split_whitespace().count() != 1 {
        return Err(ExternalVolumeQueryError::InvalidOutput(output.to_owned()));
    }

    let volume = trimmed
        .parse::<u32>()
        .map_err(|_| ExternalVolumeQueryError::InvalidOutput(output.to_owned()))?;

    u16::try_from(volume).map_err(|_| ExternalVolumeQueryError::VolumeOutOfRange(volume))
}

#[cfg(test)]
mod tests {
    use super::{ExternalMixer, ExternalVolumeQueryError, parse_external_volume};
    use crate::mixer::{Mixer, MixerConfig};

    #[test]
    fn external_mixer_tracks_requested_volume() {
        let mixer = ExternalMixer::open(MixerConfig::default()).unwrap();

        for volume in [0, u16::MAX / 2, u16::MAX] {
            mixer.set_volume(volume);
            assert_eq!(mixer.volume(), volume);
        }
    }

    #[test]
    fn external_mixer_never_attenuates_audio() {
        let mixer = ExternalMixer::open(MixerConfig::default()).unwrap();

        for volume in [0, u16::MAX / 2, u16::MAX] {
            mixer.set_volume(volume);
            assert_eq!(mixer.get_soft_volume().attenuation_factor(), 1.0);
        }
    }

    #[test]
    fn external_query_parses_valid_volume() {
        assert_eq!(parse_external_volume(b"0").unwrap(), 0);
        assert_eq!(parse_external_volume(b"32768\n").unwrap(), 32768);
        assert_eq!(parse_external_volume(b" 65535 \n").unwrap(), u16::MAX);
    }

    #[test]
    fn external_query_rejects_invalid_volume() {
        assert!(matches!(
            parse_external_volume(b"not a volume"),
            Err(ExternalVolumeQueryError::InvalidOutput(_))
        ));
        assert!(matches!(
            parse_external_volume(b"1 2"),
            Err(ExternalVolumeQueryError::InvalidOutput(_))
        ));
        assert!(matches!(
            parse_external_volume(b"65536"),
            Err(ExternalVolumeQueryError::VolumeOutOfRange(65536))
        ));
    }

    #[cfg(unix)]
    #[test]
    fn external_mixer_query_success_updates_cached_volume() {
        let mixer = ExternalMixer::open(MixerConfig {
            external_volume_query: Some("/bin/sh -c 'printf 4242'".to_owned()),
            ..MixerConfig::default()
        })
        .unwrap();

        assert_eq!(mixer.refresh_volume(), Some(4242));
        assert_eq!(mixer.volume(), 4242);
        assert_eq!(mixer.get_soft_volume().attenuation_factor(), 1.0);
    }

    #[cfg(unix)]
    #[test]
    fn external_mixer_query_failure_preserves_cached_volume() {
        let mixer = ExternalMixer::open(MixerConfig {
            external_volume_query: Some("/bin/sh -c 'exit 1'".to_owned()),
            ..MixerConfig::default()
        })
        .unwrap();

        mixer.set_volume(1234);

        assert_eq!(mixer.refresh_volume(), None);
        assert_eq!(mixer.volume(), 1234);
        assert_eq!(mixer.get_soft_volume().attenuation_factor(), 1.0);
    }
}
