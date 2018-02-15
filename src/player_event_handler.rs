use std::process::Command;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use librespot::playback::config::PlayerEvent;

fn run_program(program: &str, args: Vec<String>) {
    info!("Running {}", program);
    let mut v: Vec<&str> = program.split_whitespace().collect();
    let status = Command::new(&v.remove(0))
        .args(&v)
        .args(args)
        .status()
        .expect("program failed to start");
    info!("Exit status: {}", status);
}

pub fn run_program_on_events(onstart: Option<String>, 
                             onstop: Option<String>,
                             onchange: Option<String>) -> Option<Sender<PlayerEvent>> {
    if onstart.is_none() && onstop.is_none() && onchange.is_none() {
        None
    } else {
        let (sender, receiver) = channel();
        thread::spawn(move || {
            while let Ok(msg) = receiver.recv() {
                match msg {
                    PlayerEvent::Changed { old_track_id, new_track_id } => {
                        let args = vec![old_track_id.to_base16(), new_track_id.to_base16()];
                        if let Some(ref onchange) = onchange.as_ref() {
                            run_program(onchange, args);
                        }
                    },
                    PlayerEvent::Started { track_id } => {
                        let args = vec![track_id.to_base16()];
                        if let Some(ref onstart) = onstart.as_ref() {
                            run_program(onstart, args);
                        }
                    }
                    PlayerEvent::Stopped { track_id } =>  {
                        let args = vec![track_id.to_base16()];
                        if let Some(ref onstop) = onstop.as_ref() {
                            run_program(onstop, args);
                        }
                    }
                }
            }
        });
        Some(sender)
    }
}
