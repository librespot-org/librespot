use librespot::playback::player::PlayerEvent;
use std::collections::HashMap;
use std::process::Command;
use std::thread;

fn run_program(program: &str, env_vars: HashMap<&str, String>) {
    let mut v: Vec<&str> = program.split_whitespace().collect();
    info!("Running {:?} with environment variables {:?}", v, env_vars);
    let mut child = Command::new(&v.remove(0))
        .args(&v)
        .envs(env_vars.iter())
        .spawn()
        .expect("program failed to start");
    thread::spawn(move || {
        child.wait().expect("failed to wait for program to finish");
    });
}

pub fn run_program_on_events(event: PlayerEvent, onevent: &str) {
    let mut env_vars = HashMap::new();
    match event {
        PlayerEvent::Changed {
            old_track_id,
            new_track_id,
        } => {
            env_vars.insert("PLAYER_EVENT", "change".to_string());
            env_vars.insert("OLD_TRACK_ID", old_track_id.to_base16());
            env_vars.insert("TRACK_ID", new_track_id.to_base16());
        }
        PlayerEvent::Started { track_id } => {
            env_vars.insert("PLAYER_EVENT", "start".to_string());
            env_vars.insert("TRACK_ID", track_id.to_base16());
        }
        PlayerEvent::Stopped { track_id } => {
            env_vars.insert("PLAYER_EVENT", "stop".to_string());
            env_vars.insert("TRACK_ID", track_id.to_base16());
        }
    }
    run_program(onevent, env_vars);
}
