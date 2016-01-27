use eventual::Async;
use protobuf::{self, Message};

use util;
use session::Session;
use util::SpotifyId;
use util::version::version_string;
use mercury::{MercuryRequest, MercuryMethod};
use player::{Player, PlayerState};

use std::sync::{Mutex, Arc};

use protocol;
pub use protocol::spirc::PlayStatus;

pub struct SpircManager(Arc<Mutex<SpircInternal>>);

struct SpircInternal {
    player: Player,
    session: Session,

    seq_nr: u32,

    name: String,
    ident: String,
    device_type: u8,
    can_play: bool,

    repeat: bool,
    shuffle: bool,

    is_active: bool,
    became_active_at: i64,

    last_command_ident: String,
    last_command_msgid: u32,

    tracks: Vec<SpotifyId>,
    index: u32,
}

impl SpircManager {
    pub fn new(session: Session, player: Player) -> SpircManager {
        let ident = session.device_id();
        let name = session.config().device_name.clone();

        SpircManager(Arc::new(Mutex::new(SpircInternal {
            player: player,
            session: session,

            seq_nr: 0,

            name: name,
            ident: ident,
            device_type: 5,
            can_play: true,

            repeat: false,
            shuffle: false,

            is_active: false,
            became_active_at: 0,

            last_command_ident: String::new(),
            last_command_msgid: 0,

            tracks: Vec::new(),
            index: 0,
        })))
    }

    pub fn run(&mut self) {
        let rx = {
            let mut internal = self.0.lock().unwrap();

            let rx = internal.session.mercury_sub(internal.uri());

            internal.notify(true, None);

            // Use a weak pointer to avoid creating an Rc cycle between the player and the
            // SpircManager
            let _self = Arc::downgrade(&self.0);
            internal.player.add_observer(Box::new(move |state| {
                if let Some(_self) = _self.upgrade() {
                    let mut internal = _self.lock().unwrap();
                    internal.on_update(state);
                }
            }));

            rx
        };

        for pkt in rx {
            let data = pkt.payload.first().unwrap();
            let frame = protobuf::parse_from_bytes::<protocol::spirc::Frame>(data).unwrap();

            println!("{:?} {} {} {} {}",
                     frame.get_typ(),
                     frame.get_device_state().get_name(),
                     frame.get_ident(),
                     frame.get_seq_nr(),
                     frame.get_state_update_id());

            self.0.lock().unwrap().handle(frame);
        }
    }
}

impl SpircInternal {
    fn on_update(&mut self, player_state: &PlayerState) {
        let end_of_track = player_state.end_of_track();
        if end_of_track {
            self.index = (self.index + 1) % self.tracks.len() as u32;
            let track = self.tracks[self.index as usize];
            self.player.load(track, true, 0);
        } else {
            self.notify_with_player_state(false, None, player_state);
        }
    }

    fn handle(&mut self, frame: protocol::spirc::Frame) {
        if frame.get_ident() == self.ident ||
           (frame.get_recipient().len() > 0 && !frame.get_recipient().contains(&self.ident)) {
            return;
        }

        if frame.get_recipient().len() > 0 {
            self.last_command_ident = frame.get_ident().to_owned();
            self.last_command_msgid = frame.get_seq_nr();
        }
        match frame.get_typ() {
            protocol::spirc::MessageType::kMessageTypeHello => {
                self.notify(false, Some(frame.get_ident()));
            }
            protocol::spirc::MessageType::kMessageTypeLoad => {
                if !self.is_active {
                    self.is_active = true;
                    self.became_active_at = util::now_ms();
                }

                self.reload_tracks(&frame);

                let play = frame.get_state().get_status() == PlayStatus::kPlayStatusPlay;
                let track = self.tracks[self.index as usize];
                let position = frame.get_state().get_position_ms();
                self.player.load(track, play, position);
            }
            protocol::spirc::MessageType::kMessageTypePlay => {
                self.player.play();
            }
            protocol::spirc::MessageType::kMessageTypePause => {
                self.player.pause();
            }
            protocol::spirc::MessageType::kMessageTypeNext => {
                self.index = (self.index + 1) % self.tracks.len() as u32;
                let track = self.tracks[self.index as usize];
                self.player.load(track, true, 0);
            }
            protocol::spirc::MessageType::kMessageTypePrev => {
                self.index = (self.index - 1) % self.tracks.len() as u32;
                let track = self.tracks[self.index as usize];
                self.player.load(track, true, 0);
            }
            protocol::spirc::MessageType::kMessageTypeSeek => {
                self.player.seek(frame.get_position());
            }
            protocol::spirc::MessageType::kMessageTypeReplace => {
                self.reload_tracks(&frame);
            }
            protocol::spirc::MessageType::kMessageTypeNotify => {
                if self.is_active && frame.get_device_state().get_is_active() {
                    self.is_active = false;
                    self.player.stop();
                }
            }
            protocol::spirc::MessageType::kMessageTypeVolume => {
                self.player.volume(frame.get_volume() as u16);
            }
            _ => (),
        }
    }

    fn reload_tracks(&mut self, ref frame: &protocol::spirc::Frame) {
        self.index = frame.get_state().get_playing_track_index();
        self.tracks = frame.get_state()
                           .get_track()
                           .iter()
                           .filter(|track| track.has_gid())
                           .map(|track| SpotifyId::from_raw(track.get_gid()))
                           .collect();
    }

    // FIXME: this entire function is duplicated in notify_with_player_state, but the borrow
    // checker makes it hard to refactor
    fn notify(&mut self, hello: bool, recipient: Option<&str>) {
        let player_state = self.player.state();

        let mut pkt = protobuf_init!(protocol::spirc::Frame::new(), {
            version: 1,
            ident: self.ident.clone(),
            protocol_version: "2.0.0".to_owned(),
            seq_nr: { self.seq_nr += 1; self.seq_nr  },
            typ: if hello {
                protocol::spirc::MessageType::kMessageTypeHello
            } else {
                protocol::spirc::MessageType::kMessageTypeNotify
            },

            device_state: self.device_state(&player_state),
            recipient: protobuf::RepeatedField::from_vec(
                recipient.map(|r| vec![r.to_owned()] ).unwrap_or(vec![])
            ),
            state_update_id: player_state.update_time() as i64
        });

        if self.is_active {
            pkt.set_state(self.spirc_state(&player_state));
        }

        self.session
            .mercury(MercuryRequest {
                method: MercuryMethod::SEND,
                uri: self.uri(),
                content_type: None,
                payload: vec![pkt.write_to_bytes().unwrap()],
            })
            .await()
            .unwrap();
    }

    fn notify_with_player_state(&mut self,
                                hello: bool,
                                recipient: Option<&str>,
                                player_state: &PlayerState) {
        let mut pkt = protobuf_init!(protocol::spirc::Frame::new(), {
            version: 1,
            ident: self.ident.clone(),
            protocol_version: "2.0.0".to_owned(),
            seq_nr: { self.seq_nr += 1; self.seq_nr  },
            typ: if hello {
                protocol::spirc::MessageType::kMessageTypeHello
            } else {
                protocol::spirc::MessageType::kMessageTypeNotify
            },

            device_state: self.device_state(&player_state),
            recipient: protobuf::RepeatedField::from_vec(
                recipient.map(|r| vec![r.to_owned()] ).unwrap_or(vec![])
            ),
            state_update_id: player_state.update_time() as i64
        });

        if self.is_active {
            pkt.set_state(self.spirc_state(&player_state));
        }

        self.session
            .mercury(MercuryRequest {
                method: MercuryMethod::SEND,
                uri: self.uri(),
                content_type: None,
                payload: vec![pkt.write_to_bytes().unwrap()],
            })
            .fire();
    }

    fn spirc_state(&self, player_state: &PlayerState) -> protocol::spirc::State {
        let (position_ms, position_measured_at) = player_state.position();

        protobuf_init!(protocol::spirc::State::new(), {
            status: player_state.status(),
            position_ms: position_ms,
            position_measured_at: position_measured_at as u64,

            playing_track_index: self.index,
            track: self.tracks.iter().map(|track| {
                protobuf_init!(protocol::spirc::TrackRef::new(), {
                    gid: track.to_raw().to_vec()
                })
            }).collect(),

            shuffle: self.shuffle,
            repeat: self.repeat,

            playing_from_fallback: true,

            last_command_ident: self.last_command_ident.clone(),
            last_command_msgid: self.last_command_msgid
        })
    }

    fn device_state(&self, player_state: &PlayerState) -> protocol::spirc::DeviceState {
        protobuf_init!(protocol::spirc::DeviceState::new(), {
            sw_version: version_string(),
            is_active: self.is_active,
            can_play: self.can_play,
            volume: player_state.volume() as u32,
            name: self.name.clone(),
            error_code: 0,
            became_active_at: if self.is_active { self.became_active_at as i64 } else { 0 },
            capabilities => [
                @{
                    typ: protocol::spirc::CapabilityType::kCanBePlayer,
                    intValue => [0]
                },
                @{
                    typ: protocol::spirc::CapabilityType::kDeviceType,
                    intValue => [ self.device_type as i64 ]
                },
                @{
                    typ: protocol::spirc::CapabilityType::kGaiaEqConnectId,
                    intValue => [1]
                },
                @{
                    typ: protocol::spirc::CapabilityType::kSupportsLogout,
                    intValue => [0]
                },
                @{
                    typ: protocol::spirc::CapabilityType::kIsObservable,
                    intValue => [1]
                },
                @{
                    typ: protocol::spirc::CapabilityType::kVolumeSteps,
                    intValue => [10]
                },
                @{
                    typ: protocol::spirc::CapabilityType::kSupportedContexts,
                    stringValue => [
                        "album".to_owned(),
                        "playlist".to_owned(),
                        "search".to_owned(),
                        "inbox".to_owned(),
                        "toplist".to_owned(),
                        "starred".to_owned(),
                        "publishedstarred".to_owned(),
                        "track".to_owned(),
                    ]
                },
                @{
                    typ: protocol::spirc::CapabilityType::kSupportedTypes,
                    stringValue => [
                        "audio/local".to_owned(),
                        "audio/track".to_owned(),
                        "local".to_owned(),
                        "track".to_owned(),
                    ]
                }
            ],
        })
    }

    fn uri(&self) -> String {
        format!("hm://remote/user/{}", self.session.username())
    }
}
