//! Task keeping the MPRIS interfaces in sync with the player.

use std::sync::Arc;

use log::{debug, warn};
use tokio::sync::mpsc;
use zbus::object_server::{InterfaceRef, SignalEmitter};

use librespot::{
    core::SpotifyUri,
    playback::player::{Player, PlayerEvent},
};

use super::{
    MprisCommand, OBJECT_PATH,
    metadata::{Metadata, MprisMetadata, XesamMetadata},
    player::{MprisPlayerService, Position},
    types::{LoopStatus, PlaybackStatus},
};

pub(super) struct MprisTask {
    player: Arc<Player>,
    connection: zbus::Connection,
    cmd_rx: mpsc::UnboundedReceiver<MprisCommand>,
}

impl MprisTask {
    pub(super) fn new(
        player: Arc<Player>,
        connection: zbus::Connection,
        cmd_rx: mpsc::UnboundedReceiver<MprisCommand>,
    ) -> Self {
        Self {
            player,
            connection,
            cmd_rx,
        }
    }

    pub(super) async fn run(mut self) {
        let mut player_events = self.player.get_player_event_channel();

        loop {
            tokio::select! {
                Some(event) = player_events.recv() => {
                    if let Err(e) = self.handle_event(event).await {
                        warn!("Error handling PlayerEvent: {e}");
                    }
                }

                cmd = self.cmd_rx.recv() => {
                    match cmd {
                        Some(MprisCommand::SetSpirc(spirc)) => {
                            self.mpris_player_iface().await
                                .get_mut().await
                                .spirc = Some(spirc);
                        }
                        Some(MprisCommand::Quit) => break,

                        // Keep running if the cmd sender was dropped
                        None => (),
                    }
                }

                // If player_events yields None, shutdown
                else => break,
            }
        }

        debug!("Shutting down MprisTask ...");
    }

    async fn mpris_player_iface(&self) -> InterfaceRef<MprisPlayerService> {
        self.connection
            .object_server()
            .interface::<_, MprisPlayerService>(OBJECT_PATH)
            .await
            .expect("iface missing on object server")
    }

    async fn handle_event(&self, event: PlayerEvent) -> zbus::Result<()> {
        let iface_ref = self.mpris_player_iface().await;
        let emitter = iface_ref.signal_emitter();

        match event {
            PlayerEvent::TrackChanged { audio_item } => {
                // Choose biggest cover
                let art_url = audio_item
                    .covers
                    .iter()
                    .max_by_key(|cover| cover.size as u8)
                    .map(|cover| cover.url.clone());

                let mut xesam: XesamMetadata = audio_item.unique_fields.into();
                xesam.title = Some(audio_item.name);

                let metadata = Metadata {
                    mpris: MprisMetadata {
                        track_id: Some(audio_item.track_id),
                        length: Some(audio_item.duration_ms as i64 * 1000),
                        art_url,
                    },
                    xesam,
                };

                let mut iface = iface_ref.get_mut().await;
                set_metadata(&mut iface, emitter, metadata).await?;
            }
            PlayerEvent::Stopped { track_id, .. } => {
                let mut iface = iface_ref.get_mut().await;

                iface.position = None;
                match track_id {
                    Some(track_id) => ensure_track(&mut iface, emitter, track_id).await?,
                    None if iface.metadata.mpris.track_id.is_some() => {
                        set_metadata(&mut iface, emitter, Metadata::default()).await?
                    }
                    None => (),
                }
                set_playback_status(&mut iface, emitter, PlaybackStatus::Stopped).await?;
            }
            PlayerEvent::Playing {
                track_id,
                position_ms,
                ..
            } => {
                let mut iface = iface_ref.get_mut().await;

                iface.position = Some(Position::from(position_ms));
                ensure_track(&mut iface, emitter, track_id).await?;
                set_playback_status(&mut iface, emitter, PlaybackStatus::Playing).await?;
            }
            PlayerEvent::Paused {
                track_id,
                position_ms,
                ..
            } => {
                let mut iface = iface_ref.get_mut().await;

                iface.position = Some(Position::from(position_ms));
                ensure_track(&mut iface, emitter, track_id).await?;
                set_playback_status(&mut iface, emitter, PlaybackStatus::Paused).await?;
            }
            PlayerEvent::EndOfTrack { track_id, .. } => {
                let mut iface = iface_ref.get_mut().await;

                ensure_track(&mut iface, emitter, track_id).await?;
                iface.position = iface
                    .metadata
                    .mpris
                    .length
                    .map(|length_us| Position::from((length_us / 1000) as u32));
            }
            PlayerEvent::VolumeChanged { volume } => {
                let mut iface = iface_ref.get_mut().await;
                if iface.volume != volume {
                    iface.volume = volume;
                    iface.volume_changed(emitter).await?;
                }
            }
            // Both are inconsistent with the playback rate, which is what the `Seeked` signal is
            // meant for.
            PlayerEvent::Seeked {
                track_id,
                position_ms,
                ..
            }
            | PlayerEvent::PositionCorrection {
                track_id,
                position_ms,
                ..
            } => {
                let mut iface = iface_ref.get_mut().await;

                iface.position = Some(Position::from(position_ms));
                ensure_track(&mut iface, emitter, track_id).await?;
                MprisPlayerService::seeked(emitter, position_ms as i64 * 1000).await?;
            }
            PlayerEvent::ShuffleChanged { shuffle } => {
                let mut iface = iface_ref.get_mut().await;
                if iface.shuffle != shuffle {
                    iface.shuffle = shuffle;
                    iface.shuffle_changed(emitter).await?;
                }
            }
            PlayerEvent::RepeatChanged { context, track } => {
                let repeat = if track {
                    LoopStatus::Track
                } else if context {
                    LoopStatus::Playlist
                } else {
                    LoopStatus::None
                };

                let mut iface = iface_ref.get_mut().await;
                if iface.repeat != repeat {
                    iface.repeat = repeat;
                    iface.loop_status_changed(emitter).await?;
                }
            }
            // Clients extrapolate the position from the playback rate, and get notified of any
            // inconsistency through the `Seeked` signal, so regular position updates are useless.
            PlayerEvent::PositionChanged { .. }
            | PlayerEvent::PlayRequestIdChanged { .. }
            | PlayerEvent::Loading { .. }
            | PlayerEvent::Preloading { .. }
            | PlayerEvent::TimeToPreloadNextTrack { .. }
            | PlayerEvent::Unavailable { .. }
            | PlayerEvent::SessionConnected { .. }
            | PlayerEvent::SessionDisconnected { .. }
            | PlayerEvent::SessionClientChanged { .. }
            | PlayerEvent::AutoPlayChanged { .. }
            | PlayerEvent::FilterExplicitContentChanged { .. }
            | PlayerEvent::SetQueue { .. } => {}
        }

        Ok(())
    }
}

/// Replaces the metadata and notifies the properties which depend on it.
async fn set_metadata(
    iface: &mut MprisPlayerService,
    emitter: &SignalEmitter<'_>,
    metadata: Metadata,
) -> zbus::Result<()> {
    let had_track = iface.metadata.mpris.track_id.is_some();
    iface.metadata = metadata;
    iface.metadata_changed(emitter).await?;

    if had_track != iface.metadata.mpris.track_id.is_some() {
        iface.can_play_changed(emitter).await?;
        iface.can_pause_changed(emitter).await?;
    }

    Ok(())
}

/// Makes sure the metadata matches the given track, in case its `TrackChanged` event was missed.
async fn ensure_track(
    iface: &mut MprisPlayerService,
    emitter: &SignalEmitter<'_>,
    track_id: SpotifyUri,
) -> zbus::Result<()> {
    if iface.metadata.mpris.track_id.as_ref() == Some(&track_id) {
        return Ok(());
    }

    warn!("Missed TrackChanged event, metadata missing");
    let mut metadata = Metadata::default();
    metadata.mpris.track_id = Some(track_id);
    set_metadata(iface, emitter, metadata).await
}

async fn set_playback_status(
    iface: &mut MprisPlayerService,
    emitter: &SignalEmitter<'_>,
    playback_status: PlaybackStatus,
) -> zbus::Result<()> {
    if iface.playback_status != playback_status {
        iface.playback_status = playback_status;
        iface.playback_status_changed(emitter).await?;
    }

    Ok(())
}
