use librespot::playback::player::PlayerEvent;
use librespot::playback::player::SinkStatus;
use log::info;
use tokio::process::{Child as AsyncChild, Command as AsyncCommand};

use std::collections::HashMap;
use std::io;
use std::io::{Error, ErrorKind};
use std::process::{Command, ExitStatus};

pub fn run_program_on_events(event: PlayerEvent, onevent: &str) -> Option<io::Result<AsyncChild>> {
    let mut env_vars = HashMap::new();
    match event {
        PlayerEvent::Changed {
            old_track_id,
            new_track_id,
        } => match old_track_id.to_base62() {
            Err(e) => {
                return Some(Err(Error::new(
                    ErrorKind::InvalidData,
                    format!(
                        "PlayerEvent::Changed: Invalid old track id: {}",
                        e.utf8_error()
                    ),
                )))
            }
            Ok(old_id) => match new_track_id.to_base62() {
                Err(e) => {
                    return Some(Err(Error::new(
                        ErrorKind::InvalidData,
                        format!(
                            "PlayerEvent::Changed: Invalid new track id: {}",
                            e.utf8_error()
                        ),
                    )))
                }
                Ok(new_id) => {
                    env_vars.insert("PLAYER_EVENT", "changed".to_string());
                    env_vars.insert("OLD_TRACK_ID", old_id);
                    env_vars.insert("TRACK_ID", new_id);
                }
            },
        },
        PlayerEvent::Started { track_id, .. } => match track_id.to_base62() {
            Err(e) => {
                return Some(Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("PlayerEvent::Started: Invalid track id: {}", e.utf8_error()),
                )))
            }
            Ok(id) => {
                env_vars.insert("PLAYER_EVENT", "started".to_string());
                env_vars.insert("TRACK_ID", id);
            }
        },
        PlayerEvent::Stopped { track_id, .. } => match track_id.to_base62() {
            Err(e) => {
                return Some(Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("PlayerEvent::Stopped: Invalid track id: {}", e.utf8_error()),
                )))
            }
            Ok(id) => {
                env_vars.insert("PLAYER_EVENT", "stopped".to_string());
                env_vars.insert("TRACK_ID", id);
            }
        },
        PlayerEvent::Playing {
            track_id,
            duration_ms,
            position_ms,
            ..
        } => match track_id.to_base62() {
            Err(e) => {
                return Some(Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("PlayerEvent::Playing: Invalid track id: {}", e.utf8_error()),
                )))
            }
            Ok(id) => {
                env_vars.insert("PLAYER_EVENT", "playing".to_string());
                env_vars.insert("TRACK_ID", id);
                env_vars.insert("DURATION_MS", duration_ms.to_string());
                env_vars.insert("POSITION_MS", position_ms.to_string());
            }
        },
        PlayerEvent::Paused {
            track_id,
            duration_ms,
            position_ms,
            ..
        } => match track_id.to_base62() {
            Err(e) => {
                return Some(Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("PlayerEvent::Paused: Invalid track id: {}", e.utf8_error()),
                )))
            }
            Ok(id) => {
                env_vars.insert("PLAYER_EVENT", "paused".to_string());
                env_vars.insert("TRACK_ID", id);
                env_vars.insert("DURATION_MS", duration_ms.to_string());
                env_vars.insert("POSITION_MS", position_ms.to_string());
            }
        },
        PlayerEvent::Preloading { track_id, .. } => match track_id.to_base62() {
            Err(e) => {
                return Some(Err(Error::new(
                    ErrorKind::InvalidData,
                    format!(
                        "PlayerEvent::Preloading: Invalid track id: {}",
                        e.utf8_error()
                    ),
                )))
            }
            Ok(id) => {
                env_vars.insert("PLAYER_EVENT", "preloading".to_string());
                env_vars.insert("TRACK_ID", id);
            }
        },
        PlayerEvent::VolumeSet { volume } => {
            env_vars.insert("PLAYER_EVENT", "volume_set".to_string());
            env_vars.insert("VOLUME", volume.to_string());
        }
        _ => return None,
    }

    let mut v: Vec<&str> = onevent.split_whitespace().collect();
    info!("Running {:?} with environment variables {:?}", v, env_vars);
    Some(
        AsyncCommand::new(&v.remove(0))
            .args(&v)
            .envs(env_vars.iter())
            .spawn(),
    )
}

pub fn emit_sink_event(sink_status: SinkStatus, onevent: &str) -> io::Result<ExitStatus> {
    let mut env_vars = HashMap::new();
    env_vars.insert("PLAYER_EVENT", "sink".to_string());
    let sink_status = match sink_status {
        SinkStatus::Running => "running",
        SinkStatus::TemporarilyClosed => "temporarily_closed",
        SinkStatus::Closed => "closed",
    };
    env_vars.insert("SINK_STATUS", sink_status.to_string());
    let mut v: Vec<&str> = onevent.split_whitespace().collect();
    info!("Running {:?} with environment variables {:?}", v, env_vars);

    Command::new(&v.remove(0))
        .args(&v)
        .envs(env_vars.iter())
        .spawn()?
        .wait()
}
