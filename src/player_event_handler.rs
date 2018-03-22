use librespot::core::events::Event;
use std::collections::HashMap;
use std::process::Command;

fn run_program(program: &str, env_vars: HashMap<&str, String>) {
    let mut v: Vec<&str> = program.split_whitespace().collect();
    info!("Running {:?} with environment variables {:?}", v, env_vars);
    Command::new(&v.remove(0))
        .args(&v)
        .envs(env_vars.iter())
        .spawn()
        .expect("program failed to start");
}

pub fn run_program_on_events(event: Event, onevent: &str) {
    let mut env_vars = HashMap::new();
    match event {
        Event::TrackChanged {
            old_track_id,
            track_id,
        } => {
            env_vars.insert("PLAYER_EVENT", "change".to_string());
            env_vars.insert("OLD_TRACK_ID", old_track_id.to_base16());
            env_vars.insert("TRACK_ID", track_id.to_base16());
        }
        Event::PlaybackStarted { track_id } => {
            env_vars.insert("PLAYER_EVENT", "start".to_string());
            env_vars.insert("TRACK_ID", track_id.to_base16());
        }
        Event::PlaybackStopped { track_id } => {
            env_vars.insert("PLAYER_EVENT", "stop".to_string());
            env_vars.insert("TRACK_ID", track_id.to_base16());
        }
        _ => (),
    }
    run_program(onevent, env_vars);
}
