use librespot_core::config::DeviceType;
use librespot_core::spclient::SpClientResult;
use librespot_core::{version, Session};
use librespot_protocol::connect::{
    Capabilities, Device, DeviceInfo, MemberType, PutStateReason, PutStateRequest,
};
use librespot_protocol::player::{ContextPlayerOptions, PlayOrigin, PlayerState, Suppressions};
use protobuf::{EnumOrUnknown, MessageField};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct ConnectStateConfig {
    pub initial_volume: u32,
    pub name: String,
    pub device_type: DeviceType,
    pub zeroconf_enabled: bool,
    pub volume_steps: i32,
    pub hidden: bool,
    pub is_group: bool,
}

impl Default for ConnectStateConfig {
    fn default() -> Self {
        Self {
            initial_volume: u32::from(u16::MAX) / 2,
            name: "librespot".to_string(),
            device_type: DeviceType::Speaker,
            zeroconf_enabled: false,
            volume_steps: 64,
            hidden: false,
            is_group: false,
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct ConnectState {
    pub active: bool,
    pub active_since: Option<SystemTime>,

    pub has_been_playing_for: Option<Instant>,

    pub device: DeviceInfo,
    pub player: PlayerState,

    pub tracks: Vec<()>,

    pub last_command: Option<(u32, String)>,
}

impl ConnectState {
    pub fn new(cfg: ConnectStateConfig, session: &Session) -> Self {
        let mut state = Self {
            device: DeviceInfo {
                can_play: true,
                volume: cfg.initial_volume,
                name: cfg.name,
                device_id: session.device_id().to_string(),
                device_type: EnumOrUnknown::new(cfg.device_type.into()),
                device_software_version: version::SEMVER.to_string(),
                client_id: session.client_id(),
                spirc_version: "3.2.6".to_string(),
                is_group: cfg.is_group,
                capabilities: MessageField::some(Capabilities {
                    volume_steps: cfg.volume_steps,
                    hidden: cfg.hidden,
                    gaia_eq_connect_id: true,
                    can_be_player: true,

                    needs_full_player_state: true,

                    is_observable: true,
                    is_controllable: true,

                    supports_logout: cfg.zeroconf_enabled,
                    supported_types: vec!["audio/episode".to_string(), "audio/track".to_string()],
                    supports_playlist_v2: true,
                    supports_transfer_command: true,
                    supports_command_request: true,
                    supports_gzip_pushes: true,
                    supports_set_options_command: true,

                    is_voice_enabled: false,
                    restrict_to_local: false,
                    disable_volume: false,
                    connect_disabled: false,
                    supports_rename: false,
                    supports_external_episodes: false,
                    supports_set_backend_metadata: false, // TODO: impl
                    supports_hifi: MessageField::none(),

                    command_acks: true,
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        };
        state.reset();
        state
    }

    fn reset(&mut self) {
        self.active = false;
        self.active_since = None;
        self.player = PlayerState {
            is_system_initiated: true,
            playback_speed: 1.,
            play_origin: MessageField::some(PlayOrigin::new()),
            suppressions: MessageField::some(Suppressions::new()),
            options: MessageField::some(ContextPlayerOptions::new()),
            ..Default::default()
        }
    }

    pub async fn update_remote(&self, session: &Session, reason: PutStateReason) -> SpClientResult {
        if matches!(reason, PutStateReason::BECAME_INACTIVE) {
            todo!("handle became inactive")
        }

        let now = SystemTime::now();
        let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let client_side_timestamp = u64::try_from(since_the_epoch.as_millis())?;

        let member_type = EnumOrUnknown::new(MemberType::CONNECT_STATE);
        let put_state_reason = EnumOrUnknown::new(reason);

        let state = self.clone();

        let is_active = state.active;
        let device = MessageField::some(Device {
            device_info: MessageField::some(state.device),
            player_state: MessageField::some(state.player),
            ..Default::default()
        });

        let mut put_state = PutStateRequest {
            client_side_timestamp,
            member_type,
            put_state_reason,
            is_active,
            device,
            ..Default::default()
        };

        if let Some(has_been_playing_for) = state.has_been_playing_for {
            match has_been_playing_for.elapsed().as_millis().try_into() {
                Ok(ms) => put_state.has_been_playing_for_ms = ms,
                Err(why) => warn!("couldn't update has been playing for because {why}"),
            }
        }

        if let Some(active_since) = state.active_since {
            if let Ok(active_since_duration) = active_since.duration_since(UNIX_EPOCH) {
                match active_since_duration.as_millis().try_into() {
                    Ok(active_since_ms) => put_state.started_playing_at = active_since_ms,
                    Err(why) => warn!("couldn't update active since because {why}"),
                }
            }
        }

        if let Some((message_id, device_id)) = state.last_command {
            put_state.last_command_message_id = message_id;
            put_state.last_command_sent_by_device_id = device_id;
        }

        session
            .spclient()
            .put_connect_state_request(put_state)
            .await
    }
}
