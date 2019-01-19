use librespot::playback::player::PlayerEvent;
use tokio_process::{Child, CommandExt};
use std::collections::HashMap;
use std::io;
use std::io::{LineWriter, Write};
use std::fs::File;
use std::process::Command;

fn run_program(program: &str, env_vars: HashMap<&str, String>) -> io::Result<Child> {
    let mut v: Vec<&str> = program.split_whitespace().collect();
    info!("Running {:?} with environment variables {:?}", v, env_vars);
    Command::new(&v.remove(0))
        .args(&v)
        .envs(env_vars.iter())
        .spawn_async()
}

pub fn run_program_on_events(event: PlayerEvent, onevent: &str) -> io::Result<Child> {
    let mut env_vars = HashMap::new();
    let mut fifo = File::open("/var/run/librespot").expect("file not found");
    let mut fifo = LineWriter::new(fifo);
    match event {
        PlayerEvent::Changed {
            old_track_id,
            new_track_id,
            new_state,
        } => {
            env_vars.insert("PLAYER_EVENT", "change".to_string());
            env_vars.insert("OLD_TRACK_ID", old_track_id.to_base62());
            env_vars.insert("TRACK_ID", new_track_id.to_base62());
            write!(fifo, "{}\n", new_state).expect("failed");
        }
        PlayerEvent::Started { track_id, new_state } => {
            env_vars.insert("PLAYER_EVENT", "start".to_string());
            env_vars.insert("TRACK_ID", track_id.to_base62());
            write!(fifo, "{} {}\n", new_state, track_id.to_base62()).expect("failed");
        }
        PlayerEvent::Stopped { track_id, new_state } => {
            env_vars.insert("PLAYER_EVENT", "stop".to_string());
            env_vars.insert("TRACK_ID", track_id.to_base62());
            write!(fifo, "{}\n", new_state).expect("failed");
        }
    }
    run_program(onevent, env_vars)
}
