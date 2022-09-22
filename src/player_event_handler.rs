use log::{debug, error, warn};

use std::{collections::HashMap, process::Command, thread};

use librespot::{
    metadata::audio::UniqueFields,
    playback::player::{PlayerEvent, PlayerEventChannel, SinkStatus},
};

pub struct EventHandler {
    thread_handle: Option<thread::JoinHandle<()>>,
}

impl EventHandler {
    pub fn new(mut player_events: PlayerEventChannel, onevent: &str) -> Self {
        let on_event = onevent.to_string();
        let thread_handle = Some(thread::spawn(move || loop {
            match player_events.blocking_recv() {
                None => break,
                Some(event) => {
                    let mut env_vars = HashMap::new();

                    match event {
                        PlayerEvent::TrackChanged { audio_item } => {
                            match audio_item.track_id.to_base62() {
                                Err(e) => {
                                    warn!("PlayerEvent::TrackChanged: Invalid track id: {}", e)
                                }
                                Ok(id) => {
                                    env_vars.insert("PLAYER_EVENT", "track_changed".to_string());
                                    env_vars.insert("TRACK_ID", id);
                                    env_vars.insert("URI", audio_item.uri);
                                    env_vars.insert("NAME", audio_item.name);
                                    env_vars.insert(
                                        "COVERS",
                                        audio_item
                                            .covers
                                            .into_iter()
                                            .map(|c| c.url)
                                            .collect::<Vec<String>>()
                                            .join("\n"),
                                    );
                                    env_vars.insert("LANGUAGE", audio_item.language.join("\n"));
                                    env_vars
                                        .insert("DURATION_MS", audio_item.duration_ms.to_string());
                                    env_vars
                                        .insert("IS_EXPLICIT", audio_item.is_explicit.to_string());

                                    match audio_item.unique_fields {
                                        UniqueFields::Track {
                                            artists,
                                            album,
                                            album_artists,
                                            popularity,
                                            number,
                                            disc_number,
                                        } => {
                                            env_vars.insert("ITEM_TYPE", "Track".to_string());
                                            env_vars.insert(
                                                "ARTISTS",
                                                artists
                                                    .0
                                                    .into_iter()
                                                    .map(|a| a.name)
                                                    .collect::<Vec<String>>()
                                                    .join("\n"),
                                            );
                                            env_vars
                                                .insert("ALBUM_ARTISTS", album_artists.join("\n"));
                                            env_vars.insert("ALBUM", album);
                                            env_vars.insert("POPULARITY", popularity.to_string());
                                            env_vars.insert("NUMBER", number.to_string());
                                            env_vars.insert("DISC_NUMBER", disc_number.to_string());
                                        }
                                        UniqueFields::Episode {
                                            description,
                                            publish_time,
                                            show_name,
                                        } => {
                                            env_vars.insert("ITEM_TYPE", "Episode".to_string());
                                            env_vars.insert("DESCRIPTION", description);
                                            env_vars.insert(
                                                "PUBLISH_TIME",
                                                publish_time.unix_timestamp().to_string(),
                                            );
                                            env_vars.insert("SHOW_NAME", show_name);
                                        }
                                    }
                                }
                            }
                        }
                        PlayerEvent::Stopped { track_id, .. } => match track_id.to_base62() {
                            Err(e) => warn!("PlayerEvent::Stopped: Invalid track id: {}", e),
                            Ok(id) => {
                                env_vars.insert("PLAYER_EVENT", "stopped".to_string());
                                env_vars.insert("TRACK_ID", id);
                            }
                        },
                        PlayerEvent::Playing {
                            track_id,
                            position_ms,
                            ..
                        } => match track_id.to_base62() {
                            Err(e) => warn!("PlayerEvent::Playing: Invalid track id: {}", e),
                            Ok(id) => {
                                env_vars.insert("PLAYER_EVENT", "playing".to_string());
                                env_vars.insert("TRACK_ID", id);
                                env_vars.insert("POSITION_MS", position_ms.to_string());
                            }
                        },
                        PlayerEvent::Paused {
                            track_id,
                            position_ms,
                            ..
                        } => match track_id.to_base62() {
                            Err(e) => warn!("PlayerEvent::Paused: Invalid track id: {}", e),
                            Ok(id) => {
                                env_vars.insert("PLAYER_EVENT", "paused".to_string());
                                env_vars.insert("TRACK_ID", id);
                                env_vars.insert("POSITION_MS", position_ms.to_string());
                            }
                        },
                        PlayerEvent::Loading { track_id, .. } => match track_id.to_base62() {
                            Err(e) => warn!("PlayerEvent::Loading: Invalid track id: {}", e),
                            Ok(id) => {
                                env_vars.insert("PLAYER_EVENT", "loading".to_string());
                                env_vars.insert("TRACK_ID", id);
                            }
                        },
                        PlayerEvent::Preloading { track_id, .. } => match track_id.to_base62() {
                            Err(e) => warn!("PlayerEvent::Preloading: Invalid track id: {}", e),
                            Ok(id) => {
                                env_vars.insert("PLAYER_EVENT", "preloading".to_string());
                                env_vars.insert("TRACK_ID", id);
                            }
                        },
                        PlayerEvent::TimeToPreloadNextTrack { track_id, .. } => {
                            match track_id.to_base62() {
                                Err(e) => warn!(
                                    "PlayerEvent::TimeToPreloadNextTrack: Invalid track id: {}",
                                    e
                                ),
                                Ok(id) => {
                                    env_vars.insert("PLAYER_EVENT", "preload_next".to_string());
                                    env_vars.insert("TRACK_ID", id);
                                }
                            }
                        }
                        PlayerEvent::EndOfTrack { track_id, .. } => match track_id.to_base62() {
                            Err(e) => warn!("PlayerEvent::EndOfTrack: Invalid track id: {}", e),
                            Ok(id) => {
                                env_vars.insert("PLAYER_EVENT", "end_of_track".to_string());
                                env_vars.insert("TRACK_ID", id);
                            }
                        },
                        PlayerEvent::Unavailable { track_id, .. } => match track_id.to_base62() {
                            Err(e) => warn!("PlayerEvent::Unavailable: Invalid track id: {}", e),
                            Ok(id) => {
                                env_vars.insert("PLAYER_EVENT", "unavailable".to_string());
                                env_vars.insert("TRACK_ID", id);
                            }
                        },
                        PlayerEvent::VolumeChanged { volume } => {
                            env_vars.insert("PLAYER_EVENT", "volume_changed".to_string());
                            env_vars.insert("VOLUME", volume.to_string());
                        }
                        PlayerEvent::Seeked {
                            track_id,
                            position_ms,
                            ..
                        } => match track_id.to_base62() {
                            Err(e) => warn!("PlayerEvent::Seeked: Invalid track id: {}", e),
                            Ok(id) => {
                                env_vars.insert("PLAYER_EVENT", "seeked".to_string());
                                env_vars.insert("TRACK_ID", id);
                                env_vars.insert("POSITION_MS", position_ms.to_string());
                            }
                        },
                        PlayerEvent::PositionCorrection {
                            track_id,
                            position_ms,
                            ..
                        } => match track_id.to_base62() {
                            Err(e) => {
                                warn!("PlayerEvent::PositionCorrection: Invalid track id: {}", e)
                            }
                            Ok(id) => {
                                env_vars.insert("PLAYER_EVENT", "position_correction".to_string());
                                env_vars.insert("TRACK_ID", id);
                                env_vars.insert("POSITION_MS", position_ms.to_string());
                            }
                        },
                        PlayerEvent::SessionConnected {
                            connection_id,
                            user_name,
                        } => {
                            env_vars.insert("PLAYER_EVENT", "session_connected".to_string());
                            env_vars.insert("CONNECTION_ID", connection_id);
                            env_vars.insert("USER_NAME", user_name);
                        }
                        PlayerEvent::SessionDisconnected {
                            connection_id,
                            user_name,
                        } => {
                            env_vars.insert("PLAYER_EVENT", "session_disconnected".to_string());
                            env_vars.insert("CONNECTION_ID", connection_id);
                            env_vars.insert("USER_NAME", user_name);
                        }
                        PlayerEvent::SessionClientChanged {
                            client_id,
                            client_name,
                            client_brand_name,
                            client_model_name,
                        } => {
                            env_vars.insert("PLAYER_EVENT", "session_client_changed".to_string());
                            env_vars.insert("CLIENT_ID", client_id);
                            env_vars.insert("CLIENT_NAME", client_name);
                            env_vars.insert("CLIENT_BRAND_NAME", client_brand_name);
                            env_vars.insert("CLIENT_MODEL_NAME", client_model_name);
                        }
                        PlayerEvent::ShuffleChanged { shuffle } => {
                            env_vars.insert("PLAYER_EVENT", "shuffle_changed".to_string());
                            env_vars.insert("SHUFFLE", shuffle.to_string());
                        }
                        PlayerEvent::RepeatChanged { repeat } => {
                            env_vars.insert("PLAYER_EVENT", "repeat_changed".to_string());
                            env_vars.insert("REPEAT", repeat.to_string());
                        }
                        PlayerEvent::AutoPlayChanged { auto_play } => {
                            env_vars.insert("PLAYER_EVENT", "auto_play_changed".to_string());
                            env_vars.insert("AUTO_PLAY", auto_play.to_string());
                        }

                        PlayerEvent::FilterExplicitContentChanged { filter } => {
                            env_vars.insert(
                                "PLAYER_EVENT",
                                "filter_explicit_content_changed".to_string(),
                            );
                            env_vars.insert("FILTER", filter.to_string());
                        }
                    }

                    if !env_vars.is_empty() {
                        run_program(env_vars, &on_event);
                    }
                }
            }
        }));

        Self { thread_handle }
    }
}

impl Drop for EventHandler {
    fn drop(&mut self) {
        debug!("Shutting down EventHandler thread ...");
        if let Some(handle) = self.thread_handle.take() {
            if let Err(e) = handle.join() {
                error!("EventHandler thread Error: {:?}", e);
            }
        }
    }
}

pub fn run_program_on_sink_events(sink_status: SinkStatus, onevent: &str) {
    let mut env_vars = HashMap::new();

    env_vars.insert("PLAYER_EVENT", "sink".to_string());

    let sink_status = match sink_status {
        SinkStatus::Running => "running",
        SinkStatus::TemporarilyClosed => "temporarily_closed",
        SinkStatus::Closed => "closed",
    };

    env_vars.insert("SINK_STATUS", sink_status.to_string());

    run_program(env_vars, onevent);
}

fn run_program(env_vars: HashMap<&str, String>, onevent: &str) {
    let mut v: Vec<&str> = onevent.split_whitespace().collect();

    debug!(
        "Running {} with environment variables:\n{:#?}",
        onevent, env_vars
    );

    match Command::new(&v.remove(0))
        .args(&v)
        .envs(env_vars.iter())
        .spawn()
    {
        Err(e) => warn!("On event program {} failed to start: {}", onevent, e),
        Ok(mut child) => match child.wait() {
            Err(e) => warn!("On event program {} failed: {}", onevent, e),
            Ok(e) if e.success() => (),
            Ok(e) => {
                if let Some(code) = e.code() {
                    warn!("On event program {} returned exit code {}", onevent, code);
                } else {
                    warn!("On event program {} returned failure: {}", onevent, e);
                }
            }
        },
    }
}
