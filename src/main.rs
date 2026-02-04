use librespot::connect::Spirc;
use log::{error, info, warn};
use std::{
    env,
    process::exit,
    time::{Duration, Instant},
};

#[cfg(discovery)]
use futures_util::StreamExt;
#[cfg(discovery)]
use std::sync::Arc;

mod config;
use config::{Config, set_env_var};

mod player_event_handler;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    const RUST_BACKTRACE: &str = "RUST_BACKTRACE";
    const RECONNECT_RATE_LIMIT_WINDOW: Duration = Duration::from_secs(600);
    const RECONNECT_RATE_LIMIT: usize = 5;

    if env::var(RUST_BACKTRACE).is_err() {
        set_env_var(RUST_BACKTRACE, "full").await;
    }

    let setup = Config::setup().await;

    let mut spirc: Option<Spirc> = None;
    let mut spirc_task: Option<_> = None;
    let mut auto_connect_times: Vec<Instant> = vec![];
    let mut connecting = false;

    let mut session = setup.session();

    #[cfg(discovery)]
    let mut discovery = setup.get_discovery().await;

    #[allow(unused_mut)]
    let mut last_credentials = setup.get_credentials(&mut connecting);

    // if last_credentials.is_none() && discovery.is_none() {
    //     error!(
    //         "Discovery is unavailable and no credentials provided. Authentication is not possible."
    //     );
    //     exit(1);
    // }

    let mixer = setup.get_mixer();
    let player = setup.get_player(mixer.clone(), session.clone());

    let _player_event_handler = setup.get_player_event_handler(player.clone());

    #[cfg(not(discovery))]
    macro_rules! select_wrapper {
        {$($branches:tt)*} => {
            tokio::select! {
                $($branches)*
            }
        };
    }

    #[cfg(discovery)]
    macro_rules! select_wrapper {
        {$($branches:tt)*} => {
            tokio::select! {
                credentials = async {
                    match discovery.as_mut() {
                        Some(d) => d.next().await,
                        _ => None
                    }
                }, if discovery.is_some() => {
                    match credentials {
                        Some(credentials) => {
                            last_credentials = Some(Arc::new(credentials));
                            auto_connect_times.clear();

                            if let Some(spirc) = spirc.take() && let Err(e) = spirc.shutdown() {
                                error!("error sending spirc shutdown message: {e}");
                            }
                            if let Some(spirc_task) = spirc_task.take() {
                                // Continue shutdown in its own task
                                tokio::spawn(spirc_task);
                            }
                            if !session.is_invalid() {
                                session.shutdown();
                            }

                            connecting = true;
                        },
                        None => {
                            error!("Discovery stopped unexpectedly");
                            exit(1);
                        }
                    }
                },

                $($branches)*
            }
        };
    }

    loop {
        select_wrapper! {
            _ = async {}, if connecting => {
                if let Some(credentials) = &last_credentials {
                    if session.is_invalid() {
                        session = session.renew();
                        player.set_session(session.clone());
                    }

                    (spirc, spirc_task) = setup.get_spirc(session.clone(),(**credentials).clone(), player.clone(), mixer.clone()).await;
                    connecting = false;
                }
            },
            _ = async {
                if let Some(task) = spirc_task.as_mut() {
                    task.await;
                }
            }, if spirc_task.is_some() && !connecting => {
                spirc_task = None;

                warn!("Spirc shut down unexpectedly");

                let mut reconnect_exceeds_rate_limit = || {
                    auto_connect_times.retain(|&t| t.elapsed() < RECONNECT_RATE_LIMIT_WINDOW);
                    auto_connect_times.len() > RECONNECT_RATE_LIMIT
                };

                if last_credentials.is_some() && !reconnect_exceeds_rate_limit() {
                    auto_connect_times.push(Instant::now());
                    if !session.is_invalid() {
                        session.shutdown();
                    }
                    connecting = true;
                } else {
                    error!("Spirc shut down too often. Not reconnecting automatically.");
                    exit(1);
                }
            },
            _ = async {}, if player.is_invalid() => {
                error!("Player shut down unexpectedly");
                exit(1);
            },
            _ = tokio::signal::ctrl_c() => {
                break;
            },
            else => break,
        }
    }

    info!("Gracefully shutting down");

    let mut shutdown_tasks = tokio::task::JoinSet::new();

    // Shutdown spirc if necessary
    if let Some(spirc) = spirc {
        if let Err(e) = spirc.shutdown() {
            error!("error sending spirc shutdown message: {e}");
        }

        if let Some(spirc_task) = spirc_task {
            shutdown_tasks.spawn(spirc_task);
        }
    }

    #[cfg(discovery)]
    if let Some(discovery) = discovery {
        shutdown_tasks.spawn(discovery.shutdown());
    }

    tokio::select! {
        _ = tokio::signal::ctrl_c() => (),
        _ = shutdown_tasks.join_all() => (),
    }
}
