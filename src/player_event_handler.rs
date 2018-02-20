use std::process::Command;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::collections::HashMap;
use librespot::playback::player::PlayerEvent;

fn run_program(program: &str, env_vars: HashMap<&str, String>) {
    let mut v: Vec<&str> = program.split_whitespace().collect();
    info!("Running {:?}", v);
    let status = Command::new(&v.remove(0))
        .args(&v)
        .envs(env_vars.iter())
        .status()
        .expect("program failed to start");
    info!("Exit status: {}", status);
}

pub fn run_program_on_events(onevent: String) -> Sender<PlayerEvent> {
    let (sender, receiver) = channel();
    thread::spawn(move || {
        while let Ok(msg) = receiver.recv() {
            let mut env_vars = HashMap::new();
            match msg {
                PlayerEvent::Changed { old_track_id, new_track_id } => {
                    env_vars.insert("PLAYER_EVENT", "change".to_string());
                    env_vars.insert("OLD_TRACK_ID", old_track_id.to_base16());
                    env_vars.insert("TRACK_ID", new_track_id.to_base16());
                },
                PlayerEvent::Started { track_id } => {
                    env_vars.insert("PLAYER_EVENT", "start".to_string());
                    env_vars.insert("TRACK_ID", track_id.to_base16());
                }
                PlayerEvent::Stopped { track_id } =>  {
                    env_vars.insert("PLAYER_EVENT", "stop".to_string());
                    env_vars.insert("TRACK_ID", track_id.to_base16());
                }
            }
            run_program(&onevent, env_vars);
        }
    });
    sender
}
