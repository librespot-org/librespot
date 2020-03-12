use librespot::playback::player::PlayerEvent;
use librespot_playback::player::TrackMetaData;
use log::info;
use std::collections::HashMap;
use std::io;
use std::process::Command;
use tokio_process::{Child, CommandExt};

fn run_program(program: &str, env_vars: HashMap<&str, String>) -> io::Result<Child> {
    let mut v: Vec<&str> = program.split_whitespace().collect();
    info!("Running {:?} with environment variables {:?}", v, env_vars);
    Command::new(&v.remove(0))
        .args(&v)
        .envs(env_vars.iter())
        .spawn_async()
}

fn add_meta_data_to_env_vars(env_vars: &mut HashMap<&str, String>, track_meta_data: TrackMetaData) {
    env_vars.insert("TRACK_ID", track_meta_data.track_id.to_base62());
    env_vars.insert("DURATION_MS", track_meta_data.duration_ms.to_string());
    env_vars.insert("TRACK_TITLE", track_meta_data.title);
}

pub fn run_program_on_events(event: PlayerEvent, onevent: &str) -> Option<io::Result<Child>> {
    let mut env_vars = HashMap::new();
    match event {
        PlayerEvent::Changed {
            old_track_id,
            new_track_id,
        } => {
            env_vars.insert("PLAYER_EVENT", "change".to_string());
            env_vars.insert("OLD_TRACK_ID", old_track_id.to_base62());
            env_vars.insert("TRACK_ID", new_track_id.to_base62());
        }
        PlayerEvent::Started { track_id, .. } => {
            env_vars.insert("PLAYER_EVENT", "start".to_string());
            env_vars.insert("TRACK_ID", track_id.to_base62());
        }
        PlayerEvent::Stopped { track_id, .. } => {
            env_vars.insert("PLAYER_EVENT", "stop".to_string());
            env_vars.insert("TRACK_ID", track_id.to_base62());
        }
        PlayerEvent::Playing {
            track_meta_data,
            position_ms,
            ..
        } => {
            env_vars.insert("PLAYER_EVENT", "playing".to_string());
            add_meta_data_to_env_vars(&mut env_vars, track_meta_data);
            env_vars.insert("POSITION_MS", position_ms.to_string());
        }
        PlayerEvent::Paused {
            track_meta_data,
            position_ms,
            ..
        } => {
            env_vars.insert("PLAYER_EVENT", "paused".to_string());
            add_meta_data_to_env_vars(&mut env_vars, track_meta_data);
            env_vars.insert("POSITION_MS", position_ms.to_string());
        }
        PlayerEvent::VolumeSet { volume } => {
            env_vars.insert("PLAYER_EVENT", "volume_set".to_string());
            env_vars.insert("VOLUME", volume.to_string());
        }
        _ => return None,
    }
    Some(run_program(onevent, env_vars))
}
