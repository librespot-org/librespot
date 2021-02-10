use librespot::playback::player::PlayerEvent;
use log::info;
use std::collections::HashMap;
use std::io;
use std::process::Command;
use tokio_process::{Child, CommandExt};

use futures::Future;
use librespot::playback::player::SinkStatus;

fn run_program(program: &str, env_vars: HashMap<&str, String>) -> io::Result<Child> {
    let mut v: Vec<&str> = program.split_whitespace().collect();
    info!("Running {:?} with environment variables {:?}", v, env_vars);
    Command::new(&v.remove(0))
        .args(&v)
        .envs(env_vars.iter())
        .spawn_async()
}

pub fn run_program_on_events(event: PlayerEvent, onevent: &str) -> Option<io::Result<Child>> {
    let mut env_vars = HashMap::new();
    match event {
        PlayerEvent::Changed {
            old_track_id,
            new_track_id,
        } => {
            env_vars.insert("PLAYER_EVENT", "changed".to_string());
            env_vars.insert("OLD_TRACK_ID", old_track_id.to_base62());
            env_vars.insert("TRACK_ID", new_track_id.to_base62());
        }
        PlayerEvent::Started { track_id, .. } => {
            env_vars.insert("PLAYER_EVENT", "started".to_string());
            env_vars.insert("TRACK_ID", track_id.to_base62());
        }
        PlayerEvent::Stopped { track_id, .. } => {
            env_vars.insert("PLAYER_EVENT", "stopped".to_string());
            env_vars.insert("TRACK_ID", track_id.to_base62());
        }
        PlayerEvent::Playing {
            track_id,
            duration_ms,
            position_ms,
            ..
        } => {
            env_vars.insert("PLAYER_EVENT", "playing".to_string());
            env_vars.insert("TRACK_ID", track_id.to_base62());
            env_vars.insert("DURATION_MS", duration_ms.to_string());
            env_vars.insert("POSITION_MS", position_ms.to_string());
        }
        PlayerEvent::Paused {
            track_id,
            duration_ms,
            position_ms,
            ..
        } => {
            env_vars.insert("PLAYER_EVENT", "paused".to_string());
            env_vars.insert("TRACK_ID", track_id.to_base62());
            env_vars.insert("DURATION_MS", duration_ms.to_string());
            env_vars.insert("POSITION_MS", position_ms.to_string());
        }
        PlayerEvent::Preloading { track_id, .. } => {
            env_vars.insert("PLAYER_EVENT", "preloading".to_string());
            env_vars.insert("TRACK_ID", track_id.to_base62());
        }
        PlayerEvent::VolumeSet { volume } => {
            env_vars.insert("PLAYER_EVENT", "volume_set".to_string());
            env_vars.insert("VOLUME", volume.to_string());
        }
        _ => return None,
    }
    Some(run_program(onevent, env_vars))
}

pub fn emit_sink_event(sink_status: SinkStatus, onevent: &str) {
    let mut env_vars = HashMap::new();
    env_vars.insert("PLAYER_EVENT", "sink".to_string());
    let sink_status = match sink_status {
        SinkStatus::Running => "running",
        SinkStatus::TemporarilyClosed => "temporarily_closed",
        SinkStatus::Closed => "closed",
    };
    env_vars.insert("SINK_STATUS", sink_status.to_string());

    let _ = run_program(onevent, env_vars).and_then(|child| child.wait());
}
